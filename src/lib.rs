use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, env, Balance, Timestamp, Promise};
use near_sdk::collections::{UnorderedMap};
use near_sdk::json_types::U128;
use crate::utils::unordered_map_pagination;

mod web4;
mod utils;
mod ui;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
// contract state
pub struct Contract {
    pub owner_id: AccountId,
    pub deadline: Option<Timestamp>,
    pub prize_pool: Balance,
    pub prize_pool_distributed: Balance,
    pub applications: UnorderedMap<AccountId, ApplicationData>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ApplicationData {
    pub description: String,
    pub github_url: String,
    pub contact_data: String,
    pub contract_id: AccountId,
    pub youtube_url: Option<String>,

    // Admin fields
    pub reward: Option<Balance>,
    pub hidden: Option<bool>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Applications,
}

#[near_bindgen]
impl Contract {
    #[payable]
    #[init]
    // contract initialization
    // How to use:
    // near call $CONTRACT_ID new '{"owner_id": "'$CONTRACT_ID'", "deadline": 1672480800000000000}' --accountId $CONTRACT_ID --deposit 10
    pub fn new(owner_id: AccountId, deadline: Option<Timestamp>) -> Self {
        let prize_pool = env::attached_deposit();
        assert!(prize_pool > 0, "ERR_NO_PRIME_POOL");
        Self {
            owner_id,
            deadline,
            prize_pool,
            prize_pool_distributed: 0,
            applications: UnorderedMap::new(StorageKey::Applications),
        }
    }

    // register new application from any hackathon participant. One application per NEAR account.
    // reapplying updates previous data
    pub fn register(&mut self, application: ApplicationData) {
        if let Some(deadline) = self.deadline {
            assert!(env::block_timestamp() <= deadline);
        }

        let new_application = ApplicationData {
            description: application.description,
            github_url: application.github_url,
            contact_data: application.contact_data,
            contract_id: application.contract_id,
            youtube_url: application.youtube_url,
            reward: None,
            hidden: Some(false)
        };

        self.applications.insert(&env::predecessor_account_id(), &new_application);
    }

    // Return applications, from_index & limit may be used for pagination
    pub fn get_applications(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<(AccountId, ApplicationData)> {
        unordered_map_pagination(&self.applications, from_index, limit)
    }

    // Owner method only to set the winner and transfer the reward to his account
    // How to use:
    // near call $CONTRACT_ID set_winner '{"account_id": "awesomeweb4.testnet", "reward": "1000000000000000000000000"}' --accountId $CONTRACT_ID
    pub fn set_winner(&mut self, account_id: AccountId, reward: U128) -> Promise {
        self.assert_owner();
        let mut application = self.applications.get(&account_id).expect("ERR_APPLICATION_NOT_FOUND");
        assert!(application.reward.is_none(), "ERR_REWARD_ALREADY_SET");

        assert!(self.prize_pool >= self.prize_pool_distributed + reward.0, "ERR_NOT_ENOUGH_REWARDS");

        self.prize_pool_distributed += reward.0;

        application.reward = Some(reward.0);
        self.applications.insert(&account_id, &application);

        Promise::new(account_id).transfer(reward.0)
    }

    // Owner method only to hide application from the frontend
    pub fn set_hidden(&mut self, account_id: AccountId, hidden: bool) {
        self.assert_owner();
        let mut application = self.applications.get(&account_id).expect("ERR_APPLICATION_NOT_FOUND");
        application.hidden = Some(hidden);
        self.applications.insert(&env::predecessor_account_id(), &application);
    }

    // Owner method only to update the deadline
    pub fn set_deadline(&mut self, deadline: Timestamp) {
        self.assert_owner();
        self.deadline = Some(deadline);
    }

    // helper method to check if current user is a hackathon owner
    fn assert_owner(&self) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "ERR_NOT_OWNER");
    }
}