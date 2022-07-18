use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Gas, PanicOnDefault, Promise};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_meta() -> Self {
        let this: Contract = Self {
            owner_id: env::current_account_id(),
        };
        this
    }

    pub fn cross(&self) -> Promise {
        ext_example::log_signer(
            AccountId::new_unchecked("contract_two.jeph.testnet".to_string()),
            0,
            Gas(5_000_000_000_000),
        )
    }

    pub fn cross_dos(&self) -> Promise {
        ext_example::ft_transfer(
            "jephtest.testnet".to_string(),
            "10000000".to_string(),
            "".to_string(),
            AccountId::new_unchecked("usdc.fakes.testnet".to_string()),
            1,
            Gas(5_000_000_000_000),
        )
    }
}

#[ext_contract(ext_example)]
pub trait ExtExample {
    fn log_signer(&self);
    fn ft_transfer(&self, receiver_id: String, amount: String, memo: String);
}
