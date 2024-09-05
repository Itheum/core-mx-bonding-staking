PROXY=https://gateway.multiversx.com
CHAIN_ID="1"

ADDRESS=$(mxpy data load --key=address-mainnet)
DEPLOY_TRANSACTION=$(mxpy data load --key=deployTransaction-mainnet)

TOKEN="ITHEUM-df6f26"
TOKEN_HEX="0x$(echo -n ${TOKEN} | xxd -p -u | tr -d '\n')"

# to deploy from last reprodubible build, we need to change or vice versa
# --bytecode output/core-mx-liveliness-stake-sc.wasm \
# to 
# --bytecode output-docker/core-mx-liveliness-stake/core-mx-liveliness-stake.wasm \
deployMainnet(){
  mxpy --verbose contract deploy \
  --bytecode output-docker/core-mx-liveliness-stake/core-mx-liveliness-stake.wasm \
  --outfile deployOutput \
  --metadata-not-readable \
  --metadata-payable-by-sc \
  --proxy ${PROXY} \
  --chain ${CHAIN_ID} \
  --gas-limit 150000000 \
  --send \
  --recall-nonce \
  --ledger \
  --ledger-address-index 0 \
  --outfile="./interaction/deploy-mainnet.interaction.json" || return

  TRANSACTION=$(mxpy data parse --file="./interaction/deploy-mainnet.interaction.json" --expression="data['emittedTransactionHash']")
  ADDRESS=$(mxpy data parse --file="./interaction/deploy-mainnet.interaction.json" --expression="data['contractAddress']")

  mxpy data store --key=address-mainnet --value=${ADDRESS}
  mxpy data store --key=deployTransaction-mainnet --value=${TRANSACTION}
}

# any change to code or property requires a full upgrade 
# always check if you are deploy via a reprodubible build and that the code hash is the same before and after upgrade (that is if you are only changing props and not code.. for code, the RB will be different)
# if only changing props, you can't just "append" new props. you have to add the old ones again and then add a new prop you need. i.e. it's not append, it's a whole reset
# for upgrade, --outfile deployOutput is not needed
# in below code example we added --metadata-payable to add PAYABLE to the prop of the SC and removed --metadata-not-readable to make it READABLE
upgradeMainnet(){
  # $1 = address

  # address="0x$(mxpy wallet bech32 --decode ${1})"

  mxpy --verbose contract upgrade ${ADDRESS} \
  --bytecode output-docker/core-mx-liveliness-stake/core-mx-liveliness-stake.wasm \
  --metadata-not-readable \
  --metadata-payable-by-sc \
  # --arguments $address \
  --proxy ${PROXY} \
  --chain ${CHAIN_ID} \
  --gas-limit 150000000 \
  --recall-nonce \
  --ledger \
  --ledger-address-index 0 \
  --send || return
}

# if you interact without calling deploy(), then you need to 1st run this to restore the vars from data
restoreDeployDataMainnet() {
  TRANSACTION=$(mxpy data parse --file="./interaction/deploy-mainnet.interaction.json" --expression="data['emittedTransactionHash']")
  ADDRESS=$(mxpy data parse --file="./interaction/deploy-mainnet.interaction.json" --expression="data['contractAddress']")

  # after we upgraded to mxpy 8.1.2, mxpy data parse seems to load the ADDRESS correctly but it breaks when used below with a weird "Bad address" error
  # so, we just hardcode the ADDRESS here. Just make sure you use the "data['contractAddress'] from the latest deploy-mainnet.interaction.json file
  ADDRESS="erd1qqqqqqqqqqqqqpgq65rn8zmf2tckftpu5lvxg2pzlg0dhfrwc77qcuynw7"
}

setAdministratorMainnet(){
  # $1 = address

  address="0x$(mxpy wallet bech32 --decode ${1})"

  mxpy --verbose contract call ${ADDRESS} \
  --recall-nonce \
  --gas-limit=6000000 \
  --function "setAdministrator" \
  --arguments $address \
  --proxy ${PROXY} \
  --chain ${CHAIN_ID} \
  --ledger \
  --ledger-address-index 0 \
  --send || return
}

setBondContractAddressMainnet(){
  # $1 = address

  address="0x$(mxpy wallet bech32 --decode ${1})"

  mxpy --verbose contract call ${ADDRESS} \
  --recall-nonce \
  --gas-limit=6000000 \
  --function "setBondContractAddress" \
  --arguments $address \
  --proxy ${PROXY} \
  --chain ${CHAIN_ID} \
  --ledger \
  --ledger-address-index 0 \
  --send || return
}

setRewardsTokenIdentifierMainnet(){
  # $1 = token identifier

  token="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"

  mxpy --verbose contract call ${ADDRESS} \
  --recall-nonce \
  --gas-limit=6000000 \
  --function "setRewardsTokenIdentifier" \
  --arguments $token \
  --proxy ${PROXY} \
  --chain ${CHAIN_ID} \
  --ledger \
  --ledger-address-index 0 \
  --send || return
}

setPerBlockRewardAmountMainnet(){
  # $1 = amount (with token decimals)

  mxpy --verbose contract call ${ADDRESS} \
  --recall-nonce \
  --gas-limit=9000000 \
  --function "setPerBlockRewardAmount" \
  --arguments $1 \
  --proxy ${PROXY} \
  --chain ${CHAIN_ID} \
  --ledger \
  --ledger-address-index 0 \
  --send || return
}

topUpRewardsMainnet(){
  # $1 = amount of esdt to send 

  # 14 is erd1djdd2fgdqdgmq5l75aluvqttpq7ra9umrwr8v0cdhwk9mnuk7kwsg9ak90

  method="0x$(echo -n "topUpRewards" | xxd -p -u | tr -d '\n')"

  mxpy --verbose contract call ${ADDRESS} \
  --recall-nonce \
  --gas-limit=100000000 \
  --function "ESDTTransfer" \
  --arguments ${TOKEN_HEX} $1 $method \
  --proxy ${PROXY} \
  --chain ${CHAIN_ID} \
  --ledger \
  --ledger-address-index 14 \
  --send || return
}

setContractStateActiveMainnet(){
  mxpy --verbose contract call ${ADDRESS} \
  --recall-nonce \
  --gas-limit=6000000 \
  --function "setContractStateActive" \
  --proxy ${PROXY} \
  --chain ${CHAIN_ID} \
  --ledger \
  --ledger-address-index 0 \
  --send || return
}

setContractStateInactiveMainnet(){
  mxpy --verbose contract call ${ADDRESS} \
  --recall-nonce \
  --gas-limit=6000000 \
  --function "setContractStateInactive" \
  --proxy ${PROXY} \
  --chain ${CHAIN_ID} \
  --ledger \
  --ledger-address-index 0 \
  --send || return
}

withdrawRewardsMainnet(){
  # $1 = amount of esdt to receive

  mxpy --verbose contract call ${ADDRESS} \
  --recall-nonce \
  --gas-limit=6000000 \
  --function "withdrawRewards" \
  --arguments $1 \
  --proxy ${PROXY} \
  --chain ${CHAIN_ID} \
  --ledger \
  --ledger-address-index 0 \
  --send || return
}

startProduceRewardsMainnet(){
  mxpy --verbose contract call ${ADDRESS} \
  --recall-nonce \
  --gas-limit=6000000 \
  --function "startProduceRewards" \
  --proxy ${PROXY} \
  --chain ${CHAIN_ID} \
  --ledger \
  --ledger-address-index 0 \
  --send || return
}

endProduceRewardsMainnet(){
  mxpy --verbose contract call ${ADDRESS} \
  --recall-nonce \
  --gas-limit=6000000 \
  --function "endProduceRewards" \
  --proxy ${PROXY} \
  --chain ${CHAIN_ID} \
  --ledger \
  --ledger-address-index 0 \
  --send || return
}

setMaxAprMainnet(){
  # $1 = max apr (10000 = 100%)

  mxpy --verbose contract call ${ADDRESS} \
  --recall-nonce \
  --gas-limit=60000000 \
  --function "setMaxApr" \
  --arguments $1 \
  --proxy ${PROXY} \
  --chain ${CHAIN_ID} \
  --ledger \
  --ledger-address-index 0 \
  --send || return
 }
