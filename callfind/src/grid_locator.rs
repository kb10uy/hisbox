/// Calculates grid locator from longitude and latitude.
/// Returns `None` if the values are out of range.
/// Returned value is guaranteed to be valid as ASCII string.
pub fn calculate_grid(longitude: f64, latitude: f64) -> Option<[u8; 10]> {
    fn to_letters(value: usize) -> (u8, u8, u8, u8, u8) {
        let field = (value / 57600) as u8;
        let square = (value / 5760 % 10) as u8;
        let subsq1 = (value / 240 % 24) as u8;
        let subsq2 = (value / 24 % 10) as u8;
        let subsq3 = (value % 24) as u8;
        (
            b'A' + field,
            b'0' + square,
            b'a' + subsq1,
            b'0' + subsq2,
            b'a' + subsq3,
        )
    }

    let offset_lng = longitude + 180.0;
    let offset_lat = latitude + 90.0;
    if !(0.0..360.0).contains(&offset_lng) || !(0.0..180.0).contains(&offset_lat) {
        return None;
    }

    // sub-degree square has 5760 blocks
    let scaled_lng = (offset_lng / 2.0 * 5760.0) as usize;
    let scaled_lat = (offset_lat * 5760.0) as usize;
    let lng_letters = to_letters(scaled_lng);
    let lat_letters = to_letters(scaled_lat);

    Some([
        lng_letters.0,
        lat_letters.0,
        lng_letters.1,
        lat_letters.1,
        lng_letters.2,
        lat_letters.2,
        lng_letters.3,
        lat_letters.3,
        lng_letters.4,
        lat_letters.4,
    ])
}
