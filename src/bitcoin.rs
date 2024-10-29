use crate::errors::{AnyhowExt, ResultExt};
use crate::hashes::{hash160, ripemd160, sha1, sha256, sha256d, DigestType};
use bitcoin::hashes::Hash;
use bitcoin::key::Secp256k1;
use bitcoin::script::{PushBytes, ScriptBufExt};
use bitcoin::script::ScriptExt;
use bitcoin::secp256k1::{Message, SecretKey};
use bitcoin::sighash::SighashCache;
use bitcoin::transaction::Version;
use bitcoin::{absolute, consensus, Address, Amount, Network, OutPoint, PrivateKey, PublicKey, Script, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use bitcoin::address::script_pubkey::BuilderExt;
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

impl TryFrom<JsTx> for Transaction {
    type Error = anyhow::Error;

    fn try_from(tx: JsTx) -> Result<Self, Self::Error> {
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

impl TryFrom<TxIn> for JsTxIn {
    type Error = anyhow::Error;

    fn try_from(tx: TxIn) -> Result<Self, Self::Error> {
        Ok(JsTxIn {
            outpoint_tx_id: tx.previous_output.txid.to_string(),
            outpoint_index: tx.previous_output.vout,
            sequence: tx.sequence.0,
            script_sig: tx.script_sig.hex(),
        })
    }
}

impl TryFrom<TxOut> for JsTxOut {
    type Error = anyhow::Error;

    fn try_from(tx: TxOut) -> Result<Self, Self::Error> {
        Ok(JsTxOut {
            amount: tx.value.to_sat(),
            script_pub_key: tx.script_pubkey.hex(),
        })
    }
}

impl TryFrom<Transaction> for JsTx {
    type Error = anyhow::Error;

    fn try_from(tx: Transaction) -> Result<Self, Self::Error> {
        let tx = JsTx {
            version: tx.version.0 as u8,
            lock_time: tx.lock_time.to_consensus_u32(),
            r#in: tx
                .input
                .into_iter()
                .map(|x| JsTxIn::try_from(x))
                .collect::<Result<_, _>>()?,
            out: tx
                .output
                .into_iter()
                .map(JsTxOut::try_from)
                .collect::<Result<_, _>>()?,
        };
        Ok(tx)
    }
}

impl FromStr for JsTx {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tx: JsTx = serde_json::from_str(s)?;
        Ok(tx)
    }
}

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
            let tx = Transaction::try_from(tx)?;
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

    pub fn script_to_asm(hex: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> = try { Script::from_bytes(&hex::decode(hex)?).to_asm_string() };
        r.map_err_string()
    }

    pub fn sign_for_script_sig(
        tx: &str,
        input_index: usize,
        txo_script_pubkey: &str,
        sighash_type: u32,
        secret_key: &str,
        public_key: &str,
    ) -> crate::Result<String> {
        let r: anyhow::Result<_> = try {
            let tx: JsTx = tx.parse()?;
            let tx: Transaction = tx.try_into()?;
            let txo_script_pubkey = ScriptBuf::from_bytes(hex::decode(txo_script_pubkey)?);
            let secret = SecretKey::from_slice(&hex::decode(secret_key)?)?;
            let public = PublicKey::from_slice(&hex::decode(public_key)?)?;

            let sighash_cache = SighashCache::new(tx);
            let sighash = sighash_cache.legacy_signature_hash(
                input_index,
                &txo_script_pubkey,
                sighash_type,
            )?;

            let secp = Secp256k1::default();
            let message = Message::from_digest(sighash.to_byte_array());
            let signature = secp.sign_ecdsa(&message, &secret);

            let script_sig = ScriptBuf::builder()
                .push_slice_try_from(signature.serialize_der().as_ref())?
                .push_key(public)
                .into_script();
            script_sig.hex()
        };
        r.map_err_string()
    }

    pub fn wif_to_ec_hex(wif: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> = try {
            let private_key = PrivateKey::from_wif(wif)?;
            private_key.inner.as_ref().hex()
        };
        r.map_err_string()
    }

    pub fn wif_to_public_key(wif: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> = try {
            let k = PrivateKey::from_wif(wif)?;
            let public_key = k.public_key(&Default::default());
            if public_key.compressed {
                public_key.inner.serialize().hex()
            } else {
                public_key.inner.serialize_uncompressed().hex()
            }
        };
        r.map_err_string()
    }
}

pub trait ScriptsBuilderExt
where
    Self: Sized,
{
    fn push_slice_try_from(self, slice: &[u8]) -> anyhow::Result<Self>;
}

impl ScriptsBuilderExt for bitcoin::script::Builder {
    fn push_slice_try_from(self, slice: &[u8]) -> anyhow::Result<Self> {
        Ok(self.push_slice(<&PushBytes>::try_from(slice)?))
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
