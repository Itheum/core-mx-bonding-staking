PROXY=https://gateway.multiversx.com
CHAIN_ID="1"

ADDRESS=$(mxpy data load --key=address-mainnet)
DEPLOY_TRANSACTION=$(mxpy data load --key=deployTransaction-mainnet)

TOKEN="ITHEUM-df6f26"
TOKEN_HEX="0x$(echo -n ${TOKEN} | xxd -p -u | tr -d '\n')"

# to deploy from last reprodubible build, we need to change or vice versa
# --bytecode output/core-mx-life-bonding-sc.wasm \
# to 
# --bytecode output-docker/core-mx-life-bonding-sc/core-mx-life-bonding-sc.wasm \
deployLedgerMainnet(){
    mxpy --verbose contract deploy \
    --bytecode output-docker/core-mx-life-bonding-sc/core-mx-life-bonding-sc.wasm \
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
upgrade(){
    mxpy --verbose contract upgrade ${ADDRESS} \
    --bytecode output-docker/core-mx-life-bonding-sc/core-mx-life-bonding-sc.wasm \
    --metadata-not-readable \
    --metadata-payable-by-sc \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --gas-limit 150000000 \
    --recall-nonce \
    --ledger \
    --ledger-address-index 0 \
    --send || return
}

# if you interact without calling deploy(), then you need to 1st run this to restore the vars from data
restoreDeployDataLedgerMainnet(){
  TRANSACTION=$(mxpy data parse --file="./interaction/deploy-mainnet.interaction.json" --expression="data['emittedTransactionHash']")
  ADDRESS=$(mxpy data parse --file="./interaction/deploy-mainnet.interaction.json" --expression="data['contractAddress']")

  # after we upgraded to mxpy 8.1.2, mxpy data parse seems to load the ADDRESS correctly but it breaks when used below with a weird "Bad address" error
  # so, we just hardcode the ADDRESS here. Just make sure you use the "data['contractAddress'] from the latest deploy-devnet.interaction.json file
  ADDRESS="erd1qqqqqqqqqqqqqpgq9yfa4vcmtmn55z0e5n84zphf2uuuxxw9c77qgqqwkn"
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

setAcceptedCallersMainnet(){
    # $1 = address

    address="0x$(mxpy wallet bech32 --decode ${1})"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --gas-limit=6000000 \
    --function "setAcceptedCallers" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --ledger \
    --ledger-address-index 0 \
    --send || return
}

setBondTokenMainnet(){
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --gas-limit=6000000 \
    --function "setBondToken" \
    --arguments $TOKEN_HEX \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --ledger \
    --ledger-address-index 0 \
    --send || return
}

setPeriodsBondsMainnet(){    
    # $1 = lockPeriod
    # $2 = bond

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --gas-limit=6000000 \
    --function "setPeriodsBonds" \
    --arguments $1 $2 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --ledger \
    --ledger-address-index 0 \
    --send || return
}

setMinimumPenaltyMainnet(){
    # $1 = minimumPenalty

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --gas-limit=6000000 \
    --function "setMinimumPenalty" \
    --arguments $1 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --ledger \
    --ledger-address-index 0 \
    --send || return
}

setMaximumPenaltyMainnet(){ 
    # $1 = maximumPenalty

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --gas-limit=6000000 \
    --function "setMaximumPenalty" \
    --arguments $1 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --ledger \
    --ledger-address-index 0 \
    --send || return
}

setWithdrawPenaltyMainnet(){
    # $1 = withdrawPenalty

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --gas-limit=6000000 \
    --function "setWithdrawPenalty" \
    --arguments $1 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --ledger \
    --ledger-address-index 0 \
    --send || return
}

sanctionMainnet(){
    # $1 = token identifier
    # $2 = nonce
    # $3 = penalty (0=minimum, 1=custom, 2=maximum)
    # $4 = custom penalty (if $3=1)

    token_identifier="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"
    penalty=$3
    custom_penalty=$4
    
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --gas-limit=6000000 \
    --function "sanction" \
    --arguments $token_identifier $2 $penalty $custom_penalty \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --ledger \
    --ledger-address-index 0 \
    --send || return
}

modifyBondMainnet(){
    # $1 = token identifier
    # $2 = nonce

    token_identifier="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --gas-limit=6000000 \
    --function "modifyBond" \
    --arguments $token_identifier $2 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --ledger \
    --ledger-address-index 0 \
    --send || return
}

withdrawMainnet(){
    # $1 = token identifier
    # $2 = nonce 

    token_identifier="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --gas-limit=20000000 \
    --function "withdraw" \
    --arguments $token_identifier $2 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --ledger \
    --ledger-address-index 0 \
    --send || return
}

renewMainnet(){    
    # $1 = token identifier
    # $2 = nonce 

    token_identifier="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --gas-limit=6000000 \
    --function "renew" \
    --arguments $token_identifier $2 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --ledger \
    --ledger-address-index 0 \
    --send || return
}

renewWithNewLockPeriodMainnet(){     
    # $1 = token identifier
    # $2 = nonce 
    # $3 = lockPeriod

    token_identifier="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --gas-limit=6000000 \
    --function "renew" \
    --arguments $token_identifier $2 $3 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --ledger \
    --ledger-address-index 0 \
    --send || return
}

initiateBondMainnet() {
    # $1 = the address for who the bond is initiated
    # $2 = token identifier
    # $3 = nonce

    address="0x$(mxpy wallet bech32 --decode ${1})"
    token_identifier="0x$(echo -n ${2} | xxd -p -u | tr -d '\n')"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --gas-limit=20000000 \
    --function "initiateBond" \
    --arguments $address $token_identifier $3 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --ledger \
    --ledger-address-index 0 \
    --send || return
}
