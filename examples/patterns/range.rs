// https://doc.rust-lang.org/reference/patterns.html#range-patterns

fn main() {
    let valid_variable = match c {
        'a'..='z' => true,
        'A'..='Z' => true,
        'α'..='ω' => true,
        _ => false,
    };

    println!(
        "{}",
        match ph {
            0..7 => "acid",
            7 => "neutral",
            8..=14 => "base",
            _ => unreachable!(),
        }
    );

    match uint {
        0 => "zero!",
        1.. => "positive number!",
    };

    // using paths to constants:
    println!(
        "{}",
        match altitude {
            TROPOSPHERE_MIN..=TROPOSPHERE_MAX => "troposphere",
            STRATOSPHERE_MIN..=STRATOSPHERE_MAX => "stratosphere",
            MESOSPHERE_MIN..=MESOSPHERE_MAX => "mesosphere",
            _ => "outer space, maybe",
        }
    );

    if let size @ binary::MEGA..=binary::GIGA = n_items * bytes_per_item {
        println!("It fits and occupies {} bytes", size);
    }

    // using qualified paths:
    println!(
        "{}",
        match 0xfacade {
            0..=<u8 as MaxValue>::MAX => "fits in a u8",
            0..=<u16 as MaxValue>::MAX => "fits in a u16",
            0..=<u32 as MaxValue>::MAX => "fits in a u32",
            _ => "too big",
        }
    );
}
