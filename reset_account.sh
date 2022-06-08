#!/bin/bash

chmod +x reset_account.sh

echo ""
echo "deleting subaccount crossword.harshrathi2511.testnet and sending the rest of NEAR to master account"

near delete crossword.harshrathi2511.testnet harshrathi2511.testnet
# deleting account also creates a tx for the masterAccount 

echo ""
echo "sub-account deleted"