#!/bin/bash


export NETWORK=$1
export PROPOSER_NEURON_ID=$2

. ./sns/scripts/utils/setup_env.sh "$NETWORK" "$PROPOSER_NEURON_ID"


TITLE="Upgrade SNS Root canister to next version"
URL="https://dashboard.internetcomputer.org/proposal/131807"
SUMMARY="Upgrade SNS Root canister wasm to the version contained in proposal 131807 (https://dashboard.internetcomputer.org/proposal/131807)."

echo "Proposal to upgrade SNS Root canister to next version"

PROPOSAL="(record { \
    title=\"$TITLE\"; \
    url=\"$URL\"; \
    summary=\"$SUMMARY\"; \
    action=opt variant { \
        UpgradeSnsToNextVersion = record {} \
    } \
})"


./sns/scripts/utils/submit_proposal.sh "$PROPOSAL"