#!/bin/bash

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR"

if [ "$#" -ne 10 ]; then
    echo "Usage: $0 <title> <url> <summary> <function_id> <function_name> <function_desc> <target_canister_id> <target_name> <validator_canister_id> <validator_name>"
    exit 1
fi

TITLE=$1
URL=$2
SUMMARY=$3
FUNCTION_ID=$4
FUNCTION_NAME=$5
FUNCTION_DESC=$6
TARGET_CANISTER_ID=$7
TARGET_METHOD_NAME=$8
VALIDATOR_CANISTER_ID=$9
VALIDATOR_METHOD_NAME=${10}


echo "Creating custom SNS function with the following details:"
echo "Title: $TITLE"
echo "URL: $URL"
echo "Summary: $SUMMARY"
echo "Function ID: $FUNCTION_ID"
echo "Function Name: $FUNCTION_NAME"
echo "Function Description: $FUNCTION_DESC"
echo "Target Canister ID: $TARGET_CANISTER_ID"
echo "Target Method Name: $TARGET_METHOD_NAME"
echo "Validator Canister ID: $VALIDATOR_CANISTER_ID"
echo "Validator Method Name: $VALIDATOR_METHOD_NAME"

PROPOSAL="(record {
  title=\"$TITLE\";
  url=\"$URL\";
  summary=\"$SUMMARY\";
  action=opt variant {
    AddGenericNervousSystemFunction = record {
      id=($FUNCTION_ID:nat64);
      name=\"$FUNCTION_NAME\";
      description=opt \"$FUNCTION_DESC\";
      function_type=opt variant {
        GenericNervousSystemFunction = record {
          validator_canister_id=opt principal \"$VALIDATOR_CANISTER_ID\";
          target_canister_id=opt principal \"$TARGET_CANISTER_ID\";
          validator_method_name=opt \"$VALIDATOR_METHOD_NAME\";
          target_method_name=opt \"$TARGET_METHOD_NAME\"
        }
      }
    }
  }
})"

# Submit the proposal
. ./submit_proposal.sh "$PROPOSAL"
