#!/bin/bash

export TARGET_CANISTER_ID="$1"

TITLE="Register generic function on ICVC backend canister"

SUMMARY="This proposal registers a generic function to vote on projects."
FUNCTION_ID=4001
FUNCTION_NAME="Project voting"
FUNCTION_DESC="Allow voting on ICVC projects"
TARGET_CANISTER_ID="$TARGET_CANISTER_ID" 
TARGET_METHOD_NAME="execute_project_vote_proposal"
VALIDATOR_CANISTER_ID="$TARGET_CANISTER_ID"
VALIDATOR_METHOD_NAME="validate_project_vote_proposal"

./sns/scripts/utils/create_custom_proposal.sh "$TITLE" "$URL" "$SUMMARY" "$FUNCTION_ID" "$FUNCTION_NAME" "$FUNCTION_DESC" "$TARGET_CANISTER_ID" "$TARGET_METHOD_NAME" "$VALIDATOR_CANISTER_ID" "$VALIDATOR_METHOD_NAME"
