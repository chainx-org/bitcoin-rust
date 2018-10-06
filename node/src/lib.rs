// Copyright 2018 Chainpool

extern crate primitives;
extern crate storage;
extern crate sync;
extern crate miner;
extern crate chain;
extern crate keys;
extern crate script;
#[macro_use]
extern crate log;

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use primitives::bigint::{U256, Uint};
use primitives::bytes::Bytes;
use primitives::hash::H256;
use chain::{merkle_root, IndexedTransaction, Transaction, IndexedBlockHeader, TransactionInput,
            TransactionOutput, IndexedBlock, BlockHeader};
use script::Builder;
use miner::{BlockTemplate, find_solution, CoinbaseTransactionBuilder};

pub struct P2shCoinbaseTransactionBuilder {
    transaction: Transaction,
}

impl P2shCoinbaseTransactionBuilder {
    pub fn new(hash: &keys::AddressHash, value: u64) -> Self {
        let script_pubkey = Builder::build_p2sh(hash).into();
        let transaction = Transaction {
            version: 0,
            inputs: vec![TransactionInput::coinbase(Bytes::default())],
            outputs: vec![
                TransactionOutput {
                    value: value,
                    script_pubkey: script_pubkey,
                },
            ],
            lock_time: 0,
        };

        P2shCoinbaseTransactionBuilder { transaction: transaction }
    }
}

impl CoinbaseTransactionBuilder for P2shCoinbaseTransactionBuilder {
    fn set_extranonce(&mut self, extranonce: &[u8]) {
        self.transaction.inputs[0].script_sig = extranonce.to_vec().into();
    }

    fn hash(&self) -> H256 {
        self.transaction.hash()
    }

    fn finish(self) -> Transaction {
        self.transaction
    }
}


const SECRET: &'static str = "000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943";

pub fn build_block(
    block_template: BlockTemplate,
    running: Arc<AtomicBool>,
) -> Option<IndexedBlock> {
   let private_key = keys::Private {
       network: keys::Network::Testnet,
       secret: SECRET.into(),
       compressed: false,
    };
    info!("miner private_key:{:?}", private_key);
    let kp = keys::KeyPair::from_private(private_key).unwrap();
    info!("coin base reward: {:?}", kp.public().address_hash());
    let coinbase_builder =
        P2shCoinbaseTransactionBuilder::new(&kp.public().address_hash(), 1000000000);
    if let Some(solution) = find_solution(
        &block_template,
        coinbase_builder,
        U256::max_value(),
        running,
    )
    {
        let coinbase_hash = solution.coinbase_transaction.hash();
        let mut transactions =
            vec![
                IndexedTransaction::new(coinbase_hash, solution.coinbase_transaction),
            ];
        transactions.extend(block_template.transactions.iter().map(|tx| tx.clone()));
        let mut merkle_tree = vec![];
        merkle_tree.extend(transactions.iter().map(|tx| &tx.hash));
        let merkle_root_hash = merkle_root(&merkle_tree);
        let header = BlockHeader {
            version: block_template.version,
            previous_header_hash: block_template.previous_header_hash,
            merkle_root_hash: merkle_root_hash,
            time: solution.time,
            bits: block_template.bits,
            nonce: solution.nonce,
        };
        let block = IndexedBlock::new(
            IndexedBlockHeader::new(header.hash(), header),
            transactions.clone(),
        );
        return Some(block);
    }
    return None;
}
