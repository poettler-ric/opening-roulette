#![deny(missing_docs)]
#![deny(deprecated)]
#![warn(missing_doc_code_examples)]

//! Picks chess openings at random for you.

use clap::Parser;
use rand::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

/// Picks chess openings at random for you.
#[derive(Parser, Debug)]
struct Cli {
    /// File containing all openings including their weights
    opening_table: String,
}

/// Collections of possible openings with weights
#[derive(Deserialize, Debug)]
struct Openings {
    /// Openings when playing white
    white: HashMap<String, u32>,
    /// Responses to 1.e4
    black_e4: HashMap<String, u32>,
    /// Responses to 1.d4
    black_d4: HashMap<String, u32>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let openings = toml::from_str::<Openings>(&std::fs::read_to_string(cli.opening_table)?)?;

    let mut rng = thread_rng();
    println!(
        "white: {}",
        Vec::from_iter(openings.white.iter())
            .choose_weighted(&mut rng, |item| item.1)
            .expect("no opening for white")
            .0
    );
    println!(
        "against 1.e4: {}",
        Vec::from_iter(openings.black_e4.iter())
            .choose_weighted(&mut rng, |item| item.1)
            .expect("no response for 1.e4")
            .0
    );
    println!(
        "against 1.d4: {}",
        Vec::from_iter(openings.black_d4.iter())
            .choose_weighted(&mut rng, |item| item.1)
            .expect("no response for 1.d4")
            .0
    );

    Ok(())
}
