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

fn from_yocto_near(yocto_near: u128) -> f64 {
    format!(
        "{:.2}",
        yocto_near as f64 / 1_000_000_000_000_000_000_000_000.0
    )
    .parse::<f64>()
    .unwrap()
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Record {
    from: String,
    donation: u128,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RecordJson {
    from: String,
    donation: f64,
}

#[near_bindgen]
impl Donations {
    #[payable]
    pub fn send_donation(&mut self) {
        let donation_yocto = env::attached_deposit();
        let donation = from_yocto_near(donation_yocto);

        let sender = env::signer_account_id();

        let record = Record {
            from: sender.clone(),
            donation: donation_yocto,
        };

        self.donations.push(&record);

        // TODO: formatting for attached deposit
        log!("attached_deposit: {} from {}", donation, sender);
    }

    pub fn withdraw_donations(&mut self) {
        // TODO: implement function
        // TODO: allow only for contract creator
    }

    pub fn show_donations(&self) -> Vec<RecordJson> {
        self.donations
            .iter()
            .map(|record| RecordJson {
                from: record.from,
                donation: from_yocto_near(record.donation),
            })
            .collect()
    }

    pub fn show_donations_sum(&self) -> u128 {
        // TODO: implement
        todo!()
    }
}
