export PEM_FILE="$HOME/.config/dfx/identity/$(dfx identity whoami)/identity.pem"

export NETWORK=$1
export PROPOSER_NEURON_ID=$2
export HOTKEY_PRINCIPAL="$3"


. ./sns/scripts/utils/setup_env.sh "$NETWORK" "$PROPOSER_NEURON_ID"

echo "NETWORK: $NETWORK"

quill sns \
  --canister-ids-file ./sns_canister_ids.json \
  --pem-file "$PEM_FILE" \
  neuron-permission \
  --principal "${HOTKEY_PRINCIPAL}" \
  --permissions vote,submit-proposal,manage-voting-permission \
  add \
  "${PROPOSER_NEURON_ID}" \
  > msg.json

if [ "$NETWORK" = "local" ]; then
    quill --insecure-local-dev-mode send msg.json

elif [ "$NETWORK" = "ic" ]; then
    quill send msg.json
fi

rm -f msg.json