# 🚀 azmr_enterprise_currency
**The Definitive High-Precision Financial Engine for Modern Rust Applications.**

[![Rust](https://img.shields.io/badge/rust-v1.65+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

`azmr_enterprise_currency` is a production-ready, enterprise-grade financial library built specifically for the Indonesian financial landscape. It solves critical localization challenges—such as decimal/thousand separator standards and Indonesian banking bilyet compliance—for core banking systems, payment gateways, and fintech platforms. Built on **high-precision decimal logic** and **idiomatic Rust performance**.

---

## ✨ Key Features

- 💎 **High-Precision Decimals**: Powered by `rust_decimal`, ensuring zero floating-point rounding errors for mission-critical transactions.
- 🏗️ **Clean Code Architecture**: Strictly follows domain-driven design principles with a clear separation of concerns.
- 🇮🇩 **Core Banking Compliance**: Specifically designed for Indonesian banking standards, handling complex localization (Rp 1.000,00 vs $ 1,000.00).
- ✍️ **Indonesian "Terbilang" Engine**: A robust, range-pattern based spell-out engine for bilyet/check requirements.
- 🌍 **Multi-Currency & Forex**: Built-in support for global currencies and dynamic Foreign Exchange (FX) pair settlements.
- ⚡ **Zero-Boilerplate Macros**: Intuitive `money!` and `fx_pair!` macros for elegant financial data modeling.

---

## 🛠️ Tech Stack

- **Language**: Rust (Edition 2021)
- **Core Library**: `rust_decimal`
- **Architecture**: Domain-Driven Design (DDD) / Clean Architecture

---

## 🚀 Quick Start

### 1. Installation
Add the following to your `Cargo.toml`:
```toml
[dependencies]
rust_decimal = "1.32"
rust_decimal_macros = "1.32"
```

### 2. Core Usage
```rust
use enterprise_currency::{money, fx_pair};
use enterprise_currency::domain::currency::EnterpriseCurrency;

fn main() {
    // Initialize high-precision currency objects
    let balance = money!(12500000.00, IDR);
    
    // Professional formatting
    println!("Local: {}", balance.format_local()); // Output: Rp 12.500.000,00
    println!("Intl:  {}", balance.format_intl());  // Output: Rp 12,500,000.00
    
    // Professional Spell-out (IDR Terbilang)
    println!("Spell: {}", balance.spell_out_id()); // Output: Dua Belas Juta Lima Ratus Ribu Rupiah
}
```

### 3. Forex Transactions
```rust
let invoice = money!(1000.00, USD);
let exchange_rate = fx_pair!(USD / IDR @ 16250.00);

if let Ok(debit_idr) = exchange_rate.exchange(&invoice) {
    println!("Converted: {}", debit_idr.format_local());
}
```

---

## 📂 Project Structure

```text
src/
├── macros.rs          # Elegant financial declarative macros
├── domain/            # Core business logic
│   ├── currency.rs    # Money struct & EnterpriseCurrency trait
│   └── forex.rs       # FX Pair engine
└── utils/             # High-performance utilities
    └── formatter.rs   # Range-pattern based "Terbilang" engine
```

---

## 📜 License
Distributed under the **MIT** and **Apache-2.0** Licenses. See `LICENSE-MIT` and `LICENSE-APACHE` for more information.

---

## 🤝 Contributing
Contributions are welcome! Feel free to open an issue or submit a pull request to enhance the financial ecosystem of Rust.

---
**Developed with ❤️ for the Rust Fintech Community.**
