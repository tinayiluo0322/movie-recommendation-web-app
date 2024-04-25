// qdrant collections setup --neccessary to set up a collection
// modified https://github.com/qdrant/internal-examples/blob/master/lambda-search/src/setup_collection.rs
//! This program can be used to set up the collection in Qdrant.
//! `cargo run --release --bin setup_collection`

use qdrant_client::prelude::*;
use qdrant_client::qdrant::{
    value::Kind, vectors_config::Config, CreateCollection, Distance, FieldType, PointId,
    PointStruct, Value, VectorParams, Vectors, VectorsConfig,
};
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsBuilder;
use serde::Deserialize;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = SentenceEmbeddingsBuilder::local("model/")
        .with_device(tch::Device::cuda_if_available())
        .create_model()?;
    let collection_name: String = "cloudfinal".to_string();
    let qdrant_uri = std::env::var("QDRANT_URI").expect("need QDRANT_URI set");
    let mut config = QdrantClientConfig::from_url(&qdrant_uri);
    config.api_key = std::env::var("QDRANT_API_KEY").ok();
    let qdrant_client = QdrantClient::new(Some(config)).expect("Failed to connect to Qdrant");

    if !qdrant_client.collection_exists(&collection_name).await? {
        qdrant_client
            .create_collection(&CreateCollection {
                collection_name: collection_name.clone(),
                vectors_config: Some(VectorsConfig {
                    config: Some(Config::Params(VectorParams {
                        size: 384,
                        distance: Distance::Cosine as i32,
                        ..Default::default()
                    })),
                }),
                ..Default::default()
            })
            .await?;
    }
    let file = std::env::args()
        .nth(1)
        .expect("Needs the JSONL file in the first argument");
    let mut points: Vec<PointStruct> = Vec::new();
    let abstracts = File::open(&file).expect("couldn't open JSONL");
    let abstracts = BufReader::new(abstracts);
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    let i = &mut 1;
    for line in abstracts.lines() {
        let payload: HashMap<String, Value> = serde_json::from_str(&line?)?;
        let text = payload.get("description");
        let Some(Value {
            kind: Some(Kind::StringValue(text)),
        }) = text
        else {
            panic!("text isn't a string")
        };
        // // test to make sure it works
        // let response = client
        //     .post("https://api.cohere.ai/embed")
        //     .header("Authorization", &format!("Bearer {}", cohere_api_key))
        //     .header("Content-Type", "application/json")
        //     .header("Cohere-Version", "2021-11-08")
        //     .body(format!("{{\"texts\":[\"{}\"],\"model\":\"small\"}}", text))
        //     .send()
        //     .await?;

        // // Debug: Print request parameters
        // println!("Request URL: {}", response.url());
        // println!("Request Headers: {:#?}", response.headers());
        // println!("Request Body: {:?}", response.text().await);

        // // Parse JSON response
        // let CohereResponse {
        //     id,
        //     texts,
        //     embeddings,
        //     meta,
        //     response_type,
        // } = client
        //     .post("https://api.cohere.ai/embed")
        //     .header("Authorization", &format!("Bearer {cohere_api_key}"))
        //     .header("Content-Type", "application/json")
        //     .header("Cohere-Version", "2021-11-08")
        //     .body(format!("{{\"texts\":[\"{}\"],\"model\":\"small\"}}", text))
        //     .send()
        //     .await?
        //     .json()
        //     .await?;
        let embeddings: Vec<Vec<f32>> = model.encode(&[text]).unwrap();

        for e in embeddings {
            points.push(PointStruct {
                id: Some(PointId::from(std::mem::replace(i, *i + 1) as u64)),
                payload: payload.clone(),
                vectors: Some(Vectors::from(e)),
            });

            // write!(stdout, ".")?;

            // write!(stdout, "{}", i)?;
            // removed part where it readded points for some reason
            stdout.flush()?;
        }
    }
    write!("starting upload!");
    qdrant_client
        .upsert_points_batch_blocking(&collection_name, None, points, None, 200)
        .await?;
    Ok(())
}
