#!/usr/bin/env bash

# Function to extract neuron ID based on identity name from neuron_info.json
get_neuron_id() {
    local identity_name=$1
    local info_file=$2

    neuron_id=$(jq -r --arg identity_name "$identity_name" '.[] | select(.identity_name == $identity_name) | .neuron_id' "$info_file")

    if [ -z "$neuron_id" ]; then
        echo "Neuron ID not found for identity name: $identity_name"
        exit 1
    fi

    echo "$neuron_id"
}
