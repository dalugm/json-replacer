mod parse;

use anyhow::Result;
use clap::{Args, Parser};

use parse::{payload::parse_payload, reference::parse_reference, response::parse_resposne};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub inclusive: Inclusive,

    /// Path to reference file, should be an oa id hashmap file
    pub reference_file: String,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = true)]
pub struct Inclusive {
    /// Path to payload file
    #[arg(short, long, value_name = "PAYLOAD_JSON_FILE")]
    pub payload: Option<String>,

    /// Path to response file
    #[arg(short, long, value_name = "RESPONSE_JSON_FILE")]
    pub response: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    /// Path to reference file, should be a oa id hashmap file
    reference_file_path: String,
    response_file_path: Option<String>,
    payload_file_path: Option<String>,
}

impl Config {
    pub fn new(args: Cli) -> Self {
        Config {
            reference_file_path: args.reference_file,
            response_file_path: args.inclusive.response,
            payload_file_path: args.inclusive.payload,
        }
    }
}

fn pretty_print(title: &str, count: usize) {
    println!("\n{:#^width$}\n", format!(" {title} "), width = count);
}

pub fn run(config: Config) -> Result<()> {
    let oa_id_hashmap = parse_reference(config.reference_file_path)?;

    if let Some(file_path) = config.response_file_path {
        let object_entities = parse_resposne(file_path, &oa_id_hashmap)?;

        pretty_print("response", 80);
        println!("{object_entities:#?}");
    };

    if let Some(file_path) = config.payload_file_path {
        let parsed_payload = parse_payload(file_path, &oa_id_hashmap)?;

        pretty_print("payload", 80);
        println!("{parsed_payload:#?}");
    };

    Ok(())
}
