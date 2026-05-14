use enterprise_currency::{fx_pair, money};
use enterprise_currency::domain::currency::EnterpriseCurrency;

fn main() {
    println!("=== CORE BANKING SYSTEM ENGINE ===\n");
    
    // Skenario 1: Saldo Rekening Lokal (contoh: Bank Umum / Branch Utama)
    let saldo_idr = money!(125500000.00, IDR);
    println!("[INFO] Pengecekan Saldo Nasabah");
    println!("Mutasi Lokal : {}", saldo_idr.format_local());
    println!("SWIFT Intl   : {}", saldo_idr.format_intl());
    println!("Cetak Bilyet : {}\n", saldo_idr.spell_out_id());
    
    // Skenario 2: Transaksi QRIS / Virtual Account
    let tagihan_qris = money!(45000.50, IDR);
    let sisa_saldo = money!(saldo_idr.amount() - tagihan_qris.amount(), IDR);
    println!("[INFO] Pembayaran Merchant QRIS");
    println!("Tagihan      : {}", tagihan_qris.format_local());
    println!("Sisa Saldo   : {}\n", sisa_saldo.format_local());

    // Skenario 3: Remitansi / Valuta Asing (Forex)
    let invoice_usd = money!(2450.75, USD);
    let kurs_jual_usd_idr = fx_pair!(USD / IDR @ 16255.50);
    
    println!("[INFO] Settlement Valuta Asing");
    println!("Invoice USD  : {}", invoice_usd.format_intl());
    println!("Kurs Jual    : {}", kurs_jual_usd_idr.rate);
    
    match kurs_jual_usd_idr.exchange(&invoice_usd) {
        Ok(debit_idr) => {
            println!("Debit IDR    : {}", debit_idr.format_local());
            println!("Validasi Legal : {}", debit_idr.spell_out_id());
        },
        Err(e) => println!("[-] Transaksi Ditolak: {}", e),
    }
}
