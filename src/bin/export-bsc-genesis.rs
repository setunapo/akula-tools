use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use anyhow::bail;
use akula_tools::models::*;
use akula_tools::models::chainspec::*;

use clap::Parser;
use ethereum_types::{Address, U256};
use hex_literal::hex;
use akula_tools::models::bls::*;
use std::str::FromStr;

#[derive(Parser)]
#[clap(name = "Akula-tools", about = "a set of tools for akula.")]
struct Opt {
    #[clap(long)]
    pub name: Option<String>,
    #[clap(long, help = "output path.")]
    pub output: Option<String>,
    #[clap(long, help = "input the genesis.json file location.")]
    pub genesis: String,
    #[clap(long, help = "input the config.toml file location.")]
    pub config: String,
}

fn main() -> anyhow::Result<()> {
    let opt: Opt = Opt::parse();

    let genesis: bsc::Genesis = serde_json::from_str(&fs::read_to_string(opt.genesis)?)?;
    let config: bsc::TomlConfig = toml::from_str(&fs::read_to_string(opt.config)?)?;

    let mut chain_spec = ChainSpec {
        name: opt.name.unwrap_or(String::from("BSC-devnet")),
        consensus: ConsensusParams {
            seal_verification: SealVerificationParams::Parlia {
                period: genesis.config.parlia.period,
                epoch: genesis.config.parlia.epoch
            },
            eip1559_block: None
        },
        upgrades: Upgrades {
            homestead: genesis.config.homestead_block,
            tangerine: genesis.config.eip_150_block,
            spurious: genesis.config.eip_155_block,
            byzantium: genesis.config.byzantium_block,
            constantinople: genesis.config.constantinople_block,
            petersburg: genesis.config.petersburg_block,
            istanbul: genesis.config.istanbul_block,
            berlin: genesis.config.berlin_block,
            london: genesis.config.london_block,
            // not support in bsc
            paris: None,
            // bsc fork start
            ramanujan: genesis.config.ramanujan_block,
            niels: genesis.config.niels_block,
            mirrorsync: genesis.config.mirror_sync_block,
            bruno: genesis.config.bruno_block,
            euler: genesis.config.euler_block,
            gibbs: genesis.config.gibbs_block,
            boneh: genesis.config.boneh_block,
            lynn: genesis.config.lynn_block
        },
        params: Params {
            chain_id: ChainId(genesis.config.chain_id),
            network_id: NetworkId(genesis.config.chain_id),
            additional_forks: BTreeSet::new(),
        },
        genesis: Genesis {
            number: BlockNumber(genesis.number.as_u64()),
            author: genesis.coinbase,
            gas_limit: genesis.gas_limit.as_u64(),
            timestamp: genesis.timestamp.as_u64(),
            seal: Seal::Unknown,
            base_fee_per_gas: None
        },
        contracts: Default::default(),
        balances: Default::default(),
        p2p: P2PParams {
            bootnodes: config.node.p2p.static_nodes,
            dns: None
        }
    };

    // parse contracts and balances
    let mut contracts = BTreeMap::new();
    let mut balances = BTreeMap::new();
    for (addr, account) in genesis.alloc {
        balances.insert(addr, if account.balance.starts_with("0x") {
            U256::from_str(&account.balance)?
        } else {
            U256::from_dec_str(&account.balance)?
        });
        if let Some(code) = account.code {
            contracts.insert(addr, Contract::Contract {
                code: hex::decode(code.strip_prefix("0x").unwrap_or(&code))
                    .map_err(|e| e)?
                    .into()
            });
        }
    }
    chain_spec.contracts.insert(chain_spec.genesis.number, contracts);
    chain_spec.balances.insert(chain_spec.genesis.number, balances);

    // set base_fee_per_gas
    if chain_spec.is_london(&chain_spec.genesis.number) {
        chain_spec.genesis.base_fee_per_gas = Some(U256::from(1000000000_u128));
    }

    // parse signers and bls keys
    let extra_data = genesis.extra_data;
    let extra_len = extra_data.len();
    let mut signers = Vec::new();
    let mut bls_keys = None;
    if !chain_spec.is_boneh(&chain_spec.genesis.number) {
        let val_bytes = &extra_data[EXTRA_VANITY_LEN..extra_len - EXTRA_SEAL_LEN];
        let count = val_bytes.len() / EXTRA_VALIDATOR_LEN;
        for i in 0..count {
            let start = i * EXTRA_VALIDATOR_LEN;
            signers.push(Address::from_slice(&val_bytes[start..start + EXTRA_VALIDATOR_LEN]));
        }
    } else {
        let mut tmp = Vec::new();

        let count = extra_data[EXTRA_VANITY_LEN_WITH_NUM_IN_BONEH - 1] as usize;
        let start = EXTRA_VANITY_LEN_WITH_NUM_IN_BONEH;
        let end = start + count * EXTRA_VALIDATOR_LEN_IN_BONEH;
        let val_bytes = &extra_data[start..end];

        for i in 0..count {
            let start = i * EXTRA_VALIDATOR_LEN_IN_BONEH;
            let end = start + EXTRA_VALIDATOR_LEN;
            signers.push(Address::from_slice(&val_bytes[start..end]));

            let start = i * EXTRA_VALIDATOR_LEN_IN_BONEH + EXTRA_VALIDATOR_LEN;
            let end = i * EXTRA_VALIDATOR_LEN_IN_BONEH + EXTRA_VALIDATOR_LEN_IN_BONEH;
            tmp.push(BLSPublicKey::from_slice(&val_bytes[start..end]));
        }
        bls_keys = Some(tmp);
    }
    chain_spec.genesis.seal = Seal::Parlia {
        vanity: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
        score: if genesis.difficulty.as_u64() == 1 {
            BlockScore::NoTurn
        } else if genesis.difficulty.as_u64() == 2 {
            BlockScore::InTurn
        } else {
            return bail!("wrong parlia difficulty");
        },
        signers,
        bls_pub_keys: bls_keys
    };

    let output = opt.output.unwrap_or(String::from("."));
    let path = Path::new(&output).join(format!("{}.ron", chain_spec.name));
    fs::write(&path, ron::ser::to_string_pretty(&chain_spec, ron::ser::PrettyConfig::new())?)?;
    println!("akula's chain spec saved in path: {:?}", &path.to_str());
    Ok(())
}