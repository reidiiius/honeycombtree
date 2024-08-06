use crate::datum::{
    adaptor, caboose, codices, dynamos, horolog, machine, melodia, nodules, proctor, qualify,
    records, signats, tunings, QTY,
};
use crate::utile::{
    catalog, diatoms, enclave, entirety, groupie, lattice, octopus, polaris, spandex, trellis,
    veranda,
};

#[test]
fn datum_const_qty_value_size() {
    let duos: usize = records().len();

    assert!(QTY == duos, "constant QTY value incorrect");
}

#[test]
fn datum_codices_return_size() {
    let (dyns, tuns, keys) = codices();

    assert!(dyns.len() == 7, "codices dyns size");
    assert!(tuns.len() == 7, "codices tuns size");
    assert!(keys.len() == QTY, "codices keys size");
}

#[test]
fn datum_dynamos_encode_ascii() {
    let dyns: Vec<String> = dynamos();

    for proc in dyns {
        assert!(proc.is_ascii());
    }
}

#[test]
fn datum_proctor_return_value() {
    let inks = ["cgdae".to_string(), "tonal".to_string()];
    let rout: String = proctor(&inks);

    assert_eq!(rout, "tonal");
}

#[test]
fn datum_adaptor_return_value() {
    let (dyns, tuns, keys) = codices();
    let proc = String::from(&dyns[2]);
    let viol = String::from(&tuns[2]);
    let clef = String::from(&keys[2]);
    let inks = [viol.clone(), clef, proc];
    let tune = adaptor(&inks);

    assert_eq!(viol, tune);
}

#[test]
fn datum_tunings_encode_ascii() {
    let tuns: Vec<String> = tunings();

    for item in tuns {
        assert!(item.is_ascii());
    }
}

#[test]
fn datum_machine_return_value() {
    let tune = String::from("cgdae");
    let pegs: Vec<usize> = machine(Some(&tune));

    assert_eq!([12, 27, 6, 21, 0].to_vec(), pegs);
}

#[test]
fn datum_qualify_return_value() {
    let tune = String::from("cgdae");
    let cogs: (String, Vec<usize>) = qualify(tune);
    let (mast, pegs) = cogs;

    assert!(!mast.is_empty(), "qualify mast is empty");
    assert!(!pegs.is_empty(), "qualify pegs is empty");
}

#[test]
fn datum_horolog_return_value() {
    let past: u64 = 1721093758;
    let aeon: u64 = horolog();

    assert!(past < aeon, "horolog assertion failed");
}

#[test]
fn datum_caboose_return_bool() {
    let flag = caboose("k6sv");

    assert!(flag, "caboose assertion failed");
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
fn datum_nodules_return_size() {
    let nods: Vec<String> = nodules();
    let span: usize = 84;

    assert!(nods.len() == span, "nodules size incorrect");
}

#[test]
fn datum_nodules_value_size() {
    let nods: Vec<String> = nodules();
    let span: usize = 2;

    for duet in nods {
        assert_eq!(duet.len(), span);
    }
}

#[test]
fn datum_records_encode_ascii() {
    let recs: [(&str, &str); QTY] = records();

    for (key, val) in recs {
        assert!(key.is_ascii() && val.is_ascii());
    }
}

#[test]
fn datum_records_keys_size() {
    let recs: [(&str, &str); QTY] = records();
    let span: usize = 10;

    for pair in recs {
        assert!(!pair.0.is_empty() && pair.0.len() < span);
    }
}

#[test]
fn datum_records_value_size() {
    let recs: [(&str, &str); QTY] = records();
    let span: usize = 36;

    for pair in recs {
        assert_eq!(pair.1.len(), span);
    }
}

#[test]
fn utile_trellis_return_type() {
    let hits = ["n0pz".to_string(), "n0yy".to_string()];

    assert_eq!((), trellis(&hits, "\t"));
}

#[test]
fn utile_octopus_return_type() {
    let kind: () = octopus();

    assert_eq!((), kind);
}

#[test]
fn utile_polaris_return_type() {
    let kind: () = polaris();

    assert_eq!((), kind);
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
    let (dyns, tuns, keys) = codices();
    let proc = String::from(&dyns[0]);
    let tune = String::from(&tuns[0]);
    let clef = String::from(&keys[0]);
    let inks = vec![tune.clone(), clef, proc];
    let kind: () = veranda(inks, tune);

    assert_eq!((), kind);
}

#[test]
fn utile_spandex_return_type() {
    let clef = String::from("n0");
    let tune = String::from("beadgcf");
    let cogs: (String, Vec<usize>) = qualify(tune);
    let recs: [(&str, &str); QTY] = records();
    let kind: () = spandex(&clef, &cogs, &recs);

    assert_eq!((), kind);
}

#[test]
fn utile_lattice_return_type() {
    let tune = String::from("beadgcf");
    let cogs: (String, Vec<usize>) = qualify(tune);
    let recs: [(&str, &str); QTY] = records();
    let pair: (&str, &str) = recs[0];
    let kind: () = lattice(pair, &cogs);

    assert_eq!((), kind);
}

#[test]
fn utile_entirety_return_type() {
    let tune = String::from("beadgcf");
    let kind: () = entirety(tune);

    assert_eq!((), kind);
}

#[test]
fn utile_diatoms_return_type() {
    let kind: () = diatoms();

    assert_eq!((), kind);
}

#[test]
fn utile_catalog_return_type() {
    let kind: () = catalog();

    assert_eq!((), kind);
}
