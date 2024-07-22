mod datum;
mod utile;

use crate::datum::{records, tunings, QTY};
use crate::utile::{entirety, pitcher, qualify, spandex, stylist};
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

    let arts: [(&str, &str); QTY] = records();
    let axes: [&str; 7] = tunings();

    if inks.len() > 1 {
        // shift head which holds executable path
        inks.remove(0);

        // lent character set and limit amount of characters
        inks.retain(|argo| argo.is_ascii() && argo.len() < 10);

        let tune = pitcher(&axes, &inks);
        let urns: (String, Vec<usize>) = qualify(tune);

        let gulp = String::from("gamut");

        if inks.contains(&gulp) {
            entirety(arts, urns);
        } else {
            let pres: [char; 4] = ['i', 'j', 'k', 'n'];

            for clef in inks {
                // sift through items for key signatures
                if clef.starts_with(pres) {
                    spandex(&clef, &arts, &urns);
                }
            }
        }
    } else {
        stylist(&axes, arts);
    }
    println!();
}
