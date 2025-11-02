//! Place a pizza order via CLI

use crate::commands::PizzaSelection;
use ferrispizza_lib::errors::{BillingError, OrderError};
use ferrispizza_lib::order::order;
use ferrispizza_lib::pizza::*;
use ferrispizza_lib::concurrency::SharedOrderState;

pub fn place_order(state: &SharedOrderState, items: Vec<PizzaSelection>) -> Result<(),OrderError> {
    let mut pizza: Vec<Box<dyn Pizza>>  = Vec::new();

    for item in items {
        match item {
            PizzaSelection::Margherita => pizza.push(Box::new(Margherita::new())),
            PizzaSelection::Farmhouse => pizza.push(Box::new(Farmhouse::new())),
            PizzaSelection::ThinMargherita => {
                pizza.push(Box::new(ThinCrust::new(Margherita::new())))
            },
            PizzaSelection::CheeseBurstFarmhouse => {
                pizza.push(Box::new(CheeseBurst::new(Farmhouse::new())))
            }
        }
    }
    let order1 = order::new(pizza);
    let y = &order1;
    state.add_order(order1.clone());
    println!(" Order placed successfully! Order ID: {}", y.id.0);
    Ok(())
}
