extract_txhash() {
    output=$("$@")

    txhash_line=$(echo "$output" | grep -o 'txhash: [[:xdigit:]]*')

    txhash=${txhash_line##* }

    echo $txhash
}

extract_contract_address() {
    output=$("$@")

    contract_address=$(echo "$output" | awk -F 'key: _contract_address|value: ' '/key: _contract_address/ { getline; print $2; exit }')

    echo "$contract_address"
}


elysd tx wasm store artifacts/trade_shield_contract.wasm --from=treasury --keyring-backend=test --chain-id=elystestnet-1 --gas=auto --gas-adjustment=1.3 -y -b=sync  > /dev/null 2>&1
sleep 2
instantiate_hash=$(extract_txhash elysd tx wasm instantiate 1 '{}' --from=treasury --label "Contract" --chain-id=elystestnet-1 --gas=auto --gas-adjustment=1.3 -b=sync --keyring-backend=test --no-admin -y 2> /dev/null)  
sleep 2
addr=$(extract_contract_address elysd q tx $instantiate_hash) 
echo "contract addr:"
echo     "$addr"
elysd q wasm contract-state smart $addr  '{"get_all_prices": {}}'