use crate::datum::{
    adaptor, caboose, codices, dynamos, horolog, machine, melodia, nodules, proctor, qualify,
    records, signats, tunings, QTY,
};
use crate::utile::{
    enclave, entirety, groupie, lattice, octopus, polaris, refined, spandex, stylist, veranda,
    waxwork,
};

#[test]
fn datum_const_qty_value_size() {
    let duos: usize = records().len();

    assert!(QTY == duos, "constant QTY value incorrect");
}

#[test]
fn datum_codices_return_size() {
    let (dyns, tuns, keys): (Vec<String>, Vec<String>, Vec<String>) = codices();

    assert!(dyns.len() == 6, "codices dyns size");
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
    let inks = vec!["cgdae".to_string(), "tonal".to_string()];
    let rout: String = proctor(&inks);

    assert_eq!(rout, "tonal");
}

#[test]
fn datum_adaptor_return_value() {
    let (dyns, tuns, keys): (Vec<String>, Vec<String>, Vec<String>) = codices();
    let proc = String::from(&dyns[2]);
    let viol = String::from(&tuns[2]);
    let inks = [keys[0].clone(), viol.clone(), keys[1].clone(), proc];
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

    assert!(flag, "caboose returns boolean");
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
fn utile_waxwork_return_type() {
    let mut hits: Vec<String> = vec![];

    for clef in ["n0pz", "n0", "n0yy", "n0zp"] {
        hits.push(clef.to_string());
    }

    assert!(hits.len() > 0, "vector is empty");
    assert_eq!((), waxwork(&hits));
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
    let (dyns, tuns, keys): (Vec<String>, Vec<String>, Vec<String>) = codices();
    let proc = String::from(&dyns[2]);
    let viol = String::from(&tuns[2]);
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
    let kind: () = spandex(&clef, &cogs, &arts);

    assert_eq!((), kind);
}

#[test]
fn utile_lattice_return_type() {
    let tune = String::from("beadgcf");
    let cogs: (String, Vec<usize>) = qualify(tune);
    let arts: [(&str, &str); QTY] = records();
    let pair: (&str, &str) = arts[QTY - 1];
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
fn utile_refined_return_type() {
    let kind: () = refined();

    assert_eq!((), kind);
}

#[test]
fn utile_stylist_return_type() {
    let kind: () = stylist();

    assert_eq!((), kind);
}
