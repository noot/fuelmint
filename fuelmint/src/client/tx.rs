use fuel_core::schema::scalars::HexString;

use async_graphql::{Context, Object};

use fuel_core_interfaces::{
    block_producer::BlockProducer,
    common::{
        fuel_tx::{Cacheable, Transaction as FuelTx},
        fuel_vm::prelude::Deserializable,
    },
};

// use fuel_txpool::Service as TxPoolService;
use fuel_core::schema::tx::{receipt, types};
use std::sync::Arc;

#[derive(Default)]
pub struct TxMutation;

#[Object]
impl TxMutation {
    /// Execute a dry-run of the transaction using a fork of current state, no changes are committed.
    async fn dry_run(
        &self,
        ctx: &Context<'_>,
        tx: HexString,
        // If set to false, disable input utxo validation, overriding the configuration of the node.
        // This allows for non-existent inputs to be used without signature validation
        // for read-only calls.
        utxo_validation: Option<bool>,
    ) -> async_graphql::Result<Vec<receipt::Receipt>> {
        // Modify to use App's dry_run function
        let block_producer = ctx.data_unchecked::<Arc<dyn BlockProducer>>();

        let mut tx = FuelTx::from_bytes(&hex::decode(tx.to_string()).unwrap())?;
        tx.precompute();

        let receipts = block_producer.dry_run(tx, None, utxo_validation).await?;
        Ok(receipts.iter().map(Into::into).collect())
    }

    /// Submits transaction to the txpool
    async fn submit(
        &self,
        _ctx: &Context<'_>,
        tx: HexString,
    ) -> async_graphql::Result<types::Transaction> {
        // Send request through broadcast_tx
        let hex_string = tx.to_string();
        let tx = hex_string.strip_prefix("0x").unwrap();
        println!("Transaction String: {:?}", tx);
        let mut fuel_tx = FuelTx::from_bytes(&hex::decode(tx).unwrap())?;
        println!("Transaction: {:?}", fuel_tx);
        println!("Precomputing transaction...");
        fuel_tx.precompute();

        println!("Sending request to rollmint...");
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/broadcast_tx", "http://127.0.0.1:26657"))
            .query(&[("tx", &tx)])
            .send()
            .await?;

        println!("Response from rollmint: {:?}", res);

        let fuel_tx = types::Transaction::from(fuel_tx);
        println!("returning transaction...");
        Ok(fuel_tx)
    }
}