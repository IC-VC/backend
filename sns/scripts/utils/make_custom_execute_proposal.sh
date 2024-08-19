#!/bin/bash

# Check if the required arguments are provided
if [ $# -lt 5 ]; then
  echo "Usage: $0 <function_id> <title> <summary> <url> <args>"
  exit 1
fi

# Assign arguments to variables with meaningful names
FUNCTION_ID="$1"
TITLE="$2"
SUMMARY="$3"
URL="$4"
ARGS="$5"

echo "$ARGS"

# Candid encode the payload as binary and check if the encoding is successful
PAYLOAD=$(didc encode "$ARGS" --format blob)
if [ $? -ne 0 ]; then
  echo "Error: Failed to encode the payload."
  exit 1
fi

echo "$PAYLOAD" | didc decode --format blob

PROPOSAL="(record {
  title = \"$TITLE\";
  url = \"$URL\";
  summary = \"$SUMMARY\";
  action = opt variant {
    ExecuteGenericNervousSystemFunction = record {
      function_id = ($FUNCTION_ID:nat64);
      payload = ${PAYLOAD};
    }
  }
})"

# Check if the proposal was built successfully
if [ -z "$PROPOSAL" ]; then
  echo "Error: Failed to build the proposal."
  exit 1
fi

# Submit the proposal
./sns/scripts/utils/submit_proposal.sh "$PROPOSAL"
if [ $? -ne 0 ]; then
  echo "Error: Failed to make the proposal."
  exit 1
fi

echo "Proposal created and submitted successfully."
