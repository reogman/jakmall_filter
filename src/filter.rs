use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::filter_builder::FilterBuilder;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Filter {
    pub concurrent_number: usize,
    pub min_sold: usize,
    pub min_price: usize,
    pub max_price: usize,
    pub can_single_co: bool,
}

impl Filter {
    pub fn builder() -> FilterBuilder {
        FilterBuilder::default()
    }

    pub async fn load_from_file<P>(path: P) -> Result<Self>
    where
        P: Into<PathBuf>,
    {
        let content = fs::read_to_string(path.into()).await?;
        let filter = toml::from_str::<Filter>(&content)?;

        Ok(filter)
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            concurrent_number: 50,
            min_sold: 1,
            min_price: 10_000,
            max_price: 50_000,
            can_single_co: true,
        }
    }
}
