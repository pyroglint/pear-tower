use anyhow::{Context, Result};
use reth_evm_ethereum::EthEvmConfig;
use reth_network::{
    EthNetworkPrimitives, NetworkConfig, NetworkHandle, NetworkManager, config::SecretKey,
};
use reth_storage_api::noop::NoopProvider;
use reth_tasks::{RuntimeBuilder, RuntimeConfig, TokioConfig};
use reth_transaction_pool::{
    CoinbaseTipOrdering, EthPooledTransaction, EthTransactionValidator, Pool,
    TransactionValidationTaskExecutor,
    blobstore::{InMemoryBlobStore, NoopBlobStore},
};
use tokio::runtime::Handle;

/// TODO: ...
pub type RethTransactionPool = Pool<
    TransactionValidationTaskExecutor<
        EthTransactionValidator<NoopProvider, EthPooledTransaction, EthEvmConfig>,
    >,
    CoinbaseTipOrdering<EthPooledTransaction>,
    InMemoryBlobStore,
>;

/// Reth's P2P stack wrapper.
#[derive(Clone)]
pub struct RethP2P {
    /// Reth transactions pool. See alias [`RethTransactionPool`].
    pub pool: RethTransactionPool,
    _net_handle: NetworkHandle<EthNetworkPrimitives>,
}

/// Configurator for [`RethP2P`].
#[derive(bon::Builder)]
pub struct RethP2PConfig {
    #[builder(default = reth_network::config::rng_secret_key())]
    network_key: SecretKey,
}

impl RethP2P {
    /// Bootstraps the p2p network against the mainnet boot nodes and starts observing gossiped
    /// transactions. Must be called from within a tokio runtime.
    pub async fn new(config: RethP2PConfig) -> Result<Self> {
        let runtime = RuntimeBuilder::new(
            RuntimeConfig::default().with_tokio(TokioConfig::existing_handle(Handle::current())),
        )
        .build()?;

        let client = NoopProvider::default();

        let validator =
            TransactionValidationTaskExecutor::eth_builder(client.clone(), EthEvmConfig::mainnet())
                .build_with_tasks(runtime.clone(), NoopBlobStore::default());
        let pool = Pool::eth_pool(validator, InMemoryBlobStore::default(), Default::default());

        let net_key = config.network_key;
        let net_config = NetworkConfig::<_, EthNetworkPrimitives>::builder(net_key, runtime)
            .mainnet_boot_nodes()
            .build(client);
        let tx_manager_config = net_config.transactions_manager_config.clone();

        let (_net_handle, net_manager, tx_manager, _) = NetworkManager::builder(net_config)
            .await
            .context("failed to build p2p network manager")?
            .transactions(pool.clone(), tx_manager_config)
            .split_with_handle();

        tokio::spawn(net_manager);
        tokio::spawn(tx_manager);

        Ok(Self { pool, _net_handle })
    }
}
