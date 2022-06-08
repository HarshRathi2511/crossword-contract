#!/bin/bash 
set -e

echo "creating a new testnet subaccount of harshrathi2511.testnet"

# should ideally create a random testnet account and then deploy to it  
near create-account crossword.harshrathi2511.testnet --masterAccount harshrathi2511.testnet

echo "viewing the state of the subaccount crossword.harshrathi2511.testnet"
echo ""

echo "if the code hash has all ones :- NO CONTRACT DEPLOYED YET "
echo ""

near state crossword.harshrathi2511.testnet
echo ""

echo "deploying the contract (wasm file) to the subaccount "
echo ""

near deploy crossword.harshrathi2511.testnet --wasmFile res/my_crossword.wasm 
echo ""

echo " checking if the code hashes are all ones.........."
echo ""

near state crossword.harshrathi2511.testnet 
echo ""

echo "contract deployed successfully    "
echo ""