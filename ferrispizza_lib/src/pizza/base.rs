use crate::pizza::Pizza;

/// Margherita Pizza
#[derive(Clone)]
pub struct Margherita {
    price: f32,
}

impl Margherita {
    pub fn new() -> Self {
        Self { price: 120.0 }
    }

    pub fn with_price(price: f32) -> Self {
        Self { price }
    }
}

impl Pizza for Margherita {
    fn description(&self) -> String {
        "Margherita".to_string()
    }

    fn cost(&self) -> f32 {
        self.price
    }

    fn clone_box(&self) -> Box<dyn crate::pizza::Pizza> {
        Box::new(self.clone())
    }
}

/// Farmhouse Pizza
#[derive(Clone)]
pub struct Farmhouse {
    price: f32,
}

impl Farmhouse {
    pub fn new() -> Self {
        Self { price: 150.0 }
    }

    pub fn with_price(price: f32) -> Self {
        Self { price }
    }
}

impl Pizza for Farmhouse {
    fn description(&self) -> String {
        "Farmhouse".to_string()
    }

    fn cost(&self) -> f32 {
        self.price
    }

    fn clone_box(&self) -> Box<dyn crate::pizza::Pizza> {
        Box::new(self.clone())
    }
}
