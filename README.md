# Vara ETH Base Smart Contract

Base contract for `vara.eth / ethexe` using Sails.

This repository is already adapted to an EVM-oriented interface:

- ABI-safe exported methods
- EVM events through `emit_eth_event`
- standard `root + app + client` workspace layout
- tests focused on logic, flow, and balances

> Important testing note: the contract **does** integrate EVM events, but for now they should **not** be tested as traditional Sails/gtest contract events. Tests should focus on deploy/init, return values, state, balances, and business errors.

---

## Repository Structure

- `app/` — actual contract logic
- `client/` — client generated from the IDL
- `src/lib.rs` — exports WASM and client artifacts
- `tests/` — gtest / integration tests
- `tools/veth` — CLI for uploading, creating, and interacting with the contract on `vara.eth`
- `client/contract_client.idl` — current contract IDL

---

## Contract Methods

Constructor:

- `Init()`

Service:

- `Greet() -> str`
- `Increment() -> u64`
- `Decrement() -> u64`
- `SendValue() -> str` (`payable`)
- `GetValue(to_return: u128) -> str`
- `ContractTotalEth() -> u128` (`query`)
- `CounterValue() -> u64` (`query`)

Declared events:

- `Hello([u8; 20])`
- `ValueReceived(u128)`
- `ValueSent(u128)`
- `Incremented`
- `Decremented`

---

## Build

Build:

```bash
cargo build 
```

Build release:

```bash
cargo build --release
```

Important artifacts:

- base WASM: `target/wasm32-gear/debug/contract.opt.wasm`
- IDL: `target/wasm32-gear/debug/contract.idl`
- generated client: `target/wasm32-gear/debug/contract_client.rs`

If you build in release mode, use the same paths under `release/`.

---

## Tests

Service unit tests:

```bash
cargo test -p contract-app
```

Integration tests:

```bash
cargo test
```

What is normally tested:

- deploy / init
- method calls
- state changes
- balances
- return values
- business errors

What should not currently be assumed to be testable:

- direct assertions on EVM events as if they were classic Sails/gtest events

---

## Contract Behavior

### `greet`

Returns:

```text
Hello <ActorId>
```

Also emits `Hello([u8; 20])`.

### `increment`

Increments the counter and returns the updated value.

### `decrement`

Decrements the counter.

If the counter is already `0`, it returns this error:

```text
Counter can not be negative!
```

### `send_value`

`payable` method. Receives value and returns:

```text
Value get: <amount>
```

### `get_value`

Attempts to return value from the contract balance back to the caller.

Response:

```text
Value returned: <amount>
```

If there is not enough balance available, it fails with:

```text
Cant transfer tokens
```

### queries

- `CounterValue` returns the current counter
- `ContractTotalEth` returns the ETH currently available in the contract

---

# Appendix: Using this contract with `veth`

This repository already includes the local binary:

```bash
./tools/veth
```

To view help:

```bash
./tools/veth --help
```

## 1. Minimal configuration

Set your defaults once:

```bash
./tools/veth set --rpc wss://hoodi-reth-rpc.gear-tech.io/ws
./tools/veth set --router 0xe549b0afeda978271ff7e712232b9f7f39a0b060
./tools/veth set --sender 0xSENDER
./tools/veth set --vara-rpc wss://vara-eth-validator-1.gear-tech.io
```

Show current config:

```bash
./tools/veth set --show
```

## Fast way to upload, creat and fund your contract

veth allow to do all this steps with one command in order to test your contract in a fast way:

```bash
cargo b -r
```

We will use:

- WASM: `target/wasm32-gear/debug/contract.opt.wasm`
- IDL: `client/contract_client.idl`

And then:

```bash
./tools/veth upload ./target/wasm32-gear/debug/contract.opt.wasm --create --fund-wvara "15 WVARA" --watch
```

This will do all the steps listed below (you can follow the commands, but, you can do this in a shorten way).

## 2. Build the contract

```bash
cargo build --release
```

We will use:

- WASM: `target/wasm32-gear/debug/contract.opt.wasm`
- IDL: `client/contract_client.idl`

> If your flow generates `contract.opt.wasm`, use it instead of `contract.wasm`.

## 3. Upload the code

```bash
./tools/veth upload target/wasm32-gear/debug/contract.opt.wasm --watch
```

This stores `last_code_id` in the `veth` config.

## 4. Create the program

```bash
./tools/veth create --json
```

This uses the previous `last_code_id` and stores `last_program_id`.

If you want to pass the code ID manually:

```bash
./tools/veth create --code-id 0xCODE_ID --json
```

## 5. Fund the mirror

WVARA:

```bash
./tools/veth fund wvara "10 WVARA" --approve
```

You can also specify the mirror explicitly:

```bash
./tools/veth fund eth --mirror 0xMIRROR "1 ETH"
./tools/veth fund wvara --mirror 0xMIRROR "10 WVARA" --approve
```

## 6. Initialize the contract

This contract uses the constructor:

- `Init`

Since it takes no arguments:

```bash
./tools/veth init --idl client/contract_client.idl --ctor Init --watch --json
```

Or explicitly:

```bash
./tools/veth init 0xMIRROR --idl client/contract_client.idl --ctor Init --watch --json
```

---

## 7. Sending messages with `veth send`

### Classic

The default mode sends the transaction through Ethereum RPC.

### Injected

Injected mode uses:

```bash
--injected --vara-rpc <WS_URL> # --vara-rpc is optional if you already set it
```

Important:

- `--injected` **cannot send value**
- because of that, `SendValue` does not apply in injected mode

---

## 8. Examples for all contract methods

### 8.1 `Greet`

Classic:

```bash
./tools/veth send 0xMIRROR \
  --idl client/contract_client.idl \
  --service ContractSvc \
  --function Greet \
  --watch --json
```

Injected:

```bash
./tools/veth send 0xMIRROR #0xMIRROR is optional \
  --idl client/contract_client.idl \
  --service ContractSvc \
  --function Greet \
  --injected \
  --watch --json
```

### 8.2 `Increment`

Classic:

```bash
./tools/veth send \
  --idl client/contract_client.idl \
  --service ContractSvc \
  --function Increment \
  --watch --json
```

Injected:

```bash
./tools/veth send \
  --idl client/contract_client.idl \
  --service ContractSvc \
  --function Increment \
  --injected \
  --watch --json
```

### 8.3 `Decrement`

Classic:

```bash
./tools/veth send \
  --idl client/contract_client.idl \
  --service ContractSvc \
  --function Decrement \
  --watch --json
```

Injected:

```bash
./tools/veth send  \
  --idl client/contract_client.idl \
  --service ContractSvc \
  --function Decrement \
  --injected \
  --watch --json
```

### 8.4 `SendValue` (`payable`)

Classic:

```bash
./tools/veth send \
  --idl client/contract_client.idl \
  --service ContractSvc \
  --function SendValue \
  --value "1 ETH" \
  --watch --json
```

Injected:

```text
Not applicable: injected mode cannot send value.
```

### 8.5 `GetValue(to_return: u128)`

Example returning `0.5 ETH` in wei:

```bash
./tools/veth send 0xMIRROR \
  --idl client/contract_client.idl \
  --service ContractSvc \
  --function GetValue \
  --args '{"to_return":"500000000000000000"}' \
  --watch --json
```

### 8.6 `CounterValue` (`query`)

```bash
./tools/veth query \
  --idl client/contract_client.idl \
  --service ContractSvc \
  --function CounterValue \
  --json
```

### 8.7 `ContractTotalEth` (`query`)

```bash
./tools/veth query \
  --mirror 0xMIRROR # your contract address \
  --idl client/contract_client.idl \
  --service ContractSvc \
  --function ContractTotalEth \
  --json
```

---

## 9. Recommended full sequence

```bash
cargo b -r

./tools/veth set --rpc wss://hoodi-reth-rpc.gear-tech.io/ws
./tools/veth set --router 0xe549b0afeda978271ff7e712232b9f7f39a0b060
./tools/veth set --sender 0xSENDER
./tools/veth set --vara-rpc wss://vara-eth-validator-1.gear-tech.io

./tools/veth upload target/wasm32-gear/debug/contract.opt.wasm  --create --fund-wvara "5 WVARA" --watch
./tools/veth init --idl client/contract_client.idl --ctor Init --watch --json

./tools/veth send --idl client/contract_client.idl --service ContractSvc --function Greet --watch --json
./tools/veth send --idl client/contract_client.idl --service ContractSvc --function Increment --watch --json
./tools/veth send --idl client/contract_client.idl --service ContractSvc --function SendValue --value "0.05 ETH" --watch --json
./tools/veth send --idl client/contract_client.idl --service ContractSvc --function GetValue --args '{"to_return":"500000000000000000"}' --watch --json
./tools/veth query --idl client/contract_client.idl --service ContractSvc --function CounterValue --json
./tools/veth query --idl client/contract_client.idl --service ContractSvc --function ContractTotalEth --json
```

---

## 10. Practical notes

- `SendValue` requires classic mode because it sends value.
- `GetValue` depends on the contract having enough balance first.
- `ContractTotalEth` is the direct query for checking how much ETH is available in the contract.
- if `Decrement` is called when the counter is `0`, it returns an error.
- if `GetValue` tries to return more than the available balance, the call fails.
- use `veth query` for reads, not `send`.
- if you change the ABI, regenerate and re-check the IDL before continuing to use old commands.
