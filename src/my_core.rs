use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
use near_sdk::{env::log_str, require, PromiseOrValue};

use crate::*;

// impl NonFungibleTokenCore for Contract {
#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
        self.tokens
            .nft_transfer(receiver_id, token_id, approval_id, memo)
    }

    #[payable]
    pub fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        self.tokens
            .nft_transfer_call(receiver_id, token_id, approval_id, memo, msg)
    }

    pub fn nft_token(&self, token_id: TokenId) -> Option<JsonToken> {
        if !self.tokens.owner_by_id.contains_key(&token_id) {
            return None;
        }
        if env::predecessor_account_id() != env::current_account_id()
            && !self.token_is_not_expired(&token_id)
        {
            log_str(&format!("Token is expired."));
            return None;
        }
        if self.tokens.owner_by_id.get(&token_id).unwrap() == env::predecessor_account_id()
            || env::predecessor_account_id() == env::current_account_id()
        {
            self.tokens.nft_token(token_id).map(|token| JsonToken {
                expiration_date: self.expiration_timestamp.get(&token.token_id),
                royalty: self.royalty.get(&token.token_id),
                token_id: token.token_id,
                owner_id: token.owner_id,
                metadata: token.metadata,
                approved_account_ids: token.approved_account_ids,
            })
        } else {
            self.tokens.nft_token(token_id).map(|token| JsonToken {
                expiration_date: self.expiration_timestamp.get(&token.token_id),
                royalty: self.royalty.get(&token.token_id),
                token_id: token.token_id,
                owner_id: token.owner_id,
                metadata: None,
                approved_account_ids: token.approved_account_ids,
            })
        }
    }
}
