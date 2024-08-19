#!/bin/bash

NETWORK=$1
PROPOSER_NEURON_ID=$2

. ./sns/scripts/utils/setup_env.sh "$NETWORK" "$PROPOSER_NEURON_ID"

# Assign arguments to variables with meaningful names
FUNCTION_ID="2001"
TITLE="Update canister config"
SUMMARY="This proposal will update the icvc canister config."
URL="https://ic-vc.com/"

# Function Args
MAX_STABLE_MEMORY_SIZE="null"
SUBACCOUNT="896119f03e96f7644ad4af3cc3bf5539ebee4838af76530b201044385f2ccfac"
SNS_GOVERNANCE_ID=""

# Handle optional fields
SNS_GOVERNANCE_FIELD="opt null"

if [ -n "$SNS_GOVERNANCE_ID" ]; then
    SNS_GOVERNANCE_FIELD="opt principal \"$SNS_GOVERNANCE_ID\""
fi

# Handle the max_stable_memory_size field, adding "opt" if it's not null
if [ "$MAX_STABLE_MEMORY_SIZE" == "null" ]; then
    MAX_STABLE_MEMORY_SIZE_FIELD="opt null"
else
    MAX_STABLE_MEMORY_SIZE_FIELD="$MAX_STABLE_MEMORY_SIZE : nat64"
fi

# Handle the subaccount field, adding "opt" if it's empty
if [ -z "$SUBACCOUNT" ]; then
    SUBACCOUNT_FIELD="opt null"
else
    SUBACCOUNT_FIELD="\"$SUBACCOUNT\""
fi

# Construct the ARGS with proper Candid formatting
ARGS="(record {
    max_stable_memory_size = $MAX_STABLE_MEMORY_SIZE_FIELD;
    subaccount = $SUBACCOUNT_FIELD;
    sns_governance_id = $SNS_GOVERNANCE_FIELD
})"

echo "$ARGS"

./sns/scripts/utils/make_custom_execute_proposal.sh "$FUNCTION_ID" "$TITLE" "$SUMMARY" "$URL" "$ARGS"