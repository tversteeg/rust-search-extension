use std::clone::Clone;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::Result;
use argh::FromArgs;
use serde_derive::Deserialize;
use tokio::runtime::Runtime;

use crate::minify::Minifier;
use crate::tasks::Task;

const LINT_URL: &str = "https://rust-lang.github.io/rust-clippy/master/lints.json";
const LINTS_INDEX_PATH: &str = "../extension/index/lints.js";

/// Lint task
#[argh(subcommand, name = "lints")]
#[derive(FromArgs)]
pub struct LintsTask {
    /// destination path
    #[argh(option, short = 'd', default = "LINTS_INDEX_PATH.to_string()")]
    dest_path: String,
}

#[derive(Deserialize, Debug)]
enum LintLevel {
    Allow,
    Warn,
    Deny,
    Deprecated,
}

impl ToString for LintLevel {
    fn to_string(&self) -> String {
        match self {
            Self::Allow => "Allow".to_string(),
            Self::Warn => "Warn".to_string(),
            Self::Deny => "Deny".to_string(),
            Self::Deprecated => "Deprecated".to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
struct LintDocs {
    #[serde(rename(deserialize = "What it does"))]
    desc: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Lint {
    id: String,
    level: LintLevel,
    docs: LintDocs,
}

async fn fetch_clippy_lints() -> Result<Vec<Lint>> {
    let lints = reqwest::get(LINT_URL).await?.json().await?;
    Ok(lints)
}

impl Task for LintsTask {
    fn execute(&self) -> Result<()> {
        let mut rt = Runtime::new()?;
        rt.block_on(self.run())?;
        Ok(())
    }
}

impl LintsTask {
    async fn run(&self) -> Result<()> {
        let lints: HashMap<String, [String; 2]> = fetch_clippy_lints()
            .await?
            .iter()
            .filter_map(|lint| {
                if let Some(mut desc) = lint.docs.desc.clone() {
                    desc = desc.replace("`", "");
                    desc.truncate(100);
                    Some((lint.id.clone(), [lint.level.to_string(), desc]))
                } else {
                    None
                }
            })
            .collect();

        let contents = format!("var lintsIndex={};", serde_json::to_string(&lints)?);
        let path = Path::new(&self.dest_path);
        fs::write(path, &Minifier::minify_js(contents))?;
        println!("\nGenerate javascript lints index successful!");
        Ok(())
    }
}
