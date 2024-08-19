#!/bin/bash

# Check if the first argument is provided and is either "local" or "ic"
if [ -z "$1" ] || { [ "$1" != "local" ] && [ "$1" != "ic" ]; }; then
  echo "Usage: $0 <local|ic>"
  exit 1
fi

NETWORK=$1
PROPOSER_NEURON_ID=$2

# Set default values based on the environment
if [ "$NETWORK" = "local" ]; then
  NETWORK="local"
  IC_URL="http://localhost:8000"
  ICVC_BACKEND_CANISTER="$(dfx canister id "icvc_backend")"

elif [ "$NETWORK" = "ic" ]; then
  NETWORK="ic"
  IC_URL="https://ic0.app"
  ICVC_BACKEND_CANISTER="zwxjr-7aaaa-aaaap-qhodq-cai"
fi

IDENTITY=$(dfx identity whoami)
PEM_FILE="$HOME/.config/dfx/identity/$(dfx identity whoami)/identity.pem"
WASM_FOLDER="./target/wasm32-unknown-unknown/release"

# Export the variables to make them available in the environment
export NETWORK
export IDENTITY
export IC_URL
export PROPOSER_NEURON_ID
export PEM_FILE
export WASM_FOLDER
export ICVC_BACKEND_CANISTER

# Print the values of the environment variables
echo "NETWORK: $NETWORK"
echo "IDENTITY: $IDENTITY"
echo "IC_URL: $IC_URL"
echo "PROPOSER_NEURON_ID: $PROPOSER_NEURON_ID"
echo "PEM_FILE: $PEM_FILE"
echo "WASM_FOLDER: $WASM_FOLDER"

# Call the script to generate SNS canister IDs
./sns/scripts/utils/create_sns_canister_ids.sh "$NETWORK"
