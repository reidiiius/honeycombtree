use crate::datum::{records, signats, tunings, QTY};
use crate::utile::{
    choices, entirety, figures, horolog, lattice, pitcher, qualify, spandex, stylist, veranda,
};

#[test]
fn datum_constant_variable_qty() {
    let duos: usize = records().len();

    assert!(QTY == duos, "constant QTY value incorrect");
}

#[test]
fn datum_tunings_content_ascii() {
    let ouds: Vec<String> = tunings();

    for item in ouds {
        assert!(item.is_ascii());
    }
}

#[test]
fn datum_signats_return_length() {
    let keys: Vec<String> = signats();

    assert!(keys.len() == QTY, "signats length incorrect");
}

#[test]
fn datum_records_content_ascii() {
    let arts: [(&str, &str); QTY] = records();

    for (key, val) in arts {
        assert!(key.is_ascii() && val.is_ascii());
    }
}

#[test]
fn datum_records_value_lengths() {
    let arts: [(&str, &str); QTY] = records();
    let span: usize = 36;

    for pair in arts {
        assert_eq!(pair.1.len(), span);
    }
}

#[test]
fn utile_pitcher_return_value() {
    let (ouds, keys): (Vec<String>, Vec<String>) = choices();
    let viol = String::from(&ouds[2]);
    let inks = [keys[0].clone(), viol.clone(), keys[1].clone()];
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
fn utile_qualify_return_values() {
    let tune = String::from("cgdae");
    let cogs: (String, Vec<usize>) = qualify(tune);
    let (stem, pegs) = cogs;

    assert!(!stem.is_empty(), "qualify stem is empty");
    assert!(!pegs.is_empty(), "qualify pegs is empty");
}

#[test]
fn utile_figures_return_values() {
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
fn utile_veranda_return_type() {
    let (ouds, keys): (Vec<String>, Vec<String>) = choices();
    let viol = String::from(&ouds[2]);
    let tune = String::from(&viol);
    let inks = vec![keys[0].clone(), viol.clone(), keys[1].clone()];
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
fn utile_choices_returns_tuple() {
    let numb: usize = tunings().len();
    let (ouds, keys): (Vec<String>, Vec<String>) = choices();

    assert!(ouds.len() == numb, "choices ouds length");
    assert!(keys.len() == QTY, "choices keys length");
}

#[test]
fn utile_stylist_return_type() {
    let kind: () = stylist();

    assert_eq!((), kind);
}
