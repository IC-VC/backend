#!/usr/bin/env bash


export PEM_FILE="$HOME/.config/dfx/identity/$(dfx identity whoami)/identity.pem"
export IC_URL="http://localhost:8000"


export DEVELOPER_NEURON_ID="$1"
export NETWORK="${2:-local}"
export MAJORITY="${3:-true}"


echo "DEVELOPER_NEURON_ID: $DEVELOPER_NEURON_ID"
echo "NETWORK: $NETWORK"
echo "MAJORITY: $MAJORITY"

DFX_IC_COMMIT=94bbea43c7585a1ef970bd569a447c269af9650b dfx sns import
DFX_IC_COMMIT=94bbea43c7585a1ef970bd569a447c269af9650b dfx sns download


INIT_CONFIG_FILE="./sns/sns_init_test_flight.yaml"
CANISTER_IDS_FILE="./sns_canister_ids.json"
NEURON_INFO_FILE="neuron_info.json"
DFX_JSON_FILE="dfx.json"
DFX_JSON_BACKUP_FILE="dfx.json.bak"
IDENTITY_NAME="SECOND_PRINCIPAL"


CURRENT_PRINCIPAL=$(dfx identity get-principal)

# Function to get canister ID and update the JSON file
update_canister_id() {
    local canister_name=$1
    local key=$2
    local canister_id=$(dfx canister id "$canister_name")

    if [[ -f "$CANISTER_IDS_FILE" ]]; then
        # Update existing file
        jq --arg key "$key" --arg id "$canister_id" '.[$key] = $id' "$CANISTER_IDS_FILE" > "${CANISTER_IDS_FILE}.tmp" && mv "${CANISTER_IDS_FILE}.tmp" "$CANISTER_IDS_FILE"
    else
        # Create new file with initial canister ID
        jq -n --arg key "$key" --arg id "$canister_id" '{($key): $id}' > "$CANISTER_IDS_FILE"
    fi
}

# Backup dfx.json file
backup_dfx_json() {
    if [[ -e "$DFX_JSON_FILE" ]]; then
        cp "$DFX_JSON_FILE" "$DFX_JSON_BACKUP_FILE"
        echo "Backed up $DFX_JSON_FILE to $DFX_JSON_BACKUP_FILE"
    else
        echo "$DFX_JSON_FILE does not exist"
    fi
}

# Function to convert hex string to vector subaccount
convert_hex_to_subaccount() {
    local hex_string=$1
    local subaccount="vec {"
    for ((i=0; i<${#hex_string}; i+=2)); do
        byte="${hex_string:$i:2}"
        subaccount+=" $((16#$byte));"
    done
    subaccount+=" }"
    echo "$subaccount"
}


save_neuron_info() {
    local principal=$1
    local identity_name=$2
    local id=$3
    local subaccount=$4
    local info_file=$5

    if [ -f "$info_file" ]; then
        existing_data=$(cat "$info_file")
    else
        existing_data="[]"
    fi

    updated_data=$(echo "$existing_data" | jq --arg principal "$principal" --arg identity_name "$identity_name" --arg id "$id" --arg subaccount "$subaccount" \
    '. + [{principal: $principal, identity_name: $identity_name, neuron_id: $id, subaccount: $subaccount}]')

    echo "$updated_data" > "$info_file"

    echo "Neuron information saved to $info_file"
}

list_and_save_neurons() {
    local principal=$1
    local identity_name=$2
    local neuron_info_file=$3

    if [ -f "$neuron_info_file" ]; then
        existing_data=$(cat "$neuron_info_file")
    else
        existing_data="[]"
    fi

    # Check if the identity name already exists in the existing data
    identity_exists=$(echo "$existing_data" | jq --arg identity_name "$identity_name" 'map(select(.identity_name == $identity_name)) | length > 0')

    if [ "$identity_exists" == "true" ]; then
        echo "Identity name $identity_name already exists in $neuron_info_file"
        return
    fi

    # List neurons for the principal using the governance canister
    list_neurons_result=$(dfx canister --network $NETWORK call $governance_id list_neurons "(record {of_principal=opt principal\"$principal\"; limit=100; start_page_at=null})")

    echo "Neurons for principal ${principal}:"
    echo "$list_neurons_result"

    neuron_ids=$(echo "$list_neurons_result" | perl -n -e 'print "$1\n" if /id = blob "([^"]+)"/')
    echo "Neuron IDs: $neuron_ids"

    for neuron_id in $neuron_ids; do
        cleaned_neuron_id=$(echo "$neuron_id" | sed 's/\\//g')
        echo "Extracted Neuron ID: $cleaned_neuron_id"
        subaccount=$(convert_hex_to_subaccount "$cleaned_neuron_id")
        save_neuron_info "$principal" "$identity_name" "$cleaned_neuron_id" "$subaccount" "$neuron_info_file"
    done
}

# ---------START----------

backup_dfx_json

# Check if the neurons need to be added to the sns init file
if [[ "${NETWORK}" == "local" ]]; then
    if [[ "$MAJORITY" == "True" ]]; then
        # Add only the current principal with 1000 tokens
        sed -i'' -e "/^    Neurons:/a\\
        - principal: ${CURRENT_PRINCIPAL}\\
          stake: 1000 tokens\\
          memo: 0\\
          dissolve_delay: 2 years\\
          vesting_period: 4 years
        " "${INIT_CONFIG_FILE}"
    else
        if dfx identity list | grep -q "${IDENTITY_NAME}"; then
            echo "Identity ${IDENTITY_NAME} already exists."
        else
            echo "Creating new identity ${IDENTITY_NAME}."
            dfx identity new --storage-mode=plaintext "${IDENTITY_NAME}"
        fi

        dfx identity use "${IDENTITY_NAME}"
        SECOND_PRINCIPAL=$(dfx identity get-principal)

        dfx identity use default

        # Add the current principal and second principal
        sed -i'' -e "/^    Neurons:/a\\
        - principal: ${CURRENT_PRINCIPAL}\\
          stake: 500 tokens\\
          memo: 0\\
          dissolve_delay: 2 years\\
          vesting_period: 4 years\\
        - principal: ${SECOND_PRINCIPAL}\\
          stake: 500 tokens\\
          memo: 0\\
          dissolve_delay: 2 years\\
          vesting_period: 4 years
        " "${INIT_CONFIG_FILE}"
    fi
else
    echo "No action required for non-local network without the 'majority' flag."
fi

echo "Checked ${INIT_CONFIG_FILE} and updated if necessary."

dfx sns deploy-testflight --init-config-file="${INIT_CONFIG_FILE}" --network  "$NETWORK"

# Update all canister IDs
update_canister_id "sns_governance" "governance_canister_id"
update_canister_id "sns_index" "index_canister_id"
update_canister_id "sns_ledger" "ledger_canister_id"
update_canister_id "sns_root" "root_canister_id"
update_canister_id "sns_swap" "swap_canister_id"

cat "$CANISTER_IDS_FILE"

ledger_id=$(jq -r '.ledger_canister_id' "$CANISTER_IDS_FILE")
governance_id=$(jq -r '.governance_canister_id' "$CANISTER_IDS_FILE")

if [[ -z "$ledger_id" || -z "$governance_id" ]]; then
    echo "Failed to extract canister IDs from sns_canister_ids.json."
    exit 1
fi

echo "Ledger ID: $ledger_id"
echo "Governance ID: $governance_id"

#Since the deploy-testflight fails, we install the sns_index manually
#dfx canister install sns_index --argument "(record {ledger_id = principal \"$ledger_id\"})" --wasm ./ic-icrc1-index-ng.wasm

#if [[ $? -ne 0 ]]; then
#  echo "Failed to install sns_index canister."
#  exit 1
#fi

#echo "sns_index canister installed successfully with ledger_id: $ledger_id"

list_and_save_neurons "$CURRENT_PRINCIPAL" "default" "$NEURON_INFO_FILE"

if [[ "$MAJORITY" == "false" ]]; then
  dfx identity use "${IDENTITY_NAME}"
  SECOND_PRINCIPAL=$(dfx identity get-principal)

  list_and_save_neurons "$SECOND_PRINCIPAL" "$IDENTITY_NAME" "$NEURON_INFO_FILE"

  dfx identity use default
fi

#subaccount=$(convert_hex_to_subaccount "${DEVELOPER_NEURON_ID}")
subaccount=$DEVELOPER_NEURON_ID

#Deploy the icvc_backend and pass the neuronID and the gov canister id as args
dfx deploy icvc_backend --argument "(opt record{ subaccount = opt \"$subaccount\"; sns_governance_id= opt principal \"$governance_id\"; max_stable_memory_size= 0})"

if [[ $? -ne 0 ]]; then
  echo "Failed to install icvc_backend canister."
  exit 1
fi
