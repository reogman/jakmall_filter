pub mod filter;
pub mod filter_builder;
pub mod traits;

mod filter_applier;

pub use filter::Filter;
pub use filter_builder::FilterBuilder;

#[cfg(test)]
mod tests {
    use crate::Filter;

    #[tokio::test]
    async fn initial_test() {
        let fbuilder = Filter::builder().build_and_init().await;

        assert!(fbuilder.is_ok())
    }

    #[tokio::test]
    async fn load_from_file() {
        let filter = Filter::load_from_file("filter/default_filter.toml").await;
        assert!(filter.is_ok())
    }
}
