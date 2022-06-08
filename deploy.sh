#!/bin/bash 
set -e

chmod +x deploy.sh

# echo "enter masterAccount..."
# read masterAccount
# echo "master account is $masterAccount"

echo ""
echo "building wasm file to be deployed........."
echo ""
# giving permissions to the build script to execute 
chmod +x build.sh   
./build.sh

echo "creating a new testnet subaccount of harshrathi2511.testnet"

# should ideally create a random testnet account and then deploy to it  
near create-account crossword.harshrathi2511.testnet --masterAccount harshrathi2511.testnet
echo ""

echo "viewing the state of the subaccount crossword.harshrathi2511.testnet"
echo ""

echo "if the code hash has all ones :- NO CONTRACT DEPLOYED YET "
echo ""

near state crossword.harshrathi2511.testnet
echo ""

echo "deploying the contract (wasm file) to the subaccount ...using batch actions to #[init] "
echo ""

# using batch actions to instantiate the function 
near deploy crossword.harshrathi2511.testnet --wasmFile res/my_crossword.wasm \
 --initFunction 'new'  \
 --initArgs '{"owner_id": "crossword.harshrathi2511.testnet"}'
 
echo ""

echo " checking if the code hashes are all ones.........."
echo ""

near state crossword.harshrathi2511.testnet 
echo ""

echo "contract deployed successfully    "
echo ""