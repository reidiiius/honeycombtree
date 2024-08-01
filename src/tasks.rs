use crate::datum::{choices, dynamos, flavors, melodia, records, signats, tunings, QTY};
use crate::utile::{
    enclave, entirety, figures, groupie, horolog, lattice, pitcher, qualify, refined, spandex,
    stylist, veranda, waxwork,
};

#[test]
fn datum_const_qty_value_size() {
    let duos: usize = records().len();

    assert!(QTY == duos, "constant QTY value incorrect");
}

#[test]
fn datum_dynamos_encode_ascii() {
    let dyns: Vec<String> = dynamos();

    for proc in dyns {
        assert!(proc.is_ascii());
    }
}

#[test]
fn datum_tunings_encode_ascii() {
    let ouds: Vec<String> = tunings();

    for item in ouds {
        assert!(item.is_ascii());
    }
}

#[test]
fn datum_signats_return_size() {
    let keys: Vec<String> = signats();

    assert!(keys.len() == QTY, "signats size incorrect");
}

#[test]
fn datum_melodia_return_size() {
    let vals: Vec<String> = melodia();

    assert!(vals.len() == QTY, "melodia size incorrect");
}

#[test]
fn datum_flavors_return_size() {
    let urns: Vec<String> = flavors();
    let span: usize = 84;

    assert!(urns.len() == span, "flavors size incorrect");
}

#[test]
fn datum_flavors_value_size() {
    let urns: Vec<String> = flavors();
    let span: usize = 2;

    for duet in urns {
        assert_eq!(duet.len(), span);
    }
}

#[test]
fn datum_records_encode_ascii() {
    let arts: [(&str, &str); QTY] = records();

    for (key, val) in arts {
        assert!(key.is_ascii() && val.is_ascii());
    }
}

#[test]
fn datum_records_value_size() {
    let arts: [(&str, &str); QTY] = records();
    let span: usize = 36;

    for pair in arts {
        assert_eq!(pair.1.len(), span);
    }
}

#[test]
fn utile_pitcher_return_value() {
    let (dyns, ouds, keys): (Vec<String>, Vec<String>, Vec<String>) = choices();
    let proc = String::from(&dyns[2]);
    let viol = String::from(&ouds[2]);
    let inks = [keys[0].clone(), viol.clone(), keys[1].clone(), proc];
    let tune = pitcher(&inks);

    assert_eq!(viol, tune);
}

#[test]
fn utile_horolog_return_value() {
    let past: u64 = 1721093758;
    let aeon: u64 = horolog();

    assert!(past < aeon, "horolog assertion failed");
}

#[test]
fn utile_qualify_return_value() {
    let tune = String::from("cgdae");
    let cogs: (String, Vec<usize>) = qualify(tune);
    let (stem, pegs) = cogs;

    assert!(!stem.is_empty(), "qualify stem is empty");
    assert!(!pegs.is_empty(), "qualify pegs is empty");
}

#[test]
fn utile_figures_return_value() {
    let tune = String::from("cgdae");
    let pegs: Vec<usize> = figures(Some(&tune));

    assert_eq!([12, 27, 6, 21, 0].to_vec(), pegs);
}

#[test]
fn utile_entirety_return_type() {
    let tune = String::from("beadgcf");
    let kind: () = entirety(tune);

    assert_eq!((), kind);
}

#[test]
fn utile_waxwork_return_type() {
    let keys: Vec<String> = signats();
    let mut hits: Vec<String> = vec![];
    let mut last: usize = 0;

    for clef in &keys {
        if clef.contains("n0") {
            hits.push(clef.to_string());
            last += 1;
        }
    }
    assert!(last > 0, "variable incremental?");
    assert!(hits.len() > 0, "vector is empty");
    assert_eq!((), waxwork(&hits, last));
}

#[test]
fn utile_groupie_return_type() {
    let inks = vec!["group".to_string(), "yq".to_string()];
    let kind: () = groupie(inks);

    assert_eq!((), kind);
}

#[test]
fn utile_enclave_return_type() {
    let inks = vec!["query".to_string(), "56".to_string()];
    let kind: () = enclave(inks);

    assert_eq!((), kind);
}

#[test]
fn utile_veranda_return_type() {
    let (dyns, ouds, keys): (Vec<String>, Vec<String>, Vec<String>) = choices();
    let proc = String::from(&dyns[2]);
    let viol = String::from(&ouds[2]);
    let tune = String::from(&viol);
    let inks = vec![keys[0].clone(), viol.clone(), keys[1].clone(), proc];
    let kind: () = veranda(inks, tune);

    assert_eq!((), kind);
}

#[test]
fn utile_spandex_return_type() {
    let clef = String::from("n0");
    let tune = String::from("beadgcf");
    let cogs: (String, Vec<usize>) = qualify(tune);
    let arts: [(&str, &str); QTY] = records();
    let kind: () = spandex(&clef, &arts, &cogs);

    assert_eq!((), kind);
}

#[test]
fn utile_lattice_return_type() {
    let tune = String::from("beadgcf");
    let cogs: (String, Vec<usize>) = qualify(tune);
    let (stem, pegs) = cogs;
    let arts: [(&str, &str); QTY] = records();
    let pair: (&str, &str) = arts[QTY - 1];
    let kind: () = lattice(pair, stem, pegs);

    assert_eq!((), kind);
}

#[test]
fn datum_choices_return_size() {
    let (dyns, ouds, keys): (Vec<String>, Vec<String>, Vec<String>) = choices();

    assert!(dyns.len() == 4, "choices dyns size");
    assert!(ouds.len() == 7, "choices ouds size");
    assert!(keys.len() == QTY, "choices keys size");
}

#[test]
fn utile_refined_return_type() {
    let kind: () = refined();

    assert_eq!((), kind);
}

#[test]
fn utile_stylist_return_type() {
    let kind: () = stylist();

    assert_eq!((), kind);
}
