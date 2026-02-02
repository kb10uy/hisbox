use std::env::args;

use callfind::grid_locator::calculate_grid;

fn main() {
    let lnglats = args().skip(1);
    for lnglat in lnglats {
        let Some((lng, lat)) = lnglat.split_once(',') else {
            continue;
        };
        let Some((longitude, latitude)) = (lng.parse().ok()).zip(lat.parse().ok()) else {
            continue;
        };

        let Some(gl) = calculate_grid(longitude, latitude) else {
            continue;
        };
        let gl = str::from_utf8(&gl).expect("must be valid");

        println!("{lnglat}: {gl}");
    }
}
