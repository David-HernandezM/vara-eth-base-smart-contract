use contract_client::{
    ContractClient, contract_svc::{
        ContractSvc,
        events::ContractSvcEvents
    }
};
use sails_rs::{
    ActorId, client::Route, futures::StreamExt as _
};
use fixture::*; 


mod fixture;
mod utils;

fn evm_address_bytes(actor_id: ActorId) -> [u8; 20] {
    let bytes = actor_id.into_bytes();
    bytes[12..].try_into().unwrap()
}

#[tokio::test]
async fn hello_world() {
    // Create fixture and get your contract
    let fixture = Fixture::new();
    let contract_program = fixture
        .create_contract(vec![1])
        .await;

    // get contract service client  
    let mut service_client = contract_program.contract_svc();
    
    // Act

    // Use generated client code for calling Service service.
    // To send a message, you must specify:
    // - Service to send the message
    // - Service method to call
    // Or use the client that you get before.
    let response = service_client
        .greet() // Service method
        .await
        .unwrap(); 

    // Assert
    assert_eq!(format!("Hello {:?}", ActorId::from(ADMIN_ID)), response);
}

#[tokio::test]
async fn increment_and_decrement() {
    // Create fixture and get your contract
    let fixture = Fixture::new();
    let contract_program = fixture
        .create_contract(vec![1])
        .await;

    // get contract service client  
    let mut service_client = contract_program.contract_svc();
    
    // Assert increment

    let response = service_client // Service
        .increment() // Service method
        .await
        .unwrap(); 

    assert_eq!(response, 1);

    // Assert value

    let response = contract_program
        .contract_svc()
        .counter_value()
        .await
        .unwrap();

    assert_eq!(response, 1);

    // Assert decrement

    let response = contract_program
        .contract_svc()
        .decrement()
        .await
        .unwrap();

    assert_eq!(response, 0);

    // Assert error - decrement value

    let response = contract_program
        .contract_svc()
        .decrement()
        .await;

    assert!(response.is_err());
}


#[tokio::test]
async fn send_and_get_value() {
     // Create fixture and get your contract
    let fixture = Fixture::new();
    let contract_program = fixture
        .create_contract(vec![1])
        .await;

    // get contract service client  
    let mut service_client = contract_program.contract_svc();
    
    // Assert send value

    let response = service_client
        .send_value()
        .with_value(utils::ONE_VARA)
        .await
        .unwrap();

    assert_eq!(response, format!("Value get: {}", utils::ONE_VARA));

    let contract_balance = fixture.balance_of(contract_program.id());

    assert_eq!(contract_balance, utils::ONE_VARA * 2);

    // Assert get value

    let response = contract_program
        .contract_svc()
        .get_value(utils::ONE_VARA)
        .await
        .unwrap();

    assert_eq!(response, format!("Value returned: {}", utils::ONE_VARA));

    let contract_balance = fixture.balance_of(contract_program.id());

    assert_eq!(contract_balance, utils::ONE_VARA);

    // Assert error - Get balance
    
    let result = contract_program
        .contract_svc()
        .get_value(utils::ONE_VARA)
        .await;

    assert!(result.is_err());

}
