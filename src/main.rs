use actix_files::Files;

use actix_web::post;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use qdrant_client::prelude::*;

use qdrant_client::qdrant::{
    Value,
};

use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsBuilder;

use std::collections::HashMap;




// top 5
const SEARCH_LIMIT: u64 = 5;

#[derive(serde::Deserialize)]
struct MovieDescription {
    description: String,
}

#[post("/movie")]
async fn movie(description: web::Form<MovieDescription>) -> impl Responder {
    let query = description.description.clone();
    let message = match infer(query).await {
        Ok(inference_result) => {
            let mut message = String::new();
            message.push_str("<html><head>");
            message.push_str("<style>");
            message.push_str("body { background-color: navy; color: white; font-family: Arial, sans-serif; display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; }");
            message.push_str(".container { display: flex; flex-direction: column; align-items: center; text-align: center; }");
            message.push_str(".title { font-size: 24px; margin-bottom: 20px; }");
            message.push_str(".table-image-container { display: flex; flex-direction: row; align-items: flex-start; justify-content: center; margin-bottom: 20px; }");
            message.push_str("table { border-collapse: collapse; width: 50%; margin-right: 20px; }");
            message.push_str("th, td { padding: 8px; text-align: left; border-bottom: 1px solid #ddd; }");
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
            message.push_str("<img src='/imgs/Movie_background.png' alt='Movie Background Image' style='width: 100%; height: 100%; position: fixed; top: 0; left: 0; z-index: -1;'>");

            // Go back button
            message.push_str("<button class='go-back-button' onclick='history.back()'>Go Back</button>");

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


async fn infer(prompt: String) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
    let collection_name: String = "cloudfinal".to_string();
    let qdrant_uri = std::env::var("QDRANT_URI").expect("need QDRANT_URI set");
    let mut config = QdrantClientConfig::from_url(&qdrant_uri);
    config.api_key = std::env::var("QDRANT_API_KEY").ok();
    let qdrant_client = QdrantClient::new(Some(config)).expect("Failed to connect to Qdrant");

    let model = SentenceEmbeddingsBuilder::local("model/")
        .with_device(tch::Device::cuda_if_available())
        .create_model()?;
    let vector = model.encode(&[prompt])?.into_iter().next().unwrap();
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(movie)
            .service(Files::new("/", "./static/root/").index_file("index.html"))
    })
    .bind(("0.0.0.0", 50516))?
    .run()
    .await
}
