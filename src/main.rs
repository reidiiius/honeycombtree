mod datum;
mod utile;

use crate::datum::{adaptor, dynamos, proctor, QTY};
use crate::utile::{enclave, entirety, groupie, octopus, polaris, refined, stylist, veranda};
use std::env;

#[cfg(test)]
mod tasks;

/// Entry point of application
fn main() {
    let mut inks: Vec<String> = env::args().collect();

    // limit amount of arguments
    if inks.len() > QTY {
        inks.clear();
        println!("\n\tRequest denied!\n");
        return;
    }

    if inks.len() > 1 {
        // shift head which holds executable path
        inks.remove(0);

        // lent character set and limit amount of characters
        inks.retain(|argo| argo.is_ascii() && argo.len() < 10);

        let tune: String = adaptor(&inks);
        let rout: String = proctor(&inks);
        let dyns: Vec<String> = dynamos();

        if rout.is_empty() {
            veranda(inks, tune);
        } else if rout.eq(&dyns[0]) {
            entirety(tune); // gamut
        } else if rout.eq(&dyns[1]) {
            groupie(inks); // group
        } else if rout.eq(&dyns[2]) {
            octopus(); // octad
        } else if rout.eq(&dyns[3]) {
            polaris(); // polar
        } else if rout.eq(&dyns[4]) {
            enclave(inks); // query
        } else if rout.eq(&dyns[5]) {
            refined(); // tonal
        } else {
            veranda(inks, tune);
        }
    } else {
        stylist();
    }
    println!();
}
