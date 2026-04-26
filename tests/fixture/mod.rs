#[warn(dead_code)]
use sails_rs::{
    ActorId, 
    CodeId, 
    client::{
        Actor, 
        GearEnv, 
        GtestEnv
    }, 
    gtest::{
        System,
    }
};
use contract_client::{ContractClientCtors, ContractClientProgram};

use contract::WASM_BINARY;

pub(crate) const ADMIN_ID: u64 = 10;
pub(crate) const ONE_VARA: u128 = 1_000_000_000_000;

pub(crate) struct Fixture {
    program_space: GtestEnv,
    code_id: CodeId
}

impl Fixture {
    pub(crate) fn new() -> Self {
        // Create the system for tests with gtest
        let system = System::new();
        system.init_logger_with_default_filter(
            "gwasm=debug,gtest=info,sails_rs=debug,ping_pong_stack=debug"
        );
        
        // Mint 1_000 tokens to ADMIN_ID
        system.mint_to(ADMIN_ID, ONE_VARA * 1_000);

        // Submit program code into the system
        let code_id = system.submit_code(WASM_BINARY);

        // Create a remoting instance for the system
        // and set the block run mode to Next,
        // cause we don't receive any reply on `Exit` call
        let env = GtestEnv::new(system, ADMIN_ID.into());

        // Fixture with contract id and env
        Self {
            program_space: env,
            code_id
        }
    }

    pub(crate) fn balance_of(&self, address: ActorId) -> u128 {
        self
            .program_space
            .system()
            .balance_of(address)
    }

    pub(crate) fn contract_code_id(&self) -> CodeId {
        self.code_id
    }

    // Function to create contract
    // Notes:
    // - In your applications constructors, it is recommended that 
    //   you use names that you can easily identify.
    pub(crate) async fn create_contract(&self, salt: Vec<u8>) -> Actor<ContractClientProgram, GtestEnv> {
        self.program_space
            .deploy::<ContractClientProgram>(self.code_id, salt)
            // If you set params in your applications constructors, you must set your arguments.
            // Use generated client code for activating Contract program using
            // the `init` constructor
            .init() // Smart contract constructor
            // Optional functions when deploying a contract:
            // .with_actor_id(actor_id) <- who will sign for contract deployment
            // .with_gas_limit(gas_limit) <- gas limit for deployment
            // .with_value(value) <- Tokens to send to the contract when deployment
            .await
            .unwrap()
    }
}

