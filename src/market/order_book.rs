use super::order::{Order, OrderAction, OrderDirection};
use super::order_pool::OrderPool;
use super::storage::OrderStore;

pub struct OrderBook {
    ask: OrderStore,
    bid: OrderStore,
    pool: OrderPool,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            ask: OrderStore::new(false),
            bid: OrderStore::new(true),
            pool: OrderPool::new(300, 200, 50),
        }
    }

    pub fn insert(
        &mut self,
        action: OrderAction,
        direction: OrderDirection,
        price: u128,
        amount: u128,
        reference: u128,
    ) {
        let order: Order = self
            .pool
            .build_order(reference, action, direction, price, amount);

        match order.action() {
            OrderAction::LIMIT => self.process_limit(order),
            OrderAction::MARKET => self.process_limit(order),
            OrderAction::CANCEL => self.process_limit(order),
            OrderAction::IDLE => return,
        }
    }

    pub fn process_limit(&mut self, order: Order) {
        match order.direction() {
            OrderDirection::BID => self.find_matching_orders(&self.ask, order),
            OrderDirection::ASK => self.find_matching_orders(&self.bid, order),
            OrderDirection::IDLE => return,
        }
    }

    pub fn pool(&self) -> &OrderPool {
        &self.pool
    }

    fn find_matching_orders(&self, order_store: &OrderStore, order: Order) {
        match order.action() {
            OrderAction::LIMIT => {
                let orders_iterators = order_store.head_set(order.price());
                return;
            }
            _ => {
                order_store.set();
                return;
            }
        }
    }
}

impl Default for OrderBook {
    fn default() -> Self {
        Self::new()
    }
}
