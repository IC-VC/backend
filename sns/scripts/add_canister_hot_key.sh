export PEM_FILE="$HOME/.config/dfx/identity/$(dfx identity whoami)/identity.pem"
export CID="$(dfx canister id icvc_backend)"

export DEVELOPER_NEURON_ID="$1"
export HOTKEY_PRINCIPAL="$2"


quill sns \
  --canister-ids-file ./sns_canister_ids.json \
  --pem-file "$PEM_FILE" \
  neuron-permission \
  --principal "${HOTKEY_PRINCIPAL}" \
  --permissions vote,submit-proposal,manage-voting-permission \
  add \
  "${DEVELOPER_NEURON_ID}" \
  > msg.json

quill send --insecure-local-dev-mode msg.json 