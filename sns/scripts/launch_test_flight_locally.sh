#!/usr/bin/env bash


export DEVELOPER_NEURO_ID="896119f03e96f7644ad4af3cc3bf5539ebee4838af76530b201044385f2ccfac"
export NETWORK="local"

./sns/scripts/setup_test_flight.sh "$DEVELOPER_NEURO_ID" "$NETWORK" false
