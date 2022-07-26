use near_contract_standards::non_fungible_token::{
    events::NftMint, refund_deposit_to_account, Token,
};
use near_sdk::{collections::UnorderedSet, near_bindgen, require};

use crate::{my_internal::parse_time, *};

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        token_metadata: TokenMetadata,
        expiration_period: Option<String>,
        perpetual_royalties: Option<HashMap<AccountId, u32>>,
    ) -> JsonToken {
        require!(env::predecessor_account_id() == env::current_account_id());
        // self.expiration_timestamp[token_id] = parse_time(expiration_period.unwrap());
        // Remember current storage usage if refund_id is Some
        let initial_storage_usage = (env::predecessor_account_id(), env::storage_usage());

        if self.tokens.owner_by_id.get(&token_id).is_some() {
            env::panic_str("token_id must be unique");
        }

        // create a royalty map to store in the token
        let mut royalty = HashMap::new();

        // if perpetual royalties were passed into the function:
        if let Some(perpetual_royalties) = perpetual_royalties {
            //make sure that the length of the perpetual royalties is below 7 since we won't have enough GAS to pay out that many people
            assert!(
                perpetual_royalties.len() < 7,
                "Cannot add more than 6 perpetual royalty amounts"
            );

            //iterate through the perpetual royalties and insert the account and amount in the royalty map
            for (account, amount) in perpetual_royalties {
                royalty.insert(account, amount);
            }

            self.royalty.insert(&token_id, &royalty);
        }

        let owner_id: AccountId = receiver_id;

        // Core behavior: every token must have an owner
        self.tokens.owner_by_id.insert(&token_id, &owner_id);

        // Metadata extension: Save metadata, keep variable around to return later.
        // Note that check above already panicked if metadata extension in use but no metadata
        // provided to call.
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &token_metadata));

        // Enumeration extension: Record tokens_per_owner for use with enumeration view methods.
        if let Some(tokens_per_owner) = &mut self.tokens.tokens_per_owner {
            let mut token_ids = tokens_per_owner.get(&owner_id).unwrap_or_else(|| {
                UnorderedSet::new(StorageKey::TokensPerOwner {
                    account_hash: env::sha256(owner_id.as_bytes()),
                })
            });
            token_ids.insert(&token_id);
            tokens_per_owner.insert(&owner_id, &token_ids);
        }

        // Approval Management extension: return empty HashMap as part of Token
        let approved_account_ids = if self.tokens.approvals_by_id.is_some() {
            Some(HashMap::new())
        } else {
            None
        };

        // Return any extra attached deposit not used for storage

        let token = Token {
            token_id,
            owner_id,
            metadata: Some(token_metadata),
            approved_account_ids,
        };
        NftMint {
            owner_id: &token.owner_id,
            token_ids: &[&token.token_id],
            memo: None,
        }
        .emit();
        if let Some(time) = expiration_period {
            self.expiration_timestamp.insert(
                &token.token_id,
                &(env::block_timestamp() + parse_time(&time)),
            );
        }

        let (id, storage_usage) = initial_storage_usage;
        refund_deposit_to_account(env::storage_usage() - storage_usage, id);
        JsonToken {
            expiration_date: self.expiration_timestamp.get(&token.token_id),
            royalty: self.royalty.get(&token.token_id),
            token_id: token.token_id,
            owner_id: token.owner_id,
            metadata: token.metadata,
            approved_account_ids: token.approved_account_ids,
        }
    }
}
