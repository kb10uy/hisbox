use serde::{Deserialize, Serialize};
use time::{UtcOffset, format_description::BorrowedFormatItem, macros::format_description};

use crate::qso::record::QsoRecord;

const RECORD_DATE: &[BorrowedFormatItem<'_>] = format_description!("[year]-[month]-[day]");
const RECORD_TIME: &[BorrowedFormatItem<'_>] = format_description!("[hour]:[minute]:[second]");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub date: String,
    pub time: String,
    pub band: String,
    pub mode: String,
    pub call: String,
    pub tx_report: String,
    pub tx_number: String,
    pub rx_report: String,
    pub rx_number: String,
}

impl Record {
    pub fn new(qso_record: QsoRecord, process_offset: UtcOffset) -> Option<Record> {
        let (tx, rx) = qso_record.sent.zip(qso_record.received)?;
        let process_datetime = qso_record.datetime.to_offset(process_offset);
        Some(Record {
            date: process_datetime.format(RECORD_DATE).expect("format failed"),
            time: process_datetime.format(RECORD_TIME).expect("format failed"),
            band: qso_record.band.to_string(),
            mode: qso_record.mode,
            call: qso_record.call,
            tx_report: tx.report,
            tx_number: tx.number,
            rx_report: rx.report,
            rx_number: rx.number,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct QsoSummary {
    pub multiplier: i64,
    pub point: i64,
}
