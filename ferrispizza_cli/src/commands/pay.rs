//! Process order payment via CLI

use crate::commands::PaymentMethodCommand;
use ferrispizza_lib::concurrency::SharedOrderState;
use ferrispizza_lib::billing::BillingEngine;
use ferrispizza_lib::errors::BillingError;
use ferrispizza_lib::patterns::{PaymentAdapter, UpiPayment, CardPayment};

pub fn pay_order(state: &SharedOrderState, order_id: u64, method: PaymentMethodCommand) -> Result<(),BillingError> {
    let mut order = state.get_order(order_id).clone().unwrap();
    let billing = BillingEngine::new();
    let adapter: Box<dyn PaymentAdapter> = match method {
        PaymentMethodCommand::UPI => Box::new(UpiPayment::with_success_rate("tester@upi", 1.0)),
        PaymentMethodCommand::Card => Box::new(CardPayment::new("4111222233334444", 123)),
    };

    let receipt = adapter.pay(&order)?;
    println!("Payment successful for Order {}!", order_id);
    println!("Total paid: {}", receipt.total_amount);
    order.mark_paid();
    Ok(())
}
