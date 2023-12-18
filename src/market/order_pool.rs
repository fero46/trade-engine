use std::collections::HashMap;
use std::collections::VecDeque;

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
    pub fn new(_size: u128, _resize: u128, _threshold: u128) -> OrderPool {
        let mut pool = OrderPool {
            memory: VecDeque::new(),
            in_use: HashMap::new(),
            size: _size,
            counter: 0,
            index: 0,
            resize: _resize,
            threshold: _threshold,
        };
        {
            let ref mut this = pool;
            let mut i = 0;
            while i < this.size {
                this.index = this.index + 1;
                let mut o = Order::new(this.index);
                this.memory.push_front(o);
                i += 1;
            }
        };
        pool
    }

    pub fn counter(&self) -> u128 {
        self.counter
    }
    fn init(&mut self) {
        let mut i = 0;
        while i < self.size {
            self.index = self.index + 1;
            let mut o = Order::new(self.index);
            self.memory.push_front(o);
            i += 1;
        }
    }

    fn realloc(&mut self) {
        if self.threshold < self.size - self.counter {
            return;
        }
        let mut i = 0;
        while i < self.resize {
            self.index = self.index + 1;
            let o = Order::new(self.index);
            self.memory.push_front(o);
            i += 1;
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
        self.counter = self.counter + 1;
        let option: Option<Order> = self.memory.pop_front();
        if option.is_some() {
            let mut order: Order = option.unwrap();
            order.setValues(action, direction, price, amount, reference);
            self.in_use.insert(order.id(), order);
            return order;
        } else {
            self.index = self.index + 1;
            let mut net_order: Order = Order::new(self.index);
            net_order.setValues(action, direction, price, amount, reference);
            net_order
        }
    }

    /// .
    pub fn free_order(&mut self, mut order: Order) {
        self.in_use.remove(&order.id());
        order.clear();
        self.memory.push_back(order);
        self.counter = self.counter - 1;
    }
}
