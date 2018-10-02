// Copyright 2018 Chainpool

use config::Config;
use util::init_db;
use node::build_block;
use sync::SimpleNode;
use std::sync::Arc;
use miner::MemoryPool;
use parking_lot::RwLock;

pub fn dev(cfg: Config) -> Result<(), String> {
	try!(init_db(&cfg));
    let memory_pool = Arc::new(RwLock::new(MemoryPool::new()));
    let node = Arc::new(SimpleNode::new(cfg.consensus, cfg.db.clone(), memory_pool));
    let db = cfg.db.clone();
	while true {
       if let Some(block) = build_block(node.clone()) {
           db.insert(block.clone());
           info!("new block number:{:?}, hash:#{:?}", db.best_block().number, db.best_block().hash);
       } else {
           warn!("build block failed")
       }
	}

	Ok(())
}
