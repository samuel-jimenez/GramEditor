use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::Path;

mod after_release;
mod compare_perf;
mod danger;
mod extension_tests;
mod release_nightly;
mod run_bundling;

mod release;
mod run_tests;
mod runners;
mod steps;
mod vars;

#[derive(Parser)]
pub struct GenerateWorkflowArgs {}

pub fn run_workflows(_: GenerateWorkflowArgs) -> Result<()> {
    if !Path::new("crates/tehanu/").is_dir() {
        anyhow::bail!("xtask workflows must be ran from the project root");
    }
    let dir = Path::new(".github/workflows");

    let workflows = vec![
        ("danger.yml", danger::danger()),
        ("run_bundling.yml", run_bundling::run_bundling()),
        ("release_nightly.yml", release_nightly::release_nightly()),
        ("run_tests.yml", run_tests::run_tests()),
        ("release.yml", release::release()),
        ("compare_perf.yml", compare_perf::compare_perf()),
        ("after_release.yml", after_release::after_release()),
        ("extension_tests.yml", extension_tests::extension_tests()),
    ];
    fs::create_dir_all(dir)
        .with_context(|| format!("Failed to create directory: {}", dir.display()))?;

    for (filename, workflow) in workflows {
        let content = workflow
            .to_string()
            .map_err(|e| anyhow::anyhow!("{}: {:?}", filename, e))?;
        let content = format!(
            "# Generated from xtask::workflows::{}\n# Rebuild with `cargo xtask workflows`.\n{}",
            workflow.name.unwrap(),
            content
        );
        let file_path = dir.join(filename);
        fs::write(&file_path, content)?;
    }

    Ok(())
}
