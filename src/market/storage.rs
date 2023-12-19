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
            min: u128::MAX, // Initialize with the maximum value for the correct comparison
            max: 0,
        }
    }

    pub fn insert(&mut self, _order: Order) {
        let price_point = self
            .map
            .entry(_order.price().clone())
            .or_insert_with(|| PricePoint::new(_order.price()))
            .clone();

        self.search.insert(_order.price().clone());
        let mut price_point_mut = self.map.get_mut(&_order.price()).unwrap();
        price_point_mut.add(&_order.into());
    }

    pub(crate) fn head_set(&self, price: u128) -> Either<Rev<Range<u128>>, Range<u128>> {
        let (min, max) = if self.reverse {
            (price, *self.search.iter().rev().next().unwrap_or(&0))
        } else {
            (*self.search.iter().next().unwrap_or(&u128::MAX), price)
        };

        let range: Range<u128> = self.search.range((Included(&min), Included(&max)));

        if self.reverse {
            Either::Left(range.rev().into())
        } else {
            Either::Right(range.into())
        }
    }

    pub(crate) fn set(&self) -> Either<Rev<Range<u128>>, Range<u128>> {
        if self.reverse {
            self.head_set(*self.search.iter().next().unwrap_or(&0))
        } else {
            self.head_set(*self.search.iter().rev().next().unwrap_or(&u128::MAX))
        }
    }
}
