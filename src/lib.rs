use anyhow::Result;
use clap::{Args, Parser};

use parse::{
    object_entity::parse as parse_entity, payload::parse as parse_payload,
    reference::parse as parse_reference, response::parse as parse_response,
};

pub mod parse;

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

    /// Path to object entity file, or object entity content
    #[arg(short = 'e', long)]
    pub object_entity: Option<String>,

    /// Path to response file, or response content
    #[arg(short, long)]
    pub response: Option<String>,
}

fn pretty_print(title: &str, count: usize) {
    println!("\n{:#^width$}\n", format!(" {title} "), width = count);
}

pub fn run(cli: Cli) -> Result<()> {
    let oa_id_hashmap = parse_reference(cli.reference_file)?;

    if let Some(response) = cli.inclusive.response {
        let object_entities = parse_response(response, &oa_id_hashmap)?;

        pretty_print("response", 80);
        println!("{object_entities:#?}");
    };

    if let Some(payload) = cli.inclusive.payload {
        let parsed_payload = parse_payload(payload, &oa_id_hashmap)?;

        pretty_print("payload", 80);
        println!("{parsed_payload:#?}");
    };

    if let Some(object_entity) = cli.inclusive.object_entity {
        let parsed_entity = parse_entity(object_entity, &oa_id_hashmap)?;

        pretty_print("object entity", 80);
        println!("{parsed_entity:#?}");
    };

    Ok(())
}
