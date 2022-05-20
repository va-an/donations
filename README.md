# Donations contract for NEAR Protocol

## Description
This contract implements donations functional - users can send donations in NEARs to contract, and only contract owner can withdraw this donations.

## Example
1. Mike deploys contract and sets `fundraiser` address as `mike.near`.
2. Alice and Bob decided to send donation to Mike, they checks that `fundraiser` is Mike's address.
3. Alice sends 1 NEAR donations to contract.
4. Bob sends 1.5 NEAR donations to contract.
5. Mike sees these donations in history.
6. Mike withdraws donations from Alice and Bob to `mike.near`
7. Alice and Bob sees in history that Mike withdrew donations.

# Build
```bash
cargo build --target wasm32-unknown-unknown --release
```
# API
## View methods
```rust
pub fn show_donations(&self) -> Vec<Record>; // show donations history
pub fn show_donations_sum(&self) -> Balance; // show current donations sum 
pub fn show_fundraiser(&self) -> AccountId;  // show contract owner
pub fn current_state(&self) -> DebugState;   // debug info
```
---

init

```bash
near dev-deploy \
--wasmFile target/wasm32-unknown-unknown/release/donations.wasm \
--initFunction new \
--initArgs '{"fundraiser": "va-an.testnet"}' \
--helperUrl https://near-contract-helper.onrender.com \
-f
```
---
send

```bash
near call donations.va-an.testnet send_donation --account_id va-an.testnet --amount 7.77
```

---
withdraw

```bash
near call $CONTRACT_DEV withdraw_donations --account_id v_a.testnet
```

---
show donations

```bash
near view $CONTRACT_DEV show_donations
```

---
show donations sum

```bash
near view $CONTRACT_DEV show_donations_sum
```

---
show fundraiser

```bash
near view $CONTRACT_DEV show_fundraiser
```
