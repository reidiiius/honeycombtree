use crate::datum::{records, tunings, QTY};
use crate::utile::{
    entirety, figures, horolog, lattice, options, pitcher, qualify, spandex, stylist, veranda,
};

#[test]
fn constant_variable_qty() {
    assert!(QTY == 125, "constant QTY value incorrect");
}

#[test]
fn tunings_return_length() {
    let ouds: Vec<String> = tunings();

    assert!(ouds.len() == 7, "tunings length incorrect");
}

#[test]
fn records_return_length() {
    let arts: [(&str, &str); QTY] = records();

    assert_eq!(arts.len(), QTY);
}

#[test]
fn records_value_lengths() {
    let arts: [(&str, &str); QTY] = records();

    for pair in arts {
        assert_eq!(pair.1.len(), 36);
    }
}

#[test]
fn pitcher_return_value() {
    let (ouds, keys): (Vec<String>, Vec<String>) = options();
    let viol = String::from(&ouds[2]);
    let inks = [keys[0].clone(), viol.clone(), keys[1].clone()];
    let tune = pitcher(&inks);

    assert_eq!(viol, tune);
}

#[test]
fn horolog_return_value() {
    let past: u64 = 1721093758;
    let aeon: u64 = horolog();

    assert!(past < aeon, "horolog assertion failed");
}

#[test]
fn qualify_return_values() {
    let tune = String::from("cgdae");
    let cogs: (String, Vec<usize>) = qualify(tune);
    let (stem, pegs) = cogs;

    assert!(!stem.is_empty(), "qualify stem is empty");
    assert!(!pegs.is_empty(), "qualify pegs is empty");
}

#[test]
fn figures_return_values() {
    let tune = String::from("cgdae");
    let pegs: Vec<usize> = figures(&tune);

    assert_eq!([12, 27, 6, 21, 0].to_vec(), pegs);
}

#[test]
fn entirety_return_type() {
    let tune = String::from("beadgcf");
    let kind: () = entirety(tune);

    assert_eq!((), kind);
}

#[test]
fn veranda_return_type() {
    let (ouds, keys): (Vec<String>, Vec<String>) = options();
    let viol = String::from(&ouds[2]);
    let tune = String::from(&viol);
    let inks = vec![keys[0].clone(), viol.clone(), keys[1].clone()];
    let kind: () = veranda(inks, tune);

    assert_eq!((), kind);
}

#[test]
fn spandex_return_type() {
    let clef = String::from("n0");
    let tune = String::from("beadgcf");
    let cogs: (String, Vec<usize>) = qualify(tune);
    let arts: [(&str, &str); QTY] = records();
    let kind: () = spandex(&clef, &arts, &cogs);

    assert_eq!((), kind);
}

#[test]
fn lattice_return_type() {
    let tune = String::from("beadgcf");
    let cogs: (String, Vec<usize>) = qualify(tune);
    let (stem, pegs) = cogs;
    let arts: [(&str, &str); QTY] = records();
    let pair: (&str, &str) = arts[QTY - 1];
    let kind: () = lattice(pair, stem, pegs);

    assert_eq!((), kind);
}

#[test]
fn options_returns_tuple() {
    let (ouds, keys): (Vec<String>, Vec<String>) = options();

    assert!(ouds.len() == 7, "options ouds length");
    assert!(keys.len() == QTY, "options keys length");
}

#[test]
fn stylist_return_type() {
    let kind: () = stylist();

    assert_eq!((), kind);
}
