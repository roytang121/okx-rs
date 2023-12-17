use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};

use rust_decimal::Decimal;

use crate::api::v5::Side;

pub mod book_manager;
type Fixed = Decimal;

#[derive(Debug)]
pub struct PartialLevel {
    size: Fixed,
}
impl From<Fixed> for PartialLevel {
    fn from(size: Fixed) -> Self {
        Self { size }
    }
}

#[derive(Default)]
pub struct OrderBook {
    bids: BTreeMap<Reverse<Decimal>, PartialLevel>,
    asks: BTreeMap<Decimal, PartialLevel>,
}

impl std::fmt::Debug for OrderBook {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?} asks / ", self.asks.iter().take(8).rev().map(|(price, level)| format!("({},{})", price, level.size)).collect::<Vec<String>>())?;
        writeln!(f, "bids {:?}", self.bids.iter().take(8).map(|(price, level)| format!("({},{})", price.0, level.size)).collect::<Vec<String>>())
    }
}

impl OrderBook {
    pub fn handle_level(&mut self, price: Fixed, size: Fixed, side: Side, bbo: bool) {
        if size <= Decimal::ZERO {
            self.remove_level(price, side);
        } else {
            self.update_level(price, size, side);
        }

        if bbo {
            self.handle_bbo(price, size, side);
        }
    }

    fn handle_bbo(&mut self, price: Fixed, size: Fixed, side: Side) {
        match side {
            Side::Buy => self.bids.retain(|k, v| k.0 <= price),
            Side::Sell => self.asks.retain(|k, v| *k >= price)
        };
    }

    fn update_level(&mut self, price: Fixed, size: Fixed, side: Side) {
        let partial_level = match side {
            Side::Buy => self.bids.entry(Reverse(price)).or_insert_with(|| size.into()),
            Side::Sell => self.asks.entry(price).or_insert_with(|| size.into()),
        };
        partial_level.size = size;
    }

    fn remove_level(&mut self, price: Fixed, side: Side) {
        match side {
            Side::Buy => self.bids.remove(&Reverse(price)),
            Side::Sell => self.asks.remove(&price),
        };
    }

    fn best_bid(&self) -> Option<(Fixed, Fixed)> {
        self.bids.iter().next().map(|(k, v)| (k.0, v.size))
    }

    fn best_ask(&self) -> Option<(Fixed, Fixed)> {
        self.asks.iter().next().map(|(k, v)| (*k, v.size))
    }

    fn crossed(&self) -> bool {
        match (self.best_bid(), self.best_ask()) {
            (Some((bid, _)), Some((ask, _))) => bid > ask,
            _ => false
        }
    }
}