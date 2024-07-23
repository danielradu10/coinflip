#![no_std]



use multiversx_sc::imports::*;

mod ctf_coinflip_proxy;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait CtfTryTheCoinflip: bump_common::BumpCommon {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    // functia in care verific daca e ok sa apelez coinflip-ul din contractul de coinflip

    #[endpoint]
    fn call_the_coinflip(&self, adresa: ManagedAddress){
        
        // let string_address = String::from("erd1qqqqqqqqqqqqqpgq0jtfsyk7rfgu50v6wh5wwtk2mjgseca54wzqyntf2v");
        // let bech32_address = Some(Bech32Address::from_bech32_string(string_address)); 
        // let ref_bech32_address_of_coinflip_contract = bech32_address.as_ref().unwrap();


        let lucky_enough_to_execute = self.flip_coin();
        if lucky_enough_to_execute
        {
            self.tx()
            .to(adresa)
            .gas(2_000_000)
            .typed(ctf_coinflip_proxy::CtfCoinflipProxy)
            .coinflip()
            .sync_call();
        }
        
        
        
    }

    // functia de flip_coin copiata
    fn flip_coin(&self) -> bool {
        let block_nonce = self.blockchain().get_block_nonce();
        block_nonce & 1 == block_nonce & 2
    }


}
