use crate::errors::{AnyhowExt, ResultExt};
use crate::hashes::{hash160, ripemd160, sha1, sha256, sha256d, DigestType};
use bitcoin::transaction::Version;
use bitcoin::{
    absolute, consensus, Address, Amount, Network, OutPoint, ScriptBuf, Sequence, Transaction,
    TxIn, TxOut, Witness,
};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Bitcoin;

#[wasm_bindgen]
impl Bitcoin {
    pub fn parse_script_hex(hex_string: &str) -> crate::Result<String> {
        let result: anyhow::Result<_> = try {
            let vec = hex::decode(hex_string)?;
            let script_buf = ScriptBuf::from(vec);
            script_buf.to_asm_string()
        };
        result.map_err_string()
    }

    pub fn base58_check(hex: &str) -> crate::Result<String> {
        let result: anyhow::Result<_> = try { bitcoin::base58::encode_check(&hex::decode(hex)?) };
        result.map_err_string()
    }

    pub fn parse_hex_str(hex: &str) -> crate::Result<Vec<u8>> {
        let result = hex::decode(hex);
        result.map_err_string()
    }

    pub fn digest(data: &[u8], name: &str) -> crate::Result<String> {
        let result: anyhow::Result<_> = try {
            let r#type = DigestType::from_str(name)?;
            match r#type {
                DigestType::Ripemd160 => hex::encode(ripemd160(data)),
                DigestType::Sha256 => hex::encode(sha256(data)),
                DigestType::Sha256d => hex::encode(sha256d(data)),
                DigestType::Hash160 => hex::encode(hash160(data)),
                DigestType::Sha1 => hex::encode(sha1(data)),
            }
        };
        result.map_err_string()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsTx {
    version: u8,
    lock_time: u32,
    r#in: Vec<JsTxIn>,
    out: Vec<JsTxOut>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsTxIn {
    outpoint_tx_id: String,
    outpoint_index: u32,
    sequence: u32,
    script_sig: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsTxOut {
    amount: u64,
    script_pub_key: String,
}

#[wasm_bindgen]
pub struct TxBuilder;

trait TransactionExt {
    fn from_js_tx(tx: &JsTx) -> anyhow::Result<Transaction> {
        let tx = Transaction {
            version: Version(tx.version as i32),
            lock_time: absolute::LockTime::from_consensus(tx.lock_time),
            input: tx
                .r#in
                .iter()
                .map(|x| {
                    Ok::<_, anyhow::Error>(TxIn {
                        previous_output: OutPoint {
                            txid: x.outpoint_tx_id.parse()?,
                            vout: x.outpoint_index,
                        },
                        script_sig: ScriptBuf::from_hex(&x.script_sig)?,
                        sequence: Sequence(x.sequence),
                        witness: Witness::default(), /* TODO: witness is not implemented yet */
                    })
                })
                .collect::<Result<_, _>>()?,
            output: tx
                .out
                .iter()
                .map(|x| {
                    Ok::<_, anyhow::Error>(TxOut {
                        value: Amount::from_sat(x.amount),
                        script_pubkey: ScriptBuf::from_hex(&x.script_pub_key)?,
                    })
                })
                .collect::<Result<_, _>>()?,
        };
        Ok(tx)
    }
}

impl TransactionExt for Transaction {}

trait EncodeHex {
    fn hex(&self) -> String;
}

impl<T> EncodeHex for T
where
    T: AsRef<[u8]>,
{
    fn hex(&self) -> String {
        hex::encode(self.as_ref())
    }
}

#[wasm_bindgen]
impl TxBuilder {
    pub fn json_to_tx_hex(json: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> = try {
            let tx: JsTx = serde_json::from_str(json)?;
            let tx = Transaction::from_js_tx(&tx)?;
            consensus::encode::serialize(&tx).hex()
        };
        r.map_err_string()
    }

    pub fn address_to_script_pub_key(addr: &str, network: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> = try {
            let network = Network::from_str(network)?;
            let script = addr
                .parse::<Address<_>>()?
                .require_network(network)?
                .script_pubkey();
            script.hex()
        };
        r.map_err_string()
    }
}

trait ConsensusEncodeExt {
    fn consensus_encode_hex(&self) -> String;
}

impl<T> ConsensusEncodeExt for T
where
    T: consensus::Encodable,
{
    fn consensus_encode_hex(&self) -> String {
        consensus::encode::serialize(self).hex()
    }
}
