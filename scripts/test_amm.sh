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


elysd tx wasm store artifacts/trade_shield_contract.wasm --from=treasury --keyring-backend=test --chain-id=elystestnet-1 --gas=auto --gas-adjustment=1.3 -y -b=sync
sleep 2
instantiate_hash=$(extract_txhash elysd tx wasm instantiate 1 '{"process_order_executor": "elys12tzylat4udvjj56uuhu3vj2n4vgp7cf9fwna9w"}' --from=treasury --label "Contract" --chain-id=elystestnet-1 --gas=auto --gas-adjustment=1.3 -b=sync --keyring-backend=test --no-admin -y)  
sleep 2
addr=$(extract_contract_address elysd q tx $instantiate_hash)
# elysd q wasm contract-state smart $addr  '{"get_all_prices": {}}'
sleep 2
elysd tx amm create-pool 100uatom,100uusdc 100000000000uatom,100000000000uusdc --swap-fee=0.00 --exit-fee=0.00 --use-oracle=false  --from=treasury --keyring-backend=test --chain-id=elystestnet-1 --yes --gas=1000000
sleep 2
elysd tx wasm exec $addr '{"create_spot_order": {"order_amm_routes" : [{"pool_id" : 1 , "token_out_denom" : "uatom"}], "order_price" : {"base_denom": "uusdc","quote_denom" : "uatom" ,"rate" : "1"}, "order_type" : "stop_loss", "order_target_denom" : "uatom", "order_source_denom" : "uusdc"}}' --from treasury --gas-prices 0.25uelys --gas auto --gas-adjustment 1.3 -b sync -y  --keyring-backend=test --chain-id=elystestnet-1 --amount=200uusdc
sleep 2
balance_before=$(elysd q bank balances $addr)
#NOT WORKING ANYMORE
# sleep 2
# elysd tx wasm exec $addr '{"process_spot_orders": {}}' --from treasury --gas-prices 0.25uelys --gas auto --gas-adjustment 1.3 -b sync -y  --keyring-backend=test --chain-id=elystestnet-1
# sleep 2
# elysd tx wasm exec $addr '{"process_spot_orders": {}}' --from treasury --gas-prices 0.25uelys --gas auto --gas-adjustment 1.3 -b sync -y  --keyring-backend=test --chain-id=elystestnet-1
