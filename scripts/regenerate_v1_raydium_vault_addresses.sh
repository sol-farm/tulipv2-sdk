#! /bin/bash

CONF="$1"
OUTPUT_FILE="common/src/vaults/v1/raydium.rs"


if [[ "$CONF" == "" ]]; then
    echo "[ERROR]: invalid invocation: ./regenerate_v1_raydium_vault_addresses.sh /path/to/config.json"
    exit 1
fi

echo -e "//! raydium v1 vault account address, used to derive all other addresses\n\n" > "$OUTPUT_FILE"
echo "use anchor_lang::solana_program::{self, pubkey::Pubkey};" >> "$OUTPUT_FILE"
echo -e "use static_pubkey::static_pubkey;\n\n" >> "$OUTPUT_FILE"

for VAULT in $(jq -r '.vault.accounts[].account' $CONF); do
    VAULT_NAME=$(cat $CONF | jq -r --arg VAULTID "$VAULT" '.vault.accounts[] | select(.account == $VAULTID) | .name' | sed 's/-/_/g')
    OLD_VAULT_INFO=$(cat $CONF | jq -r --arg VAULTID "$VAULT" '.vault.accounts[] | select(.account == $VAULTID) | .oldUserInfoAccount' | sed 's/-/_/g')
    VAULT_INFO=$(cat $CONF | jq -r --arg VAULTID "$VAULT" '.vault.accounts[] | select(.account == $VAULTID) | .infoAccount' | sed 's/-/_/g')
    VAULT_LP_TOKEN=$(cat $CONF | jq -r --arg VAULTID "$VAULT" '.vault.accounts[] | select(.account == $VAULTID) | .lpTokenAccount' | sed 's/-/_/g')

    case "$VAULT_NAME" in
        RAY_SRM_DUAL | HBB_USDH | TULIP_RAY | COPE_USDC)
            continue
            ;;
    esac

    echo "/// vault address account for the $VAULT_NAME farm" >> "$OUTPUT_FILE"
    echo "pub const $VAULT_NAME: Pubkey = static_pubkey!(\"$VAULT\");" >> "$OUTPUT_FILE"
    echo "/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this" >> "$OUTPUT_FILE"
    echo "pub const ${VAULT_NAME}_INFO_ACCOUNT: Pubkey = static_pubkey!(\"$VAULT_INFO\");" >> "$OUTPUT_FILE"
    echo "/// vault lp token account for the $VAULT_NAME farm" >> "$OUTPUT_FILE"
    echo "pub const ${VAULT_NAME}_LP_TOKEN_ACCOUNT: Pubkey = static_pubkey!(\"$VAULT_LP_TOKEN\");" >> "$OUTPUT_FILE"
    if [[ "$OLD_VAULT_INFO" != "" && "$OLD_VAULT_INFO" != "null" ]]; then
        echo "/// old vault info account for the $VAULT_NAME farm, this is the output of the 'derive_vault_user_info_address' function" >> $OUTPUT_FILE
        echo -e "pub const ${VAULT_NAME}_OLD_INFO: Pubkey = static_pubkey!(\"$OLD_VAULT_INFO\");\n" >> $OUTPUT_FILE
    else
        echo "" >> $OUTPUT_FILE
    fi


done

rustfmt --edition 2021 "$OUTPUT_FILE"