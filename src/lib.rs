use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{env, log, near_bindgen, require, AccountId, Balance, PanicOnDefault, Promise};
use serde::{Deserialize, Serialize};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Donations {
    records: Vector<Record>,
    sum: Balance,
    fundraiser: AccountId,
}

type BalanceHumanReadable = f64;

fn from_yocto_near(yocto_near: Balance) -> BalanceHumanReadable {
    format!("{:.2}", yocto_near as f64 / 1e24)
        .parse::<f64>()
        .unwrap()
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Record {
    from: AccountId,
    donation: Balance,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RecordJson {
    from: AccountId,
    donation: BalanceHumanReadable,
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
            from: sender.clone(),
            donation: donation_yocto,
        };

        self.records.push(&record);
        self.sum += donation_yocto;

        log!("attached_deposit: {} from {}", donation, sender);
    }

    pub fn withdraw_donations(&mut self) -> Promise {
        require!(
            env::signer_account_id() == self.fundraiser,
            "Owner's method"
        );

        let transfer_amount = self.sum;
        self.sum = 0;

        Promise::new(self.fundraiser.clone()).transfer(transfer_amount)
    }

    pub fn show_donations(&self) -> Vec<RecordJson> {
        self.records
            .iter()
            .map(|record| RecordJson {
                from: record.from,
                donation: from_yocto_near(record.donation),
            })
            .collect()
    }

    pub fn show_donations_sum(&self) -> BalanceHumanReadable {
        from_yocto_near(self.sum)
    }

    pub fn show_fundraiser(&self) -> AccountId {
        self.fundraiser.clone()
    }
}
