use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{env, log, near_bindgen, require, AccountId, Balance, PanicOnDefault, Promise};
use serde::{Deserialize, Serialize};

type BalanceHumanReadable = f64;

fn from_yocto_near(yocto_near: Balance) -> BalanceHumanReadable {
    format!("{:.2}", yocto_near as f64 / 1e24)
        .parse::<f64>()
        .unwrap()
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Debug, PartialEq, Eq)]
pub enum Operation {
    Donation,
    Withdrawal,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Debug)]
pub struct Record {
    who: AccountId,
    operation: Operation,
    amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RecordJson {
    from: AccountId,
    donation: BalanceHumanReadable,
}

#[derive(Debug, Serialize)]
pub struct DebugState {
    records: Vec<RecordJson>,
    sum: BalanceHumanReadable,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Donations {
    records: Vector<Record>,
    sum: Balance,
    fundraiser: AccountId,
}

#[near_bindgen]
impl Donations {
    #[init]
    pub fn new(fundraiser: AccountId) -> Self {
        Self {
            records: Vector::new(b"r".to_vec()),
            sum: 0,
            fundraiser,
        }
    }

    #[payable]
    pub fn send_donation(&mut self) {
        let donation_yocto: Balance = env::attached_deposit();
        let donation: BalanceHumanReadable = from_yocto_near(donation_yocto);

        let sender = env::signer_account_id();

        let record = Record {
            who: sender.clone(),
            operation: Operation::Donation,
            amount: donation_yocto,
        };

        self.records.push(&record);
        self.sum += donation_yocto;

        log!("attached_deposit: {} from {}", donation, sender);
    }

    pub fn withdraw_donations(&mut self) -> Promise {
        require!(
            env::signer_account_id() == self.fundraiser,
            format!(
                "this method permitted only for contract owner '{}'",
                self.fundraiser
            )
        );

        let transfer_amount = self.sum;
        self.sum = 0;

        self.records.push(&Record {
            who: self.fundraiser.clone(),
            operation: Operation::Withdrawal,
            amount: transfer_amount,
        });

        Promise::new(self.fundraiser.clone()).transfer(transfer_amount)
    }

    pub fn show_history(&self) -> Vec<Record> {
        self.records.to_vec()
    }

    pub fn show_donations_sum(&self) -> Balance {
        self.sum
    }

    pub fn show_fundraiser(&self) -> AccountId {
        self.fundraiser.clone()
    }

    pub fn current_state(&self) -> DebugState {
        let records = self
            .records
            .iter()
            .map(|record| RecordJson {
                from: record.who,
                donation: from_yocto_near(record.amount),
            })
            .collect();

        let sum = from_yocto_near(self.sum);

        DebugState { records, sum }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext, ONE_NEAR};
    use std::convert::TryInto;

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("contract-owner".to_string().try_into().unwrap())
            .is_view(is_view)
            .build()
    }

    fn create_contract(fundraiser: &str) -> Donations {
        let fundraiser_account = AccountId::new_unchecked(fundraiser.to_string());
        let conract = Donations::new(fundraiser_account.clone());

        conract
    }

    #[test]
    fn show_fundraiser() {
        let fundraiser = "contract-owner";
        let contract = create_contract(fundraiser);

        assert_eq!(fundraiser, contract.show_fundraiser().as_str());
    }

    #[test]
    fn send_donation() {
        let mut context = get_context(false);
        context.attached_deposit = 777 * ONE_NEAR / 100; // 7.77 NEAR
        testing_env!(context);

        let mut contract = create_contract("contract-owner");

        assert!(contract.show_history().is_empty());
        assert_eq!(0, contract.show_donations_sum());

        contract.send_donation();

        assert!(!contract.show_history().is_empty());
        assert!(contract.show_donations_sum() != 0);
    }

    #[test]
    fn withdraw_donations() {
        let mut context = get_context(false);
        let attached_deposit = 555 * ONE_NEAR / 100; // 5.55 NEAR
        context.attached_deposit = attached_deposit;
        context.signer_account_id = AccountId::new_unchecked("some-account-name".to_string());
        testing_env!(context);

        let mut contract = create_contract("contract-owner");
        assert_eq!(0, contract.show_donations_sum());
        assert_eq!(0, contract.show_history().len());

        contract.send_donation();
        assert_ne!(0, contract.show_donations_sum());
        assert_eq!(1, contract.show_history().len());

        let history = contract.show_history();
        let record = history.first().unwrap();
        assert_eq!(Operation::Donation, record.operation);
        assert_eq!(attached_deposit, record.amount);

        context = get_context(false);
        context.signer_account_id = AccountId::new_unchecked("contract-owner".to_string());
        testing_env!(context);

        contract.withdraw_donations();
        assert_eq!(0, contract.show_donations_sum());
    }

    #[test]
    #[should_panic]
    fn panics_on_withdraw_donations() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = create_contract("not-contract-owner");
        contract.withdraw_donations();
    }
}
