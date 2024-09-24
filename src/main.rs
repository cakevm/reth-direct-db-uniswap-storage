mod univ2;
mod univ3;

use crate::univ2::read_univ2_pairs;
use alloy_sol_types::SolValue;
use reth_chainspec::ChainSpecBuilder;
use reth_db::mdbx::DatabaseArguments;
use reth_db::{open_db_read_only, ClientVersion, DatabaseEnv};
use reth_node_ethereum::EthereumNode;
use reth_node_types::NodeTypesWithDBAdapter;
use reth_provider::{providers::StaticFileProvider, AccountReader, ProviderFactory, StateProvider};
use std::path::Path;
use std::sync::Arc;

fn main() -> eyre::Result<()> {
    let db_path = std::env::var("RETH_DB_PATH")?;
    let db_path = Path::new(&db_path);

    let db = Arc::new(open_db_read_only(db_path, DatabaseArguments::new(ClientVersion::default()))?);
    let spec = Arc::new(ChainSpecBuilder::mainnet().build());
    let factory = ProviderFactory::<NodeTypesWithDBAdapter<EthereumNode, Arc<DatabaseEnv>>>::new(
        db.clone(),
        spec.clone(),
        StaticFileProvider::read_only(db_path.join("static_files"), false)?,
    );

    /*
    // Read all pairs from UniswapV2Factory
    let pairs = read_univ2_pairs(factory.latest()?)?;
    for pair in pairs.iter().take(3) {
        println!("Pair: {:#?}", pair);
    }
    println!("Total pairs: {}", pairs.len());
    */

    let provider = factory.provider()?;

    // Read all pairs from UniswapV3Factory
    let pairs = univ3::read_univ3_pairs(provider)?;
    for pair in pairs.iter().take(3) {
        println!("Pair: {:#?}", pair);
    }
    println!("Total pairs: {}", pairs.len());

    Ok(())
}
