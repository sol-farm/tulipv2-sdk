#! /bin/bash

# finds any "missing" accounts from the localnet environment, namely by looking for configurations
# in Anchor.toml which specify localnet accounts to clone during runtime.

grep "test\.validator\.account" -A 1 examples/Anchor.toml | grep "address = .*" | awk -F '=' '{print $2}' | tr -d '" ' | tee accounts.txt