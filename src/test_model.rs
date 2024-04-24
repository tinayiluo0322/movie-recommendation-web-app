use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsBuilder;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use tch::Device;

#[derive(Debug, Deserialize)]
struct MovieRecord {
    #[serde(rename = "description")]
    description: String,
}

fn read_descriptions(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    let mut descriptions = Vec::new();

    for result in rdr.deserialize() {
        let record: MovieRecord = result?;
        descriptions.push(record.description);
    }

    Ok(descriptions)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load descriptions from a CSV file
    let descriptions = read_descriptions("combined_data.csv")?;

    // Setup the model for sentence embeddings
    let model = SentenceEmbeddingsBuilder::local("model/")
        .with_device(Device::cuda_if_available())
        .create_model()?;

    // Compute embeddings and handle potential errors
    let embeddings = model.encode(&descriptions)?;

    // Check if the embeddings Tensor is empty and attempt to print the embedding
    //println!("Embeddings: {:?}", embeddings);

    // Check if the embeddings Vec is not empty and attempt to print the first embedding
    if !embeddings.is_empty() && !embeddings[0].is_empty() {
        // Access the first embedding vector directly, assuming embeddings is not empty and contains at least one Vec<f32>
        let first_embedding = &embeddings[0];
        println!("Embedding for the first description: {:?}", first_embedding);
    } else {
        println!("No embeddings found or unable to retrieve the first embedding.");
    }

    Ok(())
}
