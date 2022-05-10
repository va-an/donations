use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{env, log, near_bindgen};
use serde::{Deserialize, Serialize};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Donations {
    donations: Vector<Record>,
}

impl Default for Donations {
    fn default() -> Self {
        Self {
            donations: Vector::new(b"r-".to_vec()),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Record {
    from: String,
    donation: u128,
}

#[near_bindgen]
impl Donations {
    #[payable]
    pub fn send_donation(&mut self) {
        let donation = env::attached_deposit();
        let sender = env::signer_account_id();

        let record = Record {
            from: sender.clone(),
            donation,
        };

        self.donations.push(&record);

        // TODO: formatting for attached deposit
        log!("attached_deposit: {} from {}", donation, sender);
    }

    pub fn withdraw_donations(&mut self) {
        // TODO: implement function
        // TODO: allow only for contract creator
    }

    pub fn show_donations(&self) -> Vec<Record> {
        return self.donations.to_vec();
    }

    pub fn show_donations_sum(&self) -> u128 {
        // TODO: implement
        todo!()
    }
}
