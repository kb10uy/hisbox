mod error;
mod record;

use adif_reader::document::Record;
use time::{
    Date, OffsetDateTime, Time, UtcDateTime, UtcOffset, format_description::BorrowedFormatItem,
    macros::format_description,
};

use crate::qso::error::QsoError;

const ADIF_DATE: &[BorrowedFormatItem<'_>] = format_description!(
    "[year repr:full padding:zero][month repr:numerical padding:zero][day padding:zero]"
);
const ADIF_TIME: &[BorrowedFormatItem<'_>] =
    format_description!("[hour repr:24 padding:zero][minute padding:zero][second padding:zero]");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QsoRecord {
    pub datetime: UtcDateTime,
    pub band: String,
    pub mode: String,
    pub callsign: String,
    /*
        pub sent_report: String,
        pub sent_number: String,
        pub received_report: String,
        pub received_number: String,
    */
}

impl QsoRecord {
    pub fn new(record: &Record, offset: UtcOffset) -> Result<QsoRecord, QsoError> {
        let datetime = OffsetDateTime::new_in_offset(
            Date::parse(get_required_field(record, "QSO_DATE")?, ADIF_DATE)?,
            Time::parse(get_required_field(record, "TIME_ON")?, ADIF_TIME)?,
            offset,
        )
        .to_utc();

        let adif_band = get_required_field(record, "BAND")?;
        let mode = get_required_field(record, "MODE")?;
        let callsign = get_required_field(record, "CALL")?;

        Ok(QsoRecord {
            datetime,
            band: adif_band.to_string(),
            mode: mode.to_string(),
            callsign: callsign.to_string(),
        })
    }
}

fn get_required_field<'a>(record: &'a Record, field: &'static str) -> Result<&'a str, QsoError> {
    record.field(field).ok_or(QsoError::MissingAdifField(field))
}
