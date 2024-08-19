#!/bin/bash

export NETWORK=$1
export PROPOSER_NEURON_ID=$2
export TARGET_CANISTER_ID=$3
export NEW_CONTROLLERS=$4 

. ./sns/scripts/utils/setup_env.sh "$NETWORK" "$PROPOSER_NEURON_ID"

TITLE="Deregister ICVC backend canister from SNS"
URL="https://ic-vc.com/"
SUMMARY="This proposal will deregister the ICVC backend canister from SNS."

echo "Deregister $TARGET_CANISTER_ID from SNS"

PROPOSAL="(record { \
    title=\"$TITLE\"; \
    url=\"$URL\"; \
    summary=\"$SUMMARY\"; \
    action=opt variant { \
        DeregisterDappCanisters = record { \
            canister_ids=vec {principal \"$TARGET_CANISTER_ID\"}; \
            new_controllers=vec {principal \"$NEW_CONTROLLERS\"} \
        } \
    } \
})"

./sns/scripts/utils/submit_proposal.sh "$PROPOSAL"
