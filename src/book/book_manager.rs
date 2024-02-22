use crate::api::v5::{BookUpdate, Levels, Side};
use crate::book::OrderBook;

type Seq = i64;

#[derive(Debug, Default)]
pub struct BookManager {
    book: OrderBook,
    pub last_seq: Option<Seq>,
    pub last_exch_ts: Option<u64>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum BookUpdateType {
    BBO,
    Diff,
    Snapshot,
}

impl BookManager {
    #[allow(clippy::all)]
    pub fn handle_book_update(&mut self, update: BookUpdate, update_type: BookUpdateType) -> bool {
        if update.seq_id < update.prev_seq_id.expect("no prev seq") {
            // sequence reset due to maintenance. just panics for now
            // TODO: handle seq reset
            todo!("unhandled seq reset");
        }
        let should_update = if let Some(last_seq) = self.last_seq {
            if update.seq_id == last_seq {
                // TODO: verify all updates matches current book
                // TODO: verify exch timestamp
                // TODO: verify checksum
                false
            } else if update.seq_id > last_seq {
                // TODO: commit update
                true
            } else {
                // drop
                false
            }
        } else {
            // first book update has to be snapshot
            update_type == BookUpdateType::Snapshot
        };

        if should_update {
            let BookUpdate {
                seq_id,
                ts,
                bids,
                asks,
                ..
            } = update;
            self.last_seq = Some(seq_id);
            self.last_exch_ts = Some(ts.expect("no ts"));

            // imply depth levels if bbo
            if update_type == BookUpdateType::BBO {
                match bids {
                    Levels::Depth1(bids) => {
                        let bid = bids[0];
                        self.book.handle_level(
                            bid.price.parse().unwrap(),
                            bid.size.parse().unwrap(),
                            Side::Buy,
                            true,
                        );
                    }
                    _ => unreachable!("not an bbo"),
                }
                match asks {
                    Levels::Depth1(asks) => {
                        let ask = asks[0];
                        self.book.handle_level(
                            ask.price.parse().unwrap(),
                            ask.size.parse().unwrap(),
                            Side::Sell,
                            true,
                        );
                    }
                    _ => unreachable!("not an bbo"),
                }
            } else {
                for bid in bids.iter() {
                    self.book.handle_level(
                        bid.price.parse().unwrap(),
                        bid.size.parse().unwrap(),
                        Side::Buy,
                        false,
                    );
                }
                for ask in asks.iter() {
                    self.book.handle_level(
                        ask.price.parse().unwrap(),
                        ask.size.parse().unwrap(),
                        Side::Sell,
                        false,
                    );
                }
            }

            self.last_seq = Some(seq_id);
            self.last_exch_ts = *ts;
            // println!("{:?}", self.book);
            debug_assert!(!self.book.crossed(), "crossed book");
        }
        should_update
    }
}
