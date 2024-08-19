#!/bin/bash

# Set current directory to the directory this script is in
SCRIPT=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
cd $SCRIPT_DIR

echo "NETWORK: $NETWORK"

# Extract the proposal
PROPOSAL=$1

echo "$PROPOSAL"

# Make the proposal using quill
quill sns --canister-ids-file ./sns_canister_ids.json --pem-file $PEM_FILE \
    make-proposal --proposal "$PROPOSAL" "$PROPOSER_NEURON_ID" > msg.json


if [ "$NETWORK" = "local" ]; then
    quill --insecure-local-dev-mode send msg.json

elif [ "$NETWORK" = "ic" ]; then
    quill send msg.json
fi

rm -f msg.json