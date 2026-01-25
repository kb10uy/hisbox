use std::env::args;

use callfind::lookup_prefix_area;

fn main() {
    let calls = args().skip(1);
    for mut call in calls {
        if !call.is_ascii() {
            continue;
        }

        call.make_ascii_uppercase();
        println!("{}: {:?}", call, lookup_prefix_area(call.as_bytes()));
    }
}
