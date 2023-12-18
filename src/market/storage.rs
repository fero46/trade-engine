pub(crate) use super::order::Order;
use super::price_point::PricePoint;
use itertools::Either;
use std::collections::btree_set::Range;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::iter::Rev;
use std::ops::Bound::Included;

pub struct OrderStore {
    search: BTreeSet<u128>,
    map: HashMap<u128, PricePoint>,
    reverse: bool,
    min: u128,
    max: u128,
}

impl OrderStore {
    pub fn new(reverse: bool) -> OrderStore {
        OrderStore {
            search: BTreeSet::new(),
            map: HashMap::new(),
            reverse,
            min: 0,
            max: 0,
        }
    }

    pub fn insert(&mut self, _order: Order) {
        let price_point;
        if let Some(price) = self.map.get(&_order.price()) {
            price_point = price
        } else {
            price_point = &PricePoint::new(_order.price());
            self.map.insert(_order.price(), price_point);
        }
        self.search.insert(_order.price());
        price_point.add(_order);
    }

    pub(crate) fn headSet(&self, price: u128) -> Either<Rev<Range<u128>>, Range<u128>> {
        let min;
        let max;
        if self.reverse {
            min = price;
            max = *self.search.iter().rev().next().unwrap();
        } else {
            min = *self.search.iter().next().unwrap();
            max = price;
        }
        let range: Range<u128> = self.search.range((Included(&min), Included(&max)));
        if self.reverse {
            Either::Left(range.rev().into())
        } else {
            return Either::Right(range.into());
        }
    }

    pub(crate) fn set(&self) -> Either<Rev<Range<u128>>, Range<u128>> {
        if self.reverse {
            return self.headSet(*self.search.iter().next().unwrap());
        } else {
            return self.headSet(*self.search.iter().rev().next().unwrap());
        }
    }
}
