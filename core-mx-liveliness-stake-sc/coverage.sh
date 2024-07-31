#!/bin/bash

cargo llvm-cov --ignore-filename-regex '(storage.rs|events.rs|errors.rs|views.rs|liveliness_stake_proxy.rs|contexts*|core-mx-life-bonding-sc|proxy_contracts)'  --open


