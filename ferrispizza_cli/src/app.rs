use ferrispizza_lib::{
    concurrency::{OrderChannel, SharedOrderState},
    utils::IdGenerator,
    billing::BillingEngine,
};

pub struct FerrisPizzaApp {
    pub order_channel: OrderChannel,
    pub order_state: SharedOrderState,
    pub id_gen: IdGenerator,
    pub billing: BillingEngine,
}

impl FerrisPizzaApp {
    pub fn new() -> Self {
        Self {
            order_channel: OrderChannel::new(),
            order_state: SharedOrderState::new(),
            id_gen: IdGenerator::new(),
            billing: BillingEngine::new(),
        }
    }

    pub fn run(&self) -> Result<(), String> {
        crate::cli::run_cli(self)
    }
}
