use super::super::pizza::Pizza;
use super::base::{Margherita, Farmhouse};

#[derive(Clone)]
pub struct ThinCrust {
    pizza: Box<dyn Pizza>,
}

impl ThinCrust {
    pub fn new(pizza: impl Pizza + 'static) -> Self {
        Self {
            pizza: Box::new(pizza),
        }
    }
}

impl Pizza for ThinCrust {
    fn description(&self) -> String {
        format!("{}, Thin Crust", self.pizza.description())
    }

    fn cost(&self) -> f32 {
        self.pizza.cost() + 20.0
    }

    fn clone_box(&self) -> Box<dyn crate::pizza::Pizza> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct CheeseBurst {
    pizza: Box<dyn Pizza>,
}

impl CheeseBurst {
    pub fn new(pizza: impl Pizza + 'static) -> Self {
        Self {
            pizza: Box::new(pizza),
        }
    }
}

impl Pizza for CheeseBurst {
    fn description(&self) -> String {
        format!("{}, CheeseBurst Crust", self.pizza.description())
    }

    fn cost(&self) -> f32 {
        self.pizza.cost() + 50.0
    }
    fn clone_box(&self) -> Box<dyn crate::pizza::Pizza> {
        Box::new(self.clone())
    }
}


#[test]
fn test_thin_crust_margherita() {
    let pizza = Margherita::new();
    let crust = ThinCrust::new(pizza);

    assert_eq!(crust.description(), "Margherita, Thin Crust");
    assert_eq!(crust.cost(), 100.0 + 40.0);
}

#[test]
fn test_thick_crust_farmhouse() {
    let pizza = Farmhouse::new();
    let crust = CheeseBurst::new(pizza);

    assert_eq!(crust.description(), "Farmhouse, CheeseBurst Crust");
    assert_eq!(crust.cost(), 150.0 + 50.0);
}
