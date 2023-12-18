pub enum OrderDirection {
    BID,
    ASK,
    IDLE,
}

pub enum OrderAction {
    LIMIT,
    MARKET,
    CANCLE,
    IDLE,
}

pub struct Order {
    id: u128,
    action: OrderAction,
    direction: OrderDirection,
    price: u128,
    amount: u128,
    reference: u128,
}

impl Order {
    pub fn new(identifer: u128) -> Order {
        Order {
            id: identifer,
            action: OrderAction::IDLE,
            direction: OrderDirection::IDLE,
            price: 0,
            amount: 0,
            reference: 0,
        }
    }

    pub fn clear(&mut self) {
        self.id = 0;
        self.action = OrderAction::IDLE;
        self.direction = OrderDirection::IDLE;
        self.price = 0;
        self.amount = 0;
        self.reference = 0;
    }

    pub fn setValues(
        &mut self,
        action: OrderAction,
        direction: OrderDirection,
        price: u128,
        amount: u128,
        reference: u128,
    ) {
        self.action = action;
        self.direction = direction;
        self.price = price;
        self.amount = amount;
        self.reference = reference;
    }

    pub fn id(&self) -> u128 {
        self.id
    }

    pub fn action(&self) -> &OrderAction {
        &self.action
    }

    pub fn direction(&self) -> &OrderDirection {
        &self.direction
    }

    pub fn price(&self) -> u128 {
        self.price
    }

    pub fn reference(&self) -> u128 {
        self.reference
    }

    pub fn price_mut(&mut self) -> &mut u128 {
        &mut self.price
    }
}
