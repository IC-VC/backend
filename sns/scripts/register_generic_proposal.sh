#!/usr/bin/env bash

export DEVELOPER_NEURON_ID="$1"
export CID="$2"
export NETWORK="${3:-local}"

export PEM_FILE="$HOME/.config/dfx/identity/$(dfx identity whoami)/identity.pem"

dfx identity use default

quill sns  \
   --canister-ids-file ./sns_canister_ids.json  \
   --pem-file "${PEM_FILE}"  \
   make-proposal --proposal "(record 
   { 
      title=\"Register generic function on icvc backend canister.\";
      url=\"https://example.com/\";
      summary=\"This proposal register a generic function to custom vote on projects.\";
      action=opt variant 
         {
            AddGenericNervousSystemFunction = record {
               id=4001:nat64; 
               name=\"Project voting\"; 
               function_type=opt variant 
               {
                  GenericNervousSystemFunction=record
                  {
                     validator_canister_id=opt principal\"$CID\";
                     target_canister_id=opt principal\"$CID\";
                     validator_method_name=opt\"validate_project_vote_proposal\";
                     target_method_name=opt\"execute_project_vote_proposal\"
                  }
               }
            }
         }
   })" "$DEVELOPER_NEURON_ID" > msg.json

quill --insecure-local-dev-mode send msg.json
