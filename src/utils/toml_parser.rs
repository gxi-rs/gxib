use std::borrow::BorrowMut;
use std::collections::{HashMap, BTreeMap};
use std::fs::read;
use std::path::Path;

use serde_derive::{Deserialize, Serialize};
use toml::Value;

/// Default version for the gxi crate
const DEFAULT_GXI_VER: &str = "*";

#[derive(Debug, Deserialize, Serialize)]
pub struct CargoToml {
    #[serde(default)]
    dependencies: CargoTomlDeps,
    #[serde(flatten)]
    extra: Value
}

/// \[dependencies\] section of Cargo.toml file
#[derive(Debug, Deserialize, Serialize)]
pub struct CargoTomlDeps {
    #[serde(default)]
    gxi: String,
}

impl Default for CargoTomlDeps {
    fn default() -> Self {
        CargoTomlDeps {
            gxi: String::from(DEFAULT_GXI_VER)
        }
    }
}

pub fn parse_cargo_toml(path: &Path) -> CargoToml {
    let mut cargo_toml: CargoToml = toml::from_slice(
        &read(path).unwrap()
    ).unwrap();

    cargo_toml
}