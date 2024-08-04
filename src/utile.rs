use crate::datum::{caboose, codices, dynamos, nodules, qualify, records, signats, tunings, QTY};

/// Prints passed collection columned to screen
pub fn waxwork(hits: &[String]) {
    let last: usize = hits.len();
    let mut numb: usize = 1;
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

/// Prints matched octad key Strings from `records` columned
pub fn octopus() {
    let keys: Vec<String> = signats();
    let mut orcs: Vec<String> = vec![];

    for sign in keys {
        if caboose(sign.as_str()) {
            orcs.push(sign);
        }
    }

    if orcs.is_empty() {
        eprintln!("\n\toctopus: orcs vacancy");
    } else {
        waxwork(&orcs);
    }
}

/// Prints polarized key Strings from `records` columned
pub fn polaris() {
    let recs: [(&str, &str); QTY] = records();
    let mut prots: Vec<String> = vec![];
    let mut neuts: Vec<String> = vec![];
    let mut elecs: Vec<String> = vec![];
    let mut incs;

    for (clef, raga) in recs {
        incs = raga.char_indices();

        if clef.contains("n0") {
            neuts.push(clef.to_string());
            continue;
        } else {
            for (numb, atom) in incs {
                if numb == 0 && atom.eq(&'v') {
                    prots.push(clef.to_string());
                    break;
                } else if numb == 1 && atom.eq(&'r') {
                    elecs.push(clef.to_string());
                    break;
                } else if numb < 15 {
                    continue;
                } else if numb == 15 && atom.eq(&'w') {
                    prots.push(clef.to_string());
                    break;
                } else if numb == 16 && atom.eq(&'q') {
                    elecs.push(clef.to_string());
                    break;
                } else if numb < 33 {
                    continue;
                } else if numb == 33 && atom.eq(&'q') {
                    prots.push(clef.to_string());
                    break;
                } else if numb == 34 && atom.eq(&'w') {
                    elecs.push(clef.to_string());
                    break;
                } else {
                    break;
                }
            }
        }
    }
    let total: usize = prots.len() + neuts.len() + elecs.len();

    if total == QTY {
        for parts in [prots, neuts, elecs] {
            waxwork(&parts);
        }
    } else {
        eprintln!("\npolaris parts: {}, unequal to records: {}", total, QTY);
    }
}

/// Prints matched digraph Strings from `records` columned
pub fn groupie(inks: Vec<String>) {
    if inks.len() > 1 {
        let nods: Vec<String> = nodules();
        let mut held: bool = false;

        for argo in &inks {
            if nods.contains(argo) {
                held = true;
                break;
            }
        }

        if held {
            let arts: [(&str, &str); QTY] = records();
            let dyns: Vec<String> = dynamos();
            let tuns: Vec<String> = tunings();
            let mut hits: Vec<String> = vec![];

            for argo in inks {
                if dyns.contains(&argo) || tuns.contains(&argo) {
                    continue;
                }

                for (clef, raga) in arts {
                    if raga.contains(&argo) {
                        hits.push(clef.to_string());
                    }
                }

                if hits.is_empty() {
                    println!("\n\t{} ?", argo);
                } else {
                    waxwork(&hits);
                }

                hits.clear();
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
        let (dyns, tuns, keys) = codices();
        let mut held: bool = false;

        for argo in &inks {
            if !dyns.contains(argo) && !tuns.contains(argo) {
                held = true;
                break;
            }
        }

        if held {
            let mut hits: Vec<String> = vec![];

            for argo in inks {
                if dyns.contains(&argo) || tuns.contains(&argo) {
                    continue;
                }

                for clef in &keys {
                    if clef.contains(&argo) {
                        hits.push(clef.to_string());
                    }
                }

                if hits.is_empty() {
                    println!("\n\t{} ?", argo);
                } else {
                    waxwork(&hits);
                }

                hits.clear();
            }
        } else {
            stylist();
        }
    } else {
        stylist();
    }
}

/// Parses input for `codices` Strings, passes matched key String to `spandex`
pub fn veranda(inks: Vec<String>, tune: String) {
    let (dyns, tuns, keys) = codices();
    let cogs: (String, Vec<usize>) = qualify(tune);
    let arts: [(&str, &str); QTY] = records();
    let mut have: bool = false;

    // sift arguments for absence or presence of keys
    for argo in &inks {
        if keys.contains(argo) {
            have = true;
            break;
        }
    }

    for argo in &inks {
        if keys.contains(argo) {
            spandex(argo, &cogs, &arts);
        } else if tuns.contains(argo) || dyns.contains(argo) {
            if have {
                continue;
            } else {
                stylist();
                break;
            }
        } else {
            println!("\n\t{} ?", argo);
        }
    }
}

/// Parses input for key matches in `records`, passes matched Tuple to `lattice`
pub fn spandex(argo: &str, cogs: &(String, Vec<usize>), arts: &[(&str, &str); QTY]) {
    let span: usize = argo.len();

    for (spot, pair) in (0_usize..).zip(arts.iter()) {
        if span == pair.0.len() && argo.eq(pair.0) {
            lattice(arts[spot], cogs);
            break;
        }
    }
}

/// Prints selected Tuple from `records` formatted to screen
pub fn lattice(pair: (&str, &str), cogs: &(String, Vec<usize>)) {
    let (clef, raga) = pair;
    let (mast, pegs) = cogs;
    let span: usize = raga.len();

    println!("\n\t{}{}", clef, mast);
    for gear in pegs {
        if gear < &span {
            println!("\t{}{}", &raga[*gear..span], &raga[..*gear]);
        } else {
            eprintln!(" Index Out of Bounds: {}", gear);
        }
    }
}

/// Prints all Tuples from `records` passing each to `lattice`
pub fn entirety(tune: String) {
    let arts: [(&str, &str); QTY] = records();
    let cogs: (String, Vec<usize>) = qualify(tune);

    for pair in arts {
        lattice(pair, &cogs);
    }
}

/// Prints sorted digraph Strings from `records` columned
pub fn refined() {
    let nods: Vec<String> = nodules();
    let mut numb: usize = 1;
    let cols: usize = 7;

    println!();
    while numb <= nods.len() {
        print!("  {}", nods[numb - 1]);
        if numb % cols == 0 {
            println!();
        }
        numb += 1;
    }
}

/// Prints routines, tunings, and Tuple keys from `records` columned
pub fn stylist() {
    let (dyns, tuns, keys) = codices();
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
