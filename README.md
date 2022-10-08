# akula-tools
a set of tools for akula in bsc.

## install

you can use `export-bsc-genesis` by `cargo install`.

```bash
RUSTC_BOOTSTRAP="1" cargo install --git https://github.com/GalaIO/akula-tools.git --bin export-bsc-genesis
```

The `export-bsc-genesis` in your cargo bin path. you can use it directly.

> you could build from source, and execute it `./target/debug/export-bsc-genesis`.

## usage

```bash
export-bsc-genesis --help
Akula-tools 
a set of tools for akula.

USAGE:
    export-bsc-genesis [OPTIONS] --genesis <GENESIS> --config <CONFIG>

OPTIONS:
        --config <CONFIG>      input the config.toml file location.
        --genesis <GENESIS>    input the genesis.json file location.
    -h, --help                 Print help information
        --name <NAME>          
        --output <OUTPUT>      output path.
```

if you want to export genesis from bsc config, you could enter:

```bash
export-bsc-genesis  --genesis "xxx/genesis.json" --config "xxx/config.toml" --output ~/
```

Default name is `BSC-devnet.ron`, you can rename by `--name` flag.