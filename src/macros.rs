#[macro_export]
macro_rules! define_currencies {
    (
        $(
            $code:ident { symbol: $sym:literal, scale: $scale:expr }
        ),* $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum CurrencyCode {
            $( $code, )*
        }

        impl CurrencyCode {
            pub fn symbol(&self) -> &'static str {
                match self { $( CurrencyCode::$code => $sym, )* }
            }

            pub fn standard_scale(&self) -> u32 {
                match self { $( CurrencyCode::$code => $scale, )* }
            }
            
            pub fn iso_code(&self) -> &'static str {
                match self { $( CurrencyCode::$code => stringify!($code), )* }
            }
        }
    };
}

#[macro_export]
macro_rules! money {
    ($amount:literal, $currency:ident) => {
        $crate::domain::currency::Money::new(
            rust_decimal_macros::dec!($amount), 
            $crate::domain::currency::CurrencyCode::$currency
        )
    };
    ($amount:expr, $currency:ident) => {
        $crate::domain::currency::Money::new(
            $amount,
            $crate::domain::currency::CurrencyCode::$currency
        )
    };
}

#[macro_export]
macro_rules! fx_pair {
    ($base:ident / $quote:ident @ $rate:literal) => {
        $crate::domain::forex::FxPair::new(
            $crate::domain::currency::CurrencyCode::$base,
            $crate::domain::currency::CurrencyCode::$quote,
            rust_decimal_macros::dec!($rate)
        )
    };
}
