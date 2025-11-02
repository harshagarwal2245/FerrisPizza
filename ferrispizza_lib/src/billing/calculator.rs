//! Billing calculator — computes total bill & generates receipts.
//!
//! The [`BillingEngine`] iterates through items in an [`order`] and sums
//! their cost using their `Pizza` trait API. It also generates a typed
//! [`PaymentReceipt`] carrying billing metadata.
//!
//! # Example
//! ```
//! use ferrispizza_lib::billing::{BillingEngine};
//! use ferrispizza_lib::pizza::Margherita;
//! use ferrispizza_lib::order::order;
//!
//! let engine = BillingEngine::new();
//! let order = order::new(vec![Box::new(Margherita::new())]);
//!
//! let total = engine.calculate_total(&order);
//! assert_eq!(total, 120.0);
//! ```

use std::time::{SystemTime, UNIX_EPOCH};
use crate::order::order;

/// Receipt generated after successful payment processing.
///
/// Contains order id, total billed amount, and generated timestamp.
#[derive(Debug, Clone, PartialEq)]
pub struct PaymentReceipt {
    pub order_id: u64,
    pub total_amount: f32,
    pub timestamp: u128,
}

/// Billing engine responsible for calculating total order cost
/// and producing [`PaymentReceipt`] objects.
///
/// # Responsibility
/// * Sum cost of all pizzas in an order
/// * Create timestamped receipt for finalized bill
pub struct BillingEngine;

impl BillingEngine {
    /// Creates a new billing engine instance.
    pub fn new() -> Self {
        BillingEngine
    }

    /// Calculate the total cost of the given order.
    ///
    /// Iterates over all pizzas and adds their cost.
    pub fn calculate_total(&self, order: &order) -> f32 {
        order.pizzas.iter().map(|p| p.cost()).sum()
    }

    /// Generate a receipt for the given order.
    ///
    /// Includes order id, computed total, and current timestamp.
    pub fn generate_receipt(&self, order: &order) -> PaymentReceipt {
        PaymentReceipt {
            order_id: order.id.0,
            total_amount: self.calculate_total(order),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pizza::{Margherita, Cheese};

    #[test]
    fn test_calculate_total() {
        let engine = BillingEngine::new();

        let pizza1 = Box::new(Margherita::new());
        let pizza2 = Box::new(Cheese::new(Box::new(Margherita::new())));

        let order = order::new(vec![pizza1, pizza2]);
        let total = engine.calculate_total(&order);

        assert_eq!(total, 250.0); // 120 + 130
    }

    #[test]
    fn test_generate_receipt() {
        let engine = BillingEngine::new();
        let pizza = Box::new(Margherita::new());
        let order = order::new(vec![pizza]);

        let receipt = engine.generate_receipt(&order);

        assert_eq!(receipt.order_id, order.id.0);
        assert_eq!(receipt.total_amount, 120.0);
        assert!(receipt.timestamp > 0);
    }
}
