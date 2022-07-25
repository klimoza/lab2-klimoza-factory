use near_sdk::require;

use crate::*;

pub(crate) fn parse_time(time: &String) -> u64 {
    let n = time.len();
    let num = &time[..(n - 1)].parse::<u64>();
    let period = time.as_bytes()[n - 1];
    require!(period == b's' || period == b'm' || period == b'h' || period == b'd', "Wrong time format.");
    require!(num.is_ok(), "Wrong time format.");
    let time : u64= match period {
      b's' => 1_000_000_000,
      b'm' => 60 * 1_000_000_000,
      b'h' => 60 * 60 * 1_000_000_000,
      _ => 24 * 60 * 60 * 1_000_000_000
    };
    num.as_ref().unwrap() * time
}

impl Contract {
    pub(crate) fn token_is_not_expired(&self, token_id: &TokenId) -> bool {
        let timestamp = self.expiration_timestamp.get(&token_id);
        timestamp.is_none() || timestamp.unwrap() >= env::block_timestamp()
    }

    pub(crate) fn enum_get_token(&self, owner_id: AccountId, token_id: TokenId) -> JsonToken {
        let metadata = self.tokens.token_metadata_by_id.as_ref().unwrap().get(&token_id);
        let approved_account_ids =
            Some(self.tokens.approvals_by_id.as_ref().unwrap().get(&token_id).unwrap_or_default());
        let expiration_date = self.expiration_timestamp.get(&token_id).map(|x| x as u64);
        JsonToken { token_id, owner_id, metadata, approved_account_ids, expiration_date}
    }
}