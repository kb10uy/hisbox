use std::{
    cmp::Ordering,
    collections::HashMap,
    env::var,
    fs::{File, read_to_string},
    io::{BufWriter, Write},
    path::Path,
};

use itertools::Itertools as _;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=prefixes.tsv");
    let out_dir = var("OUT_DIR").expect("OUT_DIR must be set");

    let (prefixes, area_names) = construct_values();
    let mut writer = BufWriter::new(
        File::create(Path::new(&out_dir).join("prefixes.rs")).expect("failed to open file"),
    );

    writeln!(
        writer,
        r#"pub const PREFIXES: &[(crate::callsign::prefix::Prefix, usize)] = &["#
    )
    .expect("failed to write");
    for (p, i) in prefixes {
        let prefix_str = match p {
            Prefix::OneAll(p1) => format!("crate::callsign::prefix::Prefix::OneAll({p1})"),
            Prefix::TwoRange(p1, (p2s, p2e)) => {
                format!("crate::callsign::prefix::Prefix::TwoRange({p1}, ({p2s}, {p2e}))")
            }
            Prefix::TwoSpecified(p1, p2) => {
                format!("crate::callsign::prefix::Prefix::TwoSpecified({p1}, {p2})")
            }
            Prefix::ThreeRange(p1, p2, (p3s, p3e)) => {
                format!("crate::callsign::prefix::Prefix::ThreeRange({p1}, {p2}, ({p3s}, {p3e}))")
            }
        };
        writeln!(writer, r#"    ({prefix_str}, {i}),"#).expect("failed to write");
    }
    writeln!(writer, r#"];"#).expect("failed to write");

    writeln!(writer, r#"pub const AREA_NAMES: &[&str] = &["#).expect("failed to write");
    for name in area_names {
        writeln!(writer, r#"    "{}","#, name.escape_default()).expect("failed to write");
    }
    writeln!(writer, r#"];"#).expect("failed to write");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Prefix {
    OneAll(u8),
    TwoRange(u8, (u8, u8)),
    TwoSpecified(u8, u8),
    ThreeRange(u8, u8, (u8, u8)),
}

impl Prefix {
    pub fn range_order(&self, other: &Prefix) -> Ordering {
        match (self, other) {
            (Prefix::OneAll(l), Prefix::OneAll(r)) => l.cmp(r),
            (Prefix::OneAll(l), Prefix::TwoRange(r, _)) => l.cmp(r),
            (Prefix::OneAll(l), Prefix::TwoSpecified(r, _)) => l.cmp(r),
            (Prefix::OneAll(l), Prefix::ThreeRange(r, _, _)) => l.cmp(r),
            (Prefix::TwoRange(l1, l2), Prefix::TwoRange(r1, r2)) => {
                l1.cmp(r1).then(l2.0.cmp(&r2.0))
            }
            (Prefix::TwoRange(l1, l2), Prefix::TwoSpecified(r1, r2)) => {
                l1.cmp(r1).then(l2.0.cmp(r2))
            }
            (Prefix::TwoRange(l1, l2), Prefix::ThreeRange(r1, r2, _)) => {
                l1.cmp(r1).then(l2.0.cmp(r2))
            }
            (Prefix::TwoSpecified(l1, l2), Prefix::TwoSpecified(r1, r2)) => {
                l1.cmp(r1).then(l2.cmp(r2))
            }
            (Prefix::TwoSpecified(l1, l2), Prefix::ThreeRange(r1, r2, _)) => {
                l1.cmp(r1).then(l2.cmp(r2))
            }
            (Prefix::ThreeRange(l1, l2, l3), Prefix::ThreeRange(r1, r2, r3)) => {
                l1.cmp(r1).then(l2.cmp(r2)).then(l3.cmp(r3))
            }

            // (Prefix::TwoRange(l, _), Prefix::OneAll(r)) => todo!(),
            // (Prefix::TwoSpecified(_, _), Prefix::OneAll(_)) => todo!(),
            // (Prefix::TwoSpecified(_, _), Prefix::TwoRange(_, _)) => todo!(),
            // (Prefix::ThreeRange(_, _, _), Prefix::OneAll(_)) => todo!(),
            // (Prefix::ThreeRange(_, _, _), Prefix::TwoRange(_, _)) => todo!(),
            // (Prefix::ThreeRange(_, _, _), Prefix::TwoSpecified(_, _)) => todo!(),
            _ => other.range_order(self).reverse(),
        }
    }
}

fn construct_values() -> (Vec<(Prefix, usize)>, Vec<String>) {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum PrefixChar {
        Full,
        Range(u8, u8),
    }

    let tsv = read_to_string("prefixes.tsv").expect("prefixes.tsv must exist");

    let mut area_names = HashMap::new();

    let mut entries_first = HashMap::new();
    for tsv_line in tsv.lines() {
        let tsv_line = tsv_line.trim();
        if tsv_line.is_empty() {
            continue;
        }
        let Some((start, end, area_name)) = tsv_line.split_once('\t').and_then(|(s, rest)| {
            rest.split_once('\t')
                .map(|(e, rest)| (s, e, rest.split_once('\t').map(|(a, _)| a).unwrap_or(rest)))
        }) else {
            panic!("invalid line: {tsv_line}");
        };
        if start.len() != 3 || end.len() != 3 || !start.is_ascii() || !end.is_ascii() {
            panic!("invalid prefix: {tsv_line}");
        }
        if start[..2] != end[..2] {
            panic!("inconsistent prefix: {tsv_line}");
        }

        let area_index = match area_names.get(area_name) {
            Some(i) => *i,
            None => {
                let index = area_names.len();
                area_names.insert(area_name.to_string(), index);
                index
            }
        };

        let start_bytes = start.as_bytes();
        let end_bytes = end.as_bytes();
        let third_range = if start_bytes[2] == b'A' && end_bytes[2] == b'Z' {
            PrefixChar::Full
        } else {
            PrefixChar::Range(start_bytes[2], end_bytes[2])
        };

        entries_first.insert((start_bytes[0], start_bytes[1], area_index), third_range);
    }

    let entries_by_p1_area = entries_first
        .into_iter()
        .into_group_map_by(|r| (r.0.0, r.0.2));
    let mut entries_second = HashMap::new();
    for ((group_p1, group_area), mut group_entries) in entries_by_p1_area {
        group_entries.sort_by_key(|e| e.0.1);

        let p2_range_chunks: Vec<(_, Vec<_>)> = group_entries
            .into_iter()
            .enumerate()
            .chunk_by(|(i, ((_, p2, _), tr))| (*p2 as usize - i, *tr))
            .into_iter()
            .map(|(p2, v)| (p2, v.map(|(_, v)| v).collect()))
            .collect();

        if p2_range_chunks.len() == 1 && p2_range_chunks[0].1.len() == 26 {
            entries_second.insert(Prefix::OneAll(group_p1), group_area);
        } else {
            for ((_, chunk_tr), entries) in p2_range_chunks {
                let prefix = if entries.len() == 1 {
                    let chunk_p2 = entries[0].0.1;
                    match chunk_tr {
                        PrefixChar::Full => Prefix::TwoSpecified(group_p1, chunk_p2),
                        PrefixChar::Range(s3s, s3e) => {
                            Prefix::ThreeRange(group_p1, chunk_p2, (s3s, s3e))
                        }
                    }
                } else {
                    let p2s = entries.first().expect("must have at least 2 elements").0.1;
                    let p2e = entries.last().expect("must have at least 2 elements").0.1;
                    assert!(chunk_tr == PrefixChar::Full, "chunk range is not full");
                    Prefix::TwoRange(group_p1, (p2s, p2e))
                };
                entries_second.insert(prefix, group_area);
            }
        }
    }

    let mut sorted_prefixes: Vec<_> = entries_second.into_iter().collect();
    let mut sorted_areas: Vec<_> = area_names.into_iter().map(|(n, i)| (i, n)).collect();
    sorted_prefixes.sort_by(|l, r| l.0.range_order(&r.0));
    sorted_areas.sort();

    // overwrapping prefix must not exist
    if let Some(window) = sorted_prefixes
        .windows(2)
        .find(|w| w[0].0.range_order(&w[1].0).is_eq())
    {
        panic!(
            "overwrapping prefix found: ({:?}, {:?})",
            window[0].0, window[1].0
        );
    }

    (
        sorted_prefixes,
        sorted_areas.into_iter().map(|(_, n)| n).collect(),
    )
}
