# ICVC Project

ICVC is a comprehensive system composed of canisters operating on the Internet Computer, designed to facilitate the submission and evaluation of projects through a structured process. Projects are evaluated in multiple steps, grouped into phases. Only the initial and final phases utilize the SNS (Service Network System) governance for voting. Other phases are assessed through grades on the steps to determine project progression. The system also integrates OpenChat to facilitate communication and collaboration among users through embedded chat functionalities.

## Installation

### Requirements

- **DFX 0.19.0**: To install, run:
    ```bash
    DFX_VERSION=0.19.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
    ```
- **Rust**: To install, run:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

### Steps

1. Start the DFX (Internet Computer SDK) environment in the background and clean any previous state:
    ```bash
    dfx start --background --clean
    ```
2. Deploy the canisters:
    ```bash
    dfx deploy
    ```

## Running SNS Locally for Testing

### Requirements

- **DFX Extension SNS v0.4.3**

To install it, remove the older version first if necessary:
```bash
dfx extension uninstall sns
dfx extension install sns --version 0.4.3
```

### Steps to Run Locally

1. Ensure you are in the root directory of the project.
2. Launch sns setup locally
   ```bash
    ./sns/scripts/launch_test_flight_locally.sh
    ```
    - This script will:
        - Set up two identities.
        - Add the identities to the SNS init file.
        - Launch SNS using testflight.
        - Retrieve neuron IDs.
        - Deploy the ICVC backend.

3.  Script to test proposals
    ```bash
    ./sns/scripts/test_proposals.sh
    ```
    - This script will:
        - Register the ICVC backend with the SNS root.
        - Add the ICVC backend as a hot key to enable proposal submissions.
        - Register a generic proposal.

### Important Post-Test Cleanup

After finishing the test run, if you want to run the test again, ensure to clean up:
```bash
./sns/scripts/cleanup.sh
```

### Casting a Vote

To cast a vote, run:
```bash
./sns/scripts/cast_sns_vote.sh <PROPOSAL_ID> <VOTE>
```
- Arguments: Proposal ID and vote (`y` for yes or `n` for no).
