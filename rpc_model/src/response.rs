use elements::bitcoin::bip32::{ExtendedPubKey, Fingerprint};
use elements::bitcoin::hash_types::XpubIdentifier;
use elements::{AssetId, Txid};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An empty response.
#[derive(JsonSchema)]
pub struct Empty {}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Version {
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GenerateSigner {
    pub mnemonic: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListSigners {
    pub signers: Vec<Signer>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Wallet {
    pub descriptor: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListWallets {
    pub wallets: Vec<Wallet>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UnloadWallet {
    pub unloaded: Wallet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnloadSigner {
    pub unloaded: Signer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signer {
    pub name: String,
    pub fingerprint: Fingerprint,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<XpubIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xpub: Option<ExtendedPubKey>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub address: elements::Address,
    pub index: u32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Balance {
    pub balance: HashMap<AssetId, u64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Pset {
    pub pset: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SinglesigDescriptor {
    pub descriptor: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultisigDescriptor {
    pub descriptor: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Xpub {
    pub keyorigin_xpub: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Broadcast {
    pub txid: Txid,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Contract {
    pub entity: Entity,
    pub issuer_pubkey: String,
    pub name: String,
    pub precision: u8,
    pub ticker: String,
    pub version: u8,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Entity {
    domain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignerDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub fingerprint: Fingerprint,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletDetails {
    #[serde(rename = "type")]
    pub type_: String,
    pub signers: Vec<SignerDetails>,
    pub warnings: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WalletCombine {
    pub pset: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletPsetDetails {
    pub has_signatures_from: Vec<SignerDetails>,
    pub missing_signatures_from: Vec<SignerDetails>,
    pub warnings: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AssetDetails {
    pub name: String,
}
