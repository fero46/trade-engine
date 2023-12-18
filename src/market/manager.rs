use std::collections::HashMap;
use super::order_book::OrderBook;
use super::order::{ OrderAction, OrderDirection};


pub struct Manager {
    order_books: HashMap<String, OrderBook>
}

impl Manager {
    pub fn new() -> Self { 
        Manager { order_books: HashMap::new()}
    }

    pub fn buildMarket(&mut self, marketplace: String){
        let book = OrderBook::new();
        self.order_books.insert(marketplace, book);
    }

    pub fn insertOrder(&mut self, marketplace: String, action: OrderAction, direction: OrderDirection, price: u128, amount: u128, reference: u128)
    {
        if !self.order_books.contains_key(&marketplace) {
            return;
        }

        let book:&mut OrderBook = self.order_books.get_mut(&marketplace).unwrap();
        book.insert(action, direction, price, amount, reference);        
    }
}

