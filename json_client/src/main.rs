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
use zksync_web3_decl::{client::L2Client};
use zksync_core::sync_layer::MainNodeClient;
use zksync_types::Miniblock;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(long)]
    server_url: String,
    #[arg(long)]
    miniblock: u32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let client = L2Client::http(&args.server_url)
        .context("Failed creating JSON-RPC client for main node")?
        .build();
    let block = client.fetch_l2_block(MiniblockNumber(args.miniblock),true).await.context("fetch_l2_block()")?;
    println!("OK block = {block:?}");
    Ok(())
}
