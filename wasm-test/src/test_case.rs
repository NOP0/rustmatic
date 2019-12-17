use anyhow::{Context, Error};
use serde_derive::{Deserialize, Serialize};
use std::{fs, path::Path, time::Duration};

/// A single test case.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub src: String,
    pub recipe: Recipe,
}

impl TestCase {
    pub fn load<P, Q>(src_file: P, recipe_file: Q) -> Result<TestCase, Error>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        let src_file = src_file.as_ref();
        let name =
            src_file
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| {
                    anyhow::format_err!("Unable to determine the filename")
                })?;

        let src = fs::read_to_string(src_file)
            .context("Couldn't read the source file")?;
        let recipe = fs::read_to_string(recipe_file)
            .context("Couldn't read the recipe file")?;

        TestCase::parse(name, src, recipe)
    }

    pub fn parse<N, S, R>(name: N, src: S, recipe: R) -> Result<TestCase, Error>
    where
        N: Into<String>,
        S: Into<String>,
        R: AsRef<str>,
    {
        let name = name.into();
        let src = src.into();
        let recipe = serde_json::from_str(recipe.as_ref())
            .context("Recipe parsing failed")?;

        Ok(TestCase { name, src, recipe })
    }
}

/// A series of snapshots containing inputs and expected outputs for the test
/// program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    pub passes: Vec<Pass>,
}

/// The inputs and expected outputs for a single call to [`Program::poll()`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pass {
    #[serde(with = "humantime_serde")]
    pub elapsed: Duration,
    pub inputs: Vec<u8>,
    pub expected_outputs: Vec<u8>,
    #[serde(default)]
    pub expected_log_messages: Vec<String>,
}
