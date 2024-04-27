use actix_files::Files;
use std::{convert::Infallible, io::Write, path::PathBuf};

use actix_web::post;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use qdrant_client::prelude::*;

use qdrant_client::qdrant::Value;
use reqwest::Client;
use serde_json::json;

// use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsBuilder;
// have to use this one as using first one is causing issues
// use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use std::collections::HashMap;
use std::vec;
// top 5
const SEARCH_LIMIT: u64 = 5;

#[derive(serde::Deserialize)]
struct MovieDescription {
    description: String,
}

#[derive(Debug, serde::Deserialize)]
struct CohereResponse {
    id: String,
    texts: Vec<String>,
    embeddings: Vec<Vec<f32>>,
    meta: Meta,
    response_type: String,
}
#[derive(Debug, serde::Deserialize)]
struct Meta {
    api_version: ApiVersion,
    billed_units: BilledUnits,
}

#[derive(Debug, serde::Deserialize)]
struct ApiVersion {
    version: String,
}

#[derive(Debug, serde::Deserialize)]
struct BilledUnits {
    input_tokens: u32,
}

#[post("/movie")]
async fn movie(description: web::Form<MovieDescription>) -> impl Responder {
    let query = description.description.clone();
    let message = match infer(query).await {
        Ok(inference_result) => {
            let mut message = String::new();
            message.push_str("<html><head>");
            message.push_str("<style>");
            message.push_str("body { background-color: navy; color: black; font-family: Arial, sans-serif; display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; }");
            message.push_str(".container { display: flex; flex-direction: column; align-items: center; text-align: center; }");
            message.push_str(".title { font-size: 24px; margin-bottom: 20px; }");
            message.push_str(".table-image-container { display: flex; flex-direction: row; align-items: flex-start; justify-content: center; margin-bottom: 20px; }");
            message
                .push_str("table { border-collapse: collapse; width: 50%; margin-right: 20px; }");
            message.push_str(
                "th, td { padding: 8px; text-align: left; border-bottom: 1px solid #ddd; }",
            );
            message.push_str("th { background-color: #333; color: white; }");
            message.push_str(".image-container img { max-width: 200px; }");
            message.push_str(".go-back-button { font-size: 16px; padding: 10px 20px; background-color: #ccc; border: none; cursor: pointer; }");
            message.push_str("</style>");
            message.push_str("</head><body>");

            // Title
            message.push_str("<div class='container'>");
            message.push_str("<div class='title'>You are watching</div>");

            // Table and Image container
            message.push_str("<div class='table-image-container'>");

            // Table container
            message.push_str("<table>");
            message.push_str("<tr><th>Field</th><th>Value</th></tr>");

            // Display Movie Name first
            if let Some(movie_name) = inference_result.get("movie_name") {
                message.push_str(&format!(
                    "<tr><td><strong>Movie Name</strong></td><td>{}</td></tr>",
                    movie_name
                ));
            }

            // Display other fields with non-empty values
            for (key, value) in inference_result {
                // Skip if value is empty or "N/A"
                if let Some(value_str) = value.as_str() {
                    if value_str.trim().is_empty() || value_str == "N/A" {
                        continue;
                    }
                }
                // Skip if key is "movie_name" since it's already displayed
                if key != "movie_name" {
                    let field_description = match key.as_str() {
                        "description" => "Description",
                        "rating" => "Rating",
                        "movie_id" => "IMDb ID",
                        "votes" => "Votes",
                        "runtime" => "Runtime",
                        "director" => "Director",
                        "gross(in $)" => "Gross (in $)",
                        "Genre" => "Genre",
                        "certificate" => "Certificate",
                        "year" => "Year",
                        "star" => "Main Stars",
                        _ => "Unknown",
                    };
                    let value = value.as_str().unwrap().replace("\n", "<br>");
                    message.push_str(&format!(
                        "<tr><td><strong>{}</strong> ({})</td><td>{}</td></tr>",
                        field_description, key, value
                    ));
                }
            }

            message.push_str("</table>");

            // Image container
            message.push_str("<div class='image-container'>");
            message.push_str("<img src='/imgs/Movie_Time.jpeg' alt='Movie Image'>");
            message.push_str("</div>"); // End of image container

            message.push_str("</div>"); // End of table-image-container

            // Background Image
            message.push_str("<img src='/imgs/jj.jpg' alt='Movie Background Image' style='width: 100%; height: 100%; position: fixed; top: 0; left: 0; z-index: -1;'>");

            // Go back button
            message.push_str(
                "<button class='go-back-button' onclick='history.back()'>Go Back</button>",
            );

            message.push_str("</div>"); // End of main container

            message.push_str("</body></html>");
            message
        }
        Err(err) => {
            format!("Error in inference: {:?}", err)
        }
    };

    HttpResponse::Ok().content_type("text/html").body(message)
}

#[post("/generate-text")]
async fn generate_text(description: web::Json<MovieDescription>) -> impl Responder {
    let p = "Make a story for: ";
    let query = description.description.clone();
    let full = p.to_owned() + &query;

    println!("{:?}", query);
    // do infer now
    let message = match infer2(full.to_string()).await {
        Ok(inference_result) => {
            format!("{}", inference_result)
        }
        Err(err) => {
            format!("Error in inference: {:?}", err)
        }
    };
    let filtered_chars: String = message.chars().filter(|&c| c != '\x08').collect();
    let cleaned_text: String = filtered_chars
        .chars()
        .filter(|&c| c.is_ascii() && c != '\u{FFFD}') // Filter out non-ASCII and replacement character
        .collect();
    // let cleaned_text_without_full = cleaned_text.replace(&full, "");

    HttpResponse::Ok().body(cleaned_text)
}

async fn infer(prompt: String) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
    let collection_name: String = "cloudfinal".to_string();
    let qdrant_uri = std::env::var("QDRANT_URI").expect("need QDRANT_URI set");
    let mut config = QdrantClientConfig::from_url(&qdrant_uri);
    config.api_key = std::env::var("QDRANT_API_KEY").ok();
    let qdrant_client = QdrantClient::new(Some(config)).expect("Failed to connect to Qdrant");

    // local model
    // let model = SentenceEmbeddingsBuilder::local("model/").create_model()?;
    // let vector = model.encode(&[prompt])?.into_iter().next().unwrap();
    // pre-trained downloadable model
    // let model = TextEmbedding::try_new(InitOptions {
    //     model_name: EmbeddingModel::AllMiniLML6V2,
    //     show_download_progress: true,
    //     ..Default::default()
    // })?;
    // let documents = vec![prompt];
    // let vectors = model.embed(documents, None)?;
    // let vector = vectors[0].clone();

    // none of the above can be dosckerized moving to cohere

    let client = Client::builder().build().unwrap();

    let cohere_api_key = std::env::var("COHERE_API_KEY").expect("need COHERE_API_KEY set");
    let mut cohere_response = CohereResponse {
        id: String::new(),
        texts: Vec::new(),
        embeddings: Vec::new(),
        meta: Meta {
            api_version: ApiVersion {
                version: String::new(),
            },
            billed_units: BilledUnits { input_tokens: 42 },
        },
        response_type: String::new(),
    };
    // let response = client
    //     .post("https://api.cohere.ai/embed")
    //     .header("Authorization", format!("Bearer {}", cohere_api_key))
    //     .header("Content-Type", "application/json")
    //     .header("Cohere-Version", "2021-11-08")
    //     .json(&json!({
    //         "texts": ["test"],
    //         "model": "embed-english-light-v3.0",
    //         "input_type": "classification"
    //     }))
    //     .send()
    //     .await?;

    // // Debug: Print request parameters
    // println!("Request URL: {}", response.url());
    // println!("Request Headers: {:#?}", response.headers());

    // // Extract and print response body
    // let response_body = response.text().await?;
    // println!("Response Body: {:?}", response_body);

    cohere_response = client
        .post("https://api.cohere.ai/embed")
        .header("Authorization", &format!("Bearer {cohere_api_key}"))
        .header("Content-Type", "application/json")
        .header("Cohere-Version", "2021-11-08")
        .json(&json!({
            "texts": [prompt],
            "model": "embed-english-light-v3.0",
            "input_type": "classification"
        }))
        .send()
        .await?
        .json()
        .await?;

    let vector = cohere_response
        .embeddings
        .into_iter()
        .next()
        .ok_or("Empty output from embedding")?;

    // query qdrant now
    let result = qdrant_client
        .search_points(&SearchPoints {
            collection_name: collection_name,
            vector,
            limit: SEARCH_LIMIT,
            with_payload: Some(true.into()),
            ..Default::default()
        })
        .await?;

    let found_point = result.result.into_iter().next().unwrap();
    let payload = found_point.payload;
    print!("{:?}", payload);
    Ok(payload)
}

// for Kahila
async fn infer2(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    let tokenizer_source = llm::TokenizerSource::Embedded;
    let model_architecture = llm::ModelArchitecture::Llama;
    // for local run
    let model_path = PathBuf::from("static/open_llama_3b-q4_0-ggjt.bin");
    // for deployment
    //let model_path = PathBuf::from("/new-lambda-project/pythia-1b-q4_0-ggjt.bin");

    let prompt = prompt.to_string();
    let model = llm::load_dynamic(
        Some(model_architecture),
        &model_path,
        tokenizer_source,
        Default::default(),
        llm::load_progress_callback_stdout,
    )?;

    let mut session = model.start_session(Default::default());
    let mut generated_tokens = String::new();

    let res = session.infer::<Infallible>(
        model.as_ref(),
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: (&prompt).into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            // specify token to use
            maximum_token_count: Some(30),
        },
        // OutputRequest
        &mut Default::default(),
        |r| match r {
            llm::InferenceResponse::PromptToken(t) | llm::InferenceResponse::InferredToken(t) => {
                print!("{t}");
                std::io::stdout().flush().unwrap();
                generated_tokens.push_str(&t);
                Ok(llm::InferenceFeedback::Continue)
            }
            _ => Ok(llm::InferenceFeedback::Continue),
        },
    );

    // Return statement
    match res {
        Ok(_) => Ok(generated_tokens),
        Err(err) => Err(Box::new(err)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(movie)
            .service(generate_text)
            .service(Files::new("/", "./static/root/").index_file("index.html"))
    })
    .bind(("0.0.0.0", 50505))?
    .run()
    .await
}
