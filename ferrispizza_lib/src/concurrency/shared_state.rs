use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use super::super::order::order;

#[derive(Clone)]
pub struct SharedOrderState {
    pub orders: Arc<Mutex<HashMap<u64, order>>>,
}

impl SharedOrderState {
    pub fn new() -> Self {
        Self {
            orders: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_order(&self, order: order) {
        self.orders.lock().unwrap().insert(order.id.0, order);
    }

    pub fn get_order(&self, id: u64) -> Option<order> {
        self.orders.lock().unwrap().get(&id).cloned()
    }

    pub fn list_orders(&self) -> Vec<order> {
        self.orders.lock().unwrap().values().cloned().collect()
    }
}
