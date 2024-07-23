mod datum;
mod utile;

use crate::datum::{tunings, QTY};
use crate::utile::{entirety, pitcher, qualify, stylist, veranda};
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

        let axes: Vec<String> = tunings();
        let tune: String = pitcher(axes, &inks);
        let urns: (String, Vec<usize>) = qualify(tune);
        let gulp = String::from("gamut");

        if inks.contains(&gulp) {
            entirety(urns);
        } else {
            veranda(inks, urns);
        }
    } else {
        stylist();
    }
    println!();
}
