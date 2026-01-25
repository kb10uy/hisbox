use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, Read, stdin, stdout},
    path::PathBuf,
};

use adif_reader::{LengthMode, read_adi, read_adx};
use anyhow::Result;
use clap::{Parser, ValueEnum};
use serde_json::json;

fn main() -> Result<()> {
    let args = Arguments::parse();

    let (mut reader, extension): (BufReader<Box<dyn Read>>, _) = match args.file {
        Some(p) => {
            let ext = p.extension().map(|e| e.to_string_lossy().to_lowercase());
            (BufReader::new(Box::new(File::open(p)?)), ext)
        }
        None => (BufReader::new(Box::new(stdin().lock())), None),
    };

    let mut adif_text = String::new();
    reader.read_to_string(&mut adif_text)?;

    let adif = if args.use_adx || extension.is_some_and(|e| e == "adx") {
        read_adx(&adif_text)?
    } else {
        read_adi(&adif_text, args.lenient_length.unwrap_or_default().into())?
    };

    let field_sorted_records: Vec<_> = adif
        .records()
        .iter()
        .map(|r| r.fields().iter().collect::<BTreeMap<_, _>>())
        .collect();
    let adif_json = json!({
        "preamble": adif.preamble(),
        "headers": adif.headers().iter().collect::<BTreeMap<_, _>>(),
        "records": field_sorted_records,
    });

    let stdout = stdout().lock();
    serde_json::to_writer(stdout, &adif_json)?;
    Ok(())
}

/// ADIF to JSON converter
/// Output JSON is an object that has `headers` and `records`.
///
/// `headers` field
#[derive(Debug, Parser)]
#[command(version, author, about, long_about)]
struct Arguments {
    /// Input file. (stdin if absent)
    file: Option<PathBuf>,

    /// Parse input as ADX.
    /// When input file is specified, it will be suggested by extension.
    #[clap(short = 'x', long = "adx")]
    use_adx: bool,

    /// Enable lenient length count for ADI file.
    /// Pedantic ADI file must not contain non-ASCII characters.
    #[clap(short, long = "lenient")]
    pub lenient_length: Option<LenientMode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum LenientMode {
    /// Count by bytes.
    #[default]
    Bytes,

    /// Count by codepoints.
    Codepoints,

    /// Count by grapheme clusters.
    Graphemes,
}

impl From<LenientMode> for LengthMode {
    fn from(value: LenientMode) -> Self {
        match value {
            LenientMode::Bytes => LengthMode::Bytes,
            LenientMode::Codepoints => LengthMode::Codepoints,
            LenientMode::Graphemes => LengthMode::Graphemes,
        }
    }
}
