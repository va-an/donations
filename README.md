# Donations contract for NEAR Protocol

## Description
This contract implements donations functional - users can send donations in NEARs to contract, and only contract owner can withdraw this donations.

Contract deployed on testnet address `donations.va-an.testnet`.

## Example Story
1. Mike deploys contract and sets `fundraiser` address as `mike.near`.
2. Alice and Bob decided to send donation to Mike, they checks that `fundraiser` is Mike's address.
3. Alice sends 1 NEAR donations to contract.
4. Bob sends 1.5 NEAR donations to contract.
5. Mike sees these donations in history.
6. Mike withdraws donations (2.5 NEARs) from Alice and Bob to `mike.near`
7. Alice and Bob sees in history that Mike withdrew donations.

# Build
add build target:
```bash
rustup target add wasm32-unknown-unknown
```
then build:
```bash
cargo build --target wasm32-unknown-unknown --release
```

# API
## View methods
```rust
pub fn show_history(&self) -> Vec<Record>;
pub fn show_donations_sum(&self) -> Balance;
pub fn show_fundraiser(&self) -> AccountId;
pub fn current_state(&self) -> DebugState;
```

## Call methods
```rust
pub fn send_donation(&mut self);
pub fn withdraw_donations(&mut self) -> Promise;
```
---

# Usage examples for [near-cli](https://docs.near.org/docs/tools/near-cli)
Some examples for near-cli below.

deploy and init contract

```bash
near deploy \
--account_id donations.va-an.testnet \
--wasmFile target/wasm32-unknown-unknown/release/donations.wasm \
--helperUrl https://near-contract-helper.onrender.com \
--initFunction new \
--initArgs '{"fundraiser": "va-an.testnet"}'
```

Don't forget change account id and init args!

---
send donation

```bash
near call donations.va-an.testnet send_donation --account_id va-an.testnet --amount 7.77
```

---
withdraw

```bash
near call donations.va-an.testnet withdraw_donations --account_id v_a.testnet
```

---
show donations history

```bash
near view donations.va-an.testnet show_history
```

---
show donations sum

```bash
near view donations.va-an.testnet show_donations_sum
```

---
show fundraiser

```bash
near view donations.va-an.testnet show_fundraiser
```
---
show contract state

```bash
near view donations.va-an.testnet current_state
```