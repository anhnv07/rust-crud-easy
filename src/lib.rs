use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::UnorderedMap;
use near_sdk::AccountId;

near_sdk::setup_alloc!();

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

// 1. Main Struct
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Job {
    pub author: AccountId,
    pub number: u64,
    pub status: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct KeyValue {
    jobs: UnorderedMap<AccountId, Job>,
}

// 2. Default Implementation
impl Default for KeyValue {
    fn default() -> Self {
        Self {
            jobs: UnorderedMap::new(b"r".to_vec())
        }
    }
}

// 3. Core Logic
#[near_bindgen]
impl KeyValue {
    pub fn create_update(&mut self, number: u64, status: String) {
        let job = Job {
            author: env::signer_account_id(),
            number,
            status,
        };
        env::log(b"created or updated");
        self.jobs.insert(&env::signer_account_id(), &job);
    }

    pub fn read(&self, author: AccountId) -> Option<Job> {
        env::log(b"read");
        return self.jobs.get(&author);
    }

    pub fn delete(&mut self, author: AccountId) {
        env::log(b"delete");
        self.jobs.remove(&author);
    }
}

// 4. Tests
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    // Test 1
    #[test]
    fn test_create_update_job() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = KeyValue::default();
        contract.create_update(98, "pending".to_string());
    }

    #[test]
    fn test_read_job() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = KeyValue::default();
        assert_eq!(
            None,
            contract.read("bob_near".to_string())
        );
    }

    #[test]
    fn test_delete_job() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let mut contract = KeyValue::default();
        contract.delete("bob_near".to_string());
    }
}