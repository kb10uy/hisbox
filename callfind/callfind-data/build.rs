use std::{env::var, fs::File, io::BufWriter, path::Path};

use callfind_core::generator::prefixes::{load_prefixes, write_prefixes};

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=data/prefixes.tsv");
    println!("cargo::rerun-if-changed=data/bureau.tsv");
    let out_dir = var("OUT_DIR").expect("OUT_DIR must be set");

    let (prefixes, area_names) = load_prefixes("data/prefixes.tsv").expect("failed to load data");
    let mut writer = BufWriter::new(
        File::create(Path::new(&out_dir).join("generated.rs")).expect("failed to open file"),
    );
    write_prefixes(&mut writer, &prefixes, &area_names).expect("failed to write");
}
