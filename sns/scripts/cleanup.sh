#!/usr/bin/env bash

# Define the paths to the generated files and directories
INIT_CONFIG_FILE="./sns/sns_init_test_flight.yaml"
DFX_JSON_FILE="dfx.json"
DFX_JSON_BACKUP_FILE="dfx.json.bak"
IDENTITY_NAME="SECOND_PRINCIPAL"
NEURONS_IDS="neuron_info.json"
CANISTER_IDS_FILE="./sns_canister_ids.json"

# Function to check and remove a file or directory
remove_if_exists() {
    local path=$1
    if [[ -e "$path" ]]; then
        rm -rf "$path"
        echo "Removed $path"
    else
        echo "$path does not exist"
    fi
}

# Function to remove old neurons from the init config file
remove_old_neurons() {
    if [[ -e "$INIT_CONFIG_FILE" ]]; then
        sed -i '' -e '/^    Neurons:/,/^    InitialBalances:/{
            /^    Neurons:/!{
                /^    InitialBalances:/!d
            }
        }' "$INIT_CONFIG_FILE"
        echo "Old neurons removed from $INIT_CONFIG_FILE"
    else
        echo "$INIT_CONFIG_FILE does not exist"
    fi
}

# Restore dfx.json file
restore_dfx_json() {
    if [[ -e "$DFX_JSON_BACKUP_FILE" ]]; then
        mv "$DFX_JSON_BACKUP_FILE" "$DFX_JSON_FILE"
        echo "Restored $DFX_JSON_FILE from $DFX_JSON_BACKUP_FILE"
    else
        echo "$DFX_JSON_BACKUP_FILE does not exist"
    fi
}

remove_old_neurons

remove_if_exists "ic-icrc1-ledger.wasm"
remove_if_exists "ic-icrc1-index.wasm"
remove_if_exists "ic-icrc1-index-ng.wasm"
remove_if_exists "ic-icrc1-archive.wasm"
remove_if_exists "sns-governance-canister.wasm"
remove_if_exists "sns-root-canister.wasm"
remove_if_exists "sns-swap-canister.wasm"
remove_if_exists "msg.json"
remove_if_exists "$NEURONS_IDS"
remove_if_exists "$CANISTER_IDS_FILE"

if dfx identity list | grep -q "${IDENTITY_NAME}"; then
    dfx identity remove "${IDENTITY_NAME}"
    echo "Identity ${IDENTITY_NAME} removed."
else
    echo "Identity ${IDENTITY_NAME} does not exist."
fi

restore_dfx_json

echo "Cleanup completed."
