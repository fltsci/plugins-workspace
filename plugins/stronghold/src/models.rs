use std::{
    collections::HashMap,
    fmt,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::stronghold::Stronghold;
use crypto::keys::bip39;
use iota_stronghold::{
    procedures::{
        BIP39Generate, BIP39Recover, Curve, Ed25519Sign, KeyType as StrongholdKeyType,
        MnemonicLanguage, PublicKey, Slip10Derive, Slip10DeriveInput, Slip10Generate,
        StrongholdProcedure,
    },
    Location,
};
use serde::{de::Visitor, Deserialize, Deserializer};

type PasswordHashFn = dyn Fn(&str) -> Vec<u8> + Send + Sync;

#[derive(Default)]
pub(crate) struct StrongholdCollection(pub(crate) Arc<Mutex<HashMap<PathBuf, Stronghold>>>);

pub(crate) struct PasswordHashFunction(pub(crate) Box<PasswordHashFn>);

#[derive(Deserialize, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[serde(untagged)]
pub enum BytesDto {
    Text(String),
    Raw(Vec<u8>),
}

impl AsRef<[u8]> for BytesDto {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Text(t) => t.as_ref(),
            Self::Raw(b) => b.as_ref(),
        }
    }
}

impl From<BytesDto> for Vec<u8> {
    fn from(v: BytesDto) -> Self {
        match v {
            BytesDto::Text(t) => t.as_bytes().to_vec(),
            BytesDto::Raw(b) => b,
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum LocationDto {
    Generic { vault: BytesDto, record: BytesDto },
    Counter { vault: BytesDto, counter: usize },
}

impl From<LocationDto> for Location {
    fn from(dto: LocationDto) -> Location {
        match dto {
            LocationDto::Generic { vault, record } => Location::generic(vault, record),
            LocationDto::Counter { vault, counter } => Location::counter(vault, counter),
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "type", content = "payload")]
#[allow(clippy::upper_case_acronyms)]
pub enum Slip10DeriveInputDto {
    Seed(LocationDto),
    Key(LocationDto),
}

impl From<Slip10DeriveInputDto> for Slip10DeriveInput {
    fn from(dto: Slip10DeriveInputDto) -> Slip10DeriveInput {
        match dto {
            Slip10DeriveInputDto::Seed(location) => Slip10DeriveInput::Seed(location.into()),
            Slip10DeriveInputDto::Key(location) => Slip10DeriveInput::Key(location.into()),
        }
    }
}

pub enum KeyType {
    Ed25519,
    X25519,
}

impl From<KeyType> for StrongholdKeyType {
    fn from(ty: KeyType) -> StrongholdKeyType {
        match ty {
            KeyType::Ed25519 => StrongholdKeyType::Ed25519,
            KeyType::X25519 => StrongholdKeyType::X25519,
        }
    }
}

impl<'de> Deserialize<'de> for KeyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct KeyTypeVisitor;

        impl Visitor<'_> for KeyTypeVisitor {
            type Value = KeyType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ed25519 or x25519")
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value.to_lowercase().as_str() {
                    "ed25519" => Ok(KeyType::Ed25519),
                    "x25519" => Ok(KeyType::X25519),
                    _ => Err(serde::de::Error::custom("unknown key type")),
                }
            }
        }

        deserializer.deserialize_str(KeyTypeVisitor)
    }
}

#[derive(Deserialize)]
#[serde(tag = "type", content = "payload")]
#[allow(clippy::upper_case_acronyms)]
pub enum ProcedureDto {
    SLIP10Generate {
        output: LocationDto,
        #[serde(rename = "sizeBytes")]
        size_bytes: Option<usize>,
    },
    SLIP10Derive {
        chain: Vec<u32>,
        input: Slip10DeriveInputDto,
        output: LocationDto,
    },
    BIP39Recover {
        mnemonic: String,
        passphrase: Option<String>,
        output: LocationDto,
    },
    BIP39Generate {
        passphrase: Option<String>,
        output: LocationDto,
    },
    PublicKey {
        #[serde(rename = "type")]
        ty: KeyType,
        #[serde(rename = "privateKey")]
        private_key: LocationDto,
    },
    Ed25519Sign {
        #[serde(rename = "privateKey")]
        private_key: LocationDto,
        msg: String,
    },
}

impl From<ProcedureDto> for StrongholdProcedure {
    fn from(dto: ProcedureDto) -> StrongholdProcedure {
        match dto {
            ProcedureDto::SLIP10Generate { output, size_bytes } => {
                StrongholdProcedure::Slip10Generate(Slip10Generate {
                    output: output.into(),
                    size_bytes,
                })
            }
            ProcedureDto::SLIP10Derive {
                chain,
                input,
                output,
            } => StrongholdProcedure::Slip10Derive(Slip10Derive {
                curve: Curve::Ed25519,
                chain,
                input: input.into(),
                output: output.into(),
            }),
            ProcedureDto::BIP39Recover {
                mnemonic,
                passphrase,
                output,
            } => StrongholdProcedure::BIP39Recover(BIP39Recover {
                mnemonic: bip39::Mnemonic::from(mnemonic),
                passphrase: bip39::Passphrase::from(passphrase.unwrap_or_default()),
                output: output.into(),
            }),
            ProcedureDto::BIP39Generate { passphrase, output } => {
                StrongholdProcedure::BIP39Generate(BIP39Generate {
                    passphrase: bip39::Passphrase::from(passphrase.unwrap_or_default()),
                    output: output.into(),
                    language: MnemonicLanguage::English,
                })
            }
            ProcedureDto::PublicKey { ty, private_key } => {
                StrongholdProcedure::PublicKey(PublicKey {
                    ty: ty.into(),
                    private_key: private_key.into(),
                })
            }
            ProcedureDto::Ed25519Sign { private_key, msg } => {
                StrongholdProcedure::Ed25519Sign(Ed25519Sign {
                    private_key: private_key.into(),
                    msg: msg.as_bytes().to_vec(),
                })
            }
        }
    }
}

pub enum PasswordHashFunctionKind {
    #[cfg(feature = "kdf")]
    Argon2(PathBuf),
    Custom(Box<PasswordHashFn>),
}
