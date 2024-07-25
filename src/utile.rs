use crate::datum::{records, signats, tunings, QTY};
use std::time::{SystemTime, UNIX_EPOCH};

/// Searches argument list for tuning String
pub fn pitcher(inks: &[String]) -> String {
    let ouds: Vec<String> = tunings();
    // default tuning predefined
    let mut tune = String::from(&ouds[4]);

    for spec in ouds {
        if inks.contains(&spec) {
            tune = spec;
            break;
        }
    }

    tune
}

/// Returns unix timestamp
pub fn horolog() -> u64 {
    let date: SystemTime = SystemTime::now();

    match date.duration_since(UNIX_EPOCH) {
        Ok(span) => span.as_secs(),
        Err(_) => 0,
    }
}

/// Establishes tuning-dateline String and indices Vector
pub fn qualify(tune: String) -> (String, Vec<usize>) {
    let aeon: u64 = horolog();
    let stem: String = format!("-{}-h{}", tune, aeon);
    let pegs: Vec<usize> = figures(Some(&tune));

    (stem, pegs)
}

/// Matches tuning String and returns a Vector of indices
pub fn figures(tune: Option<&str>) -> Vec<usize> {
    match tune {
        Some("beadgcf") => vec![30, 15, 0, 21, 6, 27, 12, 33, 18],
        Some("bfbfb") => vec![33, 15, 33, 15, 33],
        Some("cgdae") => vec![12, 27, 6, 21, 0],
        Some("dgdgbd") => vec![6, 33, 21, 6, 21, 6],
        Some("eadgbe") => vec![12, 33, 21, 6, 27, 12],
        Some("fkbjdn") => vec![6, 30, 18, 6, 30, 18],
        Some("piano") => vec![0],
        Some(&_) => vec![0],
        None => vec![0],
    }
}

/// Prints all Tuples from `records` passing each to `lattice`
pub fn entirety(tune: String) {
    let arts: [(&str, &str); QTY] = records();
    let cogs: (String, Vec<usize>) = qualify(tune);
    let (stem, pegs) = cogs;

    for pair in arts {
        lattice(pair, stem.to_string(), pegs.to_vec());
    }
}

/// Parses user input for key or tuning Strings,
/// passes matched key String to `spandex`
pub fn veranda(inks: Vec<String>, tune: String) {
    let (ouds, keys): (Vec<String>, Vec<String>) = choices();
    let cogs: (String, Vec<usize>) = qualify(tune);
    let arts: [(&str, &str); QTY] = records();

    let mut have = 0;
    for spec in &keys {
        if inks.contains(spec) {
            have += 1;
        }
    }

    for item in &inks {
        // sift through items for signatures or tunings
        if keys.contains(item) {
            spandex(item, &arts, &cogs);
        } else if ouds.contains(item) {
            if have == 0 {
                stylist();
                break;
            } else {
                continue;
            }
        } else {
            println!("\n\t{item} ?");
        }
    }
}

/// Parses user input for key matches in `records`,
/// passes each matched Tuple to `lattice`
pub fn spandex(clef: &str, arts: &[(&str, &str); QTY], cogs: &(String, Vec<usize>)) {
    let span: usize = clef.len();
    let keys: Vec<String> = signats();
    let (stem, pegs) = cogs;

    for (spot, item) in (0_usize..).zip(keys.into_iter()) {
        if span == item.len() && clef.eq(&item) {
            lattice(arts[spot], stem.to_string(), pegs.to_vec());
            break;
        }
    }
}

/// Prints selected Tuple from `records` formatted to screen
pub fn lattice(pair: (&str, &str), stem: String, pegs: Vec<usize>) {
    let (key, val) = pair;
    let span: usize = val.len();

    println!("\n\t{}{}", key, stem);
    for gear in pegs {
        if gear < span {
            println!("\t{}{}", &val[gear..span], &val[0..gear]);
        } else {
            eprintln!(" Index Out of Bounds: {}", gear);
        }
    }
}

/// Returns Tuple holding Vectors of tuning and key Strings
pub fn choices() -> (Vec<String>, Vec<String>) {
    let ouds: Vec<String> = tunings();
    let keys: Vec<String> = signats();

    (ouds, keys)
}

/// Prints tuning Strings and Tuple keys from `records` columned
pub fn stylist() {
    let (ouds, keys): (Vec<String>, Vec<String>) = choices();
    let cols: u8 = 7;

    println!();
    for spec in ouds {
        print!("\t{}", spec)
    }
    println!("\n");
    for (numb, item) in (1_u8..).zip(keys.into_iter()) {
        print!("\t{item}");
        if numb % cols == 0 {
            println!();
        }
    }
    println!();
}
