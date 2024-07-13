use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use ureq::Agent;

#[derive(Parser)]
struct Cli {
    token: String,
}

#[derive(Deserialize)]
struct Transactions {
    results: Vec<Transaction>,
}

#[derive(Deserialize)]
struct Transaction {
    transid: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let auth = format!("basic {}", cli.token);

    let agent = Agent::new();
    let transactions: Transactions = agent
        .get("https://api.wigle.net/api/v2/file/transactions")
        .set("authorization", &auth)
        .call()?
        .into_json()?;

    println!("Found {} uploads", transactions.results.len());
    for x in transactions.results {
        let id = x.transid;
        let file = PathBuf::from(format!("{id}.csv"));
        if file.exists() {
            continue;
        }

        println!("Downloading {id}");
        let data = agent
            .get(&format!("https://api.wigle.net/api/v2/file/csv/{id}"))
            .set("authorization", &auth)
            .call()?
            .into_string()?;
        fs::write(file, data)?;
    }

    Ok(())
}
