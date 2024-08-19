#!/bin/bash

# Check if the environment is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <local|ic>"
  exit 1
fi

NETWORK=$1

# Get the directory of the current script
SCRIPT_DIR=$(dirname "$(readlink -f "$0")")

if [ "$NETWORK" = "ic" ]; then
  # Extract canister IDs from the sns root canister
  RESPONSE=$(dfx canister --network ic call nuywj-oaaaa-aaaaq-aadta-cai list_sns_canisters '(record {} )')
  
  GOVERNANCE_ID=$(echo "$RESPONSE" | grep -oP '(?<=governance = opt principal ")(.*?)(?=")')
  INDEX_ID=$(echo "$RESPONSE" | grep -oP '(?<=index = opt principal ")(.*?)(?=")')
  LEDGER_ID=$(echo "$RESPONSE" | grep -oP '(?<=ledger = opt principal ")(.*?)(?=")')
  ROOT_ID=$(echo "$RESPONSE" | grep -oP '(?<=root = opt principal ")(.*?)(?=")')
  SWAP_ID=$(echo "$RESPONSE" | grep -oP '(?<=swap = opt principal ")(.*?)(?=")')

  cat <<EOF > "$SCRIPT_DIR/sns_canister_ids.json"
{
  "governance_canister_id": "$GOVERNANCE_ID",
  "index_canister_id": "$INDEX_ID",
  "ledger_canister_id": "$LEDGER_ID",
  "root_canister_id": "$ROOT_ID",
  "swap_canister_id": "$SWAP_ID"
}
EOF

elif [ "$NETWORK" = "local" ]; then
  declare -A CANISTER_IDS

  CANISTERS=("sns_governance" "sns_index" "sns_ledger" "sns_root" "sns_swap")

  for canister in "${CANISTERS[@]}"; do
      CANISTER_IDS[$canister]=$(dfx -qq canister --network "$NETWORK" id "$canister")
  done

  cat <<EOF > "$SCRIPT_DIR/sns_canister_ids.json"
{
  "governance_canister_id": "${CANISTER_IDS[sns_governance]}",
  "index_canister_id": "${CANISTER_IDS[sns_index]}",
  "ledger_canister_id": "${CANISTER_IDS[sns_ledger]}",
  "root_canister_id": "${CANISTER_IDS[sns_root]}",
  "swap_canister_id": "${CANISTER_IDS[sns_swap]}"
}
EOF

else
  echo "Invalid environment specified."
  exit 1
fi

echo "SNS canister IDs have been saved to $SCRIPT_DIR/sns_canister_ids.json"
