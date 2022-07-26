use near_sdk::{assert_one_yocto, json_types::U128, serde::Deserialize};

use crate::{my_internal::royalty_to_payout, *};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
}
pub trait Payouts {
    fn nft_payout(&self, token_id: TokenId, balance: U128, max_len_payout: u32) -> Payout;

    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: Option<String>,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout;
}

#[near_bindgen]
impl Payouts for Contract {
    fn nft_payout(&self, token_id: TokenId, balance: U128, max_len_payout: u32) -> Payout {
        let owner_id = self
            .tokens
            .owner_by_id
            .get(&token_id)
            .expect("Token doesn't exist.");

        let mut total_perpetual = 0;

        let balance_u128 = u128::from(balance);

        let mut payout_object = Payout {
            payout: HashMap::new(),
        };
        let royalty = self.royalty.get(&token_id).unwrap();

        assert!(
            royalty.len() as u32 <= max_len_payout,
            "Market cannot payout to that many receivers"
        );

        for (k, v) in royalty.iter() {
            let key = k.clone();
            if key != owner_id {
                payout_object
                    .payout
                    .insert(key, royalty_to_payout(*v, balance_u128));
                total_perpetual += *v;
            }
        }

        payout_object.payout.insert(
            owner_id,
            royalty_to_payout(10000 - total_perpetual, balance_u128),
        );

        payout_object
    }

    #[payable]
    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: Option<String>,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout {
        assert_one_yocto();
        let payout = self.nft_payout(token_id.clone(), balance, max_len_payout);
        self.nft_transfer(receiver_id, token_id, Some(approval_id), memo);
        payout
    }
}
