use crate::datum::{choices, dynamos, flavors, records, signats, tunings, QTY};
use std::time::{SystemTime, UNIX_EPOCH};

/// Searches argument list for tuning String
pub fn pitcher(inks: &[String]) -> String {
    let tuns: Vec<String> = tunings();
    // default tuning predefined
    let mut tune = String::from(&tuns[4]);

    for spec in tuns {
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

/// Prints passed collection columned to screen
pub fn waxwork(hits: &[String]) {
    let mut numb: usize = 1;
    let last: usize = hits.len();
    let cols: usize = 7;

    println!();
    while numb <= last {
        print!("\t{}", hits[numb - 1]);
        if numb % cols == 0 && numb != last {
            println!();
        }
        numb += 1;
    }
    println!();
}

/// Prints matched digraph Strings from `records` columned
pub fn groupie(inks: Vec<String>) {
    if inks.len() > 1 {
        let urns: Vec<String> = flavors();
        let mut held: bool = false;

        for argo in &inks {
            if urns.contains(argo) {
                held = true;
                break;
            }
        }

        if held {
            let arts: [(&str, &str); QTY] = records();
            let dyns: Vec<String> = dynamos();
            let tuns: Vec<String> = tunings();
            let mut hits: Vec<String> = vec![];
            let mut last: usize = 0;

            for argo in inks {
                if dyns.contains(&argo) || tuns.contains(&argo) {
                    continue;
                }

                for (clef, raga) in arts {
                    if raga.contains(&argo) {
                        hits.push(clef.to_string());
                        last += 1;
                    }
                }

                if last > 0 {
                    waxwork(&hits);
                } else {
                    println!("\n\t{} ?", argo);
                }

                hits.clear();
                last = 0;
            }
        } else {
            refined();
        }
    } else {
        refined();
    }
}

/// Prints matched key Strings from `records` columned
pub fn enclave(inks: Vec<String>) {
    if inks.len() > 1 {
        let (dyns, tuns, keys) = choices();
        let mut held: bool = false;

        for argo in &inks {
            if !dyns.contains(argo) && !tuns.contains(argo) {
                held = true;
                break;
            }
        }

        if held {
            let mut hits: Vec<String> = vec![];
            let mut last: usize = 0;

            for argo in inks {
                if dyns.contains(&argo) || tuns.contains(&argo) {
                    continue;
                }

                for clef in &keys {
                    if clef.contains(&argo) {
                        hits.push(clef.to_string());
                        last += 1;
                    }
                }

                if last > 0 {
                    waxwork(&hits);
                } else {
                    println!("\n\t{} ?", argo);
                }

                hits.clear();
                last = 0;
            }
        } else {
            stylist();
        }
    } else {
        stylist();
    }
}

/// Parses input for key or tuning Strings,
/// passes matched key String to `spandex`
pub fn veranda(inks: Vec<String>, tune: String) {
    let (dyns, tuns, keys) = choices();
    let cogs: (String, Vec<usize>) = qualify(tune);
    let arts: [(&str, &str); QTY] = records();

    let mut have = 0;
    for spec in &keys {
        if inks.contains(spec) {
            have += 1;
        }
    }

    for argo in &inks {
        // sift through items for signatures or tunings
        if keys.contains(argo) {
            spandex(argo, &arts, &cogs);
        } else if tuns.contains(argo) || dyns.contains(argo) {
            if have == 0 {
                stylist();
                break;
            } else {
                continue;
            }
        } else {
            println!("\n\t{} ?", argo);
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

/// Prints sorted digraph Strings from `records` columned
pub fn refined() {
    let urns: Vec<String> = flavors();
    let mut numb: usize = 1;
    let cols: usize = 7;

    println!();
    while numb <= urns.len() {
        print!("  {}", urns[numb - 1]);
        if numb % cols == 0 {
            println!();
        }
        numb += 1;
    }
}

/// Prints routines, tunings, and Tuple keys from `records` columned
pub fn stylist() {
    let (dyns, tuns, keys) = choices();
    let last: usize = keys.len();
    let cols: usize = 7;

    println!();
    for spec in dyns {
        print!("\t{}", spec)
    }
    println!("\n");
    for spec in tuns {
        print!("\t{}", spec)
    }
    println!("\n");
    for (numb, item) in (1_usize..).zip(keys.into_iter()) {
        print!("\t{item}");
        if numb == last {
            continue;
        }
        if numb % cols == 0 {
            println!();
        }
    }
    println!();
}
