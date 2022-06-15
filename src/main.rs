use chrono::prelude::*;

fn main() {
    let dt = Local::now().to_rfc2822();
    if &dt[..3] == "Mon" {
        println!("I sure do hate mondays..");
    } else {
        println!("Wow I sure do heckin' love lasagna!!");
    }
}
