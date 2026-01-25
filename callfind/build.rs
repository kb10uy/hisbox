use std::{
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
        r#"const PREFIXES: &[(crate::prefix::Prefix, usize)] = &["#
    )
    .expect("failed to write");
    for (p, i) in prefixes {
        let prefix_str = match p {
            Prefix::ByOneAll(p1) => format!("crate::prefix::Prefix::ByOneAll({p1})"),
            Prefix::ByTwoRange(p1, (p2s, p2e)) => {
                format!("crate::prefix::Prefix::ByTwoRange({p1}, ({p2s}, {p2e}))")
            }
            Prefix::ByTwoSpecified(p1, p2) => {
                format!("crate::prefix::Prefix::ByTwoSpecified({p1}, {p2})")
            }
            Prefix::ByThreeRange(p1, p2, (p3s, p3e)) => {
                format!("crate::prefix::Prefix::ByThreeRange({p1}, {p2}, ({p3s}, {p3e}))")
            }
        };
        writeln!(writer, r#"    ({prefix_str}, {i}),"#).expect("failed to write");
    }
    writeln!(writer, r#"];"#).expect("failed to write");

    writeln!(writer, r#"const AREA_NAMES: &[&str] = &["#).expect("failed to write");
    for name in area_names {
        writeln!(writer, r#"    "{}","#, name.escape_default()).expect("failed to write");
    }
    writeln!(writer, r#"];"#).expect("failed to write");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Prefix {
    ByOneAll(u8),
    ByTwoRange(u8, (u8, u8)),
    ByTwoSpecified(u8, u8),
    ByThreeRange(u8, u8, (u8, u8)),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PrefixChar {
    Full,
    Range(u8, u8),
}

fn construct_values() -> (Vec<(Prefix, usize)>, Vec<String>) {
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
            entries_second.insert(Prefix::ByOneAll(group_p1), group_area);
        } else {
            for ((_, chunk_tr), entries) in p2_range_chunks {
                let prefix = if entries.len() == 1 {
                    let chunk_p2 = entries[0].0.1;
                    match chunk_tr {
                        PrefixChar::Full => Prefix::ByTwoSpecified(group_p1, chunk_p2),
                        PrefixChar::Range(s3s, s3e) => {
                            Prefix::ByThreeRange(group_p1, chunk_p2, (s3s, s3e))
                        }
                    }
                } else {
                    let p2s = entries.first().expect("must have at least 2 elements").0.1;
                    let p2e = entries.last().expect("must have at least 2 elements").0.1;
                    assert!(chunk_tr == PrefixChar::Full, "chunk range is not full");
                    Prefix::ByTwoRange(group_p1, (p2s, p2e))
                };
                entries_second.insert(prefix, group_area);
            }
        }
    }

    let mut sorted_prefixes: Vec<_> = entries_second.into_iter().collect();
    let mut sorted_areas: Vec<_> = area_names.into_iter().map(|(n, i)| (i, n)).collect();
    sorted_prefixes.sort();
    sorted_areas.sort();

    (
        sorted_prefixes,
        sorted_areas.into_iter().map(|(_, n)| n).collect(),
    )
}
