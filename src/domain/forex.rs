use rust_decimal::Decimal;
use super::currency::{CurrencyCode, Money, EnterpriseCurrency};

#[derive(Debug, Clone)]
pub struct FxPair {
    pub base: CurrencyCode,
    pub quote: CurrencyCode,
    pub rate: Decimal,
}

impl FxPair {
    pub fn new(base: CurrencyCode, quote: CurrencyCode, rate: Decimal) -> Self {
        Self { base, quote, rate }
    }
    
    pub fn exchange(&self, amount: &Money) -> Result<Money, &'static str> {
        if amount.currency() != self.base {
            return Err("Mata uang base tidak valid untuk pair ini");
        }
        Ok(Money::new(amount.amount() * self.rate, self.quote))
    }
}
