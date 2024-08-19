#!/bin/bash


export NETWORK=$1
export PROPOSER_NEURON_ID=$2
export TARGET_CANISTER_ID=$3

. ./sns/scripts/utils/setup_env.sh "$NETWORK" "$PROPOSER_NEURON_ID"


TITLE="Register icvc backend canister on sns"
URL="https://ic-vc.com/"
SUMMARY="Register the icvc backend canister on sns."

echo "Register $TARGET_CANISTER_ID with SNS"

PROPOSAL="(record { \
    title=\"$TITLE\"; \
    url=\"$URL\"; \
    summary=\"$SUMMARY\"; \
    action=opt variant { \
        RegisterDappCanisters = record { \
            canister_ids=vec {principal \"$TARGET_CANISTER_ID\"} \
        } \
    } \
})"


./sns/scripts/utils/submit_proposal.sh "$PROPOSAL"