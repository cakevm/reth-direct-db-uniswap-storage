use crate::utils::wrapped_provider::WrappedProviderFactory;
use alloy::eips::BlockNumberOrTag;
use reth_chainspec::ChainSpecBuilder;
use reth_db::mdbx::DatabaseArguments;
use reth_db::{open_db_read_only, ClientVersion, DatabaseEnv};
use reth_node_ethereum::EthereumNode;
use reth_node_types::NodeTypesWithDBAdapter;
use reth_provider::providers::StaticFileProvider;
use reth_provider::{ProviderError, ProviderFactory, ProviderResult, StateProviderBox, StateProviderFactory};
use std::path::Path;
use std::sync::Arc;

/// Initialize a read-only database provider using the `RETH_DB_PATH` environment variable.
pub fn init_db_read_only_from_env() -> eyre::Result<WrappedProviderFactory> {
    let db_path = std::env::var("RETH_DB_PATH")?;
    let db_path = Path::new(&db_path);
    init_db_read_only(db_path)
}

/// Initialize a read-only database provider using the provided path.
pub fn init_db_read_only(db_path: &Path) -> eyre::Result<WrappedProviderFactory> {
    let db = Arc::new(open_db_read_only(db_path.join("db").as_path(), DatabaseArguments::new(ClientVersion::default()))?);
    let spec = Arc::new(ChainSpecBuilder::mainnet().build());

    let factory = ProviderFactory::<NodeTypesWithDBAdapter<EthereumNode, Arc<DatabaseEnv>>>::new(
        db.clone(),
        spec.clone(),
        StaticFileProvider::read_only(db_path.join("static_files"), true)?,
    );

    Ok(WrappedProviderFactory::new(factory))
}

/// Create a state provider using the provided factory and block number or tag.
pub fn state_provider<P: StateProviderFactory>(
    provider_factory: &P,
    block_number_or_tag: &BlockNumberOrTag,
) -> ProviderResult<StateProviderBox> {
    match block_number_or_tag {
        BlockNumberOrTag::Number(block_number) => provider_factory.history_by_block_number(*block_number),
        BlockNumberOrTag::Latest => provider_factory.latest(),
        _ => Err(ProviderError::UnsupportedProvider),
    }
}
