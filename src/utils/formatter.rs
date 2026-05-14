pub fn terbilang_rupiah(n: i64) -> String {
    let satuan = ["", "Satu", "Dua", "Tiga", "Empat", "Lima", "Enam", "Tujuh", "Delapan", "Sembilan", "Sepuluh", "Sebelas"];
    
    match n {
        0 => "Nol".to_string(),
        1..=11 => satuan[n as usize].to_string(),
        12..=19 => format!("{} Belas", satuan[(n - 10) as usize]),
        20..=99 => {
            let t = n / 10; 
            let u = n % 10;
            let us = if u > 0 { format!(" {}", satuan[u as usize]) } else { "".to_string() };
            format!("{} Puluh{}", satuan[t as usize], us)
        },
        100..=199 => {
            let rem = n - 100;
            if rem > 0 { format!("Seratus {}", terbilang_rupiah(rem)) } else { "Seratus".to_string() }
        }
        200..=999 => {
            let rem = n % 100;
            if rem > 0 { format!("{} Ratus {}", satuan[(n / 100) as usize], terbilang_rupiah(rem)) } else { format!("{} Ratus", satuan[(n / 100) as usize]) }
        }
        1_000..=1_999 => {
            let rem = n - 1_000;
            if rem > 0 { format!("Seribu {}", terbilang_rupiah(rem)) } else { "Seribu".to_string() }
        }
        2_000..=999_999 => {
            let rem = n % 1_000;
            if rem > 0 { format!("{} Ribu {}", terbilang_rupiah(n / 1_000), terbilang_rupiah(rem)) } else { format!("{} Ribu", terbilang_rupiah(n / 1_000)) }
        }
        1_000_000..=999_999_999 => {
            let rem = n % 1_000_000;
            if rem > 0 { format!("{} Juta {}", terbilang_rupiah(n / 1_000_000), terbilang_rupiah(rem)) } else { format!("{} Juta", terbilang_rupiah(n / 1_000_000)) }
        }
        _ => {
            let rem = n % 1_000_000_000;
            if rem > 0 { format!("{} Miliar {}", terbilang_rupiah(n / 1_000_000_000), terbilang_rupiah(rem)) } else { format!("{} Miliar", terbilang_rupiah(n / 1_000_000_000)) }
        }
    }
}
