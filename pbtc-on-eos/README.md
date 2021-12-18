# :closed_lock_with_key: pBTC App

This Provable __pBTC__ app leverages the __pToken__ core in order to manages the cross-chain conversions between __pBTC__ & __BTC__. This app implements a simple CLI and a non-HSM-using database in order to serve as an example for how to consume the core library.

&nbsp;

***

&nbsp;

### :earth_africa: App Overview

#### :lock_with_ink_pen: Security:

This demonstration app includes a simple, _unprotected_, __`rocksDB`__ database to serve as an example for how to implement the database interface required by the core library. As such, sans security-features, this app should be used for test-net and similar only.

Main-net implementations will leverage various __HSMs__ in order to provide either an encrypted database interface as is the case with __Strongbox__, or both that and a fully secure runtime environment as is the case with Intel's __SGX__.

&nbsp;

***

&nbsp;

### :point_right: Usage:

```

❍ Provable pBTC Enclave ❍

    Copyright Provable 2019
    Questions: greg@oraclize.it

❍ Info ❍

This Provable pBTC app uses the pToken core in order to manage the cross-chain
conversions between pBTC & BTC tokens.

❍ Usage ❍

Usage:  pbtc [--help]
        pbtc getEnclaveState
        pbtc debugGetAllUtxos
        pbtc getLatestBlockNumbers
        pbtc debugGetKeyFromDb <key>
        pbtc debugSetKeyInDbToValue <key> <value>
        pbtc submitEthBlock (<blockJson> | --file=<path>)
        pbtc submitBtcBlock (<blockJson> | --file=<path>)
        pbtc initializeEth (<blocksJson> | --file=<path>) [--chainId=<uint>] [--gasPrice=<uint>] [--confs=<uint>]
        pbtc initializeBtc (<blocksJson> | --file=<path>) [--network=<string>] [--difficulty=<uint>] [--fee=<uint>] [--confs=<uint>]

Commands:

    submitEthBlock      ❍ Submit an ETH block (& its receipts) to the enclave.
                          NOTE: The enclave must first have been initialized!

                        ➔ blockJson Format:
                        A valid JSON string of an object containing the fields:
                          `Block`    ➔ The block header itself.
                          `Receipts` ➔ An array containing the block's receipts.

    submitBtcBlock      ❍ Submit an BTC block & its transactions to the enclave.
                          The submission material must also include an array of
                          deposit information for `p2sh` addresses.
                          NOTE: The enclave must first have been initialized!

                        ➔ blockJson Format:
                        A valid JSON string of an object containing the fields:
                          `block`        ➔ The BTC block in JSON format.
                          `transactions` ➔ The transactions in HEX format.
                          `deposit_address_list` ➔ An array of objects:
                          {
                            `nonce`:
                                An integer nonce.
                            `eth_address`:
                                The destination ETH address in hex.
                            `btc_deposit_address`:
                                The `p2sh` BTC deposit address.
                            `eth_address_and_nonce_hash`:
                                The `sha256d` of `eth_address + nonce`
                          }

    initializeEth       ❍ Initialize the enclave with the first trusted ETH
                        block. Ensure the block has NO transactions relevant to
                        the pToken in it, because they'll be ignore by the
                        enclave. Transactions are not verified so you may omit
                        them and include an empty array in their place if needs
                        be. The enclave will initialize its ETH related database
                        from this trusted block, create the ETH private-key and
                        seal it plus any relevant settings from the `config`
                        into the database. This command will return a signed
                        transaction to broadcast, which transaction will deploy
                        the pToken contract to the ETH network.

                        ➔ blocksJson Format:
                        A valid JSON string of an object containing the fields:
                          `eth_block_and_receipts` ➔ The ETH block & receipts
                        NOTE: See `submitETHBlock` for breakdown of JSON.

    initializeBtc       ❍ Initialize the enclave with the first trusted BTC
                        block. Ensure the block has NO transactions relevant to
                        the pToken in it, because they'll be ignore by the
                        enclave. Transactions are not verified so you may omit
                        them and include an empty array in their place if needs
                        be. The enclave will initialize its BTC related database
                        from this trusted block, create the BTC private-key and
                        seal it plus any relevant settings from the `config`
                        into the database.

                        ➔ blocksJson Format:
                        A valid JSON string of an object containing the fields:
                          `btc_block_and_txs` ➔ The BTC block & transactions.
                        NOTE: See `submitBTCBlock` for breakdown of JSON.

    getEnclaveState     ❍ Returns the current state of the enclave as pulled
                          from the database.

    debugGetAllUtxos     ❍ Returns JSON formatted report of all the UTXOs
                          currently held in the DB. This function can only be
                          called if the `debug` flag is set.

    debugGetKeyFromDb    ❍ Get a given <key> from the database. This function
                           can only be called if the `debug` flag is set to
                           true when the tool was built.

    getLatestBlockNumbers ❍ Returns the current lastest ETH & BTC block numbers
                          seen by the enclave.

    debugSetKeyInDbToValue  ❍ Set a given <key> in the database to a given
                              <value>. This function can only be called if the
                              `debug` flag is set to true when the core is
                              built. Note there there are zero checks on the
                              what is passed in to the datbase. Use at own risk!

    <key>               ❍ A database key in HEX format.

    <value>             ❍ A database value in HEX format.

    <blockJson>         ❍ Valid JSON string of ETH or BTC block.

Options:

    --help               ❍ Show this message.

    --file=<path>        ❍ Path to file containg an ETH or BTC block JSON.

    --fee=<uint>         ❍ BTC fee as measured in Satoshis per byte.
                           [default: 23]

    --difficulty=<path>  ❍ The `difficulty` value above which a BTC block's
                           difficulty should be in order to be considered valid.
                           [default: 1337]

    --gasPrice=<uint>    ❍ The gas price to be used in ETH transactions.
                           [default: 20000000000]

    --confs=<uint>       ❍ The number of confirmations required before signing
                           transactions. This directly affects the length of
                           chain the light client maintains in the database.
                           [default: 0]

    --network=<string>   ❍ Desired BTC network:
                           Bitcoin = Bitcoin Main-Net (default)
                           Testnet  = Bitcoin public test-net
                           [default: Bitcoin]

    --chainId=<uint>     ❍ ID of desired chain for transaction:
                           1  = Ethereum Main-Net (default)
                           3  = Ropsten Test-Net
                           4  = Rinkeby Test-Net
                           42 = Kovan Test-Net
                           [default: 1]

```

&nbsp;

***

&nbsp;

### :wrench: Build

You need to ensure you have both __`clang`__ & __`llvm`__ (or later versions) installed on your system. Then enter the __`./app`__ directory and run:

__`❍ cargo build --release`__

To enable __`debug`__ mode in the __`pToken`__ core, do so via the __`Cargo.toml`__ like so:

__`pbtc_core = { path = "../core", features = ["debug_mode"] }`__

:radioactive: Debug mode __MUST NOT__ be used in production - it bypasses __ALL__ security measures an app may implement, and makes fully accessible the entire database, in plain-text.:radioactive:

#### Versions

 - __`llvm:`__ version 6.0.0 or later.
 - __`clang:`__ version 6.0.0-1ubuntu2 or later.
 - __`rustc & cargo:`__ version 1.42.0-nightly or later.

&nbsp;

***

&nbsp;

### :black_nib: Notes

- The maximum __`confs`__ possible during initialization is 255.

- When initializing the enclave, the merkle-roots inside the __ETH__ and __BTC__ blocks are __NOT__ verified - only the block headers are checked. For smaller initialiazation material, feel free to provide empty arrays for the transactions. Ensure not relevant transactions took place in the blocks used to initialize the enclave.

&nbsp;

***

&nbsp;

### :guardsman: Tests

To run the tests simply run:

__`❍ cargo +nightly test`__

&nbsp;

***

&nbsp;

### :cyclone: Log Rotation

A log for each run of the tool will be written to the __`./logs/`__ directory.

Log rotation occurs when the number of logs reaches the __`MAX_NUM_LOGS`__ threshold. This threshold may be set by the user upon build via an environment variable thusly:

```

MAX_NUM_LOGS=100 cargo b --release

```

__NOTE:__ If no environment variable is provided upon build, the threshold will default to __1000__ logs.

__NOTE:__ The __`MAX_NUM_LOGS`__ also has a lower bound of __20__.

&nbsp;

***

&nbsp;

### :black_nib: To Do:

- [x] Combine ETH block & receipt parser modules w/ json codec.
- [x] Use new EthHash & EthHashes & EthAddress types consistently.
- [x] rename get_sequential_eth_blocks_and_receipts since they're structs not jsons.
- [x] Use canon-to-tip length getter FROM DB for all but the SETTING of the value from the config.
- [x] Use camelcase in usage & cli args.
- [x] check for logs dir and make if it's not there.
- [x] have tool write a generic config if it doesn't find one? Or a better error msg at least.
- [x] Sort out BTC lib import and maintain a fork in our repo.
- [x] Clear compiler warnings.
- [x] Rename `eth` dir to `submit_eth_block`. Same for btc.
- [x] Split types up into eth_types and btc_types?
- [x] Fix the fact that the initter just traces out the contract tx!
- [x] Read difficulty from the db everywhere!
- [x] Read the btc network from the db everywhere!
- [x] Read the eth chain id from the db everywhere!
- [x] Remove the `PTOKEN_CONTRACT_ADDRESS` from config since it's now unused.
- [x] Move ETH specific test utils to the ETH test_utils dir.
- [x] remove superfluous result wrapper from the ancestral ETH block getter!
- [x] Move ETH specific types to ETH dir.
- [x] Ibid re utils.
- [x] Factor eth/get_linker_hash into eth/update_linker_hash.
- [ ] Could pass in path of bytecode as arg to the initter!
- [ ] Ensure we are ignoring incoming 0 value txs.
- [ ] Rm clones from & || make more efficient the __`trie.rs`__ mod.
