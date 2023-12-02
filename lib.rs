#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[ink::contract]
mod genesisgain {
    use ink::{storage::Mapping, env::account_id};

        #[ink(event)]
        pub struct LogMessage {
            #[ink(topic)]
            message: AccountId,
        }

    type doner = ink_prelude::vec::Vec<(AccountId,Balance)>;
    type post = ink_prelude::vec::Vec<u8>;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Genesisgain{
        posts: Mapping<AccountId, post>,
        doners: Mapping<AccountId, doner>,
    }
    
    impl Genesisgain {
        #[ink(constructor)]
        pub fn new(content: ink_prelude::vec::Vec<u8>, owner: AccountId,doner:ink_prelude::vec::Vec<(AccountId,Balance)>) -> Self {
            Default::default()
        }

        #[ink(message)]
        pub fn donate(&mut self, amount: Balance, post_owner: AccountId) {
            let mut donerList = match self.doners.get(&post_owner) {
                Some(existing_donors) => existing_donors.clone(),
                None => ink_prelude::vec::Vec::new(),
            };
            //need to write transaction logic            
            donerList.push((self.env().caller(),amount));
            self.doners.insert(post_owner,&donerList);
            self.env().emit_event(LogMessage {
                    message: AccountId::from(post_owner),
                });
        }

        #[ink(message)]
        pub fn get_doner_list(&mut self,post_owner: AccountId) -> Option<(
            ink_prelude::vec::Vec<(AccountId,Balance)>
        )>{
            let result = self.doners.get(&post_owner).map(|donerList| donerList.clone());
            if result.is_some() {
                return result
            } else {
                self.env().emit_event(LogMessage {
                    message: AccountId::from(post_owner),
                });
                return None
            }
        }

        

    }
}





