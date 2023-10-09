//! # OpenTelemetry
//!
//! While looking at JSON data in a terminal might be a good start, it's not very practical.  
//! Analyzing telemetry data using `jq` or scrolling through a terminal window is fairly tedious
//! and error-prone.  
//! In most real-world scenarios, we'll want to **export** our telemetry data to a centralized
//! system that provides advanced analysis capabilities.
//!
//! In the old dark days, you would have had to pick a vendor and use their SDK to export your data
//! to their systems in whatever bespoke format they devised.  
//! Thankfully, the industry has moved on and we now have a vendor-neutral format for exporting
//! telemetry data: [OpenTelemetry](https://opentelemetry.io/).  
//! It has gained a significant foothold and it's now supported by most observability vendors as
//! a valid (if not preferred!) format for data ingestion.
//!
//! # Exercise
//!
//! We'll be using the [`tracing-opentelemetry`] crate to export our test telemetry data to
//! [Honeycomb](https://www.honeycomb.io/), a hosted observability platform.  
//! Their interface does a much better job than raw JSON at visualizing the rich data we're
//! collecting thanks to the `tracing` crate.
//!
//! You'll need to sign up for a free account and grab an API key—no credit card is required.
mod subscriber;

pub use subscriber::init_test_subscriber;
use tracing::{instrument, Span};

/// Given a list of order numbers, compute the total price.
#[instrument("process total price", skip_all, fields(outcome))]
pub fn get_total(order_numbers: &[u64]) -> Result<u64, anyhow::Error> {
    let mut total = 0;
    for order_number in order_numbers {
        let order_details = get_order_details(*order_number).map_err(|e| {
            Span::current().record("outcome", "failure");
            e
        })?;
        total += order_details.price;
    }
    Span::current().record("outcome", "success");
    Ok(total)
}

pub struct OrderDetails {
    pub order_number: u64,
    pub price: u64,
}

/// A dummy function to simulate what would normally be a database query.
#[instrument("retrieve order", skip_all, fields(outcome))]
fn get_order_details(order_number: u64) -> Result<OrderDetails, anyhow::Error> {
    if order_number % 4 == 0 {
        Span::current().record("outcome", "failure");
        Err(anyhow::anyhow!("Failed to talk to the database"))
    } else {
        let prices = vec![999, 1089, 1029];
        Span::current().record("outcome", "success");
        Ok(OrderDetails {
            order_number,
            price: prices[order_number as usize % prices.len()],
        })
    }
}
