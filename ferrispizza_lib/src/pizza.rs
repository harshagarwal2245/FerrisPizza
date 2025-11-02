//! Pizza module — implements pizza types & decorators.
// Declare internal structure privately
mod base;
mod crust;
mod toppings;

// Public re-exports
pub use base::{Margherita, Farmhouse};
pub use crust::{ThinCrust, CheeseBurst};
pub use toppings::{Cheese, Olives, Jalapenos};

/// Trait for all pizzas.
pub trait Pizza: Send + Sync {
    fn description(&self) -> String;
    fn cost(&self) -> f32;
    
    fn clone_box(&self) -> Box<dyn Pizza>;
}

impl Clone for Box<dyn Pizza> {
    fn clone(&self) -> Box<dyn Pizza> {
        self.clone_box()
    }
}

// ---- TESTS ----
#[cfg(test)]
mod tests {
    use super::*;

    /// Dummy pizza to test decorator functionality
    #[derive(Clone)]
    struct TestPizza;

    impl Pizza for TestPizza {
        fn description(&self) -> String {
            "Test Pizza".to_string()
        }

        fn cost(&self) -> f32 {
            100.0
        }
        fn clone_box(&self) -> Box<dyn crate::pizza::Pizza> {
        Box::new(self.clone())
    }
    }

    // ---- Base pizza tests ----

    #[test]
    fn margherita_description_and_cost() {
        let p = crate::pizza::Margherita::new();
        assert_eq!(p.description(), "Margherita");
        assert_eq!(p.cost(), 120.0);
    }

    #[test]
    fn farmhouse_description_and_cost() {
        let p = crate::pizza::Farmhouse::new();
        assert_eq!(p.description(), "Farmhouse");
        assert_eq!(p.cost(), 150.0);
    }

    // ---- Decorator tests ----

    #[test]
    fn cheese_decorator_adds_cost() {
        let p = Cheese::new(Box::new(TestPizza));
        assert_eq!(p.description(), "Test Pizza + Cheese");
        assert_eq!(p.cost(), 110.0); // +10
    }

    #[test]
    fn olives_decorator_adds_cost() {
        let p = Olives::new(Box::new(TestPizza));
        assert_eq!(p.description(), "Test Pizza + Olives");
        assert_eq!(p.cost(), 115.0); // +15
    }

    #[test]
    fn multiple_decorators_accumulate() {
        let p = Cheese::new(Box::new(Olives::new(Box::new(TestPizza))));
        assert_eq!(p.description(), "Test Pizza + Olives + Cheese");
        assert_eq!(p.cost(), 125.0); // 100 + 15 + 10
    }
}
