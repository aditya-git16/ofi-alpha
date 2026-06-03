use serde::Deserialize;

/// Side of the orderbook
pub enum Side {
    Ask,
    Bid,
}

// We will do top of the book first for analysis
// tob -> the top bid(price,qty) and ask(price,qty)
// so let's define these types too , will use u64 , since they cant be negative and will use scaling factors for decimal conversions
// will define type alias for both

/// Price of an asset
pub type Price = u64;

/// Size of an asset
pub type Size = u64;

/// Top of the book
#[derive(Debug, Deserialize)]
pub struct TopOfBook {
    pub timestamp: Timestamp,
    pub bid_price: Price,
    pub bid_size: Size,
    pub ask_price: Price,
    pub ask_size: Size,
}

impl TopOfBook {
    // mid price feature
    pub fn mid(&self) -> f64 {
        return (self.bid_price + self.ask_price) as f64 / 2.0;
    }

    // spread
    pub fn spread(&self) -> Price {
        return self.bid_price - self.ask_price;
    }

    // order book imbalance feature (obi)
    pub fn obi(&self) -> f64 {
        return (self.bid_size - self.ask_price) as f64 / (self.bid_size + self.ask_price) as f64;
    }

    // microprice feature ( weighted mid price )
    pub fn microprice(&self) -> f64 {
        return (self.bid_price * self.ask_size + self.bid_size * self.ask_price) as f64
            / (self.bid_size + self.ask_size) as f64;
    }

    // depth feature(total liquidity at tob)
    pub fn depth(&self) -> Price {
        return self.bid_size + self.ask_size;
    }

}

/// Timestamp
pub type Timestamp = u64;
