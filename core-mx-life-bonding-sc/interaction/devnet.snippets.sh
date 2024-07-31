PROXY=https://devnet-gateway.multiversx.com
CHAIN_ID="D"

WALLET="../wallet_dev.pem"
USER="./wallet2.pem"

ADDRESS=$(mxpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(mxpy data load --key=deployTransaction-devnet)

TOKEN="ITHEUM-fce905"
TOKEN_HEX="0x$(echo -n ${TOKEN} | xxd -p -u | tr -d '\n')"

# to deploy from last reprodubible build, we need to change or vice versa
# --bytecode output/core-mx-life-bonding-sc.wasm \
# to 
# --bytecode output-docker/core-mx-life-bonding-sc/core-mx-life-bonding-sc.wasm \
deploy(){
    mxpy --verbose contract deploy \
    --bytecode output/core-mx-life-bonding-sc.wasm \
    --outfile deployOutput \
    --metadata-not-readable \
    --metadata-payable-by-sc \
    --pem ${WALLET} \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --gas-limit 150000000 \
    --send \
    --recall-nonce \
    --outfile="./interaction/deploy-devnet.interaction.json" || return

    TRANSACTION=$(mxpy data parse --file="./interaction/deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(mxpy data parse --file="./interaction/deploy-devnet.interaction.json" --expression="data['contractAddress']")

    mxpy data store --key=address-devnet --value=${ADDRESS}
    mxpy data store --key=deployTransaction-devnet --value=${TRANSACTION}
}

# any change to code or property requires a full upgrade 
# always check if you are deploy via a reprodubible build and that the code hash is the same before and after upgrade (that is if you are only changing props and not code.. for code, the RB will be different)
# if only changing props, you can't just "append" new props. you have to add the old ones again and then add a new prop you need. i.e. it's not append, it's a whole reset
# for upgrade, --outfile deployOutput is not needed
# in below code example we added --metadata-payable to add PAYABLE to the prop of the SC and removed --metadata-not-readable to make it READABLE
upgrade(){
    mxpy --verbose contract upgrade ${ADDRESS} \
    --bytecode output/core-mx-life-bonding-sc.wasm \
    --metadata-not-readable \
    --metadata-payable-by-sc \
    --pem ${WALLET} \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --gas-limit 150000000 \
    --recall-nonce \
    --send || return
}

# if you interact without calling deploy(), then you need to 1st run this to restore the vars from data
restoreDeployData() {
  TRANSACTION=$(mxpy data parse --file="./interaction/deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
  ADDRESS=$(mxpy data parse --file="./interaction/deploy-devnet.interaction.json" --expression="data['contractAddress']")

  # after we upgraded to mxpy 8.1.2, mxpy data parse seems to load the ADDRESS correctly but it breaks when used below with a weird "Bad address" error
  # so, we just hardcode the ADDRESS here. Just make sure you use the "data['contractAddress'] from the latest deploy-devnet.interaction.json file
  ADDRESS="erd1qqqqqqqqqqqqqpgqhlyaj872kyh620zsfew64l2k4djerw2tfsxsmrxlan"
}

setAdministrator(){
    # $1 = address

    address="0x$(mxpy wallet bech32 --decode ${1})"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setAdministrator" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setContractStateActive(){
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setContractStateActive" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setContractStateInactive(){
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setContractStateInactive" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setAcceptedCallers(){
    # $1 = address

    address="0x$(mxpy wallet bech32 --decode ${1})"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setAcceptedCallers" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setBondToken(){
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setBondToken" \
    --arguments $TOKEN_HEX \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setPeriodsBonds(){    
    # $1 = lockPeriod
    # $2 = bond

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setPeriodsBonds" \
    --arguments $1 $2 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setMinimumPenalty(){
    # $1 = minimumPenalty

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setMinimumPenalty" \
    --arguments $1 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setMaximumPenalty(){ 
    # $1 = maximumPenalty

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setMaximumPenalty" \
    --arguments $1 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setWithdrawPenalty(){
    # $1 = withdrawPenalty

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setWithdrawPenalty" \
    --arguments $1 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

sanction(){
    # $1 = token identifier
    # $2 = nonce
    # $3 = penalty (0=minimum, 1=custom, 2=maximum)
    # $4 = custom penalty (if $3=1)

    token_identifier="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"
    penalty=$3
    custom_penalty=$4
    
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "sanction" \
    --arguments $token_identifier $2 $penalty $custom_penalty \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

modifyBond(){
    # $1 = token identifier
    # $2 = nonce

    token_identifier="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "modifyBond" \
    --arguments $token_identifier $2 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

withdraw(){
    # $1 = token identifier
    # $2 = nonce 

    token_identifier="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${USER} \
    --gas-limit=20000000 \
    --function "withdraw" \
    --arguments $token_identifier $2 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

renew(){    
    # $1 = token identifier
    # $2 = nonce 

    token_identifier="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${USER} \
    --gas-limit=6000000 \
    --function "renew" \
    --arguments $token_identifier $2 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

renewWithNewLockPeriod(){     
    # $1 = token identifier
    # $2 = nonce 
    # $3 = lockPeriod

    token_identifier="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${USER} \
    --gas-limit=6000000 \
    --function "renew" \
    --arguments $token_identifier $2 $3 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

initiateBond() {
    # $1 = the address for who the bond is initiated
    # $2 = token identifier
    # $3 = nonce

    address="0x$(mxpy wallet bech32 --decode ${1})"
    token_identifier="0x$(echo -n ${2} | xxd -p -u | tr -d '\n')"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=20000000 \
    --function "initiateBond" \
    --arguments $address $token_identifier $3 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}


setLivelinessStakeAddress(){

    # $1 = address

    address="0x$(mxpy wallet bech32 --decode ${1})"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setLivelinessStakeAddress" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return

}


setTopUpAdministrator(){

    # $1 = address

    address="0x$(mxpy wallet bech32 --decode ${1})"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setTopUpAdministrator" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}