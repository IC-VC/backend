#!/bin/bash

export NETWORK=$1
export PROPOSER_NEURON_ID=$2
export TARGET_CANISTER_ID=$3

. ./sns/scripts/utils/setup_env.sh "$NETWORK" "$PROPOSER_NEURON_ID"

TITLE="Update ICVC canister config"
URL="https://ic-vc.com/"
SUMMARY="This proposal alows to update canister configuration."
FUNCTION_ID=2001
FUNCTION_NAME="Update canister config"
FUNCTION_DESC="Update canister configuration"
TARGET_CANISTER_ID="$TARGET_CANISTER_NAME" 
TARGET_METHOD_NAME="update_canister_config"
VALIDATOR_CANISTER_ID="$TARGET_CANISTER_NAME"
VALIDATOR_METHOD_NAME="validate_update_canister_config"


./sns/scripts/utils/create_custom_proposal.sh "$TITLE" "$URL" "$SUMMARY" "$FUNCTION_ID" "$FUNCTION_NAME" "$FUNCTION_DESC" "$TARGET_CANISTER_ID" "$TARGET_METHOD_NAME" "$VALIDATOR_CANISTER_ID" "$VALIDATOR_METHOD_NAME"
