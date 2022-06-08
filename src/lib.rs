use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env};

const PUZZLE_NUMBER:u8 = 1;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // SETUP CONTRACT STATE
    crossword_solution:String,
}

#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE
    pub fn get_puzzle_number(&self)->u8{
        PUZZLE_NUMBER
    }

    // changes state 
    pub fn set_solution(&mut self, solution:String){
        self.crossword_solution =solution;
    }

    pub fn guess_solution(&mut self, solution:String) {   //solution is the user guess 
        if solution== self.crossword_solution {
            env::log_str("You guessed right")
        } else {
            env::log_str("Try again")
        }
        // Well, logging is ultimately captured inside blocks added to the blockchain. (More accurately, transactions are contained in chunks and chunks are contained in blocks. More info in the Nomicon spec.) So while it is not changing the data in the fields of the struct, it does cost some amount of gas to log, requiring a signed transaction by an account that pays for this gas.
    }
    
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
