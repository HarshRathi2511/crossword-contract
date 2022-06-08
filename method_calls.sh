#!/bin/bash 
set -e

# does not require an --accountId flag as we are not setting up a transaction , anybody can view 
echo "viewing puzzle number..... "
echo ""
near view crossword.harshrathi2511.testnet get_puzzle_number

echo ""
echo "setting solution (and creating a transaction on NEAR......"
near call crossword.harshrathi2511.testnet set_solution '{"solution" : "near nomicon ref finance"}' ----accountId harshrathi2511.testnet

echo ""
echo "guessing solution.... "
near call crossword.harshrathi2511.testnet guess_solution '{"solution" : "near nomicon ref finance"}' --accountId harshrathi2511.testnet 
# near call <account that deployed the contract> <method name> <data> --accountId <account signing the tx>


echo ""
echo "testing incorrect solution .... "
near call crossword.harshrathi2511.testnet guess_solution '{"solution" : "lmao ded"}' --accountId harshrathi2511.testnet 