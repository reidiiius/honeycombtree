use crate::datum::{records, tunings, QTY};
use crate::utile::{
    entirety, figures, horolog, lattice, pitcher, qualify, shrouds, spandex, stylist,
};

#[test]
fn constant_variable_qty() {
    assert!(QTY == 125, "constant QTY value incorrect");
}

#[test]
fn tunings_return_length() {
    let axes: [&str; 7] = tunings();

    assert!(axes.len() == 7, "tunings length incorrect");
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
    let viol = String::from("cgdae");
    let inks = ["n0".to_string(), "j3".to_string(), viol.clone()];
    let axes = ["beadgcf", "bfbfb", "cgdae", "eadgbe", "fkbjdn"];
    let tune = pitcher(&axes, &inks);

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
    let (stem, pegs) = qualify(tune);

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
    let urns: (String, Vec<usize>) = qualify(tune);
    let arts: [(&str, &str); QTY] = records();
    let kind: () = entirety(arts, urns);

    assert_eq!((), kind);
}

#[test]
fn spandex_return_type() {
    let clef = String::from("n0");
    let tune = String::from("beadgcf");
    let urns: (String, Vec<usize>) = qualify(tune);
    let arts: [(&str, &str); QTY] = records();
    let kind: () = spandex(&clef, &arts, &urns);

    assert_eq!((), kind);
}

#[test]
fn lattice_return_type() {
    let tune = String::from("beadgcf");
    let (stem, pegs) = qualify(tune);
    let arts: [(&str, &str); QTY] = records();
    let pair: (&str, &str) = arts[QTY - 1];
    let kind: () = lattice(pair, stem, pegs);

    assert_eq!((), kind);
}

#[test]
fn shrouds_returns_tuple() {
    let (axes, opts) = shrouds();

    assert!(axes.len() == 7, "shrouds axes length");
    assert!(opts.len() == QTY, "shrouds opts length");
}

#[test]
fn stylist_return_type() {
    let kind: () = stylist();

    assert_eq!((), kind);
}
