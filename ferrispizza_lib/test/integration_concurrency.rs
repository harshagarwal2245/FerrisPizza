use ferrispizza_core::{
    order::Order,
    pizza::Margherita,
    concurrency::{OrderChannel, SharedOrderState}
};

use std::thread;
use std::time::Duration;

#[test]
fn test_order_channel_send_receive() {
    let channel = OrderChannel::new(10);

    let sender = channel.sender();
    let receiver = channel.receiver();

    let order = Order::new(vec![Box::new(Margherita::new())]);

    sender.send(order.clone()).expect("send ok");
    let received = receiver.recv().expect("recv ok");

    assert_eq!(order.id, received.id);
}

#[test]
fn test_shared_state_updates() {
    let state = SharedOrderState::new();

    let order1 = Order::new(vec![Box::new(Margherita::new())]);
    let order2 = Order::new(vec![Box::new(Margherita::new())]);

    state.add_order(order1.id);
    state.add_order(order2.id);

    let orders = state.get_all();

    assert_eq!(orders.len(), 2);
}

#[test]
fn test_concurrent_workers_processing_orders() {
    let channel = OrderChannel::new(10);
    let state = SharedOrderState::new();

    let sender = channel.sender();
    let receiver = channel.receiver();
    let state_clone = state.clone();

    // Worker thread
    thread::spawn(move || {
        while let Ok(order) = receiver.recv() {
            state_clone.add_order(order.id);
        }
    });

    // Send 5 orders
    for _ in 0..5 {
        sender.send(Order::new(vec![Box::new(Margherita::new())])).unwrap();
    }

    // give thread time
    thread::sleep(Duration::from_millis(200));

    assert_eq!(state.get_all().len(), 5);
}
