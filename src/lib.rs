use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault,log,Promise};

const PUZZLE_NUMBER: u8 = 1;
// 5 Ⓝ in yoctoNEAR
const PRIZE_AMOUNT: u128 = 5_000_000_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
enum PuzzleStatus {
    Unsolved,
    Solved { memo: String }, //eg "Took me forever to get clue six!" (winners can write this)
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum AnswerDirection {
    Across,
    Down,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CoordinatePair {
    x: u8,
    y: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Answer {
    num: u8,
    start: CoordinatePair,      //struct
    direction: AnswerDirection, //enum
    length: u8,
    clue: String,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Puzzle {
    status: PuzzleStatus,
    answer: Vec<Answer>,
}

#[derive(Serialize, Deserialize)] //json serializing
#[serde(crate = "near_sdk::serde")]
pub struct JsonPuzzle {
    //for returning data to frontend
    solution_hash: String,
    status: PuzzleStatus,
    answer: Vec<Answer>,
}

// Remember that there will be only one struct that gets the #[near_bindgen] macro placed on it; our primary struct or singleton
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Crossword {
    // SETUP Crossword STATE
    owner_id: AccountId,
    puzzles: LookupMap<String, Puzzle>, //non iterable,solution_hash and its puzzle
    unsolved_puzzles: UnorderedSet<String>, //like an interable list containing the solution hash
                                        // because cant iterate through puzzles to find unsolved ones (costs a lot of gas)
}

#[near_bindgen]
impl Crossword {
    #[init] //initialization macro for the Crossword
    pub fn new(owner_id: AccountId) -> Crossword {
        // When a contract gets complicated, there may be multiple different collections that are not all part of the main structure, but instead part of a sub-structure or nested collections. They all need to have unique prefixes.
        Crossword {
            owner_id,
            puzzles: LookupMap::new(b"c"), //unique prefix, best practice is to make a enum of it`
            unsolved_puzzles: UnorderedSet::new(b"u"),
        }
    }
    // ADD Crossword METHODS HERE
    pub fn get_puzzle_number(&self) -> u8 {
        PUZZLE_NUMBER
    }

    // for the owner to insert puzzles
    pub fn new_puzzle(&mut self, solution_hash: String, answers: Vec<Answer>) {
        // even a person with full access keys cannot call this method due to this check
        assert_eq!(
            env::predecessor_account_id(), //predecessor:- the person who most recently called this contract has to be the owner of the contract ,
            self.owner_id,
            "Only the owner of the contract may call this method"
        );

        // add the new unsolved puzzle in the existing list of puzzles

        // Inserts a key-value pair into the lookup map. If the map did not have this key present, None is returned.
        let existing = self.puzzles.insert(
            &solution_hash,
            &Puzzle {
                status: PuzzleStatus::Unsolved,
                answer: answers,
            },
        );

        // check for the key
        assert!(existing.is_none(), "Puzzle with that key already exists");

        // add the puzzle in the specefic unsolved list
        self.unsolved_puzzles.insert(&solution_hash);
    }

    // creates a transaction id while executing the script
    pub fn submit_solution(&mut self, solution: String, memo: String) {
        //solution is the user guess

        // convert the user solution to sha256 and then compare it
        let hashed_input = env::sha256(solution.as_bytes());
        let hashed_input_hex = hex::encode(&hashed_input);

        //check if the users hashed answer corresponding puzzle is in the lookup map
        let mut puzzle = self
            .puzzles
            .get(&hashed_input_hex)
            .expect("ERR_NOT_CORRECT_ANSWER");
        // Well, logging is ultimately captured inside blocks added to the blockchain. (More accurately, transactions are contained in chunks and chunks are contained in blocks. More info in the Nomicon spec.) So while it is not changing the data in the fields of the struct, it does cost some amount of gas to log, requiring a signed transaction by an account that pays for this gas.

        // Check if the puzzle is already solved. If it's unsolved, set the status to solved,
        //   then proceed to update the puzzle and pay the winner.
        puzzle.status = match puzzle.status {
            PuzzleStatus::Unsolved => PuzzleStatus::Solved { memo: memo.clone() },
            // for rest other cases err message
            _ => {
                env::panic_str("ERR_PUZZLE_SOLVED");
            }

        };

        //reinsert the puzzles after we have changed it 
        self.puzzles.insert(&hashed_input_hex,&puzzle);

        // remove from unsolved hashes 
        self.unsolved_puzzles.remove(&hashed_input_hex);

        log!(
            "Puzzle with solution hash {} solved, with memo: {}",
            hashed_input_hex,
            memo
        );

        // transfer the money to the winner 
        Promise::new(env::predecessor_account_id()).transfer(PRIZE_AMOUNT);
    }
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    //The important thing to remember is VMContext will be sending mock transactions with the context specified above. So if a unit test needs to send a test transaction coming from Alice, and then another from Bob, the get_context method may be called to change the signer_account_id or predecessor_account_id, or whatever the Crossword needs.

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

    // #[test]
    // fn check_guess_solution() {
    //     // get harsh's account id
    //     let harsh = AccountId::new_unchecked("harshrathi2511.testnet".to_string());
    //     //set the context and the test environment
    //     let context = get_context(harsh);
    //     testing_env!(context.build());

    //     //create the Crossword with the correct solution
    //     let mut Crossword = Crossword::new(
    //         "69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f".to_string(),
    //     );

    //     let guess_result = Crossword.guess_solution("near nomicon ref finance".to_string());
    //     assert!(guess_result, "this will pass for this");
    // }
}
