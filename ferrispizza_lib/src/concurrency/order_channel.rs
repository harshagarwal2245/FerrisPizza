use std::sync::mpsc::{self, Sender, Receiver};
use crate::order::order;

pub struct OrderChannel {
    sender: Sender<order>,
    receiver: Receiver<order>,
}

impl OrderChannel {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { sender: tx, receiver: rx }
    }

    pub fn sender(&self) -> Sender<order> {
        self.sender.clone()
    }

    pub fn receive(&self) -> Option<order> {
        self.receiver.recv().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::order::order;
    use crate::pizza::Margherita;

    #[test]
    fn order_channel_sends_and_receives() {
        let channel = OrderChannel::new();
        let sender = channel.sender();

        let order = order::new(vec![Box::new(Margherita::new())]);
        sender.send(order.clone()).unwrap();

        let received = channel.receive().unwrap();
        assert_eq!(received.id, order.id);
    }
}
