#!/bin/bash

for ((i = 0 ; i < 4 ; i++)); do
  ./target/release/core_loop_test_client -u User$i &
done

