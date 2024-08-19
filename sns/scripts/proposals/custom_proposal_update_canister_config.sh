#!/bin/bash

export TARGET_CANISTER_NAME="$1"
export CANISTER_ID="$(dfx canister id "$TARGET_CANISTER_NAME")"


TITLE="Update ICVC canister config"
URL="https://ic-vc.com/"
SUMMARY="This proposal alows to update canister configuration."
FUNCTION_ID=2001
FUNCTION_NAME="Update canister config"
FUNCTION_DESC="Update canister configuration"
TARGET_CANISTER_ID="$CANISTER_ID" 
TARGET_METHOD_NAME="update_canister_config"
VALIDATOR_CANISTER_ID="$CANISTER_ID"
VALIDATOR_METHOD_NAME="validate_update_canister_config"


./sns/scripts/utils/create_custom_proposal.sh "$TITLE" "$URL" "$SUMMARY" "$FUNCTION_ID" "$FUNCTION_NAME" "$FUNCTION_DESC" "$TARGET_CANISTER_ID" "$TARGET_METHOD_NAME" "$VALIDATOR_CANISTER_ID" "$VALIDATOR_METHOD_NAME"
