mod basic_interact_config;

mod ctf_bump_proxy;
mod ctf_coinflip_proxy;
mod ctf_gaspass_proxy;
mod ctf_try_coinflip_proxy;

use basic_interact_config::Config;

use multiversx_sc_snippets::{imports::*, sdk::wallet::Wallet};

const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";

const TRYER_CODE_PATH: MxscPath = MxscPath::new("../ctf-try-the-coinflip/output/ctf-try-the-coinflip.mxsc.json");


#[tokio::main]
async fn main() {
    env_logger::init();

    let mut _basic_interact: CtfBumpInteract = CtfBumpInteract::init().await;
    _basic_interact.coinflip().await;
    //_basic_interact.call_ctf_try_the_coinflip_contract().await;
}

#[allow(unused)]
struct CtfBumpInteract {
    interactor: Interactor,
    wallet_address: Bech32Address,
}

impl CtfBumpInteract {
    async fn init() -> Self {
        let config = Config::load_config();
        let mut interactor = Interactor::new(config.gateway())
            .await
            .with_tracer(INTERACTOR_SCENARIO_TRACE_PATH)
            .await;
        let wallet_address = interactor.register_wallet(Wallet::from_pem_file("walletNew.pem").unwrap());

        println!("Initializarea interactorului a fost facuta cu succes... Returnez obiectul!");

        Self {
            interactor,
            wallet_address: wallet_address.into(),
        }
        
    }

    async fn bump(&mut self) {
        
        let string_address = String::from("erd1qqqqqqqqqqqqqpgq23j27f6w0r75hfyc5td753f9ahvfpp5x4wzq65czqw");
        let bech32_address = Some(Bech32Address::from_bech32_string(string_address)); 
        let ref_bech32_address = bech32_address.as_ref().unwrap();

        self.interactor 
                .tx()
                .from(&self.wallet_address)
                .to(ref_bech32_address)
                .gas(30_000_000)
                .typed(ctf_bump_proxy::CtfBumpProxy)
                .bump()
                .prepare_async()
                .run()
                .await;

        println!("Bump executat cu succes!");
    }


    async fn call_ctf_try_the_coinflip_contract(&mut self)
    {
        let string_address_of_try = String::from("erd1qqqqqqqqqqqqqpgqqlv3wl29lnduv9lhzl7xqvnfgc96x86d3e8sm95m6k");
        let bech32_address_of_try = Some(Bech32Address::from_bech32_string(string_address_of_try)); 
        let ref_bech32_address_of_try = bech32_address_of_try.as_ref().unwrap();



        let string_address_of_coinflip_contract = String::from("erd1qqqqqqqqqqqqqpgq0jtfsyk7rfgu50v6wh5wwtk2mjgseca54wzqyntf2v");
        let bech32_address_of_coinflip_contract = Some(Bech32Address::from_bech32_string(string_address_of_coinflip_contract)); 
        let ref_bech32_address_of_coinflip_contract = bech32_address_of_coinflip_contract.as_ref().unwrap();

        self.interactor 
                .tx()
                .from(&self.wallet_address)
                .to(ref_bech32_address_of_try)
                .gas(30_000_000)
                .typed(ctf_try_coinflip_proxy::CtfTryTheCoinflipProxy)
                .call_the_coinflip(ref_bech32_address_of_coinflip_contract)
                .prepare_async()
                .run()
                .await;
        
        println!("Apelarea try_te_coinflip a fost executata cu succes!");
        
        
    }


    async fn deploy_try_coinflip_contract(&mut self) {
        // warning: multi deploy not yet fully supported
        // only works with last deployed address

       

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(30_000_000)
            .typed(ctf_try_coinflip_proxy::CtfTryTheCoinflipProxy)
            .init()
            .code(TRYER_CODE_PATH)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewBech32Address)
            .prepare_async()
            .run()
            .await;

        println!("new address: {new_address}");
        
    }

    // coinflip 
    async fn coinflip(&mut self)
    {

        // dau deploy noului contract creat si salvez adresa lui 

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(30_000_000)
            .typed(ctf_try_coinflip_proxy::CtfTryTheCoinflipProxy)
            .init()
            .code(TRYER_CODE_PATH)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewBech32Address)
            .prepare_async()
            .run()
            .await;



    
        let string_address_of_coinflip_contract = String::from("erd1qqqqqqqqqqqqqpgq0jtfsyk7rfgu50v6wh5wwtk2mjgseca54wzqyntf2v");
        let bech32_address_of_coinflip_contract = Some(Bech32Address::from_bech32_string(string_address_of_coinflip_contract)); 
        let ref_bech32_address_of_coinflip_contract = bech32_address_of_coinflip_contract.as_ref().unwrap();
    
        // apelez incercarea de coinflip 
        self.interactor 
                    .tx()
                    .from(&self.wallet_address)
                    .to(new_address)
                    .gas(30_000_000)
                    .typed(ctf_try_coinflip_proxy::CtfTryTheCoinflipProxy)
                    .call_the_coinflip(ref_bech32_address_of_coinflip_contract)
                    .prepare_async()
                    .run()
                    .await;
            
        println!("Apelarea try_the_coinflip a fost executata cu succes!");

    

    }
    
}
