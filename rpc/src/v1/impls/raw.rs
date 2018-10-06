use jsonrpc_core::Error;
use jsonrpc_macros::Trailing;
use ser::{Reader, serialize, deserialize};
use v1::traits::Raw;
use v1::types::{SignedTransactionOutput, TransactionInputScript, TransactionOutputScript,
                SignedTransactionInput, Bytes, RawTransaction, TransactionInput,
                TransactionOutput, TransactionOutputs, Transaction, GetRawTransactionResponse};
use v1::types::H256;
use v1::helpers::errors::{execution, invalid_params, transaction_not_found};
use chain::Transaction as GlobalTransaction;
use primitives::bytes::Bytes as GlobalBytes;
use primitives::hash::H256 as GlobalH256;
use std::sync::Arc;
use keys::{self, Address};
use global_script::Script;
use sync;

pub struct RawClient<T: RawClientCoreApi> {
    core: T,
}

pub trait RawClientCoreApi: Send + Sync + 'static {
    fn accept_transaction(&self, transaction: GlobalTransaction) -> Result<GlobalH256, String>;
    fn create_raw_transaction(
        &self,
        inputs: Vec<TransactionInput>,
        outputs: TransactionOutputs,
        lock_time: Trailing<u32>,
    ) -> Result<GlobalTransaction, String>;
    fn get_raw_transaction(
        &self,
        hash: H256,
        verbose: Trailing<bool>,
    ) -> Result<GetRawTransactionResponse, Error>;
}

pub struct RawClientCore {
    local_sync_node: sync::LocalNodeRef,
}

pub fn do_create_raw_transaction(
    inputs: Vec<TransactionInput>,
    outputs: TransactionOutputs,
    lock_time: Trailing<u32>,
) -> Result<GlobalTransaction, String> {
    use chain;
    use keys;
    use global_script::Builder as ScriptBuilder;

    // to make lock_time work at least one input must have sequnce < SEQUENCE_FINAL
    let lock_time = lock_time.unwrap_or_default();
    let default_sequence = if lock_time != 0 {
        chain::constants::SEQUENCE_FINAL - 1
    } else {
        chain::constants::SEQUENCE_FINAL
    };

    // prepare inputs
    let inputs: Vec<_> = inputs
        .into_iter()
        .map(|input| {
            chain::TransactionInput {
                previous_output: chain::OutPoint {
                    hash: Into::<GlobalH256>::into(input.txid).reversed(),
                    index: input.vout,
                },
                script_sig: GlobalBytes::new(), // default script
                sequence: input.sequence.unwrap_or(default_sequence),
                script_witness: vec![],
            }
        })
        .collect();

    // prepare outputs
    let outputs: Vec<_> = outputs
        .outputs
        .into_iter()
        .map(|output| match output {
            TransactionOutput::Address(with_address) => {
                let amount_in_satoshis =
                    (with_address.amount * (chain::constants::SATOSHIS_IN_COIN as f64)) as u64;
                let script = match with_address.address.kind {
                    keys::Type::P2PKH => ScriptBuilder::build_p2pkh(&with_address.address.hash),
                    keys::Type::P2SH => ScriptBuilder::build_p2sh(&with_address.address.hash),
                };

                chain::TransactionOutput {
                    value: amount_in_satoshis,
                    script_pubkey: script.to_bytes(),
                }
            }
            TransactionOutput::ScriptData(with_script_data) => {
                let script = ScriptBuilder::default()
                    .return_bytes(&*with_script_data.script_data)
                    .into_script();

                chain::TransactionOutput {
                    value: 0,
                    script_pubkey: script.to_bytes(),
                }
            }
        })
        .collect();

    // now construct && serialize transaction
    let transaction = GlobalTransaction {
        version: 1,
        inputs: inputs,
        outputs: outputs,
        lock_time: lock_time,
    };

    Ok(transaction)
}

pub struct SimpleClientCore {
    simple_node: Arc<sync::SimpleNode>,
}

impl SimpleClientCore {
    pub fn new(node: Arc<sync::SimpleNode>) -> Self {
        SimpleClientCore { simple_node: node }
    }

    pub fn do_create_raw_transaction(
        inputs: Vec<TransactionInput>,
        outputs: TransactionOutputs,
        lock_time: Trailing<u32>,
    ) -> Result<GlobalTransaction, String> {
        do_create_raw_transaction(inputs, outputs, lock_time)
    }
}

impl RawClientCore {
    pub fn new(local_sync_node: sync::LocalNodeRef) -> Self {
        RawClientCore { local_sync_node: local_sync_node }
    }

    pub fn do_create_raw_transaction(
        inputs: Vec<TransactionInput>,
        outputs: TransactionOutputs,
        lock_time: Trailing<u32>,
    ) -> Result<GlobalTransaction, String> {
        do_create_raw_transaction(inputs, outputs, lock_time)
    }
}

impl RawClientCoreApi for RawClientCore {
    fn accept_transaction(&self, transaction: GlobalTransaction) -> Result<GlobalH256, String> {
        self.local_sync_node.accept_transaction(transaction)
    }

    fn create_raw_transaction(
        &self,
        inputs: Vec<TransactionInput>,
        outputs: TransactionOutputs,
        lock_time: Trailing<u32>,
    ) -> Result<GlobalTransaction, String> {
        RawClientCore::do_create_raw_transaction(inputs, outputs, lock_time)
    }

    fn get_raw_transaction(
        &self,
        hash: H256,
        verbose: Trailing<bool>,
    ) -> Result<GetRawTransactionResponse, Error> {
        let ghash: GlobalH256 = hash.clone().into();
        if let Some(tx) = self.local_sync_node.storage.transaction(&ghash) {
            let raw_tx = Bytes::new(serialize(&tx).take());
            if verbose.unwrap_or_default() {
                let transaction = Transaction {
                    hex: raw_tx,
                    txid: hash.clone().reversed(), // segwit to do
                    hash: hash.clone().reversed(),
                    size: 0, // to do
                    vsize: 0, // to do
                    version: tx.version,
                    locktime: tx.lock_time as i32,
                    vin: tx.inputs.iter().map(|input| SignedTransactionInput{
                                      txid: H256::from(input.clone().previous_output.hash.take()).reversed(),
                                      vout: input.previous_output.index,
                                      script_sig: TransactionInputScript{asm: String::new(), hex: Bytes::new(input.clone().script_sig.take()),},
                                      sequence: input.sequence,
                                      txinwitness: input.script_witness.iter().map(|bytes| String::from_utf8(bytes.clone().take()).unwrap()).collect::<_>(),}).collect::<_>(),
                    vout: tx.outputs.iter().map(|output| {
                                           let ref script_bytes = output.clone().script_pubkey;
                                           let script: Script = script_bytes.clone().into();
                                           let script_asm = format!("{}", script);
                                           let script_addresses = script.extract_destinations().unwrap_or(vec![]);
                                           SignedTransactionOutput{value: 0.00000001f64 * output.value as f64, n: 0,// to do
                                             script: TransactionOutputScript{asm: script_asm, hex: script_bytes.clone().into(),
                                             req_sigs: script.num_signatures_required() as u32, script_type: script.script_type().into(), addresses: script_addresses.into_iter().map(|a| Address {
                                                       network: keys::Network::Testnet, /*match self.network {
                                                            Network::Mainnet => keys::Network::Mainnet,
                                                            _ => keys::Network::Testnet,
                                                       },*/
                                                       hash: a.hash,
                                                       kind: a.kind,}).collect(),},
                                           } }).collect::<_>(),
                    blockhash: Default::default(),
                    confirmations: 0, // to do:tx.is_final_in_block(height, block_time),
                    time: 0, // to do: block_time,
                    blocktime: 0, // to do: block_time,
                };
                return Ok(GetRawTransactionResponse::Verbose(transaction));
            } else {
                return Ok(GetRawTransactionResponse::Raw(raw_tx));
            }
        }
        Err(transaction_not_found(hash))
    }
}

impl<T> RawClient<T>
where
    T: RawClientCoreApi,
{
    pub fn new(core: T) -> Self {
        RawClient { core: core }
    }
}

impl RawClientCoreApi for SimpleClientCore {
    fn accept_transaction(&self, transaction: GlobalTransaction) -> Result<GlobalH256, String> {
        self.simple_node.accept_transaction(transaction)
    }

    fn create_raw_transaction(
        &self,
        inputs: Vec<TransactionInput>,
        outputs: TransactionOutputs,
        lock_time: Trailing<u32>,
    ) -> Result<GlobalTransaction, String> {
        SimpleClientCore::do_create_raw_transaction(inputs, outputs, lock_time)
    }

    fn get_raw_transaction(
        &self,
        hash: H256,
        verbose: Trailing<bool>,
    ) -> Result<GetRawTransactionResponse, Error> {
        let hash: GlobalH256 = hash.clone().into();
        if verbose.unwrap_or_default() {
            /* if let Some(tx) = self.simple_node.storage.transaction(&hash) {
              Ok(GetRawTransactionResponse::Verbose(tx))
           }*/
        } else {
            if let Some(tx) = self.simple_node.storage.transaction_bytes(&hash) {
                let tx = Bytes::new(serialize(&tx).take());
                return Ok(GetRawTransactionResponse::Raw(tx));
            }
        }
        Err(transaction_not_found(hash))
    }
}

impl<T> Raw for RawClient<T>
where
    T: RawClientCoreApi,
{
    fn send_raw_transaction(&self, raw_transaction: RawTransaction) -> Result<H256, Error> {
        let raw_transaction_data: Vec<u8> = raw_transaction.into();
        let transaction = try!(deserialize(Reader::new(&raw_transaction_data)).map_err(
            |e| {
                invalid_params("tx", e)
            },
        ));
        self.core
            .accept_transaction(transaction)
            .map(|h| h.reversed().into())
            .map_err(|e| execution(e))
    }

    fn create_raw_transaction(
        &self,
        inputs: Vec<TransactionInput>,
        outputs: TransactionOutputs,
        lock_time: Trailing<u32>,
    ) -> Result<RawTransaction, Error> {
        // reverse hashes of inputs
        let inputs: Vec<_> = inputs
            .into_iter()
            .map(|mut input| {
                input.txid = input.txid.reversed();
                input
            })
            .collect();

        let transaction = try!(
            self.core
                .create_raw_transaction(inputs, outputs, lock_time)
                .map_err(|e| execution(e))
        );
        let transaction = serialize(&transaction);
        Ok(transaction.into())
    }

    fn decode_raw_transaction(&self, _transaction: RawTransaction) -> Result<Transaction, Error> {
        rpc_unimplemented!()
    }

    fn get_raw_transaction(
        &self,
        hash: H256,
        verbose: Trailing<bool>,
    ) -> Result<GetRawTransactionResponse, Error> {
        self.core.get_raw_transaction(hash.reversed(), verbose)
    }
}

#[cfg(test)]
pub mod tests {
    use jsonrpc_macros::Trailing;
    use jsonrpc_core::IoHandler;
    use chain::Transaction;
    use primitives::hash::H256 as GlobalH256;
    use v1::traits::Raw;
    use v1::types::{TransactionInput, TransactionOutputs};
    use super::*;

    #[derive(Default)]
    struct SuccessRawClientCore;
    #[derive(Default)]
    struct ErrorRawClientCore;

    impl RawClientCoreApi for SuccessRawClientCore {
        fn accept_transaction(&self, transaction: Transaction) -> Result<GlobalH256, String> {
            Ok(transaction.hash())
        }

        fn create_raw_transaction(
            &self,
            _inputs: Vec<TransactionInput>,
            _outputs: TransactionOutputs,
            _lock_time: Trailing<u32>,
        ) -> Result<Transaction, String> {
            Ok("0100000001ad9d38823d95f31dc6c0cb0724c11a3cf5a466ca4147254a10cd94aade6eb5b3230000006b483045022100b7683165c3ecd57b0c44bf6a0fb258dc08c328458321c8fadc2b9348d4e66bd502204fd164c58d1a949a4d39bb380f8f05c9f6b3e9417f06bf72e5c068428ca3578601210391c35ac5ee7cf82c5015229dcff89507f83f9b8c952b8fecfa469066c1cb44ccffffffff0170f30500000000001976a914801da3cb2ed9e44540f4b982bde07cd3fbae264288ac00000000".into())
        }
    }

    impl RawClientCoreApi for ErrorRawClientCore {
        fn accept_transaction(&self, _transaction: Transaction) -> Result<GlobalH256, String> {
            Err("error".to_owned())
        }

        fn create_raw_transaction(
            &self,
            _inputs: Vec<TransactionInput>,
            _outputs: TransactionOutputs,
            _lock_time: Trailing<u32>,
        ) -> Result<Transaction, String> {
            Err("error".to_owned())
        }
    }

    #[test]
    fn sendrawtransaction_accepted() {
        let client = RawClient::new(SuccessRawClientCore::default());
        let mut handler = IoHandler::new();
        handler.extend_with(client.to_delegate());

        let sample = handler.handle_request_sync(&(r#"
			{
				"jsonrpc": "2.0",
				"method": "sendrawtransaction",
				"params": ["00000000013ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a0000000000000000000101000000000000000000000000"],
				"id": 1
			}"#)
		).unwrap();

        // direct hash is 0791efccd035c5fe501023ff888106eba5eff533965de4a6e06400f623bcac34
        // but client expects reverse hash
        assert_eq!(r#"{"jsonrpc":"2.0","result":"34acbc23f60064e0a6e45d9633f5efa5eb068188ff231050fec535d0ccef9107","id":1}"#, &sample);
    }

    #[test]
    fn sendrawtransaction_rejected() {
        let client = RawClient::new(ErrorRawClientCore::default());
        let mut handler = IoHandler::new();
        handler.extend_with(client.to_delegate());

        let sample = handler.handle_request_sync(&(r#"
			{
				"jsonrpc": "2.0",
				"method": "sendrawtransaction",
				"params": ["00000000013ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a0000000000000000000101000000000000000000000000"],
				"id": 1
			}"#)
		).unwrap();

        assert_eq!(r#"{"jsonrpc":"2.0","error":{"code":-32015,"message":"Execution error.","data":"\"error\""},"id":1}"#, &sample);
    }

    #[test]
    fn createrawtransaction_success() {
        let client = RawClient::new(SuccessRawClientCore::default());
        let mut handler = IoHandler::new();
        handler.extend_with(client.to_delegate());

        let sample = handler.handle_request_sync(&(r#"
			{
				"jsonrpc": "2.0",
				"method": "createrawtransaction",
				"params": [[{"txid":"4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b","vout":0}],{"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa":0.01}],
				"id": 1
			}"#)
		).unwrap();

        assert_eq!(r#"{"jsonrpc":"2.0","result":"0100000001ad9d38823d95f31dc6c0cb0724c11a3cf5a466ca4147254a10cd94aade6eb5b3230000006b483045022100b7683165c3ecd57b0c44bf6a0fb258dc08c328458321c8fadc2b9348d4e66bd502204fd164c58d1a949a4d39bb380f8f05c9f6b3e9417f06bf72e5c068428ca3578601210391c35ac5ee7cf82c5015229dcff89507f83f9b8c952b8fecfa469066c1cb44ccffffffff0170f30500000000001976a914801da3cb2ed9e44540f4b982bde07cd3fbae264288ac00000000","id":1}"#, &sample);
    }

    #[test]
    fn createrawtransaction_error() {
        let client = RawClient::new(ErrorRawClientCore::default());
        let mut handler = IoHandler::new();
        handler.extend_with(client.to_delegate());

        let sample = handler.handle_request_sync(&(r#"
			{
				"jsonrpc": "2.0",
				"method": "createrawtransaction",
				"params": [[{"txid":"4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b","vout":0}],{"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa":0.01}],
				"id": 1
			}"#)
		).unwrap();

        assert_eq!(r#"{"jsonrpc":"2.0","error":{"code":-32015,"message":"Execution error.","data":"\"error\""},"id":1}"#, &sample);
    }
}
