use time::Date;

pub mod jarl;

pub fn calculate_age(today: Date, birthday: Date) -> u32 {
    let mut age = today.year() - birthday.year();
    if today.month() < birthday.month() {
        age -= 1;
    } else if today.month() == birthday.month() && today.day() < birthday.day() {
        age -= 1;
    }
    age as u32
}
