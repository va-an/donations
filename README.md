init

```bash
$ near dev-deploy \
--wasmFile target/wasm32-unknown-unknown/release/donations.wasm \
--initFunction new \
--initArgs '{"fundraiser": "va-an.testnet"}' \
--helperUrl https://near-contract-helper.onrender.com \
-f
```
---
send

```bash
$ near call donations.va-an.testnet send_donation --account_id va-an.testnet --amount 7.77
```

---
withdraw

```bash
$ near call $CONTRACT_DEV withdraw_donations --account_id v_a.testnet
```

---
show donations

```bash
$ near view $CONTRACT_DEV show_donations
```

---
show donations sum

```bash
$ near view $CONTRACT_DEV show_donations_sum
```

---
show fundraiser

```bash
$ near view $CONTRACT_DEV show_fundraiser
```
