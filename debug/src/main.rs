#![allow(unused,missing_docs)]
use anyhow::Context as _;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use zksync_protobuf::serde::Serde;
use zksync_consensus_storage::ReplicaState;
use zksync_dal::{ConnectionPool, Core};
use zksync_core::consensus;
use zksync_concurrency::ctx;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(long)]
    replica_state: Option<PathBuf>,
    #[arg(long)]
    postgres_url: Option<String>,
}

/// Decodes a proto message from json for arbitrary ProtoFmt.
pub fn decode_json<T: serde::de::DeserializeOwned>(json: &str) -> anyhow::Result<T> {
    let mut d = serde_json::Deserializer::from_str(json);
    let p = T::deserialize(&mut d)?;
    d.end()?;
    Ok(p)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let ctx = &ctx::root();

    if let Some(p) = &args.replica_state {
        let json = fs::read_to_string(p).context("fs::read_to_string()")?;
        let state = decode_json::<Serde<ReplicaState>>(&json).context("decode_json")?.0;
        println!("state = {state:#?}");
        let pool = ConnectionPool::<Core>::singleton(args.postgres_url.as_ref().context("postgres_url required")?)
            .build()
            .await
            .context("failed to build connection_pool")?;
        let store = consensus::Store(pool);
        let db_payload = store.wait_for_payload(ctx, state.proposals[0].number).await?;
        assert_eq!(db_payload,state.proposals[0].payload);
    }

    Ok(())
}
