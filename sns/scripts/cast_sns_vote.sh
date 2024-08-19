#!/usr/bin/env bash

set -euo pipefail

source "$(dirname "$0")/utils/neuron_utils.sh"
export IC_URL="http://localhost:8000"


PROPOSAL_ID="$1"
VOTE="$2"
NETWORK="${4:-local}"

NEURON_INFO_FILE="neuron_info.json"
IDENTITY_NAME="SECOND_PRINCIPAL" 
DEVELOPER_NEURON_ID=$(get_neuron_id "$IDENTITY_NAME" "$NEURON_INFO_FILE")

dfx identity use $IDENTITY_NAME

PEM_FILE="$HOME/.config/dfx/identity/$(dfx identity whoami)/identity.pem"
CANISTER_IDS_FILE="./sns_canister_ids.json"

hex_string="${DEVELOPER_NEURON_ID}"
subaccount="vec {"
for ((i=0; i<${#hex_string}; i+=2)); do
    byte="${hex_string:$i:2}"
    subaccount+=" $((16#$byte));"
done
subaccount+=" }"

MANAGE_NEURON_PAYLOAD=$(cat <<EOF
(record {
  subaccount = $subaccount;
  command = opt variant { RegisterVote = record {
    vote = $VOTE;
    proposal = opt record { id = $PROPOSAL_ID }
  }}
})
EOF
)

quill sns \
  --canister-ids-file "$CANISTER_IDS_FILE" \
  --pem-file "$PEM_FILE" \
  register-vote $DEVELOPER_NEURON_ID --proposal-id $PROPOSAL_ID --vote $VOTE > msg.json
    
quill --insecure-local-dev-mode send msg.json

dfx identity use default