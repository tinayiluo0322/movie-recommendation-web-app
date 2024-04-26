use actix_files::Files;
use actix_web::test::ok_service;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use qdrant_client::prelude::*;
use qdrant_client::qdrant::{value::Kind, Struct};
use qdrant_client::qdrant::{
    vectors_config::Config, CreateCollection, Distance, FieldType, PointId, PointStruct, Value,
    VectorParams, Vectors, VectorsConfig,
};
use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsBuilder;
use rust_bert::pipelines::translation::{
    Language, TranslationConfig, TranslationModel, TranslationModelBuilder,
};
use std::collections::HashMap;
use std::time::Duration;
use std::{convert::Infallible, io::Write, path::PathBuf};
use tch::Device;

// top 5
const SEARCH_LIMIT: u64 = 5;

async fn coinflip() -> impl Responder {
    // first do fake query
    let query: String = "detective".to_string();
    // load the model maybe a better place to put this
    // this would need to be changed as I'm outputting an html file but in reality we would
    // jsut show the reuslt to the user on the same page hopefully
    let message = match infer(query.to_string()).await {
        Ok(inference_result) => {
            let mut message = String::new();
            for (key, value) in inference_result {
                message.push_str(&format!("<div>{}: {}</div>", key, value));
            }
            message
        }
        Err(err) => {
            format!("Error in inference: {:?}", err)
        }
    };

    HttpResponse::Ok().body(message)
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
    let mut payload = found_point.payload;
    print!("{:?}", payload);
    Ok(payload)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // have to sure your route goes first before serving other files
            .route("/coinflip", web::get().to(coinflip)) // Route to the coinflip function
            .service(Files::new("/", "./static/root/").index_file("index.html"))
    })
    .bind(("0.0.0.0", 50505))?
    .client_request_timeout(Duration::from_secs(120)) // Set client timeout to 2 minutes
    .run()
    .await
}
