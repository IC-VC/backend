NETWORK=$1
PROPOSER_NEURON_ID=$2

. ./sns/scripts/utils/setup_env.sh "$NETWORK" "$PROPOSER_NEURON_ID"


VERSION="0.1.4"
TITLE="Upgrade ICVC backend canister"
URL="https://ic-vc.com/"
SUMMARY="This proposal upgrades the code of the icvc backend canister."

function build_canister {
    # Navigate to the project root directory
    cd "./"
    
    # Compile the Rust project to WebAssembly
    cargo build --target wasm32-unknown-unknown --release

    TARGET_WASM_PATH="$BUILD_WASM_FOLDER/$ICVC_BACKEND_CANISTER_NAME.wasm"
    echo "Compiled WASM path: $TARGET_WASM_PATH"

    if [ -f "$TARGET_WASM_PATH" ]; then
        echo "WASM file successfully recompiled."
        
        cp "$TARGET_WASM_PATH" "${WASM_LOCATION}/$ICVC_BACKEND_CANISTER_NAME.wasm"

        # Compress the WASM file and save it as a .gz file
        gzip -c "${WASM_LOCATION}/$ICVC_BACKEND_CANISTER_NAME.wasm" > "${WASM_LOCATION}/$ICVC_BACKEND_CANISTER_NAME.wasm.gz"
        
        if [ -f "${WASM_LOCATION}/$ICVC_BACKEND_CANISTER_NAME.wasm.gz" ]; then
            echo "WASM file successfully compressed to .gz format."
        else
            echo "Failed to compress the WASM file."
            exit 1
        fi
    else
        echo "WASM compilation failed."
        exit 1
    fi
}

build_canister

./sns/scripts/utils/submit_upgrade_proposal.sh  "$ICVC_BACKEND_CANISTER" \
                                                "$ICVC_BACKEND_CANISTER_NAME" \
                                                "$WASM_LOCATION" \
                                                "$VERSION" \
                                                "$TITLE" \
                                                "$URL" \
                                                "$SUMMARY" \
                                                "" \
