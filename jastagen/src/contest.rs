//! Selects the contest QSOs out of a log and scores them under the JASTA rules.
//!
//! The rules followed here are those of the 47th (2026) contest. They differ
//! from what `MMJASTA.EXE` implements in three places: WARC bands are excluded,
//! a QSO with a station that sent no self-portrait still counts, and 1.9 MHz was
//! already out of range. See the crate README for the details.

use std::collections::{BTreeSet, HashSet};

use adif_reader::document::Record;
use callfind::cty::{CtyDatabase, effective_segment};
use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

/// The band groups the rules award points for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointClass {
    /// 3.5 to 28 MHz, one point.
    Hf,

    /// 50 to 430 MHz, two points.
    Vhf,

    /// 1200 MHz and up, three points.
    Shf,
}

impl PointClass {
    /// Returns the points one QSO on this class of band is worth.
    pub fn points(self) -> u32 {
        match self {
            PointClass::Hf => 1,
            PointClass::Vhf => 2,
            PointClass::Shf => 3,
        }
    }
}

/// A band that counts for the contest, labelled the way MMSSTV labels it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContestBand {
    pub label: &'static str,
    pub class: PointClass,
}

/// The outcome of classifying an ADIF band.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BandVerdict {
    /// The band counts for the contest.
    Contest(ContestBand),

    /// A WARC band, excluded by the rules since the 46th contest.
    Warc,

    /// Below 3.5 MHz, or a band this log has no business carrying.
    OutOfRange,
}

/// Classifies an ADIF band name, using the frequency only to split 80 m.
pub fn classify_band(adif_band: &str, frequency_mhz: Option<f64>) -> BandVerdict {
    use PointClass::{Hf, Shf, Vhf};

    let contest = |label, class| BandVerdict::Contest(ContestBand { label, class });

    match adif_band.trim().to_ascii_lowercase().as_str() {
        "80m" => {
            let label = match frequency_mhz {
                Some(mhz) if mhz >= 3.7 => "3.8",
                _ => "3.5",
            };
            contest(label, Hf)
        }
        "40m" => contest("7", Hf),
        "20m" => contest("14", Hf),
        "15m" => contest("21", Hf),
        "10m" => contest("28", Hf),
        "6m" => contest("50", Vhf),
        "2m" => contest("144", Vhf),
        "70cm" => contest("430", Vhf),
        "23cm" => contest("1200", Shf),
        "13cm" => contest("2400", Shf),
        "9cm" => contest("3400", Shf),
        "6cm" => contest("5600", Shf),
        "3cm" => contest("10.1G", Shf),
        "1.25cm" => contest("24G", Shf),
        "6mm" => contest("47G", Shf),
        "4mm" => contest("75G", Shf),
        "2.5mm" | "2mm" => contest("142G", Shf),
        "1mm" => contest("248G", Shf),
        "30m" | "17m" | "12m" => BandVerdict::Warc,
        _ => BandVerdict::OutOfRange,
    }
}

/// A report and serial number pair as exchanged during the contest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exchange {
    pub report: String,
    pub serial: String,
}

impl Exchange {
    /// Renders the exchange the way it appears on the log sheet, such as `595020`.
    pub fn render(&self) -> String {
        format!("{}{}", self.report, self.serial)
    }
}

/// Builds an exchange from the report and serial fields of one ADIF record.
///
/// The serial is taken from `STX`/`SRX` when present and from `STX_STRING`/
/// `SRX_STRING` otherwise. When neither carries it, a report longer than three
/// digits is split, which covers logs that keep the whole exchange in the report
/// field.
pub fn build_exchange(report: Option<&str>, serial: Option<&str>) -> Option<Exchange> {
    let report = report.map(str::trim).filter(|s| !s.is_empty());
    let serial = serial.map(str::trim).filter(|s| !s.is_empty());

    let (report, serial) = match (report, serial) {
        (report, Some(serial)) => (report.unwrap_or("599"), serial),
        (Some(report), None) if report.len() > 3 && report.bytes().all(|b| b.is_ascii_digit()) => {
            report.split_at(3)
        }
        _ => return None,
    };

    let serial = match serial.parse::<u32>() {
        Ok(number) if number < 1000 => format!("{number:03}"),
        Ok(number) => number.to_string(),
        Err(_) => serial.to_string(),
    };

    Some(Exchange {
        report: report.to_string(),
        serial,
    })
}

/// Returns the Japanese callsign district a callsign belongs to.
///
/// The 7K to 7N series is entirely the 1 district regardless of the digit that
/// follows, which the rules state explicitly. Every other Japanese callsign
/// carries its district as the third character of the prefix-bearing segment.
pub fn ja_district(callsign: &str) -> Option<char> {
    let segment = effective_segment(&callsign.to_ascii_uppercase()).to_string();
    let bytes = segment.as_bytes();
    if bytes.len() < 3 {
        return None;
    }

    let district = if bytes[0] == b'7' && bytes[1] != b'J' {
        b'1'
    } else {
        bytes[2]
    };

    district.is_ascii_digit().then_some(district as char)
}

/// The multiplier one QSO contributes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Multi {
    /// A Japanese district, held as the district digit.
    JaDistrict(char),

    /// A DXCC entity, held as its canonical prefix.
    Dxcc(String),

    /// The callsign could not be resolved and needs a human.
    Unresolved,
}

impl Multi {
    /// Renders the multiplier the way it appears on the log sheet.
    pub fn render(&self) -> String {
        match self {
            Multi::JaDistrict(district) => format!("JA{district}"),
            Multi::Dxcc(prefix) => prefix.clone(),
            Multi::Unresolved => "?".to_string(),
        }
    }
}

/// Why a QSO does or does not score.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The QSO scores.
    Counted,

    /// The same station was already worked on this UTC day.
    Duplicate,

    /// No contest number was received, so the exchange is incomplete.
    NoReceivedNumber,
}

/// One QSO that entered the contest log.
#[derive(Debug, Clone)]
pub struct ContestQso {
    pub datetime: OffsetDateTime,
    pub call: String,
    pub band: ContestBand,
    pub sent: Exchange,
    pub received: Option<Exchange>,
    pub multi: Multi,
    pub status: Status,
}

impl ContestQso {
    /// Returns the points this QSO is worth.
    pub fn points(&self) -> u32 {
        match self.status {
            Status::Counted => self.band.class.points(),
            _ => 0,
        }
    }
}

/// Why a record never became a contest QSO.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rejection {
    OtherContest,
    NotSstv,
    WarcBand,
    BandOutOfRange,
    OutsideContestPeriod,
    NoSentNumber,
    Unreadable,
}

/// The scored contest log.
#[derive(Debug, Clone)]
pub struct ContestLog {
    pub year: i32,
    pub qsos: Vec<ContestQso>,
    pub hf_qsos: u32,
    pub vhf_qsos: u32,
    pub shf_qsos: u32,
    pub score: u32,
    pub districts: BTreeSet<char>,
    pub entities: BTreeSet<String>,
    pub days: BTreeSet<Date>,
    pub rejections: Vec<(Rejection, String)>,
}

impl ContestLog {
    /// Returns the operating-day multiplier, which the rules cap at ten.
    pub fn day_multi(&self) -> u32 {
        (self.days.len() as u32).min(10)
    }

    /// Returns the sum of every multiplier.
    pub fn multi(&self) -> u32 {
        self.districts.len() as u32 + self.entities.len() as u32 + self.day_multi()
    }

    /// Returns the final score.
    pub fn total(&self) -> u32 {
        self.score * self.multi()
    }

    /// Returns how many QSOs were dropped as duplicates.
    pub fn duplicates(&self) -> usize {
        self.qsos
            .iter()
            .filter(|q| q.status == Status::Duplicate)
            .count()
    }

    /// Returns how many QSOs lack a received contest number.
    pub fn incomplete(&self) -> usize {
        self.qsos
            .iter()
            .filter(|q| q.status == Status::NoReceivedNumber)
            .count()
    }

    /// Returns the QSOs whose multiplier could not be resolved.
    pub fn unresolved(&self) -> impl Iterator<Item = (usize, &ContestQso)> {
        self.qsos
            .iter()
            .enumerate()
            .filter(|(_, q)| q.multi == Multi::Unresolved)
    }

    /// Returns how many records were dropped for a given reason.
    pub fn rejected(&self, reason: Rejection) -> usize {
        self.rejections.iter().filter(|(r, _)| *r == reason).count()
    }
}

/// Reads a log, keeps the records that belong to the contest, and scores them.
///
/// When `contest_id` is given, only records tagged with it are considered, which
/// is stricter and safer than inferring membership from mode, band, and date.
pub fn score(
    records: &[Record],
    year: i32,
    import_offset: UtcOffset,
    cty: &CtyDatabase,
    contest_id: Option<&str>,
) -> ContestLog {
    let mut qsos = Vec::new();
    let mut rejections = Vec::new();

    for record in records {
        match extract(record, year, import_offset, cty, contest_id) {
            Ok(qso) => qsos.push(qso),
            Err(reason) => {
                let call = record.field("CALL").unwrap_or("(no call)").to_string();
                rejections.push((reason, call));
            }
        }
    }

    qsos.sort_by_key(|qso| qso.datetime);

    let mut hf_qsos = 0;
    let mut vhf_qsos = 0;
    let mut shf_qsos = 0;
    let mut score = 0;
    let mut districts = BTreeSet::new();
    let mut entities = BTreeSet::new();
    let mut days = BTreeSet::new();

    let mut current_day = None;
    let mut worked_today: HashSet<String> = HashSet::new();

    for qso in &mut qsos {
        let day = qso.datetime.date();
        if current_day != Some(day) {
            current_day = Some(day);
            worked_today.clear();
        }

        if qso.received.is_none() {
            qso.status = Status::NoReceivedNumber;
            continue;
        }

        // The rules count one QSO per station per UTC day regardless of band.
        // Portable designators name the same station, so they are folded away.
        let key = dedup_key(&qso.call);
        if !worked_today.insert(key) {
            qso.status = Status::Duplicate;
            continue;
        }

        qso.status = Status::Counted;
        score += qso.band.class.points();
        match qso.band.class {
            PointClass::Hf => hf_qsos += 1,
            PointClass::Vhf => vhf_qsos += 1,
            PointClass::Shf => shf_qsos += 1,
        }

        match &qso.multi {
            Multi::JaDistrict(district) => {
                districts.insert(*district);
            }
            Multi::Dxcc(prefix) => {
                entities.insert(prefix.clone());
            }
            Multi::Unresolved => {}
        }

        days.insert(day);
    }

    ContestLog {
        year,
        qsos,
        hf_qsos,
        vhf_qsos,
        shf_qsos,
        score,
        districts,
        entities,
        days,
        rejections,
    }
}

/// Turns one ADIF record into a contest QSO, or explains why it is not one.
fn extract(
    record: &Record,
    year: i32,
    import_offset: UtcOffset,
    cty: &CtyDatabase,
    contest_id: Option<&str>,
) -> Result<ContestQso, Rejection> {
    if let Some(wanted) = contest_id {
        let tagged = record.field("CONTEST_ID").unwrap_or_default();
        if !tagged.trim().eq_ignore_ascii_case(wanted) {
            return Err(Rejection::OtherContest);
        }
    }

    let mode = record.field("MODE").unwrap_or_default();
    if !mode.eq_ignore_ascii_case("SSTV") {
        return Err(Rejection::NotSstv);
    }

    let call = record
        .field("CALL")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or(Rejection::Unreadable)?
        .to_ascii_uppercase();

    let frequency = record
        .field("FREQ")
        .and_then(|f| f.trim().parse::<f64>().ok());
    let band = match classify_band(record.field("BAND").unwrap_or_default(), frequency) {
        BandVerdict::Contest(band) => band,
        BandVerdict::Warc => return Err(Rejection::WarcBand),
        BandVerdict::OutOfRange => return Err(Rejection::BandOutOfRange),
    };

    let datetime = parse_datetime(
        record.field("QSO_DATE").unwrap_or_default(),
        record.field("TIME_ON").unwrap_or_default(),
        import_offset,
    )
    .ok_or(Rejection::Unreadable)?;

    if datetime.year() != year || datetime.month() != Month::August {
        return Err(Rejection::OutsideContestPeriod);
    }

    let sent = build_exchange(
        record.field("RST_SENT"),
        record.field("STX").or_else(|| record.field("STX_STRING")),
    )
    .ok_or(Rejection::NoSentNumber)?;

    let received = build_exchange(
        record.field("RST_RCVD"),
        record.field("SRX").or_else(|| record.field("SRX_STRING")),
    );

    let multi = resolve_multi(&call, cty);

    Ok(ContestQso {
        datetime,
        call,
        band,
        sent,
        received,
        multi,
        status: Status::Counted,
    })
}

/// Decides which multiplier a callsign contributes.
fn resolve_multi(callsign: &str, cty: &CtyDatabase) -> Multi {
    let Some(resolution) = cty.lookup(callsign) else {
        return Multi::Unresolved;
    };
    let entity = resolution.entity;

    if entity.primary_prefix == "JA" {
        match ja_district(callsign) {
            Some(district) => Multi::JaDistrict(district),
            None => Multi::Unresolved,
        }
    } else {
        Multi::Dxcc(entity.primary_prefix.clone())
    }
}

/// Reduces a callsign to the station it names, dropping portable designators.
fn dedup_key(callsign: &str) -> String {
    let upper = callsign.to_ascii_uppercase();
    let base = upper
        .split('/')
        .max_by_key(|segment| segment.len())
        .unwrap_or(&upper);
    base.to_string()
}

/// Parses an ADIF date and time, which Wavelog writes as `YYYYMMDD` and `HHMMSS`.
fn parse_datetime(date: &str, time: &str, offset: UtcOffset) -> Option<OffsetDateTime> {
    let date = date.trim();
    let time = time.trim();
    if date.len() != 8 || !date.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    if (time.len() != 4 && time.len() != 6) || !time.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }

    let year = date[0..4].parse().ok()?;
    let month = Month::try_from(date[4..6].parse::<u8>().ok()?).ok()?;
    let day = date[6..8].parse().ok()?;

    let hour = time[0..2].parse().ok()?;
    let minute = time[2..4].parse().ok()?;
    let second = if time.len() == 6 {
        time[4..6].parse().ok()?
    } else {
        0
    };

    let date = Date::from_calendar_date(year, month, day).ok()?;
    let time = Time::from_hms(hour, minute, second).ok()?;

    Some(
        PrimitiveDateTime::new(date, time)
            .assume_offset(offset)
            .to_offset(UtcOffset::UTC),
    )
}

#[cfg(test)]
mod tests {
    use super::{BandVerdict, PointClass, build_exchange, classify_band, dedup_key, ja_district};

    #[test]
    fn classifies_bands() {
        assert!(matches!(
            classify_band("20m", Some(14.330)),
            BandVerdict::Contest(band) if band.label == "14" && band.class == PointClass::Hf
        ));
        assert!(matches!(
            classify_band("80m", Some(3.530)),
            BandVerdict::Contest(band) if band.label == "3.5"
        ));
        assert!(matches!(
            classify_band("80m", Some(3.799)),
            BandVerdict::Contest(band) if band.label == "3.8"
        ));
        assert!(matches!(
            classify_band("70cm", None),
            BandVerdict::Contest(band) if band.class == PointClass::Vhf
        ));
        assert!(matches!(
            classify_band("23cm", None),
            BandVerdict::Contest(band) if band.class == PointClass::Shf
        ));
        assert_eq!(classify_band("30m", None), BandVerdict::Warc);
        assert_eq!(classify_band("17m", None), BandVerdict::Warc);
        assert_eq!(classify_band("12m", None), BandVerdict::Warc);
        assert_eq!(classify_band("160m", None), BandVerdict::OutOfRange);
    }

    #[test]
    fn builds_exchanges() {
        let exchange = build_exchange(Some("595"), Some("20")).expect("must build");
        assert_eq!(exchange.render(), "595020");

        let exchange = build_exchange(Some("595"), Some("1234")).expect("must build");
        assert_eq!(exchange.render(), "5951234");

        // A log that keeps the whole exchange in the report field.
        let exchange = build_exchange(Some("595020"), None).expect("must build");
        assert_eq!(exchange.render(), "595020");

        assert_eq!(build_exchange(Some("595"), None), None);
        assert_eq!(build_exchange(None, None), None);
    }

    #[test]
    fn finds_ja_districts() {
        assert_eq!(ja_district("JL1HIS"), Some('1'));
        assert_eq!(ja_district("JG0ABC"), Some('0'));
        assert_eq!(ja_district("7L4ABC"), Some('1'));
        assert_eq!(ja_district("7N2ABC"), Some('1'));
        assert_eq!(ja_district("7J1ABC"), Some('1'));
        assert_eq!(ja_district("8N1ABC"), Some('1'));
        assert_eq!(ja_district("JL1HIS/3"), Some('1'));
    }

    #[test]
    fn folds_portable_designators_when_deduplicating() {
        assert_eq!(dedup_key("JL1HIS/3"), "JL1HIS");
        assert_eq!(dedup_key("JL1HIS/P"), "JL1HIS");
        assert_eq!(dedup_key("jl1his"), "JL1HIS");
    }
}
