mod datum;
mod utile;

use crate::datum::{adaptor, devices, proctor, QTY};
use crate::utile::{catalog, diatoms, enclave, entirety, groupie, octopus, polaris, veranda};
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

        let tune: &str = adaptor(&inks);
        let rout: &str = proctor(&inks);
        let devs: [&str; 7] = devices();

        if rout.is_empty() {
            veranda(&inks, tune);
        } else if rout.eq(devs[0]) {
            entirety(tune); // gamut
        } else if rout.eq(devs[1]) {
            groupie(&inks); // group
        } else if rout.eq(devs[2]) {
            octopus(); // octad
        } else if rout.eq(devs[3]) {
            polaris(); // polar
        } else if rout.eq(devs[4]) {
            enclave(&inks); // query
        } else if rout.eq(devs[5]) {
            diatoms(); // tonal
        } else {
            catalog(); // usage
        }
    } else {
        catalog();
    }
    println!();
}
