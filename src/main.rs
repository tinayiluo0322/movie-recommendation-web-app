use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::translation::{
    Language, TranslationConfig, TranslationModel, TranslationModelBuilder,
};
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsBuilder;
use tch::Device;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = SentenceEmbeddingsBuilder::local("model/")
    .with_device(tch::Device::cuda_if_available())
    .create_model()?;

    let sentences = ["This is an example sentence", "Each sentence is converted"];
    let embeddings = model.encode(&sentences)?;

    println!("{:?}", embeddings);

    // let model = SentenceEmbeddingsBuilder::remote(
    //     SentenceEmbeddingsModelType::AllMiniLmL12V2
    // ).create_model()?;

    // let sentences = [
    //     "this is an example sentence",
    //     "each sentence is converted"
    // ];

    // let output = model.encode(&sentences);

    // println!("{:?}", output);


    Ok(())
}
