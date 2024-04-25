// not needed for now
// use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsBuilder;
// use csv::ReaderBuilder;
// use serde::{Deserialize, Serialize};
// use std::error::Error;
// use std::fs::File;
// use tch::{no_grad, Device};
// use reqwest::Client;

// #[derive(Debug, Deserialize)]
// struct MovieRecord {
//     #[serde(rename = "movie_id")]
//     movie_id: String,
//     #[serde(rename = "movie_name")]
//     movie_name: String,
//     #[serde(rename = "year")]
//     year: String,
//     #[serde(rename = "certificate")]
//     certificate: String,
//     #[serde(rename = "runtime")]
//     runtime: String,
//     #[serde(rename = "rating")]
//     rating: f64,
//     #[serde(rename = "description")]
//     description: String,
//     #[serde(rename = "director")]
//     director: String,
//     #[serde(rename = "star")]
//     star: String,
//     #[serde(rename = "votes")]
//     votes: f64,
//     #[serde(rename = "gross(in $)")]
//     gross: f64,
//     #[serde(rename = "Genre")]
//     genre: String,
// }

// #[derive(Serialize)]
// struct QdrantPayload {
//     id: String,
//     fields: MovieRecordFields,
// }

// #[derive(Serialize)]
// struct MovieRecordFields {
//     movie_name: String,
//     year: String,
//     certificate: String,
//     runtime: String,
//     rating: f64,
//     director: String,
//     star: String,
//     votes: f64,
//     gross: f64,
//     genre: String,
//     description_embedding: Vec<f32>,
// }

// fn read_movies(file_path: &str) -> Result<Vec<MovieRecord>, Box<dyn Error>> {
//     let file = File::open(file_path)?;
//     let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
//     let mut movies = Vec::new();

//     for result in rdr.deserialize() {
//         let record: MovieRecord = result?;
//         movies.push(record);
//     }

//     Ok(movies)
// }

// async fn store_in_qdrant(client: &Client, movie: &MovieRecord, embedding: Vec<f32>) -> Result<(), Box<dyn Error>> {
//     let collection_name: String = "Movie".to_string();
//     let qdrant_uri = std::env::var("QDRANT_URI").expect("need QDRANT_URI set");
//     let mut config = QdrantClientConfig::from_url(&qdrant_uri);
//     config.api_key = std::env::var("QDRANT_API_KEY").ok();
//     let qdrant_client = QdrantClient::new(Some(config)).expect("Failed to connect to Qdrant");

//     if !qdrant_client.collection_exists(&collection_name).await? {
//         qdrant_client
//             .create_collection(&CreateCollection {
//                 collection_name: collection_name.clone(),
//                 vectors_config: Some(VectorsConfig {
//                     config: Some(Config::Params(VectorParams {
//                         size: 1024,
//                         distance: Distance::Cosine as i32,
//                         ..Default::default()
//                     })),
//                 }),
//                 ..Default::default()
//             })
//             .await?;
//     }

//     let body = QdrantPayload {
//         id: movie.movie_id.clone(),
//         fields: MovieRecordFields {
//             movie_name: movie.movie_name.clone(),
//             year: movie.year.clone(),
//             certificate: movie.certificate.clone(),
//             runtime: movie.runtime.clone(),
//             rating: movie.rating,
//             director: movie.director.clone(),
//             star: movie.star.clone(),
//             votes: movie.votes,
//             gross: movie.gross,
//             genre: movie.genre.clone(),
//             description_embedding: embedding,
//         },
//     };

//     qdrant_client
//         .upsert_points_blocking(&collection_name, None, points, None)
//         .await?;
//     Ok(())
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // Load movie records from CSV
//     let movies = read_movies("combined_data.csv")?;

//     // Setup the model
//     let model = SentenceEmbeddingsBuilder::local("model/")
//         .with_device(Device::cuda_if_available())
//         .create_model()?;

//     let client = Client::new();

//     // Compute embeddings and store them along with other data
// for movie in movies {
//     // Compute embeddings for the movie description and handle potential errors
//     let embeddings = model.encode(&[&movie.description])?;

//     // Assuming you need to convert the embedding Tensor to a Vec<f32> as per your other operations
//     let embeddings_vec: Vec<f32> = embeddings
//         .get(0) // Get the first (and only) embedding from the result
//         .expect("No embeddings returned") // Safely handle the case where no embeddings are returned
//         .to_vec::<f32>() // Convert the tensor to a vector of f32
//         .unwrap(); // Handle or propagate an error in conversion (consider replacing `unwrap` with proper error handling)

//     // Store in Qdrant or handle the result as needed
//     store_in_qdrant(&client, &movie, embeddings_vec).await?;
// }

//     Ok(())
// }
