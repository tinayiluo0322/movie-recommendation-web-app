use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::translation::{
    Language, TranslationConfig, TranslationModel, TranslationModelBuilder,
};
use tch::Device;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = TranslationModelBuilder::new()
        .with_device(Device::cuda_if_available())
        .with_model_type(ModelType::Marian)
        .with_source_languages(vec![Language::English])
        .with_target_languages(vec![Language::French])
        .create_model()?;

    let input = ["This is a sentence to be translated"];

    let output = model.translate(&input, None, Language::French)?;

    println!("{:?}", output);

    Ok(())
}
