# Implementation Guide: EnterpriseCurrency Trait

This guide provides step-by-step instructions on how to integrate the `EnterpriseCurrency` architecture into any new Rust project. This system is designed for high-precision core banking, payment gateways, and multi-currency accounting systems.

## 1. Prerequisites

Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
rust_decimal = "1.32"
rust_decimal_macros = "1.32"
```

## 2. Core Architecture Setup

To use the `EnterpriseCurrency` trait, you need to bring over three main components:

### A. The Macros (`src/macros.rs`)
These allow you to define currencies and create money objects with zero boilerplate.
- **`define_currencies!`**: Registers valid ISO codes, symbols, and scales.
- **`money!`**: Safely creates a `Money` object from a literal or expression.
- **`fx_pair!`**: Creates an exchange rate pair.

### B. The Domain Model (`src/domain/currency.rs`)
This contains the trait definition and the `Money` struct.

```rust
pub trait EnterpriseCurrency {
    fn amount(&self) -> Decimal;        // Get raw decimal value
    fn currency(&self) -> CurrencyCode; // Get currency code (USD, IDR, etc)
    fn format_intl(&self) -> String;    // Format: $ 1,000.00
    fn format_local(&self) -> String;   // Format: Rp 1.000,00
    fn spell_out_id(&self) -> String;   // Format: "Satu Ribu Rupiah"
}
```

### C. The Formatter (`src/utils/formatter.rs`)
Essential for Indonesian banking compliance (Bilyet/Check spell-out).

---

## 3. How to Implement in a New Project

### Step 1: Define Your Currencies
In your `src/domain/currency.rs`, use the macro to register the currencies your specific app needs:

```rust
crate::define_currencies! {
    USD { symbol: "$", scale: 2 },
    IDR { symbol: "Rp", scale: 2 },
    // Add more as needed
    MYR { symbol: "RM", scale: 2 },
}
```

### Step 2: Implement for Custom Structs (Optional)
While `Money` is the default implementation, you can implement `EnterpriseCurrency` for your own structs (e.g., a `Transaction` or `Wallet` struct):

```rust
pub struct Wallet {
    pub balance: Decimal,
    pub currency: CurrencyCode,
}

impl EnterpriseCurrency for Wallet {
    fn amount(&self) -> Decimal { self.balance }
    fn currency(&self) -> CurrencyCode { self.currency }
    // ... implement other methods ...
}
```

---

## 4. Usage Patterns

### Creating Money
Always use the `money!` macro to ensure correct decimal scaling:
```rust
let salary = money!(5000.00, USD);
```

### Formatting for UI
```rust
println!("Display to User: {}", salary.format_local());
```

### Currency Exchange (Forex)
Use `FxPair` to handle cross-border transactions:
```rust
let rate = fx_pair!(USD / IDR @ 16000.00);
let converted = rate.exchange(&salary)?;
```

## 5. Best Practices
1. **Never use `f64`**: Always use `rust_decimal` via the `money!` macro to prevent cent-rounding errors.
2. **rescale()**: The `Money::new` constructor automatically rescales input to the currency's standard scale (e.g., IDR usually has 2 decimals, JPY has 0).
3. **Trait Objects**: Use `Box<dyn EnterpriseCurrency>` if you need a list of different financial objects that all need formatting.

---

## 6. Project structure Recommendation
Keep the structure clean to maintain "Enterprise-Grade" standards:
```text
src/
├── macros.rs          # Financial macros
├── domain/            
│   ├── mod.rs
│   ├── currency.rs    # Trait & Money struct
│   └── forex.rs       # Exchange logic
└── utils/             
    ├── mod.rs
    └── formatter.rs   # Spell-out logic
```
