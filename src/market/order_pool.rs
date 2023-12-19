use std::collections::{HashMap, VecDeque};

use super::order::{Order, OrderAction, OrderDirection};

pub struct OrderPool {
    memory: VecDeque<Order>,
    in_use: HashMap<u128, Order>,
    size: u128,
    counter: u128,
    resize: u128,
    threshold: u128,
    index: u128,
}

impl OrderPool {
    pub fn new(size: u128, resize: u128, threshold: u128) -> OrderPool {
        let mut pool = OrderPool {
            memory: VecDeque::new(),
            in_use: HashMap::new(),
            size,
            counter: 0,
            index: 0,
            resize,
            threshold,
        };
        pool.init();
        pool
    }

    pub fn counter(&self) -> u128 {
        self.counter
    }

    fn init(&mut self) {
        for _ in 0..self.size {
            self.index += 1;
            let order = Order::new(self.index);
            self.memory.push_front(order);
        }
    }

    fn realloc(&mut self) {
        if self.threshold < self.size - self.counter {
            return;
        }
        for _ in 0..self.resize {
            self.index += 1;
            let order = Order::new(self.index);
            self.memory.push_front(order);
        }
    }

    pub fn build_order(
        &mut self,
        reference: u128,
        action: OrderAction,
        direction: OrderDirection,
        price: u128,
        amount: u128,
    ) -> Order {
        self.realloc();
        self.counter += 1;

        if let Some(order) = self.memory.pop_front() {
            let mut order = order;
            order.set_values(action, direction, price, amount, reference);
            self.in_use.insert(order.id(), order.clone());
            order
        } else {
            self.index += 1;
            let mut net_order = Order::new(self.index);
            net_order.set_values(action, direction, price, amount, reference);
            self.in_use.insert(net_order.id(), net_order.clone());
            net_order
        }
    }

    pub fn free_order(&mut self, order: Order) {
        self.in_use.remove(&order.id());
        self.memory.push_back(order);
        self.counter = self.counter.saturating_sub(1);
    }
}
