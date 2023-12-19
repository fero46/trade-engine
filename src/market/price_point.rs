use super::order::Order;
#[derive(Clone)]
pub struct PricePoint {
    price: u128,
    list: Vec<Order>,
}

impl PricePoint {
    pub fn new(price: u128) -> Self {
        Self {
            price,
            list: Vec::new(),
        }
    }

    pub(crate) fn add(&mut self, _order: &Order) {
        self.list.push(_order.clone());
    }
}
