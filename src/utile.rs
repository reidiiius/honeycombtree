use crate::datum::{records, signats, tunings, QTY};
use std::time::{SystemTime, UNIX_EPOCH};

/// Searches argument list for tuning String
pub fn pitcher(axes: Vec<String>, inks: &[String]) -> String {
    // default tuning predefined
    let mut tune = String::from("eadgbe");

    for mode in axes {
        if inks.contains(&mode) {
            tune = mode;
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
    let pegs: Vec<usize> = figures(&tune);

    (stem, pegs)
}

/// Matches tuning String and returns a Vector of indices
pub fn figures(tune: &str) -> Vec<usize> {
    if tune.eq_ignore_ascii_case("beadgcf") {
        vec![30, 15, 0, 21, 6, 27, 12, 33, 18]
    } else if tune.eq_ignore_ascii_case("bfbfb") {
        vec![33, 15, 33, 15, 33]
    } else if tune.eq_ignore_ascii_case("cgdae") {
        vec![12, 27, 6, 21, 0]
    } else if tune.eq_ignore_ascii_case("dgdgbd") {
        vec![6, 33, 21, 6, 21, 6]
    } else if tune.eq_ignore_ascii_case("eadgbe") {
        vec![12, 33, 21, 6, 27, 12]
    } else if tune.eq_ignore_ascii_case("fkbjdn") {
        vec![6, 30, 18, 6, 30, 18]
    } else {
        vec![0]
    }
}

/// Prints all Tuples from `records` passing each to `lattice`
pub fn entirety(urns: (String, Vec<usize>)) {
    let arts: [(&str, &str); QTY] = records();
    let (stem, pegs) = urns;

    for pair in arts {
        lattice(pair, stem.to_string(), pegs.to_vec());
    }
}

/// Parses user input for key or tuning String matches,
/// passes each matched key String to `lattice`
pub fn veranda(inks: Vec<String>, urns: (String, Vec<usize>)) {
    let arts: [(&str, &str); QTY] = records();
    let (ouds, keys) = options();

    for item in &inks {
        // sift through items for signatures or tunings
        if keys.contains(item) {
            spandex(item, &arts, &urns);
        } else if ouds.contains(item) {
            if inks.len() == 1 {
                stylist();
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
pub fn spandex(clef: &str, arts: &[(&str, &str); QTY], urns: &(String, Vec<usize>)) {
    let span: usize = clef.len();
    let keys: Vec<String> = signats();
    let (stem, pegs) = urns;

    for (spot, item) in (0_usize..).zip(keys.into_iter()) {
        if span == item.len() && clef.eq_ignore_ascii_case(&item) {
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
pub fn options() -> (Vec<String>, Vec<String>) {
    let axes: Vec<String> = tunings();
    let keys: Vec<String> = signats();
    let mut ouds = Vec::new();

    for item in axes {
        ouds.push(item.to_string());
    }

    (ouds, keys)
}

/// Prints tuning Strings and Tuple keys from `records` columned
pub fn stylist() {
    let (ouds, keys) = options();
    let cols: u8 = 7;

    println!();
    for mode in ouds {
        print!("\t{}", mode)
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
