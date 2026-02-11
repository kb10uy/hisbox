use adif_reader::document::Record;
use compact_str::{CompactString, ToCompactString};
use time::{
    Date, OffsetDateTime, Time, UtcDateTime, UtcOffset, format_description::BorrowedFormatItem,
    macros::format_description,
};

use crate::{band::Band, error::QsoError, get_required_field};

const ADIF_DATE: &[BorrowedFormatItem<'_>] = format_description!(
    "[year repr:full padding:zero][month repr:numerical padding:zero][day padding:zero]"
);
const ADIF_TIME: &[BorrowedFormatItem<'_>] =
    format_description!("[hour repr:24 padding:zero][minute padding:zero][second padding:zero]");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QsoExchange {
    pub report: CompactString,
    pub number: CompactString,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QsoRecord {
    pub datetime: UtcDateTime,
    pub band: Band,
    pub mode: CompactString,
    pub call: CompactString,
    pub frequency: CompactString,
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
        let frequency = get_required_field(record, "FREQ")?;

        Ok(QsoRecord {
            datetime,
            band: adif_band.parse()?,
            mode: mode.to_compact_string(),
            call: callsign.to_compact_string(),
            frequency: frequency.to_compact_string(),
        })
    }
}
