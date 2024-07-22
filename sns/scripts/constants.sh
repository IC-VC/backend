#!/usr/bin/env bash

export DX_PRINCIPAL="$(dfx identity get-principal)"
export DX_VERSION="$(dfx --version | sed "s/^dfx //")"

export DEVELOPER_NEURON_ID="896119f03e96f7644ad4af3cc3bf5539ebee4838af76530b201044385f2ccfac"
export PEM_FILE="$HOME/.config/dfx/identity/$(dfx identity whoami)/identity.pem"

export NETWORK="local"

export CANISTER_IDS_FILE="./sns_canister_ids.json"