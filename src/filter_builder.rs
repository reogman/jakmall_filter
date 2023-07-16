use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;

use crate::filter::Filter;

#[derive(Debug)]
pub struct FilterBuilder {
    folder_location: PathBuf,
    name: String,
    filter: Filter,
}

impl FilterBuilder {
    pub fn set_folder_location<P>(&mut self, location: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.folder_location = location.into();
        self
    }

    pub fn set_name<T>(&mut self, name: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.name = name.into();
        self
    }

    pub fn set_concurrent_number(&mut self, num: usize) -> &mut Self {
        self.filter.concurrent_number = num;
        self
    }

    pub fn set_min_sold(&mut self, min: usize) -> &mut Self {
        self.filter.min_sold = min;
        self
    }

    pub fn set_min_price(&mut self, min: usize) -> &mut Self {
        self.filter.min_price = min;
        self
    }

    pub fn set_max_price(&mut self, max: usize) -> &mut Self {
        self.filter.max_price = max;
        self
    }

    pub fn set_can_single_co(&mut self, can: bool) -> &mut Self {
        self.filter.can_single_co = can;
        self
    }

    pub async fn build_and_init(&self) -> Result<Filter> {
        let content = toml::to_string(&self.filter)?;
        let full_path = &self.folder_location.join(&self.name).with_extension("toml");

        if fs::read(full_path).await.is_err() {
            fs::create_dir_all(&self.folder_location).await?;
            fs::write(
                self.folder_location.join(&self.name).with_extension("toml"),
                &content,
            )
            .await?;
        }

        Ok(self.filter.clone())
    }
}

impl Default for FilterBuilder {
    fn default() -> Self {
        let def_location = PathBuf::new().join("filter");
        let def_name = String::from("default_filter");
        let def_filter = Filter::default();

        FilterBuilder {
            folder_location: def_location,
            name: def_name,
            filter: def_filter,
        }
    }
}
