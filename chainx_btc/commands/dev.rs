// Copyright 2018 Chainpool

use config::Config;
use util::init_db;
use node::build_block;
use sync::SimpleNode;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread;
use miner::MemoryPool;
use parking_lot::RwLock;
use core_rpc::{ MetaIoHandler, Compatibility };
use core_rpc::v1::{ BlockChain, BlockChainClient, BlockChainClientCore,
               RawClient, SimpleClientCore, Raw };
use jsonrpc_http_server::ServerBuilder;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::runtime::Runtime;
use tokio::timer::Interval;
use tokio::prelude::{Stream, Future};
use std::sync::atomic::{AtomicBool, Ordering};
const TIMER_INTERVAL_MS: u64 = 2 * 60 * 1000;

pub fn dev(cfg: Config) -> Result<(), String> {
	try!(init_db(&cfg));
    let memory_pool = Arc::new(RwLock::new(MemoryPool::new()));
    let node = Arc::new(SimpleNode::new(cfg.consensus, cfg.db.clone(), memory_pool));
    let db = cfg.db.clone();

    // http server
    let mut handler = MetaIoHandler::<()>::with_compatibility(Compatibility::Both);
    handler.extend_with(BlockChainClient::new(BlockChainClientCore::new(cfg.network, cfg.db.clone())).to_delegate());
    handler.extend_with(RawClient::new(SimpleClientCore::new(node.clone())).to_delegate());
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8083);
    let _server = ServerBuilder::new(handler).start_http(&socket);
    let (exit_send, exit) = exit_future::signal();
    let mut runtime = Runtime::new().expect("failed to start runtime on current thread");
    let interval = Interval::new(Instant::now() + Duration::from_millis(TIMER_INTERVAL_MS), Duration::from_millis(TIMER_INTERVAL_MS));
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let work = interval.map_err(|e| debug!("Timer error: {:?}", e)).for_each(move |_| {
      trace!("interval store false");
      r.store(false, Ordering::SeqCst); 
      Ok(())
    });
    let child = thread::spawn(move || {
        loop {
             if let Some(block) = build_block(node.clone().get_block_template(), running.clone()) {
                 db.insert(block.clone()).unwrap();
                 db.canonize(&block.hash()).expect("Failed to canonize block");
                 info!("new block number:{:?}, hash:#{:?}", db.best_block().number, db.best_block().hash);
             } else {
                 info!("build block failed")
             }
             running.store(true, Ordering::SeqCst);
             trace!("store true");
        }
   });
   let _ = runtime.block_on(exit.until(work).map(|_| ()));
   exit_send.fire();
   child.join().expect("Couldn't join on the associated thread");
   Ok(())
}
