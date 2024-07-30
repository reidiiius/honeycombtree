mod datum;
mod utile;

use crate::datum::QTY;
use crate::utile::{enclave, entirety, groupie, pitcher, refined, stylist, veranda};
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

        let tune: String = pitcher(&inks);
        let gulp = String::from("gamut");
        let grok = String::from("group");
        let quiz = String::from("query");
        let tons = String::from("tonal");

        if inks.contains(&gulp) {
            entirety(tune);
        } else if inks.contains(&grok) {
            groupie(inks);
        } else if inks.contains(&quiz) {
            enclave(inks);
        } else if inks.contains(&tons) {
            refined();
        } else {
            veranda(inks, tune);
        }
    } else {
        stylist();
    }
    println!();
}
