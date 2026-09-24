mod cli;
mod config;
mod contest;
mod sheet;

use std::{fs::read_to_string, path::Path};

use adif_reader::read_adi;
use anyhow::{Context, Result, bail};
use clap::Parser;
use callfind::cty::CtyDatabase;
use time::{Month, OffsetDateTime, UtcOffset};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

use crate::{
    cli::{Arguments, SheetEncoding},
    config::Settings,
    contest::{Rejection, score},
};

/// The offset the summary sheet's signature date is written in.
const JST: UtcOffset = match UtcOffset::from_hms(9, 0, 0) {
    Ok(offset) => offset,
    Err(_) => unreachable!(),
};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Arguments::parse();

    let settings_text = read_to_string(&args.settings)
        .with_context(|| format!("cannot read settings {}", args.settings.display()))?;
    let settings: Settings =
        toml::from_str(&settings_text).context("cannot parse the settings file")?;

    let cty_text = read_to_string(&args.cty)
        .with_context(|| format!("cannot read country file {}", args.cty.display()))?;
    let cty = CtyDatabase::parse(&cty_text).context("cannot parse the country file")?;
    info!("{} DXCC entities loaded", cty.len());

    let adif_text = read_to_string(&args.adif_file)
        .with_context(|| format!("cannot read ADIF {}", args.adif_file.display()))?;
    let adif = read_adi(&adif_text, args.lenient_length.unwrap_or_default().into())
        .context("cannot parse the ADIF file")?;
    info!("{} records imported", adif.records().len());

    let now = OffsetDateTime::now_utc().to_offset(JST);
    let year = args.year.unwrap_or_else(|| {
        // The contest runs in August, so before it the target is the previous one.
        if now.month() < Month::August {
            now.year() - 1
        } else {
            now.year()
        }
    });

    if args.contest_id.is_none() {
        let mut tags: Vec<&str> = adif
            .records()
            .iter()
            .filter_map(|r| r.field("CONTEST_ID"))
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();
        tags.sort_unstable();
        tags.dedup();
        if !tags.is_empty() {
            info!(
                "log carries CONTEST_ID {}; pass --contest-id to filter on it",
                tags.join(", ")
            );
        }
    }

    let log = score(
        adif.records(),
        year,
        args.import_offset.unwrap_or_default().into(),
        &cty,
        args.contest_id.as_deref(),
    );

    report(&log);

    if log.qsos.is_empty() {
        bail!("no contest QSO found for {year}; check the year, the mode, and the band");
    }

    let log_sheet = sheet::log_sheet(&log, &settings);
    let summary_sheet = sheet::summary_sheet(&log, &settings, now.date());

    let stem = settings.entry.callsign.replace('/', "_");
    write_sheet(
        &args.out_dir.join(format!("{stem}.txt")),
        &log_sheet,
        args.encoding,
    )?;
    write_sheet(
        &args.out_dir.join(format!("{stem}.sum")),
        &summary_sheet,
        args.encoding,
    )?;

    println!("{}", sheet::breakdown(&log));
    println!(
        "Score {} x Multi {} = {} points",
        log.score,
        log.multi(),
        log.total()
    );

    Ok(())
}

/// Reports what was dropped and what needs a human before submission.
fn report(log: &contest::ContestLog) {
    let dropped = [
        (Rejection::OtherContest, "tagged as another contest"),
        (Rejection::NotSstv, "not SSTV"),
        (Rejection::WarcBand, "WARC band, excluded by the rules"),
        (Rejection::BandOutOfRange, "band outside the contest"),
        (
            Rejection::OutsideContestPeriod,
            "outside the contest period",
        ),
        (Rejection::NoSentNumber, "no contest number sent"),
        (Rejection::Unreadable, "unreadable record"),
    ];

    for (reason, description) in dropped {
        let count = log.rejected(reason);
        if count > 0 {
            info!("{count} record(s) skipped: {description}");
        }
    }

    let duplicates = log.duplicates();
    if duplicates > 0 {
        warn!("{duplicates} QSO(s) marked *DUP*: same station already worked that UTC day");
    }

    let incomplete = log.incomplete();
    if incomplete > 0 {
        warn!("{incomplete} QSO(s) marked *INV*: no contest number received");
    }

    for (index, qso) in log.unresolved() {
        warn!(
            "row {}: cannot resolve a multiplier for {}",
            index + 1,
            qso.call
        );
    }
}

/// Writes one sheet in the requested encoding.
///
/// Line endings are CRLF, because the sheets are Windows documents that MMJASTA
/// wrote through a text-mode stream and the secretariat reads as such.
fn write_sheet(path: &Path, contents: &str, encoding: SheetEncoding) -> Result<()> {
    let contents = contents.replace('\n', "\r\n");

    let bytes = match encoding {
        SheetEncoding::Utf8 => contents.into_bytes(),
        SheetEncoding::ShiftJis => {
            let (encoded, _, had_errors) = encoding_rs::SHIFT_JIS.encode(&contents);
            if had_errors {
                warn!(
                    "{} contains characters Shift_JIS cannot represent",
                    path.display()
                );
            }
            encoded.into_owned()
        }
    };

    std::fs::write(path, bytes).with_context(|| format!("cannot write {}", path.display()))?;
    info!("wrote {}", path.display());
    Ok(())
}
