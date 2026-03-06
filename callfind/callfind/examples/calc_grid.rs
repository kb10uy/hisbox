use std::env::args;

use callfind::grid_locator::GridLocator;

fn main() {
    let lnglats = args().skip(1);
    for lnglat in lnglats {
        let Some((lng, lat)) = lnglat.split_once(',') else {
            continue;
        };
        let Some((longitude, latitude)) = (lng.parse().ok()).zip(lat.parse().ok()) else {
            continue;
        };

        let gl = match GridLocator::from_lnglat(longitude, latitude) {
            Ok(gl) => gl,
            Err(e) => {
                eprintln!("{lnglat}: {e}");
                continue;
            }
        };

        println!("{lnglat}: {gl}");
    }
}
