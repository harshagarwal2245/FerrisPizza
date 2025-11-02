//! Order domain model for FerrisPizza.
//! Represents a pizza order with status, timestamp and total calculation.

use std::fmt::{self, Display};
use std::time::SystemTime;
use crate::pizza::Pizza;
use crate::utils::IdGenerator;

/// Unique Order Id wrapper
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrderId(pub u64);

/// Order status lifecycle
#[derive(Clone, Debug, PartialEq)]
pub enum OrderStatus {
    Created,
    Paid,
    Completed,
}

/// Represents a customer's order
#[derive(Clone)]
pub struct order {
    pub id: OrderId,
    pub pizzas: Vec<Box<dyn Pizza>>,
    pub status: OrderStatus,
    pub timestamp: SystemTime,
}

impl order {
    /// Create a new order with pizzas
    pub fn new(pizzas: Vec<Box<dyn Pizza>>) -> Self {
        Self {
            id: OrderId(IdGenerator::new().next_id().parse().unwrap()),
            pizzas,
            status: OrderStatus::Created,
            timestamp: SystemTime::now(),
        }
    }

    /// Calculate total cost by summing pizza prices
    pub fn total_cost(&self) -> f32 {
        self.pizzas.iter().map(|p| p.cost()).sum()
    }

    /// Mark order as paid
    pub fn mark_paid(&mut self) {
        self.status = OrderStatus::Paid;
    }

    /// Mark order complete
    pub fn mark_completed(&mut self) {
        self.status = OrderStatus::Completed;
    }
}

/// Nicely print an order for CLI display
impl Display for order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pizzas_str = self
            .pizzas
            .iter()
            .map(|p| format!("{} ({:.2})", p.description(), p.cost()))
            .collect::<Vec<_>>()
            .join(", ");

        write!(
            f,
            "Order #{:?}\nPizzas: {}\nTotal: {:.2}\nStatus: {:?}",
            self.id.0,
            pizzas_str,
            self.total_cost(),
            self.status
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pizza::Margherita;
    use crate::pizza::Cheese;

    #[test]
    fn new_order_has_created_status() {
        let pizzas = vec![Box::new(Margherita::new()) as Box<dyn Pizza>];
        let order = order::new(pizzas);
        assert_eq!(order.status, OrderStatus::Created);
    }

    #[test]
    fn calculate_total_for_multiple_pizzas() {
        let pizzas = vec![
            Box::new(Margherita::new()) as Box<dyn Pizza>,
            Box::new(Cheese::new(Box::new(Margherita::new()))) as Box<dyn Pizza>
        ];

        let order = order::new(pizzas);
        assert_eq!(order.total_cost(), 120.0 + 130.0);
    }

    #[test]
    fn order_status_transitions() {
        let pizzas = vec![Box::new(Margherita::new()) as Box<dyn Pizza>];
        let mut order = order::new(pizzas);

        order.mark_paid();
        assert_eq!(order.status, OrderStatus::Paid);

        order.mark_completed();
        assert_eq!(order.status, OrderStatus::Completed);
    }
}

