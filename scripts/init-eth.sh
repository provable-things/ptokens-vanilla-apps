#!/bin/bash
./vanilla initializeEth \
--bytecode="./smart-contract-bytecode" \
--confs=0 \
--chainId=3 \
--gasPrice=20000000000 \
--file=./eth-block.json
