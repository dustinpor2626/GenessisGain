#![cfg_attr(not(feature = "std"), no_std, no_main)]
use ink::storage::Mapping;


#[ink::contract]
mod genesisgain {

    type doner = ink_prelude::vec::Vec<(AccountId,Balance)>;
    type accountId = AccountId;
    type post = u64;

    #[ink(storage)]
    pub struct Genesisgain{
        posts: Mapping<accountId, Vec<post>>,
        doners:Mapping<accountId, doner>,
        accountIds: Mapping<AccountId, ()>,
    }

    pub struct profile{
        owner : AccountId,
        posts : ink_prelude::vec::Vec<profile>,
    }
    
    impl Genesisgain {
        #[ink(constructor)]
        pub fn new(content: ink_prelude::vec::Vec<u8>, owner: AccountId) -> Self {
            Self {
                posts:0,
                doners:0,
                accountIds: 0,
            }
        }

        #[ink(message)]
        pub fn post_content(&mut self, new_content: ink_prelude::vec::Vec<u8>) {
            let account_id = self.env().caller();
            let &mut posts = self.posts.get(account_id);
            if posts.length() == 0{
                posts = [];
            }
            posts.append(new_content);
            self.posts.insert(account_id, posts);

            if(!self.accountIds.get(account_id).is_some()){
                 self.accountIds.insert(account_id, ());
            }
        }

        #[ink(message)]
        pub fn donate(&mut self, amount: Balance, post_owner: AccountId) {
            let &mut donerList = self.doners.get(post_owner);
            donerList.push((self.env().caller(),amount));
            self.doners.insert((self.env().caller(),donerList));
        }

        
        #[ink(message)]
        pub fn get_random_post(&self) -> Option<(
            ink_prelude::vec::Vec<ink_prelude::vec::Vec<(AccountId,Balance)>>,
        )> {
            let len = self.accountIds.len();
            if len == 0 {
                return None;
            }
            let _random_index = 1;
            let &mut account = self.accountIds.get(&_random_index);
            let post1 = self.posts.get(account).clone();
            let account = self.accountIds.get(&2);
            let post2 = self.posts.get(account).clone();
            let posts=  vec![];
            posts.push(post1);
            posts.push(post2);
            Some(posts)
        }

    }
}






