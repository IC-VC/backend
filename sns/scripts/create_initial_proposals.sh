NETWORK=$1
PROPOSER_NEURON_ID=$2

. ./sns/scripts/utils/setup_env.sh "$NETWORK" "$PROPOSER_NEURON_ID"

echo "NETWORK: $NETWORK"


./sns/scripts/proposals/custom_proposal_update_canister_config.sh "$ICVC_BACKEND_CANISTER"

#./sns/scripts/proposals/custom_proposal_project_voting.sh "$ICVC_BACKEND_CANISTER"


