#!/usr/bin/env bash

#cd -- "$(dirname -- "${BASH_SOURCE[0]}")"

. ./sns/scripts/constants.sh

export NAME="${1:-}"


dfx build --network "${NETWORK}" "${NAME}"
export WASM="./.dfx/${NETWORK}/canisters/${NAME}/${NAME}.wasm"

echo $WASM

export CID="$(dfx canister --network "${NETWORK}" id "${NAME}")"

quill sns  \
   --canister-ids-file ./sns_canister_ids.json  \
   --pem-file "${PEM_FILE}"  \
   make-upgrade-canister-proposal  \
   --summary "This proposal upgrades canister"  \
   --title "Upgrade canister"  \
   --url "https://icvc.com/"  \
   --target-canister-id "${CID}"  \
   --wasm-path "${WASM}" \
   "${DEVELOPER_NEURON_ID}" > msg.json

quill send \
  --insecure-local-dev-mode \
  --yes msg.json | grep -v "new_canister_wasm"
