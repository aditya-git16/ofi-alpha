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
pub struct TopOfBook {
    pub bid_price: Price,
    pub bid_size: Size,
    pub ask_price: Price,
    pub ask_size: Size,
}

/// Timestamp 
pub type Timestamp = u64;