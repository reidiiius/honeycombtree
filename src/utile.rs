use crate::datum::{caboose, codices, devices, nodules, qualify, records, signats, tunings, QTY};

/// Prints passed shared slice of Strings columned to screen
pub fn trellis(buds: &[String], pads: &str) {
    let span: usize = buds.len();
    let cols: usize = 8;

    println!();
    for (numb, stem) in (1_usize..).zip(buds.iter()) {
        print!("{}{}", pads, stem);
        if numb == span {
            continue;
        }
        if numb % cols == 0 {
            println!();
        }
    }
    println!();
}

/// Prints matched octad key Strings from `records` columned
pub fn octopus() {
    let keys: &[String] = &signats();
    let mut orcs: Vec<String> = Vec::with_capacity(64);

    for sign in keys {
        if caboose(sign.as_str()) {
            orcs.push(sign.to_string());
        }
    }

    if orcs.is_empty() {
        eprintln!("\n\toctopus: orcs vacancy");
    } else {
        trellis(&orcs, "\t");
    }
}

/// Prints polarized key Strings from `records` columned
pub fn polaris() {
    let recs: [(&str, &str); QTY] = records();
    let mut prots: Vec<String> = Vec::with_capacity(128);
    let mut neuts: Vec<String> = Vec::with_capacity(8);
    let mut elecs: Vec<String> = Vec::with_capacity(128);
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
            trellis(&parts, "\t");
        }
    } else {
        eprintln!("\npolaris parts: {}, unequal to records: {}", total, QTY);
    }
}

/// Prints matched digraph Strings from `records` columned
pub fn groupie(inks: &[String]) {
    if inks.len() > 1 {
        let nods: &[String] = &nodules();
        let mut held: bool = false;

        for argo in inks {
            if nods.contains(argo) {
                held = true;
                break;
            }
        }

        if held {
            let recs: [(&str, &str); QTY] = records();
            let devs: &[String] = &devices();
            let tuns: &[String] = &tunings();
            let mut hits: Vec<String> = Vec::with_capacity(64);

            for argo in inks {
                if devs.contains(argo) || tuns.contains(argo) {
                    continue;
                }

                for (clef, raga) in recs {
                    if raga.contains(argo) {
                        hits.push(clef.to_string());
                    }
                }

                if hits.is_empty() {
                    println!("\n\t{} ?", argo);
                } else {
                    trellis(&hits, "\t");
                }

                hits.clear();
            }
        } else {
            diatoms();
        }
    } else {
        diatoms();
    }
}

/// Prints matched key Strings from `records` columned
pub fn enclave(inks: &[String]) {
    if inks.len() > 1 {
        let (devs, tuns, keys) = codices();
        let mut held: bool = false;

        for argo in inks {
            if !devs.contains(argo) && !tuns.contains(argo) {
                held = true;
                break;
            }
        }

        if held {
            let mut hits: Vec<String> = Vec::with_capacity(128);

            for argo in inks {
                if devs.contains(argo) || tuns.contains(argo) {
                    continue;
                }

                for clef in &keys {
                    if clef.contains(argo) {
                        hits.push(clef.to_string());
                    }
                }

                if hits.is_empty() {
                    println!("\n\t{} ?", argo);
                } else {
                    trellis(&hits, "\t");
                }

                hits.clear();
            }
        } else {
            catalog();
        }
    } else {
        catalog();
    }
}

/// Parses input for `codices` Strings, passes matched key String to `spandex`
pub fn veranda(inks: &[String], tune: &String) {
    let (devs, tuns, keys) = codices();
    let cogs: (String, Vec<usize>) = qualify(tune);
    let recs: [(&str, &str); QTY] = records();
    let mut have: bool = false;

    // sift arguments for absence or presence of keys
    for argo in inks {
        if keys.contains(argo) {
            have = true;
            break;
        }
    }

    for argo in inks {
        if keys.contains(argo) {
            spandex(argo, &cogs, &recs);
        } else if tuns.contains(argo) || devs.contains(argo) {
            if have {
                continue;
            } else {
                catalog();
                break;
            }
        } else {
            println!("\n\t{} ?", argo);
        }
    }
}

/// Parses input for key matches in `records`, passes matched Tuple to `lattice`
pub fn spandex(argo: &str, cogs: &(String, Vec<usize>), recs: &[(&str, &str); QTY]) {
    let span: usize = argo.len();

    for (spot, pair) in (0_usize..).zip(recs.iter()) {
        if span == pair.0.len() && argo.eq(pair.0) {
            lattice(recs[spot], cogs);
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
pub fn entirety(tune: &String) {
    let recs: [(&str, &str); QTY] = records();
    let cogs: (String, Vec<usize>) = qualify(tune);

    for pair in recs {
        lattice(pair, &cogs);
    }
}

/// Prints sorted digraph Strings from `records` columned
pub fn diatoms() {
    let nods: &[String] = &nodules();

    trellis(nods, "\x20\x20");
}

/// Prints devices, tunings, and Tuple keys from `records` columned
pub fn catalog() {
    let (devs, tuns, keys) = codices();

    trellis(&devs, "\t");
    trellis(&tuns, "\t");
    trellis(&keys, "\t");
}
