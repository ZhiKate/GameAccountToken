#![cfg_attr(not(feature = "std"), no_std,no_main)]
#![feature(min_specialization)]

        
#[openbrush::contract]
pub mod game {

	use core::ptr::null;
    use ink::prelude::vec::Vec;
	use openbrush::traits::String;
	use openbrush::traits::Storage;
	use openbrush::storage::Mapping;
	use openbrush::contracts::ownable::*;
	use openbrush::contracts::psp34::psp34::Internal;
	use openbrush::contracts::psp34::extensions::burnable::*;
	use openbrush::contracts::psp34::extensions::mintable::*;
	use openbrush::contracts::psp34::extensions::enumerable::*;
	use openbrush::contracts::psp34::extensions::metadata::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Game {
    	#[storage_field]
		psp34: psp34::Data<Balances>,
		#[storage_field]
		ownable: ownable::Data,
		#[storage_field]
		metadata: metadata::Data,
		pub SellList: Vec<Id>,
		pub PriceList:Mapping<Id,u32>,
		pub AccountBalance: Mapping<AccountId,u32>
    }
    

    impl Game {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
			_instance.AccountBalance.insert(&_instance.env().caller(),&1000);
			_instance._init_with_owner(_instance.env().caller());
			_instance._mint_to(_instance.env().caller(), Id::U8(1)).expect("Can mint");
			let collection_id = _instance.collection_id();
			_instance._set_attribute(collection_id.clone(), String::from("name"), String::from("MyPSP34"));
			_instance._set_attribute(collection_id, String::from("symbol"), String::from("MPSP"));
			_instance
        }


		//This function is used to transfer money to other account by the AccountId. 
		#[ink(message)]
		pub fn transferMoney(&mut self,to: AccountId, amount: u32) ->Result<(), PSP34Error> {
			let sender= self.env().caller();
			let recevier=to;
			let sender_balance=self.AccountBalance.get(&sender).unwrap_or(0);
			let recevier_balance=self.AccountBalance.get(&recevier).unwrap_or(0);

			if sender_balance<amount{
				return Err(PSP34Error::Custom(String::from("Sender does not have enough money.")));
			}

			self.AccountBalance.insert(&sender,&(sender_balance-amount));
			self.AccountBalance.insert(&recevier,&(recevier_balance+amount));
			Ok(())

		}

		// this function is used to check the caller's balance.
		#[ink(message)]
		pub fn checKBalance(&self) -> Option<u32>{
			return self.AccountBalance.get(&self.env().caller())
		}

		// There is only way to build up the token, every account can build the token to themselves. 
		#[ink(message)]
		pub fn mint(
            &mut self,
			id: Id
        ) -> Result<(), PSP34Error> {
			self._mint_to(self.env().caller(), id)
		}

		// There are two functions can burn the PSP34 token: Holder_burn() and burn()
		// Holder_burn():it means that the token holder can call this function to burn their own token. And cannot burn others' token.
		// burn(): this function is used by the owner of the contract(same as Game official account) to burn the token. It can burn others' token.
		#[ink(message)]
		pub fn Holder_burn(
            &mut self,
			id: Id
        ) -> Result<(), PSP34Error> {
			
			self._burn_from(self.env().caller(), id)
		}

		#[ink(message)]
		#[openbrush::modifiers(only_owner)]
		pub fn burn(
            &mut self,
            account: AccountId,
			id: Id
        ) -> Result<(), PSP34Error> {
			self._burn_from(account, id)
		}


		
		// This function is used by account to sold the token. And the token which be add the for sale list. And the owner of this smart contract 
		// will be granted the right to handle token(NFT). That means contract's owner(not only the token holder) can transfer the for_sale token(NFT) to buyer. 
		#[ink(message)]
		pub fn AddIntoSellList(&mut self,id:Id, price:u32) ->Result<(),PSP34Error>{
			let token=id.clone();
			let token1=id.clone();
			let TokenOwner=self.owner_of(id);
			let invoker=Some(self.env().caller());
            
			if invoker!=TokenOwner{
				return Err(PSP34Error::Custom(String::from("CallerMustbeOwnerOfToken")));
			}
			self.approve(self.owner(),Some(token),true);
			self.PriceList.insert(&token1,&price);
			self.SellList.push(token1);
			Ok(())
		}


		// This function is used to show that the tokens are being sold. Other can buy the tokens which are in sold. When buyer buy the for_sale token, contract's
		//owner transfers(not the token holder) the token to buyer.
		#[ink(message)]
        pub fn ForSaleNFT(&self) -> Vec<Id> {
            self.SellList.clone()
        }

		// this function is used to transfer token to other.
		// condition: Only the token holder can transfer his/her own token. 
		#[ink(message)]
		pub fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
			self._transfer_token(to, id, data)
		}


		// this function used by the user(account) to buy the token which has been added to for_sell list. The caller(buyer) will send money(equal to the price that is set by the token holder)
		// to the token holder(seller).
 		#[ink(message)]
		pub fn buy(&mut self,id: Id, to: AccountId) ->Result<(),PSP34Error>{
			let price=self.PriceList.get(&id).unwrap_or(0);
			let buyer_balance=self.AccountBalance.get(&(self.env().caller())).unwrap_or(0);
			let token_id=id.clone();
			let seller=self.owner_of(id);
	

				if !(self.PriceList.contains(&token_id)){
				return Err(PSP34Error::Custom(String::from("This token(NFT) is not in sell list ")));
			}
			if buyer_balance<price {
				return Err(PSP34Error::Custom(String::from("buyer does not have enough money to pay ")));
			}
			if Some(to)!=seller {
				return Err(PSP34Error::Custom(String::from("you must transfer money to seller,other account is not allow. ")));
			}
			self.transferMoney(to,price)
			
		}

		//this function is used to search who is the holder(owwner) of the specific token by token's id.
		#[ink(message)]
		pub fn owner_of(&self, id: Id) -> Option<AccountId> {
			Internal::_owner_of(self, &id)
		}
		

    }

	#[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn Default_Transfer_Money() {
            let mut game = Game::new();
			let owner=game.owner();
			let owner_amount=game.AccountBalance.get(&owner).unwrap_or(0);
			// the construtor initialize the contract's owner balance to be 1000.Here test if the constructor in a right way. 
            assert_eq!(owner_amount, 1000);

			// the default caller is alice. Here, alice transfer 100 to bob. Therefore, after transfer money transaction, alice has 900 and bob has 100.
			let alice=AccountId::from([1; 32]);
			let bob=AccountId::from([2; 32]);
			game.transferMoney(bob,100);
			assert_eq!(game.AccountBalance.get(&alice).unwrap_or(0),900);
			assert_eq!(game.AccountBalance.get(&bob).unwrap_or(0),100);

			//when the transfer amount is over the sender's balance, this transaction does not happen.
			// test::set_caller::<DefaultEnvironment>(bob);
			game.transferMoney(bob,1000);
			assert_eq!(game.AccountBalance.get(&alice).unwrap_or(0),900);
			assert_eq!(game.AccountBalance.get(&bob).unwrap_or(0),100);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn test_mint() {
            let mut game = Game::new();

			//after initialize, the default caller will be alice.
			let alice=AccountId::from([1; 32]);
			let bob=AccountId::from([2; 32]);

			// caller(all account) can build the PSP34 token(NFT) to themselves. That means they can just build their own token. And cannot build to token which is
			//held by others. A token with the same id cannot be mint more than one time. 
			let result=game.mint(Id::U32(10));
			assert_eq!(result,std::result::Result::Ok(()));

			let token_holder=game.owner_of(Id::U32(10));
			assert_eq!(token_holder,Some(alice))
        }
    }
}



