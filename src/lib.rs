use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

const PUZZLE_NUMBER: u8 = 1;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // SETUP CONTRACT STATE
    crossword_solution: String,
}

#[near_bindgen]
impl Contract {
    #[init] //initialization macro for the contract
    pub fn new(solution: String) -> Contract {
        Contract {
            crossword_solution: solution,
        }
    }
    // ADD CONTRACT METHODS HERE
    pub fn get_puzzle_number(&self) -> u8 {
        PUZZLE_NUMBER
    }

    // changes state
    pub fn set_solution(&mut self, solution: String) {
        self.crossword_solution = solution;
    }

    //creates a transaction id while executing the script
    pub fn guess_solution(&mut self, solution: String)->bool {
        //solution is the user guess

        // convert the user solution to sha256 and then compare it 
        let hashed_input = env::sha256(solution.as_bytes());
        let hashed_input_hex = hex::encode(&hashed_input);

        if hashed_input_hex == self.crossword_solution {
            env::log_str("You guessed right");
            true

        } else {
            env::log_str("Try again for winning the crossword puzzle");
            false
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

    //The important thing to remember is VMContext will be sending mock transactions with the context specified above. So if a unit test needs to send a test transaction coming from Alice, and then another from Bob, the get_context method may be called to change the signer_account_id or predecessor_account_id, or whatever the contract needs.

    // here the predecessor id is in our control for the tests
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut vm_ctx_builder = VMContextBuilder::new();
        vm_ctx_builder.predecessor_account_id(predecessor);
        vm_ctx_builder
    }

    #[test]
    fn debug_get_hash() {
        // setup for a unit test
        testing_env!(VMContextBuilder::new().build());

        let debug_solution = "near nomicon ref finance";
        // sha256 encoding of the solution
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        println!("{:?}", debug_hash_bytes); //this hash implements the debig trait not the display one
                                            // convert to hex format
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("{:?}", debug_hash_string);
        // Hash of the correct solution
        //"69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f"
    }

    #[test]
    fn check_guess_solution() {
        // get harsh's account id
        let harsh = AccountId::new_unchecked("harshrathi2511.testnet".to_string());
        //set the context and the test environment
        let context = get_context(harsh);
        testing_env!(context.build());

        //create the contract with the correct solution
        let mut contract = Contract::new(
            "69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f".to_string(),
        );

        let  guess_result = contract.guess_solution("near nomicon ref finance".to_string());

        assert!(guess_result,"this will pass for this");
    }
}
