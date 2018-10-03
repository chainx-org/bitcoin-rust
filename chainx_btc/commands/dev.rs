// Copyright 2018 Chainpool

use config::Config;
use util::init_db;
use node::build_block;
use sync::SimpleNode;
use std::sync::Arc;
use miner::MemoryPool;
use parking_lot::RwLock;
use core_rpc::{ MetaIoHandler, Compatibility, Remote };
use core_rpc::v1::{ BlockChain, BlockChainClient, BlockChainClientCore };
use jsonrpc_http_server::{ self, ServerBuilder, Server };
use std::net::{IpAddr, Ipv4Addr, SocketAddr};


pub fn dev(cfg: Config) -> Result<(), String> {
	try!(init_db(&cfg));
    let memory_pool = Arc::new(RwLock::new(MemoryPool::new()));
    let node = Arc::new(SimpleNode::new(cfg.consensus, cfg.db.clone(), memory_pool));
    let db = cfg.db.clone();

    // http server
    let mut handler = MetaIoHandler::<()>::with_compatibility(Compatibility::Both);
    handler.extend_with(BlockChainClient::new(BlockChainClientCore::new(cfg.network, cfg.db.clone())).to_delegate());
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8083);
    let _server = ServerBuilder::new(handler).start_http(&socket);

	while true {
       if let Some(block) = build_block(node.clone()) {
           db.insert(block.clone());
           db.canonize(&block.hash()).expect("Failed to canonize block");
           info!("new block number:{:?}, hash:#{:?}", db.best_block().number, db.best_block().hash);
       } else {
           warn!("build block failed")
       }
	}

	Ok(())
}
