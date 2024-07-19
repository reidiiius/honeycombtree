use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

/// Quantity of Tuples in the Array returned by `supply`
const VOLUME: usize = 125;

/// Entry point of application
fn main() {
    let inks: Vec<String> = env::args().collect();

    // limit amount of arguments
    if inks.len() > VOLUME {
        println!("\n\tRequest denied!\n");
        return;
    }

    let lyre = String::from("beadgcf");
    let figs: (String, Vec<usize>) = figures(lyre);
    let (tune, pegs) = figs;
    let aeon: u64 = horolog();
    let stem: String = format!("-{}-h{}", tune, aeon);
    let arts: [(&str, &str); VOLUME] = supply();

    if inks.len() > 1 {
        if inks[1].eq_ignore_ascii_case("gamut") {
            entirety(&arts, &stem, &pegs);
        } else {
            let wide: usize = 9;

            for clef in inks {
                // limit amount of characters
                if clef.len() > wide {
                    continue;
                }

                spandex(&clef, &arts, &stem, &pegs);
            }
        }
    } else {
        stylist(&arts);
    }
    println!();
}

/// Matches tuning String and returns a Tuple that
/// holds the tuning String and a Vector of indices
fn figures(tune: String) -> (String, Vec<usize>) {
    let pegs: Vec<usize>;

    if tune.eq_ignore_ascii_case("beadgcf") {
        pegs = vec![30, 15, 0, 21, 6, 27, 12, 33, 18];
    } else if tune.eq_ignore_ascii_case("bfbfb") {
        pegs = vec![33, 15, 33, 15, 33];
    } else if tune.eq_ignore_ascii_case("cgdae") {
        pegs = vec![12, 27, 6, 21, 0];
    } else if tune.eq_ignore_ascii_case("eadgbe") {
        pegs = vec![12, 33, 21, 6, 27, 12];
    } else if tune.eq_ignore_ascii_case("fkbjdn") {
        pegs = vec![6, 30, 18, 6, 30, 18];
    } else {
        pegs = vec![0];
    }

    (tune, pegs)
}

/// Returns unix timestamp
fn horolog() -> u64 {
    let date: SystemTime = SystemTime::now();

    match date.duration_since(UNIX_EPOCH) {
        Ok(span) => span.as_secs(),
        Err(_) => 0,
    }
}

/// Prints all records from `supply` by passing each to `lattice`
fn entirety(arts: &[(&str, &str); VOLUME], stem: &str, pegs: &[usize]) {
    for pair in arts {
        println!();
        lattice(*pair, stem.to_owned(), pegs.to_vec());
    }
}

/// Parses user input for key matches in `supply` records,
/// passing each matched record to `lattice`
fn spandex(clef: &str, arts: &[(&str, &str); VOLUME], stem: &str, pegs: &[usize]) {
    let span: usize = clef.len();
    let mut opts = Vec::new();

    for pair in arts {
        opts.push(pair.0)
    }

    for (spot, item) in (0_usize..).zip(opts.into_iter()) {
        if (span == item.len()) && (clef.eq_ignore_ascii_case(item)) {
            println!();
            lattice(arts[spot], stem.to_owned(), pegs.to_vec());
            break;
        }
    }
}

/// Prints all record keys from `supply` formatted to screen
fn stylist(arts: &[(&str, &str); VOLUME]) {
    let mut opts = Vec::new();

    for pair in arts {
        opts.push(pair.0)
    }

    println!();
    for (numb, item) in (1_u8..).zip(opts.into_iter()) {
        print!("\t{item}");
        if numb % 7 == 0 {
            println!();
        }
    }
    println!();
}

/// Prints selected record from `supply` formatted to screen
fn lattice(pair: (&str, &str), stem: String, pegs: Vec<usize>) {
    let (key, val) = pair;
    let span: usize = val.len();

    println!("\t{}{}", key, stem);
    for gear in pegs {
        if gear < span {
            println!("\t{}{}", &val[gear..span], &val[0..gear]);
        } else {
            eprintln!(" Index Out of Bounds: {}", gear);
        }
    }
}

/// Returns an Array of Tuples containing key-value Strings
fn supply() -> [(&'static str, &'static str); VOLUME] {
    [
        ("i0", "__ __ __ __ __ __ __ __ __ __ __ __ "),
        ("j136l7", "__ __ tw xr __ wt __ uv yq so __ qy "),
        ("j167l2", "vu __ __ __ rx wt __ uv yq __ os qy "),
        ("j17l2", "vs __ __ __ rw wr __ ut __ sv oq qo "),
        ("j17", "__ __ tu __ rw wr __ ut __ sv oq qo "),
        ("j17zy", "__ zy tu __ rw wr __ ut __ sv oq qo "),
        ("j236ot", "vv zq __ xs __ wu __ uw yr __ ot qz "),
        ("j236sp", "vv zq __ xs __ wu __ uw yr sp __ qz "),
        ("j236sq", "vy zu __ xw __ wx __ uz yv sq __ qs "),
        ("j236", "vv zq __ xs __ wu __ uw yr __ __ qz "),
        ("j23k6", "vv zq __ xs __ wu __ uw __ __ ot qz "),
        ("j23ot", "vv zq __ xs __ wu __ uw __ sx ot qz "),
        ("j23to", "vv zq to xs __ wu __ uw __ sx __ qz "),
        ("j23", "vv zq __ xs __ wu __ uw __ sx __ qz "),
        ("j246l3", "vv zq to __ ry __ __ uw yr __ __ qz "),
        ("j256", "qr vp __ __ pv rq wo __ uy __ __ ow "),
        ("j25l6", "or qp __ __ tv pq ro ws __ __ __ sw "),
        ("j25", "qr vp __ __ pv rq wo __ __ yu __ ow "),
        ("j26l34", "vv zq to xs __ __ __ uw yr __ __ qz "),
        ("j26l3", "vv zq to __ __ wu __ uw yr __ __ qz "),
        ("j26ot", "vv zq __ __ ry wu __ uw yr __ ot qz "),
        ("j26psp", "vv zq __ ps ry wu __ uw yr sp __ qz "),
        ("j26ps", "vv zq __ ps ry wu __ uw yr __ __ qz "),
        ("j26sp", "vv zq __ __ ry wu __ uw yr sp __ qz "),
        ("j26to", "vv zq to __ ry wu __ uw yr __ __ qz "),
        ("j26", "vv zq __ __ ry wu __ uw yr __ __ qz "),
        ("j2k34", "or qp __ __ __ pq ro ws __ uu __ sw "),
        ("j2k56m4", "vv zq __ __ ry __ __ uw yr __ ot qz "),
        ("j2k56", "vv zq __ __ ry wu __ __ yr __ ot qz "),
        ("j2k5", "qr vp __ __ pv rq __ __ uy yu __ ow "),
        ("j2k6l3", "vv zq to __ __ wu __ uw __ __ ot qz "),
        ("j2k6m5", "vv zq __ __ ry wu __ __ __ sx ot qz "),
        ("j2k6", "vv zq __ __ ry wu __ uw __ __ ot qz "),
        ("j2l3", "vv zq to __ __ wu __ uw __ sx __ qz "),
        ("j2ps", "vv zq __ ps ry wu __ uw __ sx __ qz "),
        ("j2to", "vv zq to __ ry wu __ uw __ sx __ qz "),
        ("j2", "vv zq __ __ ry wu __ uw __ sx __ qz "),
        ("j346l5", "yr __ ot qz vv zq __ __ ry __ __ uw "),
        ("j346ow", "vp __ tz pv rq __ __ uy yu __ ow qr "),
        ("j34k6", "vt __ tv pq ro __ __ uu __ __ or qp "),
        ("j34or", "vt __ tv pq ro __ __ uu __ sw or qx "),
        ("j36so", "vu __ tw xr __ wt __ uv yq so __ qy "),
        ("j36", "vu __ tw xr __ wt __ uv yq __ __ qy "),
        ("j3k16zs", "__ zs ty xu __ ww __ ux yt __ ov qq "),
        ("j3k56m4", "vo __ ty xu __ __ __ ux yt __ ov qq "),
        ("j3k5m4", "vu __ tw xr __ __ __ uv yq so __ qy "),
        ("j3k5to", "yr __ ot qz __ zq to __ ry wu __ uw "),
        ("j3k5", "yr __ ot qz __ zq __ __ ry wu __ uw "),
        ("j3k6", "vo __ ty xu __ ww __ ux __ __ ov qq "),
        ("j3k6yt", "vo __ ty xu __ ww __ ux yt __ ov qq "),
        ("j3or", "vt __ tv xq __ ws __ uu __ sw or qp "),
        ("j3ror", "vt __ tv xq ro ws __ uu __ sw or qp "),
        ("j3ro", "vt __ tv pq ro ws __ uu __ sw __ qx "),
        ("j3", "vt __ tv xq __ ws __ uu __ sw __ qx "),
        ("j3zz", "vt zz tv xq __ ws __ uu __ sw __ qx "),
        ("j56l7", "xr __ wt __ uv yq so __ qy vu __ __ "),
        ("j56", "xr __ wt __ uv yq so __ qy __ __ tw "),
        ("j5l6", "wr __ ut __ sv oq qo vs __ __ __ rw "),
        ("j5", "wr __ ut __ sv oq qo __ __ tu __ rw "),
        ("j6os", "vu __ tw __ rx wt __ uv yq __ os qy "),
        ("j6pros", "vu __ tw pr rp wt __ uv yq __ os qy "),
        ("j6prso", "vu __ tw pr rp wt __ uv yq so __ qy "),
        ("j6pr", "vu __ tw pr rp wt __ uv yq __ __ qy "),
        ("j6so", "vu __ tw __ rp wt __ uv yq so __ qy "),
        ("j6", "vu __ tw __ rx wt __ uv yq __ __ qy "),
        ("k125", "__ sx __ qz vv zq __ __ ry wu __ uw "),
        ("k12j5", "__ sx __ qz vv zq to __ __ wu __ uw "),
        ("k12", "__ sx __ qz vv zq __ xs __ wu __ uw "),
        ("k135m4", "__ rx wt __ __ yq __ os qy vu __ tw "),
        ("k157m6", "vv zq to __ ry wu __ __ yr __ __ qz "),
        ("k15", "__ rx wt __ uv yq __ __ qy vu __ tw "),
        ("k17j5", "or qp vt __ tv pq ro __ __ uu __ __ "),
        ("k1j56l7", "__ ux yt __ ov qq vo __ ty xu __ __ "),
        ("k1j5", "__ ux yt __ ov qq vo __ __ xu __ ww "),
        ("k1j6l7", "__ rx wt __ uv yq __ os qy vu __ __ "),
        ("k1j6ot", "__ zq to __ ry wu __ uw yr __ ot qz "),
        ("k1j6", "__ zq to __ ry wu __ uw yr __ __ qz "),
        ("k1", "__ qx vt __ tv xq __ ws __ uu __ sw "),
        ("k256", "vp __ __ pv rq wo __ __ yu __ ow qr "),
        ("k257m1", "yr __ ot qz vv zq __ __ ry wu __ __ "),
        ("k25m17", "__ sx ot qz vv zq __ __ ry wu __ __ "),
        ("k25m1", "__ __ ot qz vv zq __ __ ry wu __ uw "),
        ("k25", "yr __ __ qz vv zq __ __ ry wu __ uw "),
        ("k26m5", "vt __ __ pq ro ws __ __ __ sw or qp "),
        ("k26sw", "vt __ __ pq ro ws __ uu __ sw or qp "),
        ("k26tv", "vt __ tv pq ro ws __ uu __ __ or qx "),
        ("k26", "vp __ __ pv rq wo __ uy __ __ ow qr "),
        ("k26yu", "vp __ __ pv rq wo __ uy yu __ ow qr "),
        ("k2j17", "__ __ __ pq ro ws __ uu __ sw or qp "),
        ("k2j17tv", "__ __ tv pq ro ws __ uu __ sw or qp "),
        ("k2j56l7", "yr __ __ qz vv zq to __ ry wu __ __ "),
        ("k2j56", "yr __ __ qz vv zq to __ ry __ __ uw "),
        ("k2j5l6", "yr __ __ qz vv zq to xs __ __ __ uw "),
        ("k2j5m1", "__ __ ot qz vv zq to __ __ wu __ uw "),
        ("k2j5", "yr __ __ qz vv zq to __ __ wu __ uw "),
        ("k2j6tz", "vp __ tz pv rq wo __ uy yu __ __ qr "),
        ("k2j6", "vp __ __ pv rq wo __ uy yu __ __ qr "),
        ("k2j6zt", "vp zt __ pv rq wo __ uy yu __ __ qr "),
        ("k2m1", "__ __ ot qz vv zq __ xs __ wu __ uw "),
        ("k2", "yr __ __ qz vv zq __ xs __ wu __ uw "),
        ("k345m2", "xr __ __ __ uv yq so __ qy vu __ tw "),
        ("k34m2", "wr __ __ __ sv oq qo vs __ tu __ rw "),
        ("k34", "wr __ ut __ __ oq qo vs __ tu __ rw "),
        ("k56m4", "vu __ tw __ rx __ __ uv yq __ os qy "),
        ("k56", "vu __ tw __ rx wt __ __ yq __ os qy "),
        ("k5", "xr __ wt __ uv yq __ __ qy vu __ tw "),
        ("k6m5", "vs __ tu __ rw wr __ __ __ sv oq qo "),
        ("k6sv", "vs __ tu __ rw wr __ ut __ sv oq qo "),
        ("k6", "vs __ tu __ rw wr __ ut __ __ oq qo "),
        ("n0pz", "vr __ tt pz rv wq __ us __ su __ qw "),
        ("n0pzyy", "vr __ tt pz rv wq __ us yy su __ qw "),
        ("n0", "vr __ tt __ rv wq __ us __ su __ qw "),
        ("n0yy", "vr __ tt __ rv wq __ us yy su __ qw "),
        ("n0zp", "vr zp tt __ rv wq __ us __ su __ qw "),
        ("n0zpyy", "vr zp tt __ rv wq __ us yy su __ qw "),
        ("n167m4", "vu __ __ xr __ wt __ uv yq so __ qy "),
        ("n167", "yr __ __ qz __ zq to __ ry wu __ uw "),
        ("n25m6", "or qp vt __ __ pq ro ws __ __ __ sw "),
        ("n26l5", "__ __ tv pq ro ws __ __ __ sw or qp "),
        ("n345l7", "__ rx __ __ uv yq __ os qy vu __ tw "),
        ("n345", "__ zq __ __ ry wu __ uw yr __ ot qz "),
        ("n45l2", "vo __ __ xu __ ww __ ux yt __ ov qq "),
        ("n5l2", "vp __ __ pv rq __ __ uy yu __ ow qr "),
        ("n67m2", "__ ux __ __ ov qq vo __ ty xu __ ww "),
        ("n6m2", "qr vp __ __ pv rq wo __ uy yu __ __ "),
    ]
}

#[test]
fn check_horolog_return_value() {
    let past: u64 = 1721093758;
    let aeon: u64 = horolog();

    assert!(past < aeon, "horolog assertion failed");
}

#[test]
fn check_figures_return_values() {
    let lyre = String::from("cgdae");
    let figs: (String, Vec<usize>) = figures(lyre);
    let (tune, pegs) = figs;

    assert_eq!("cgdae", tune);
    assert_eq!([12, 27, 6, 21, 0].to_vec(), pegs);
}

#[test]
fn check_supply_return_array_size() {
    let arts: [(&str, &str); VOLUME] = supply();

    assert_eq!(arts.len(), VOLUME);
}

#[test]
fn check_supply_scale_value_lengths() {
    let arts: [(&str, &str); VOLUME] = supply();

    for pair in arts {
        assert_eq!(pair.1.len(), 36);
    }
}

#[test]
fn check_stylist_return_type() {
    let arts: [(&str, &str); VOLUME] = supply();
    let kind: () = stylist(&arts);

    assert_eq!((), kind);
}

#[test]
fn check_lattice_return_type() {
    let pegs: [usize; 9] = [30, 15, 0, 21, 6, 27, 12, 33, 18];
    let tune = String::from("beadgcf");
    let aeon: u64 = 1721093758;
    let stem: String = format!("-{}-h{}", tune, aeon);
    let arts: [(&str, &str); VOLUME] = supply();
    let pair: &(&str, &str) = &arts[VOLUME - 1];
    let kind: () = lattice(*pair, stem.to_owned(), pegs.to_vec());

    assert_eq!((), kind);
}

#[test]
fn check_entirety_return_type() {
    let pegs: [usize; 9] = [30, 15, 0, 21, 6, 27, 12, 33, 18];
    let tune = String::from("beadgcf");
    let aeon: u64 = 1721093758;
    let stem: String = format!("-{}-h{}", tune, aeon);
    let arts: [(&str, &str); VOLUME] = supply();
    let kind: () = entirety(&arts, &stem, &pegs);

    assert_eq!((), kind);
}

#[test]
fn check_spandex_return_type() {
    let clef: String = String::from("n0");
    let pegs: [usize; 9] = [30, 15, 0, 21, 6, 27, 12, 33, 18];
    let tune = String::from("beadgcf");
    let aeon: u64 = 1721093758;
    let stem: String = format!("-{}-h{}", tune, aeon);
    let arts: [(&str, &str); VOLUME] = supply();
    let kind: () = spandex(&clef, &arts, &stem, &pegs);

    assert_eq!((), kind);
}
