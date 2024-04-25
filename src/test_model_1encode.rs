// not needed at the moment
// use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsBuilder;
// use csv::ReaderBuilder;
// use serde::Deserialize;
// use std::error::Error;
// use std::fs::File;
// use tch::Device;

// #[derive(Debug, Deserialize)]
// struct MovieRecord {
//     #[serde(rename = "description")]
//     description: String,
// }

// fn read_descriptions(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
//     let file = File::open(file_path)?;
//     let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
//     let mut descriptions = Vec::new();

//     for result in rdr.deserialize() {
//         let record: MovieRecord = result?;
//         descriptions.push(record.description);
//     }

//     Ok(descriptions)
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     // Load descriptions from a CSV file
//     let descriptions = read_descriptions("combined_data.csv")?;

//     // Setup the model for sentence embeddings
//     let model = SentenceEmbeddingsBuilder::local("model/")
//         .with_device(Device::cuda_if_available())
//         .create_model()?;

//     // Check if there are any descriptions to process
//     if let Some(first_description) = descriptions.get(0) {
//         // Compute embeddings for the first description only
//         let first_embedding = model.encode(&[first_description])?;

//         // Print the embedding for the first description
//         println!("Embedding for the first description: {:?}", first_embedding);
//     } else {
//         println!("No descriptions found in the file.");
//     }

//     Ok(())
// }
