#![recursion_limit = "256"]

use anyhow::Result;

#[tokio::main]
fn main() -> Result<()> {
    electrs::run().await
}
