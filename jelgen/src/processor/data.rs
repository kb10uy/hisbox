use serde::{Deserialize, Serialize};
use time::{UtcOffset, format_description::BorrowedFormatItem, macros::format_description};

use crate::qso::record::QsoRecord;

const RECORD_DATE: &[BorrowedFormatItem<'_>] = format_description!("[year]-[month]-[day]");
const RECORD_TIME: &[BorrowedFormatItem<'_>] = format_description!("[hour]:[minute]:[second]");

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct RecordInner<'a> {
    pub date: String,
    pub time: String,
    pub band: String,
    pub mode: &'a str,
    pub call: &'a str,
    pub tx_report: &'a str,
    pub tx_number: &'a str,
    pub rx_report: &'a str,
    pub rx_number: &'a str,
}

impl<'a> RecordInner<'a> {
    pub(super) fn new(
        qso_record: &'a QsoRecord,
        process_offset: UtcOffset,
    ) -> Option<RecordInner<'a>> {
        let (tx, rx) = qso_record.sent.as_ref().zip(qso_record.received.as_ref())?;
        let process_datetime = qso_record.datetime.to_offset(process_offset);
        Some(RecordInner {
            date: process_datetime.format(RECORD_DATE).expect("format failed"),
            time: process_datetime.format(RECORD_TIME).expect("format failed"),
            band: qso_record.band.to_string(),
            mode: &qso_record.mode,
            call: &qso_record.call,
            tx_report: &tx.report,
            tx_number: &tx.number,
            rx_report: &rx.report,
            rx_number: &rx.number,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QsoSummary {
    pub multiplier: i64,
    pub point: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QsoMetadata {
    pub id: String,
    pub group: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContestSummary {
    pub total: i64,
}
