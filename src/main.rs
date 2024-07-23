mod datum;
mod utile;

use crate::datum::{records, QTY};
use crate::utile::{entirety, pitcher, qualify, shrouds, spandex, stylist};
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

        let (ouds, keys) = shrouds();
        let tune: String = pitcher(&ouds, &inks);
        let urns: (String, Vec<usize>) = qualify(tune);
        let gulp = String::from("gamut");

        if inks.contains(&gulp) {
            entirety(urns);
        } else {
            let arts: [(&str, &str); QTY] = records();

            for item in &inks {
                // sift through items for signatures or tunings
                if keys.contains(item) {
                    spandex(item, &arts, &urns);
                } else if ouds.contains(item) {
                    if inks.len() == 1 {
                        stylist();
                    } else {
                        continue;
                    }
                } else {
                    println!("\n\t{item} ?");
                }
            }
        }
    } else {
        stylist();
    }
    println!();
}
