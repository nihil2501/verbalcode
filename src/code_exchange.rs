use std::result;
use wasmbus_rpc::actor::prelude::*;

pub trait CodeExchange {
    fn create(&self, message: String) -> CreateResult;
    fn find(&self, code: String) -> FindResult;
}

pub type CreateResult = result::Result<String, CreateError>;
pub type FindResult = result::Result<String, FindError>;

pub enum CreateError {
    Invalid(String),
}

pub enum FindError {
    NotFound,
}

pub struct ActorCodeExchange<'a> {
    context: &'a Context,
}

impl ActorCodeExchange<'_> {
    pub fn new(context: &Context) -> ActorCodeExchange {
        ActorCodeExchange { context }
    }
}

impl CodeExchange for ActorCodeExchange<'_> {
    fn create(&self, message: String) -> CreateResult {
        todo!()
    }

    fn find(&self, code: String) -> FindResult {
        todo!()
    }
}
