use std::{fs, path::Path};

use anyhow::Result;
use clap::{Args, Parser};
use json_replacer::{
    preprocess_reference, process_object_entity, process_payload, process_response,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub inclusive: Inclusive,

    /// Path to reference file, should contain object_attributes api response.
    pub reference_file: String,
}

#[derive(Args)]
#[group(required = true, multiple = true)]
pub struct Inclusive {
    /// Path to payload file, or payload content
    #[arg(short, long)]
    pub payload: Option<String>,

    /// Path to response file, or response content
    #[arg(short, long)]
    pub response: Option<String>,

    /// Path to object entity file, or object entity content
    #[arg(short = 'e', long)]
    pub object_entity: Option<String>,
}

fn pretty_print(title: &str, count: usize) {
    println!("\n{:#^width$}\n", format!(" {title} "), width = count);
}

fn get_content(input: String) -> Result<String> {
    let trimmed_input = input.trim();

    // Treat as json str.
    if trimmed_input.starts_with('{') || trimmed_input.starts_with('[') {
        return Ok(input);
    }

    let path = Path::new(&input);
    if path.exists() {
        // Treat as file.
        return fs::read_to_string(path).map_err(anyhow::Error::from);
    }

    Ok(input)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let reference_content = get_content(cli.reference_file)?;
    let reference = serde_json::from_str(&reference_content)?;
    let oa_id_hashmap = preprocess_reference(reference)?;

    if let Some(payload) = cli.inclusive.payload {
        let payload_content = get_content(payload)?;
        let payload = serde_json::from_str(&payload_content)?;
        let result = process_payload(&oa_id_hashmap, payload)?;

        pretty_print("payload", 80);
        println!("{result:#?}");
    }

    if let Some(response) = cli.inclusive.response {
        let response_content = get_content(response)?;
        let response = serde_json::from_str(&response_content)?;
        let result = process_response(&oa_id_hashmap, response)?;

        pretty_print("response", 80);
        println!("{result:#?}");
    }

    if let Some(object_entity) = cli.inclusive.object_entity {
        let entity_content = get_content(object_entity)?;
        let entity = serde_json::from_str(&entity_content)?;
        let result = process_object_entity(&oa_id_hashmap, entity)?;

        pretty_print("entity", 80);
        println!("{result:#?}");
    }

    Ok(())
}
