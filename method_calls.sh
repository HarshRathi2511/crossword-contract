#!/bin/bash 
set -e
chmod +x method_calls.sh

# does not require an --accountId flag as we are not setting up a transaction , anybody can view 
echo "viewing puzzle number..... "
echo ""
near view crossword.harshrathi2511.testnet get_puzzle_number

echo ""
echo "setting answers along with the clues  on NEAR......"

near call crossword.harshrathi2511.testnet new_puzzle '{
  "solution_hash": "69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f",  
  "answers": [
   {
     "num": 1,
     "start": {
       "x": 1,
       "y": 1
     },
     "direction": "Down",
     "length": 5,
     "clue": "NFT market on NEAR that specializes in cards and comics."
   },
   {
     "num": 2,
     "start": {
       "x": 0,
       "y": 2
     },
     "direction": "Across",
     "length": 13,
     "clue": "You can move assets between NEAR and different chains, including Ethereum, by visiting ______.app"
   },
   {
     "num": 3,
     "start": {
       "x": 9,
       "y": 1
     },
     "direction": "Down",
     "length": 8,
     "clue": "NFT market on NEAR with art, physical items, tickets, and more."
   },
   {
     "num": 4,
     "start": {
       "x": 3,
       "y": 8
     },
     "direction": "Across",
     "length": 9,
     "clue": "The smallest denomination of the native token on NEAR."
   },
   {
     "num": 5,
     "start": {
       "x": 5,
       "y": 8
     },
     "direction": "Down",
     "length": 3,
     "clue": "You typically deploy a smart contract with the NEAR ___ tool."
   }
  ]
}' --accountId crossword.harshrathi2511.testnet


echo ""
echo "calling submit solution where signer_id is harshrathi2511.testnet "
near call crossword.harshrathi2511.testnet submit_solution '{"solution": "near nomicon ref finance" , "memo": "Yayy I won!"}' --accountId harshrathi2511.testnet