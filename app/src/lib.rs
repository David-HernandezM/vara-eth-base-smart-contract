#![no_std]

use sails_rs::{
    prelude::*,
    cell::RefCell,
};

pub mod services;

use services::contract_service::{Service, CounterData};

pub struct Program {
    state: RefCell<CounterData>,
}

// `program(payable)` allows receiving value with an empty payload.
// Concrete entrypoints that receive ETH/value still need `#[export(payable)]`.
#[program(payable)]
impl Program {
    pub fn init() -> Self {
        Self {
            state: RefCell::new(Default::default()),
        }
    }

    #[export(route = "ContractSvc")]
    pub fn contract_svc(&self) -> Service<'_> {
        Service::new(&self.state)
    }
}
