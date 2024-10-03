#!/bin/bash


export NETWORK=$1
export PROPOSER_NEURON_ID=$2

. ./sns/scripts/utils/setup_env.sh "$NETWORK" "$PROPOSER_NEURON_ID"


TITLE="Upgrade SNS Root canister to next version"
URL="https://ic-vc.com/"
SUMMARY="We propose an upgrade of the SNS canisters to the latest version to ensure optimal performance and security"

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