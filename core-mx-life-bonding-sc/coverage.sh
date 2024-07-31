#!/bin/bash

cargo llvm-cov --ignore-filename-regex '(storage.rs|events.rs|core-mx-liveliness-stake-sc*|proxy_contracts*|life_bonding_sc_proxy.rs)'  --open


