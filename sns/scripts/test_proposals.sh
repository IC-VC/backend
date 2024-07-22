#!/usr/bin/env bash

set -euo pipefail

source "$(dirname "$0")/utils/neuron_utils.sh"
export IC_URL="http://localhost:8000"

NEURON_INFO_FILE="neuron_info.json"
IDENTITY_NAME="default" 
NETWORK="local"

dfx identity use $IDENTITY_NAME

PEM_FILE="$HOME/.config/dfx/identity/$(dfx identity whoami)/identity.pem"
DEVELOPER_NEURON_ID=$(get_neuron_id "$IDENTITY_NAME" "$NEURON_INFO_FILE")

echo "Extracted Neuron ID: $DEVELOPER_NEURON_ID"


export ICVC_BACKEND_ID="$(dfx canister id icvc_backend)"

# Add a hot key to the backend, to be able to submit proposals
./sns/scripts/add_canister_hot_key.sh "$DEVELOPER_NEURON_ID" "$ICVC_BACKEND_ID"

# Register dapp canister with SNS root
quill sns \
  --canister-ids-file ./sns_canister_ids.json \
  --pem-file "${PEM_FILE}" \
  make-proposal --proposal "(record { \
    title=\"Register dapp's canisters with SNS.\"; \
    url=\"https://example.com/\"; \
    summary=\"This proposal registers dapp's canisters with SNS.\"; \
    action=opt variant { \
      RegisterDappCanisters = record { \
        canister_ids = vec { principal \"$ICVC_BACKEND_ID\" } \
      } \
    } \
  })" $DEVELOPER_NEURON_ID > msg.json

quill send --insecure-local-dev-mode msg.json 

./sns/scripts/cast_sns_vote.sh "1" "y" # In the future get last proposal instead

# Register generic proposals
./sns/scripts/register_generic_proposal.sh "$DEVELOPER_NEURON_ID" "$ICVC_BACKEND_ID" "$NETWORK"
./sns/scripts/cast_sns_vote.sh "2" "y" # In the future get last proposal instead
