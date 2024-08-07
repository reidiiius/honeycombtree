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

        let tune: String = adaptor(&inks);
        let rout: String = proctor(&inks);
        let devs: &[String] = &devices();

        if rout.is_empty() {
            veranda(&inks, &tune);
        } else if rout.eq(devs.first().unwrap()) {
            entirety(&tune); // gamut
        } else if rout.eq(devs.get(1).unwrap()) {
            groupie(&inks); // group
        } else if rout.eq(devs.get(2).unwrap()) {
            octopus(); // octad
        } else if rout.eq(devs.get(3).unwrap()) {
            polaris(); // polar
        } else if rout.eq(devs.get(4).unwrap()) {
            enclave(&inks); // query
        } else if rout.eq(devs.get(5).unwrap()) {
            diatoms(); // tonal
        } else {
            catalog(); // usage
        }
    } else {
        catalog();
    }
    println!();
}
