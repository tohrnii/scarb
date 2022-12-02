use anyhow::Result;

use murek::core::Config;

#[tracing::instrument(skip_all, level = "info")]
pub fn run(_conf: &Config) -> Result<()> {
    println!("Installed commands:");
    todo!("not implemented yet.")
}
