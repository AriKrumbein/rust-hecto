use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

#[allow(unused_assignments)]

fn main() {
    // assigning so the value won't be dropped.
    let _stdout = stdout().into_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        if c.is_control() {
            println!("{:?} \r", b);
        } else {
            println!("{:?} ({})\r", b, c);
        }
        if c == 'q' {
            break;
        }
    }
}
