use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use crate::utils::formatter::terbilang_rupiah;

crate::define_currencies! {
    USD { symbol: "$", scale: 2 },
    EUR { symbol: "€", scale: 2 },
    GBP { symbol: "£", scale: 2 },
    JPY { symbol: "¥", scale: 0 },
    CHF { symbol: "Fr", scale: 2 },
    AUD { symbol: "A$", scale: 2 },
    SGD { symbol: "S$", scale: 2 },
    IDR { symbol: "Rp", scale: 2 },
}

pub trait EnterpriseCurrency {
    fn amount(&self) -> Decimal;
    fn currency(&self) -> CurrencyCode;
    fn format_intl(&self) -> String;
    fn format_local(&self) -> String;
    fn spell_out_id(&self) -> String;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Money {
    amount: Decimal,
    currency: CurrencyCode,
}

impl Money {
    pub fn new(amount: Decimal, currency: CurrencyCode) -> Self {
        let mut scaled = amount;
        scaled.rescale(currency.standard_scale());
        Self { amount: scaled, currency }
    }
}

impl EnterpriseCurrency for Money {
    fn amount(&self) -> Decimal { self.amount }
    fn currency(&self) -> CurrencyCode { self.currency }

    fn format_intl(&self) -> String {
        let s = self.amount.to_string();
        let parts: Vec<&str> = s.split('.').collect();
        let main_val = parts[0].as_bytes().rchunks(3).rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<_>, _>>().unwrap().join(",");
        
        if parts.len() > 1 {
            format!("{} {}.{}", self.currency.symbol(), main_val, parts[1])
        } else {
            format!("{} {}", self.currency.symbol(), main_val)
        }
    }

    fn format_local(&self) -> String {
        let s = self.amount.to_string();
        let parts: Vec<&str> = s.split('.').collect();
        let main_val = parts[0].as_bytes().rchunks(3).rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<_>, _>>().unwrap().join(".");
        
        if parts.len() > 1 {
            format!("{} {},{}", self.currency.symbol(), main_val, parts[1])
        } else {
            format!("{} {},00", self.currency.symbol(), main_val)
        }
    }

    fn spell_out_id(&self) -> String {
        let int_val = self.amount.trunc().to_i64().unwrap_or(0);
        let terbilang = terbilang_rupiah(int_val);
        let cur_name = match self.currency {
            CurrencyCode::USD => "Dolar Amerika",
            CurrencyCode::EUR => "Euro",
            CurrencyCode::GBP => "Poundsterling",
            CurrencyCode::JPY => "Yen",
            CurrencyCode::IDR => "Rupiah",
            _ => "Unit Mata Uang",
        };
        format!("{} {}", terbilang, cur_name)
    }
}
