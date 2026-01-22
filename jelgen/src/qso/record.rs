use adif_reader::document::Record;
use time::{
    Date, OffsetDateTime, Time, UtcDateTime, UtcOffset, format_description::BorrowedFormatItem,
    macros::format_description,
};

use crate::qso::{band::Band, error::QsoError};

const ADIF_DATE: &[BorrowedFormatItem<'_>] = format_description!(
    "[year repr:full padding:zero][month repr:numerical padding:zero][day padding:zero]"
);
const ADIF_TIME: &[BorrowedFormatItem<'_>] =
    format_description!("[hour repr:24 padding:zero][minute padding:zero][second padding:zero]");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QsoExchange {
    pub report: String,
    pub number: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QsoRecord {
    pub datetime: UtcDateTime,
    pub band: Band,
    pub mode: String,
    pub call: String,
    pub sent: Option<QsoExchange>,
    pub received: Option<QsoExchange>,
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
        let sent_report = get_optional_field_oneof(record, &["RST_SENT"]);
        let sent_number = get_optional_field_oneof(record, &["STX", "STX_STRING"]);
        let received_report = get_optional_field_oneof(record, &["RST_RCVD"]);
        let received_number = get_optional_field_oneof(record, &["SRX", "SRX_STRING"]);

        let sent = sent_report.zip(sent_number).map(|(r, n)| QsoExchange {
            report: r.to_string(),
            number: n.to_string(),
        });
        let received = received_report
            .zip(received_number)
            .map(|(r, n)| QsoExchange {
                report: r.to_string(),
                number: n.to_string(),
            });

        Ok(QsoRecord {
            datetime,
            band: adif_band.parse()?,
            mode: mode.to_string(),
            call: callsign.to_string(),
            sent,
            received,
        })
    }
}

fn get_required_field<'a>(record: &'a Record, field: &'static str) -> Result<&'a str, QsoError> {
    record.field(field).ok_or(QsoError::MissingAdifField(field))
}

fn get_optional_field_oneof<'a>(record: &'a Record, fields: &[&'static str]) -> Option<&'a str> {
    assert!(!fields.is_empty(), "fields must be specified");
    fields.iter().find_map(|f| record.field(*f))
}
