use super::{bls::*};
use crate::{util::*};
use bytes::Bytes;
use serde::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    time::Duration,
};
use ethereum_types::{Address, H256, H64, U256, U64};
pub use ethnum::prelude::*;
use derive_more::*;
use fastrlp::*;
use serde::{Deserialize, Serialize};
use std::{
    iter::Step,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

/// Fixed number of extra-data prefix bytes reserved for signer vanity
pub const EXTRA_VANITY_LEN: usize = 32;
/// Fixed number of extra-data prefix bytes reserved for signer vanity, in boneh add validator num
pub const EXTRA_VANITY_LEN_WITH_NUM_IN_BONEH: usize = 33;
/// Fixed number of extra-data suffix bytes reserved for signer seal
pub const EXTRA_SEAL_LEN: usize = 65;
/// Fixed number of extra-data suffix bytes reserved for signer signature
pub const SIGNATURE_LEN: usize = 65;
/// Address length of signer
pub const ADDRESS_LENGTH: usize = 20;
/// Fixed number of extra-data suffix bytes reserved before boneh validator
pub const EXTRA_VALIDATOR_LEN: usize = ADDRESS_LENGTH;
/// Fixed number of extra-data suffix bytes reserved for boneh validator
pub const EXTRA_VALIDATOR_LEN_IN_BONEH: usize = EXTRA_VALIDATOR_LEN + BLS_PUBLIC_KEY_LEN;

macro_rules! impl_ops {
    ($type:ty, $other:ty) => {
        impl Add<$other> for $type {
            type Output = Self;
            #[inline(always)]
            fn add(self, other: $other) -> Self {
                Self(
                    self.0
                        + u64::try_from(other)
                            .unwrap_or_else(|_| unsafe { std::hint::unreachable_unchecked() }),
                )
            }
        }
        impl Sub<$other> for $type {
            type Output = Self;
            #[inline(always)]
            fn sub(self, other: $other) -> Self {
                Self(
                    self.0
                        - u64::try_from(other)
                            .unwrap_or_else(|_| unsafe { std::hint::unreachable_unchecked() }),
                )
            }
        }
        impl Mul<$other> for $type {
            type Output = Self;
            #[inline(always)]
            fn mul(self, other: $other) -> Self {
                Self(
                    self.0
                        * u64::try_from(other)
                            .unwrap_or_else(|_| unsafe { std::hint::unreachable_unchecked() }),
                )
            }
        }
        impl Div<$other> for $type {
            type Output = Self;
            #[inline(always)]
            fn div(self, other: $other) -> Self {
                Self(
                    self.0
                        / u64::try_from(other)
                            .unwrap_or_else(|_| unsafe { std::hint::unreachable_unchecked() }),
                )
            }
        }
        impl Rem<$other> for $type {
            type Output = Self;
            #[inline(always)]
            fn rem(self, other: $other) -> Self {
                Self(
                    self.0
                        % u64::try_from(other)
                            .unwrap_or_else(|_| unsafe { std::hint::unreachable_unchecked() }),
                )
            }
        }
        impl AddAssign<$other> for $type {
            #[inline(always)]
            fn add_assign(&mut self, other: $other) {
                self.0 += u64::try_from(other)
                    .unwrap_or_else(|_| unsafe { std::hint::unreachable_unchecked() });
            }
        }
        impl SubAssign<$other> for $type {
            #[inline(always)]
            fn sub_assign(&mut self, other: $other) {
                self.0 -= u64::try_from(other)
                    .unwrap_or_else(|_| unsafe { std::hint::unreachable_unchecked() });
            }
        }
        impl MulAssign<$other> for $type {
            #[inline(always)]
            fn mul_assign(&mut self, other: $other) {
                self.0 *= u64::try_from(other)
                    .unwrap_or_else(|_| unsafe { std::hint::unreachable_unchecked() });
            }
        }
        impl DivAssign<$other> for $type {
            #[inline(always)]
            fn div_assign(&mut self, other: $other) {
                self.0 /= u64::try_from(other)
                    .unwrap_or_else(|_| unsafe { std::hint::unreachable_unchecked() });
            }
        }
        impl RemAssign<$other> for $type {
            #[inline(always)]
            fn rem_assign(&mut self, other: $other) {
                self.0 %= u64::try_from(other)
                    .unwrap_or_else(|_| unsafe { std::hint::unreachable_unchecked() });
            }
        }
    };
}

macro_rules! impl_from {
    ($type:ty, $other:ty) => {
        impl From<$type> for $other {
            #[inline(always)]
            fn from(x: $type) -> $other {
                x.0 as $other
            }
        }
    };
}

macro_rules! u64_wrapper {
    ($ty:ident) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            Deref,
            DerefMut,
            Default,
            Display,
            Eq,
            From,
            FromStr,
            PartialEq,
            PartialOrd,
            Ord,
            Hash,
            Serialize,
            Deserialize,
            RlpEncodableWrapper,
            RlpDecodableWrapper,
            RlpMaxEncodedLen,
        )]
        #[serde(transparent)]
        #[repr(transparent)]
        pub struct $ty(pub u64);

        impl ::parity_scale_codec::WrapperTypeEncode for $ty {}
        impl ::parity_scale_codec::EncodeLike for $ty {}
        impl ::parity_scale_codec::EncodeLike<u64> for $ty {}
        impl ::parity_scale_codec::EncodeLike<$ty> for u64 {}
        impl ::parity_scale_codec::WrapperTypeDecode for $ty {
            type Wrapped = u64;
        }
        impl From<::parity_scale_codec::Compact<$ty>> for $ty {
            #[inline(always)]
            fn from(x: ::parity_scale_codec::Compact<$ty>) -> $ty {
                x.0
            }
        }
        impl ::parity_scale_codec::CompactAs for $ty {
            type As = u64;
            #[inline(always)]
            fn encode_as(&self) -> &Self::As {
                &self.0
            }
            #[inline(always)]
            fn decode_from(v: Self::As) -> Result<Self, ::parity_scale_codec::Error> {
                Ok(Self(v))
            }
        }
        impl PartialOrd<usize> for $ty {
            #[inline(always)]
            fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&(*other as u64))
            }
        }
        impl PartialEq<usize> for $ty {
            #[inline(always)]
            fn eq(&self, other: &usize) -> bool {
                self.0 == *other as u64
            }
        }
        impl Add<i32> for $ty {
            type Output = Self;
            #[inline(always)]
            fn add(self, other: i32) -> Self {
                Self(self.0 + u64::try_from(other).unwrap())
            }
        }

        impl_from!($ty, u64);
        impl_from!($ty, usize);

        impl_ops!($ty, u8);
        impl_ops!($ty, u64);
        impl_ops!($ty, usize);
        impl_ops!($ty, $ty);

        impl Step for $ty {
            #[inline(always)]
            fn steps_between(start: &Self, end: &Self) -> Option<usize> {
                u64::steps_between(&start.0, &end.0)
            }
            #[inline(always)]
            fn forward_checked(start: Self, count: usize) -> Option<Self> {
                u64::forward_checked(start.0, count).map(Self)
            }
            #[inline(always)]
            fn backward_checked(start: Self, count: usize) -> Option<Self> {
                u64::backward_checked(start.0, count).map(Self)
            }
        }
    };
}

u64_wrapper!(BlockNumber);
u64_wrapper!(ChainId);
u64_wrapper!(NetworkId);
u64_wrapper!(TxIndex);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ChainSpec {
    pub name: String,
    pub consensus: ConsensusParams,
    #[serde(default)]
    pub upgrades: Upgrades,
    pub params: Params,
    pub genesis: Genesis,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub contracts: BTreeMap<BlockNumber, BTreeMap<Address, Contract>>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub balances: BTreeMap<BlockNumber, BTreeMap<Address, U256>>,
    pub p2p: P2PParams,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DifficultyBomb {
    pub delays: BTreeMap<BlockNumber, BlockNumber>,
}

impl DifficultyBomb {
    pub fn get_delay_to(&self, block_number: BlockNumber) -> BlockNumber {
        self.delays
            .iter()
            .filter_map(|(&activation, &delay_to)| {
                if block_number >= activation {
                    Some(delay_to)
                } else {
                    None
                }
            })
            .last()
            .unwrap_or(BlockNumber(0))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsensusParams {
    pub seal_verification: SealVerificationParams,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub eip1559_block: Option<BlockNumber>,
}

impl ConsensusParams {

    pub fn is_parlia(&self) -> bool {
        match self.seal_verification {
            SealVerificationParams::Parlia { .. } => {
                true
            },
            _ => false
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SealVerificationParams {
    Clique {
        #[serde(with = "duration_as_millis")]
        period: Duration,
        epoch: u64,
    },
    Parlia {
        /// Number of seconds between blocks to enforce
        period: u64,
        /// Epoch length to update validatorSet
        epoch: u64,
    },
}

// deserialize_str_as_u64
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Upgrades {
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub homestead: Option<BlockNumber>,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub tangerine: Option<BlockNumber>,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub spurious: Option<BlockNumber>,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub byzantium: Option<BlockNumber>,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub constantinople: Option<BlockNumber>,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub petersburg: Option<BlockNumber>,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub istanbul: Option<BlockNumber>,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub berlin: Option<BlockNumber>,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub london: Option<BlockNumber>,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub paris: Option<BlockNumber>,

    /// bsc forks starts
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub ramanujan: Option<BlockNumber>,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub niels: Option<BlockNumber>,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub mirrorsync: Option<BlockNumber>,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub bruno: Option<BlockNumber>,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub euler: Option<BlockNumber>,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub gibbs: Option<BlockNumber>,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub boneh: Option<BlockNumber>,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub lynn: Option<BlockNumber>,
}

impl ChainSpec {

    pub fn is_on_ramanujan(&self, number: &BlockNumber) -> bool {
        is_on_forked(self.upgrades.ramanujan, number)
    }

    pub fn is_on_niels(&self, number: &BlockNumber) -> bool {
        is_on_forked(self.upgrades.niels, number)
    }

    pub fn is_on_mirror_sync(&self, number: &BlockNumber) -> bool {
        is_on_forked(self.upgrades.mirrorsync, number)
    }

    pub fn is_on_bruno(&self, number: &BlockNumber) -> bool {
        is_on_forked(self.upgrades.bruno, number)
    }

    pub fn is_on_euler(&self, number: &BlockNumber) -> bool {
        is_on_forked(self.upgrades.euler, number)
    }

    pub fn is_on_gibbs(&self, number: &BlockNumber) -> bool {
        is_on_forked(self.upgrades.gibbs, number)
    }

    pub fn is_on_boneh(&self, number: &BlockNumber) -> bool {
        is_on_forked(self.upgrades.boneh, number)
    }

    pub fn is_on_lynn(&self, number: &BlockNumber) -> bool {
        is_on_forked(self.upgrades.lynn, number)
    }

    pub fn is_ramanujan(&self, number: &BlockNumber) -> bool {
        is_forked(self.upgrades.ramanujan, number)
    }

    pub fn is_niels(&self, number: &BlockNumber) -> bool {
        is_forked(self.upgrades.niels, number)
    }

    pub fn is_mirror_sync(&self, number: &BlockNumber) -> bool {
        is_forked(self.upgrades.mirrorsync, number)
    }

    pub fn is_london(&self, number: &BlockNumber) -> bool {
        is_forked(self.upgrades.london, number)
    }

    pub fn is_bruno(&self, number: &BlockNumber) -> bool {
        is_forked(self.upgrades.bruno, number)
    }

    pub fn is_euler(&self, number: &BlockNumber) -> bool {
        is_forked(self.upgrades.euler, number)
    }

    pub fn is_gibbs(&self, number: &BlockNumber) -> bool {
        is_forked(self.upgrades.gibbs, number)
    }

    pub fn is_boneh(&self, number: &BlockNumber) -> bool {
        is_forked(self.upgrades.boneh, number)
    }

    pub fn is_lynn(&self, number: &BlockNumber) -> bool {
        is_forked(self.upgrades.lynn, number)
    }
}

/// is_forked returns whether a fork scheduled at block s is active at the given head block.
#[inline]
pub fn is_forked(forked_op: Option<BlockNumber>, current: &BlockNumber) -> bool {
    match forked_op {
        None => {
            false
        }
        Some(forked) => {
            *current >= forked
        }
    }
}

/// is_on_forked returns whether a fork is at target block number.
#[inline]
pub fn is_on_forked(fork_op: Option<BlockNumber>, current: &BlockNumber) -> bool {
    match fork_op {
        None => {
            false
        }
        Some(fork) => {
            *current == fork
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Params {
    pub chain_id: ChainId,
    pub network_id: NetworkId,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub additional_forks: BTreeSet<BlockNumber>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockScore {
    NoTurn = 1,
    InTurn = 2,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Seal {
    Ethash {
        #[serde(with = "hexbytes")]
        vanity: Bytes,
        difficulty: U256,
        nonce: H64,
        mix_hash: H256,
    },
    Parlia {
        vanity: H256,
        score: BlockScore,
        signers: Vec<Address>,
        bls_pub_keys: Option<Vec<BLSPublicKey>>,
    },
    Clique {
        vanity: H256,
        score: BlockScore,
        signers: Vec<Address>,
    },
    Unknown
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Genesis {
    pub number: BlockNumber,
    pub author: Address,
    pub gas_limit: u64,
    pub timestamp: u64,
    pub seal: Seal,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub base_fee_per_gas: Option<U256>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Contract {
    Contract {
        #[serde(with = "hexbytes")]
        code: Bytes,
    },
    Precompile(Precompile),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModExpVersion {
    ModExp198,
    ModExp2565,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Precompile {
    EcRecover { base: u64, word: u64 },
    Sha256 { base: u64, word: u64 },
    Ripemd160 { base: u64, word: u64 },
    Identity { base: u64, word: u64 },
    ModExp { version: ModExpVersion },
    AltBn128Add { price: u64 },
    AltBn128Mul { price: u64 },
    AltBn128Pairing { base: u64, pair: u64 },
    Blake2F { gas_per_round: u64 },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct P2PParams {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bootnodes: Vec<String>,
    #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub dns: Option<String>,
}

