//! Toppings module — Decorator implementations

use crate::pizza::Pizza;
use crate::patterns::ToppingDecorator;

/// Cheese topping decorator
#[derive(Clone)]
pub struct Cheese {
    pizza: Box<dyn Pizza>,
    price: f32,
}

impl Cheese {
    /// Default: +10
    pub fn new(pizza: Box<dyn Pizza>) -> Self {
        Self { pizza, price: 10.0 }
    }

    /// Custom pricing
    pub fn with_price(pizza: Box<dyn Pizza>, price: f32) -> Self {
        Self { pizza, price }
    }
}

impl Pizza for Cheese {
    fn description(&self) -> String {
        format!("{} + Cheese", self.pizza.description())
    }

    fn cost(&self) -> f32 {
        self.pizza.cost() + self.price
    }
    fn clone_box(&self) -> Box<dyn crate::pizza::Pizza> {
        Box::new(self.clone())
    }
}

impl ToppingDecorator for Cheese {}


/// Olives topping decorator
#[derive(Clone)]
pub struct Olives {
    pizza: Box<dyn Pizza>,
    price: f32,
}

impl Olives {
    pub fn new(pizza: Box<dyn Pizza>) -> Self {
        Self { pizza, price: 15.0 }
    }

    pub fn with_price(pizza: Box<dyn Pizza>, price: f32) -> Self {
        Self { pizza, price }
    }
}

impl Pizza for Olives {
    fn description(&self) -> String {
        format!("{} + Olives", self.pizza.description())
    }

    fn cost(&self) -> f32 {
        self.pizza.cost() + self.price
    }
    fn clone_box(&self) -> Box<dyn crate::pizza::Pizza> {
        Box::new(self.clone())
    }
}

impl ToppingDecorator for Olives {}


/// Jalapenos topping decorator (will test soon)
#[derive(Clone)]
pub struct Jalapenos {
    pizza: Box<dyn Pizza>,
    price: f32,
}

impl Jalapenos {
    pub fn new(pizza: Box<dyn Pizza>) -> Self {
        Self { pizza, price: 12.0 }
    }

    pub fn with_price(pizza: Box<dyn Pizza>, price: f32) -> Self {
        Self { pizza, price }
    }
}

impl Pizza for Jalapenos {
    fn description(&self) -> String {
        format!("{} + Jalapenos", self.pizza.description())
    }

    fn cost(&self) -> f32 {
        self.pizza.cost() + self.price
    }
    fn clone_box(&self) -> Box<dyn crate::pizza::Pizza> {
        Box::new(self.clone())
    }
}

impl ToppingDecorator for Jalapenos {}
