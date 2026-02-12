use std::str::FromStr;

use adif_reader::document::Record;

use crate::error::QsoError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QslReceiveStatus {
    Invalid,
    Unconfirmed,
    Confirmed,
    Requested,
    Verified,
}

impl FromStr for QslReceiveStatus {
    type Err = QsoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "I" => Ok(QslReceiveStatus::Invalid),
            "N" => Ok(QslReceiveStatus::Unconfirmed),
            "Y" => Ok(QslReceiveStatus::Confirmed),
            "R" => Ok(QslReceiveStatus::Requested),
            "V" => Ok(QslReceiveStatus::Verified),
            _ => Err(QsoError::QslParse),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QslSendStatus {
    Invalid,
    NotSent,
    Sent,
    Requested,
    Queued,
}

impl FromStr for QslSendStatus {
    type Err = QsoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "I" => Ok(QslSendStatus::Invalid),
            "N" => Ok(QslSendStatus::NotSent),
            "Y" => Ok(QslSendStatus::Sent),
            "R" => Ok(QslSendStatus::Requested),
            "V" => Ok(QslSendStatus::Queued),
            _ => Err(QsoError::QslParse),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QslVia {
    Bureau,
    Direct,
    Electronic,
    Manager,
}

impl FromStr for QslVia {
    type Err = QsoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "B" => Ok(QslVia::Bureau),
            "D" => Ok(QslVia::Direct),
            "E" => Ok(QslVia::Electronic),
            "M" => Ok(QslVia::Manager),
            _ => Err(QsoError::QslParse),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QslStatus {
    pub receive: Option<QslReceiveStatus>,
    pub receive_via: Option<QslVia>,
    pub send: Option<QslSendStatus>,
    pub send_via: Option<QslVia>,
}

impl QslStatus {
    pub fn new(record: &Record) -> Result<QslStatus, QsoError> {
        let receive = record
            .field("QSL_RCVD")
            .map(FromStr::from_str)
            .transpose()?;
        let send = record
            .field("QSL_SENT")
            .map(FromStr::from_str)
            .transpose()?;
        let receive_via = record
            .field("QSL_RCVD_VIA")
            .map(FromStr::from_str)
            .transpose()?;
        let send_via = record
            .field("QSL_SENT_VIA")
            .map(FromStr::from_str)
            .transpose()?;
        Ok(QslStatus {
            receive,
            receive_via,
            send,
            send_via,
        })
    }
}
