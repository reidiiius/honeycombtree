/// Quantity of Tuples in the Array returned by `records`
pub const QTY: usize = 126;

/// Returns Tuple holding Vectors of routine, tuning, and key Strings
pub fn codices() -> (Vec<String>, Vec<String>, Vec<String>) {
    let dyns: Vec<String> = dynamos();
    let tuns: Vec<String> = tunings();
    let keys: Vec<String> = signats();

    (dyns, tuns, keys)
}

/// Returns a Vector of routine Strings
pub fn dynamos() -> Vec<String> {
    let ways: [&str; 4] = ["gamut", "group", "query", "tonal"];
    let mut dyns: Vec<String> = vec![];

    for proc in ways {
        dyns.push(proc.to_string());
    }

    dyns
}

/// Returns a Vector of tuning Strings
pub fn tunings() -> Vec<String> {
    let ways: [&str; 7] = [
        "beadgcf", "bfbfb", "cgdae", "dgdgbd", "eadgbe", "fkbjdn", "piano",
    ];
    let mut tuns: Vec<String> = vec![];

    for tune in ways {
        tuns.push(tune.to_string());
    }

    tuns
}

/// Matches tuning String and returns a Vector of indices
pub fn machine(tune: Option<&str>) -> Vec<usize> {
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

/// Returns a Vector of key Strings from `records`
pub fn signats() -> Vec<String> {
    let arts: [(&str, &str); QTY] = records();
    let mut keys: Vec<String> = vec![];

    for pair in arts {
        keys.push(pair.0.to_string());
    }

    keys
}

/// Returns a Vector of value Strings from `records`
pub fn melodia() -> Vec<String> {
    let arts: [(&str, &str); QTY] = records();
    let mut vals: Vec<String> = vec![];

    for pair in arts {
        vals.push(pair.1.to_string());
    }

    vals
}

/// Returns sorted Vector of digraph Strings from `records`
pub fn nodules() -> Vec<String> {
    let vals: Vec<String> = melodia();
    let mut temp: Vec<String> = vec![];
    let mut nods: Vec<String> = vec![];

    for raga in vals {
        for duet in &mut raga.split_ascii_whitespace() {
            if duet.eq("__") {
                continue;
            } else {
                temp.push(duet.to_string());
            }
        }
    }
    nods.push(temp.pop().expect("temp vacant"));

    for duet in temp {
        if nods.contains(&duet) {
            continue;
        } else {
            nods.push(duet);
        }
    }
    nods.sort();

    nods
}

/// Returns an Array of Tuples containing key-value string slices
pub fn records() -> [(&'static str, &'static str); QTY] {
    SWARM
}

/// Array of Tuples containing key-value string slices
const SWARM: [(&str, &str); QTY] = [
    ("j136l7", "__ __ tw xr __ wt __ uv yq so __ qy "),
    ("j167l2", "vu __ __ __ rx wt __ uv yq __ os qy "),
    ("j17l2", "vs __ __ __ rw wr __ ut __ sv oq qo "),
    ("j17zy", "__ zy tu __ rw wr __ ut __ sv oq qo "),
    ("j17", "__ __ tu __ rw wr __ ut __ sv oq qo "),
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
    ("j25vt", "or qp vt __ tv pq ro __ __ uu __ sw "),
    ("j25", "qr vp __ __ pv rq wo __ __ yu __ ow "),
    ("j26l34", "vv zq to xs __ __ __ uw yr __ __ qz "),
    ("j26l3", "vv zq to __ __ wu __ uw yr __ __ qz "),
    ("j26ot", "vv zq __ __ ry wu __ uw yr __ ot qz "),
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
    ("j34or", "vt __ tv pq ro __ __ uu __ sw or qp "),
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
    ("j3ro", "vt __ tv pq ro ws __ uu __ sw __ qx "),
    ("j3", "vt __ tv xq __ ws __ uu __ sw __ qx "),
    ("j3zz", "vt zz tv xq __ ws __ uu __ sw __ qx "),
    ("j56l7", "xr __ wt __ uv yq so __ qy vu __ __ "),
    ("j56", "xr __ wt __ uv yq so __ qy __ __ tw "),
    ("j5l6", "wr __ ut __ sv oq qo vs __ __ __ rw "),
    ("j5vs", "wr __ ut __ sv oq qo vs __ tu __ rw "),
    ("j5", "wr __ ut __ sv oq qo __ __ tu __ rw "),
    ("j6os", "vu __ tw __ rx wt __ uv yq __ os qy "),
    ("j6pr", "vu __ tw pr rp wt __ uv yq __ __ qy "),
    ("j6so", "vu __ tw __ rx wt __ uv yq so __ qy "),
    ("j6", "vu __ tw __ rx wt __ uv yq __ __ qy "),
    ("k125", "__ sx __ qz vv zq __ __ ry wu __ uw "),
    ("k12j5", "__ sx __ qz vv zq to __ __ wu __ uw "),
    ("k12", "__ sx __ qz vv zq __ xs __ wu __ uw "),
    ("k135m4", "__ rx wt __ __ yq __ os qy vu __ tw "),
    ("k157m6", "vv zq to __ ry wu __ __ yr __ __ qz "),
    ("k15", "__ rx wt __ uv yq __ __ qy vu __ tw "),
    ("k17j5", "or qp vt __ tv pq ro __ __ uu __ __ "),
    ("k17ro", "or qp vt __ tv pq ro ws __ uu __ __ "),
    ("k1j56l7", "__ ux yt __ ov qq vo __ ty xu __ __ "),
    ("k1j5", "__ ux yt __ ov qq vo __ __ xu __ ww "),
    ("k1j6l7", "__ rx wt __ uv yq __ os qy vu __ __ "),
    ("k1j6ot", "__ zq to __ ry wu __ uw yr __ ot qz "),
    ("k1j6", "__ zq to __ ry wu __ uw yr __ __ qz "),
    ("k1zz", "__ qx vt zz tv xq __ ws __ uu __ sw "),
    ("k1", "__ qx vt __ tv xq __ ws __ uu __ sw "),
    ("k256", "vp __ __ pv rq wo __ __ yu __ ow qr "),
    ("k257m1", "yr __ ot qz vv zq __ __ ry wu __ __ "),
    ("k25m17", "__ sx ot qz vv zq __ __ ry wu __ __ "),
    ("k25m1", "__ __ ot qz vv zq __ __ ry wu __ uw "),
    ("k25", "yr __ __ qz vv zq __ __ ry wu __ uw "),
    ("k26m5", "vt __ __ pq ro ws __ __ __ sw or qp "),
    ("k26sw", "vt __ __ pq ro ws __ uu __ sw or qp "),
    ("k26tv", "vt __ tv pq ro ws __ uu __ __ or qp "),
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
    ("k34yz", "wr __ ut yz __ oq qo vs __ tu __ rw "),
    ("k34", "wr __ ut __ __ oq qo vs __ tu __ rw "),
    ("k56m4", "vu __ tw __ rx __ __ uv yq __ os qy "),
    ("k56", "vu __ tw __ rx wt __ __ yq __ os qy "),
    ("k5os", "xr __ wt __ uv yq __ os qy vu __ tw "),
    ("k5rp", "pr rp wt __ uv yq __ __ qy vu __ tw "),
    ("k5so", "xr __ wt __ uv yq so __ qy vu __ tw "),
    ("k5", "xr __ wt __ uv yq __ __ qy vu __ tw "),
    ("k6m5", "vs __ tu __ rw wr __ __ __ sv oq qo "),
    ("k6sv", "vs __ tu __ rw wr __ ut __ sv oq qo "),
    ("k6", "vs __ tu __ rw wr __ ut __ __ oq qo "),
    ("n0pz", "vr __ tt pz rv wq __ us __ su __ qw "),
    ("n0", "vr __ tt __ rv wq __ us __ su __ qw "),
    ("n0yy", "vr __ tt __ rv wq __ us yy su __ qw "),
    ("n0zp", "vr zp tt __ rv wq __ us __ su __ qw "),
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
];
