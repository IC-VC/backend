#!/bin/bash

# Set current directory to the directory this script is in
SCRIPT=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
cd $SCRIPT_DIR

# Extract the args
TARGET_CANISTER_ID=$1
ICVC_BACKEND_CANISTER_NAME=$2
WASM_LOCATION=$3
VERSION=$4
TITLE=$5
URL=$6
SUMMARY=$7
UPGRADE_ARG=$8

echo "NETWORK: $NETWORK"


# Build the WASM path
WASM_FILE=$ICVC_BACKEND_CANISTER_NAME.wasm.gz
WASM_PATH=$WASM_LOCATION/$WASM_FILE

WASM_PATH="/home/costa/Documents/icvc/backend/wasm/icvc_backend.wasm.gz"

if [ -z "$UPGRADE_ARG" ]
then
    # Parse the version string
    IFS='.' read -ra VERSION_PARTS <<< "$VERSION"
    MAJOR=${VERSION_PARTS[0]}
    MINOR=${VERSION_PARTS[1]}
    BUILD=${VERSION_PARTS[2]}

    # Build the canister-upgrade-arg
    UPGRADE_ARG="(record { wasm_version = record { major=$MAJOR:nat32; minor=$MINOR:nat32; patch=$BUILD:nat32 } })"
fi

# Make the proposal using quill
quill sns --canister-ids-file ./sns_canister_ids.json \
    --pem-file $PEM_FILE make-upgrade-canister-proposal \
    --canister-upgrade-arg "$UPGRADE_ARG" \
    --title "$TITLE" --url "$URL" --summary "$SUMMARY" \
    --target-canister-id $TARGET_CANISTER_ID \
    --wasm-path $WASM_PATH $PROPOSER_NEURON_ID > msg.json

if [ "$NETWORK" = "local" ]; then
    quill --insecure-local-dev-mode send msg.json

elif [ "$NETWORK" = "ic" ]; then
    quill send msg.json
fi
rm -f msg.json