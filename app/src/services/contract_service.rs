use sails_rs::{
    prelude::*,
    cell::RefCell
};

#[event]
// #[derive(Clone, Encode, TypeInfo, Debug, ReflectHash)]
#[derive(Clone, Encode, TypeInfo, Debug)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ContractEvent {
    Hello([u8; 20]),
    ValueReceived(u128),
    ValueSent(u128),
    Incremented,
    Decremented
}

#[derive(Default)]
pub struct CounterData {
    counter: u64
}

pub struct Service<'a> {
    state: &'a RefCell<CounterData>,
}

fn eth_address_from_actor(actor_id: ActorId) -> [u8; 20] {
    let bytes = actor_id.into_bytes();
    bytes[12..]
        .try_into()
        .expect("ActorId should always contain a 20-byte EVM address suffix")
}

impl <'a> Service<'a> {
    pub fn new(state: &'a RefCell<CounterData>) -> Self {
        Self { state }
    }
}

#[service(events = ContractEvent)]
impl Service<'_> {
    fn emit_contract_event(&mut self, event: ContractEvent) {
        self.emit_eth_event(event).unwrap();
    }

    #[export]
    pub fn greet(&mut self) -> String {
        let msg_source = Syscall::message_source();

        self.emit_contract_event(ContractEvent::Hello(eth_address_from_actor(msg_source)));

        format!("Hello {:?}", msg_source)
    }

    #[export(payable)]
    pub fn send_value(&mut self) -> String {
        let value = Syscall::message_value();
        self.emit_contract_event(ContractEvent::ValueReceived(value));

        format!("Value get: {}", value)
    }

    #[export]
    pub fn get_value(&mut self, to_return: u128) -> CommandReply<String> {
        let contract_tokens = Syscall::value_available();

        if contract_tokens >= to_return {
            self.emit_contract_event(ContractEvent::ValueSent(to_return));
            CommandReply::new(format!("Value returned: {}", to_return)).with_value(to_return)
        } else {
            panic!("Cant transfer tokens");
        }
    }

    #[export]
    pub fn increment(&mut self) -> u64 {
        let mut state = self.state.borrow_mut();
                
        self.emit_contract_event(ContractEvent::Incremented);

        state.counter += 1;
        state.counter
    }

    #[export(unwrap_result)]
    pub fn decrement(&mut self) -> Result<u64, String> {
        let mut state = self.state.borrow_mut();
        
        state.counter = state.counter
            .checked_sub(1)
            .ok_or("Counter can not be negative!".to_string())?;

        self.emit_contract_event(ContractEvent::Decremented);

        Ok(state.counter)
    }

    #[export]
    pub fn counter_value(&self) -> u64 {
        self.state.borrow().counter
    }

    #[export]
    pub fn contract_total_eth(&self) -> u128 {
        Syscall::value_available()
    }
}

#[cfg(test)]
mod tests {
    use sails_rs::gstd::services::Service as SailsService;
    use super::*;

    #[test]
    pub fn test_greet() {
        Syscall::with_message_source(ActorId::from(3));

        let state = RefCell::new(Default::default());
        let mut contract_service = Service::new(&state).expose(&[]);

        let response = contract_service.greet();

        let expected_result = format!("Hello {:?}", ActorId::from(3));

        assert_eq!(expected_result, response);
    }

    #[test]
    pub fn test_send_value() {
        Syscall::with_message_value(1000);

        let state = RefCell::new(Default::default());
        let mut contract_service = Service::new(&state).expose(&[]);

        let response = contract_service.send_value();

        assert_eq!("Value get: 1000", response);
    }

    #[test]
    pub fn test_get_value() {
        Syscall::with_value_available(10000);

        let state = RefCell::new(Default::default());
        let mut contract_service = Service::new(&state).expose(&[]);


        let (response, amount) = contract_service.get_value(1000).to_tuple();

        assert_eq!(amount, 1000);
        assert_eq!(response, "Value returned: 1000");
    }

    #[test]
    pub fn test_increment_value() {
        let state = RefCell::new(Default::default());
        let mut contract_service = Service::new(&state).expose(&[]);

        let response = contract_service.increment();

        assert_eq!(response, 1);
        assert_eq!(state.borrow().counter, 1);
    }

    #[test]
    pub fn test_decrement_value() {
        let state = RefCell::new(Default::default());
        let mut contract_service = Service::new(&state).expose(&[]);

        let response = contract_service.increment();

        assert_eq!(response, 1);
        assert_eq!(state.borrow().counter, 1);

        let response = contract_service.decrement();

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), 0);
    }

    #[test]
    pub fn test_decrement_error() {
        let state = RefCell::new(Default::default());
        let mut contract_service = Service::new(&state).expose(&[]);

        let response = contract_service.decrement();

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), "Counter can not be negative!");
    }

    #[test]
    #[should_panic(expected = "Cant transfer tokens")]
    pub fn test_get_value_error() {
        Syscall::with_value_available(10);

        let state = RefCell::new(Default::default());
        let mut contract_service = Service::new(&state).expose(&[]);

        contract_service.get_value(15);
    }
}
