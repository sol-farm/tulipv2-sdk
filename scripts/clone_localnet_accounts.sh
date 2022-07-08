#! /bin/bash

# given an input file accounts.txt, clone all specified accounts to localnet

OUTPUT_DIR="examples/deps/accounts"
URL="$1"
FORCE="$2"
while IFS= read -r account; do

    if [[ -e "$OUTPUT_DIR/$account.json" && "$FORCE" != "--force" ]]; then
        echo "account $account already stored on disk, please run with --force flag to overwrite"
        continue
    fi
    
    solana \
        --url "$URL" \
        account "$account" \
        --output json \
        --output-file "$OUTPUT_DIR/$account.json"

done < accounts.txt