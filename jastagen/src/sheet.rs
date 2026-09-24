//! Renders the log sheet and the summary sheet.
//!
//! Both reproduce the layout `MMJASTA.EXE` produces, because the contest rules
//! name that program as the way to prepare the submission and the secretariat
//! has received its output for years.

use std::fmt::Write as _;

use time::Date;

use crate::{
    config::{Category, Settings},
    contest::{ContestLog, Multi, Status},
};

/// Renders the log sheet, the `.txt` half of the submission.
pub fn log_sheet(log: &ContestLog, settings: &Settings) -> String {
    let mut out = String::new();

    let _ = writeln!(
        out,
        "JASTA Contest Log. Year:{:04}  Contest Name:{}",
        log.year, settings.contest_name
    );
    let _ = writeln!(out, "Call Sign:{}\n", settings.entry.callsign);
    let _ = writeln!(
        out,
        "Date Time UTC     Band  Station WKD. Sent No. RCVD No. Multi. Point\n"
    );

    for qso in &log.qsos {
        let multi = match qso.status {
            Status::Duplicate => "*DUP*".to_string(),
            Status::NoReceivedNumber => "*INV*".to_string(),
            Status::Counted => qso.multi.render(),
        };
        let received = qso
            .received
            .as_ref()
            .map(|e| e.render())
            .unwrap_or_default();

        let _ = writeln!(
            out,
            "{}/{:02} {:02}:{:02}U{:>11}   {:<13}{:<10}{:<9}{:<8}{}",
            qso.datetime.month() as u8,
            qso.datetime.day(),
            qso.datetime.hour(),
            qso.datetime.minute(),
            qso.band.label,
            qso.call,
            qso.sent.render(),
            received,
            multi,
            qso.points()
        );
    }

    let districts = log.districts.len() as u32;
    let entities = log.entities.len() as u32;

    let _ = writeln!(
        out,
        "-------------------------------------------------------------------"
    );
    let _ = writeln!(
        out,
        "Total                                            JA's area:{districts}"
    );
    let _ = writeln!(
        out,
        "{:2} days                                              DXCC :{:<3} {:9} Points",
        log.days.len(),
        entities,
        log.score
    );
    let _ = writeln!(
        out,
        "Total point {}*({}+{}+{})={}",
        log.score,
        districts,
        entities,
        log.day_multi(),
        log.total()
    );

    let duplicates = log.duplicates();
    let incomplete = log.incomplete();
    if duplicates > 0 || incomplete > 0 {
        let _ = writeln!(out, "\n[Note]");
        if duplicates > 0 {
            let _ = writeln!(out, "*DUP* : Duplicate QSO");
        }
        if incomplete > 0 {
            let _ = writeln!(out, "*INV* : Invalid QSO");
        }
    }

    out
}

/// Renders the summary sheet, the `.sum` half of the submission.
///
/// A domestic entry is written in Japanese and an overseas entry in English,
/// matching the two summaries the contest accepts.
pub fn summary_sheet(log: &ContestLog, settings: &Settings, today: Date) -> String {
    match settings.entry.category {
        Category::Domestic => summary_ja(log, settings, today),
        Category::Overseas => summary_en(log, settings, today),
    }
}

fn summary_ja(log: &ContestLog, settings: &Settings, today: Date) -> String {
    let mut out = String::new();
    let entry = &settings.entry;
    let operator = &settings.operator;

    let districts = log.districts.len() as u32;
    let entities = log.entities.len() as u32;
    let days = log.day_multi();
    let multi = log.multi();

    let _ = writeln!(
        out,
        "\nＪＡＳＴＡ 主催ＳＳＴＶコンテストサマリーシート(様式ＡＳ）\n"
    );
    let _ = writeln!(out, "１．コンテスト名称   : {}", settings.contest_name);
    let _ = writeln!(out, "２．参加部門         : {}", entry.category.as_str());
    let _ = writeln!(
        out,
        "３．初参加の区別     : {}",
        if entry.first_entry {
            "初参加"
        } else {
            "過去にも参加した"
        }
    );
    let _ = writeln!(out, "４．性別             : {}", entry.gender.as_str());
    let _ = writeln!(
        out,
        "５. Ｔシャツのサイズ : {} (アクティブ賞当選時用)",
        entry.tshirt_size.as_str()
    );
    let _ = writeln!(out, "６．コールサイン     : {}", entry.callsign);
    let _ = writeln!(out, "７．住所             : 〒{}", operator.zip);
    let _ = writeln!(out, "                     : {}", operator.address);
    let _ = writeln!(out, "    e-mail           : {}", operator.email);
    let _ = writeln!(out, "８. 氏名             : {}", operator.name);
    let _ = writeln!(out, "９. 無線従事者の資格 : {}", operator.license_class);
    let _ = writeln!(out, "10. 空中線電力       : {}", operator.power);
    let _ = writeln!(out, "11. 結果");
    let _ = writeln!(
        out,
        " (1) 3.5～28MHz帯での交信局数       {:5} x 1 = {} 点",
        log.hf_qsos, log.hf_qsos
    );
    let _ = writeln!(
        out,
        " (2) 50～430MHz帯での交信局数       {:5} x 2 = {} 点",
        log.vhf_qsos,
        log.vhf_qsos * 2
    );
    let _ = writeln!(
        out,
        " (3) 1200MHz帯以上での交信局数      {:5} x 3 = {} 点",
        log.shf_qsos,
        log.shf_qsos * 3
    );
    let _ = writeln!(out, "\n");
    let _ = writeln!(
        out,
        "                        交信点数：({})+({})+({}) = {}",
        log.hf_qsos,
        log.vhf_qsos * 2,
        log.shf_qsos * 3,
        log.score
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        " (4) 交信したJAのエリア数                  = {districts}"
    );
    let _ = writeln!(
        out,
        " (5) 交信したDXCCエンティティ数（JAを除く）= {entities}"
    );
    let _ = writeln!(out, " (6) 運用日数（最大１０）                  = {days}");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "                        マルチ：({districts})+({entities})+({days}) = {multi}"
    );
    let _ = writeln!(out, "\n");
    let _ = writeln!(
        out,
        "   総得点：交信点数×マルチ = {} × {} = {} 点",
        log.score,
        multi,
        log.total()
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "  以上、ここに提出するサマリーは運用した事実と相違ない事を誓います。"
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "      {:04}年 {:02}月 {:02}日",
        today.year(),
        today.month() as u8,
        today.day()
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "***************************************************************************"
    );
    let _ = writeln!(out, "（意見等あればこの欄に記入下さい。）\n\n");
    let _ = writeln!(
        out,
        "---------------------------------------------------------------------------"
    );

    out
}

fn summary_en(log: &ContestLog, settings: &Settings, today: Date) -> String {
    let mut out = String::new();
    let entry = &settings.entry;
    let operator = &settings.operator;

    let districts = log.districts.len() as u32;
    let entities = log.entities.len() as u32;
    let days = log.day_multi();
    let multi = log.multi();

    let _ = writeln!(out, "\n{:04} JASTA SSTV CONTEST SUMMARY SHEET\n", log.year);
    let _ = writeln!(out, "1. Contest Name      : {}", settings.contest_name);
    let _ = writeln!(out, "2. Entry Section     : {}", entry.category.as_str());
    let _ = writeln!(
        out,
        "3. First time entry  : {}",
        if entry.first_entry { "Yes" } else { "No" }
    );
    let _ = writeln!(out, "4. OM or YL(XYL)     : {}", entry.gender.as_str());
    let _ = writeln!(
        out,
        "5. Size of T-shirts  : {} (for an Activity Premium)",
        entry.tshirt_size.as_str()
    );
    let _ = writeln!(out, "6. Callsign          : {}", entry.callsign);
    let _ = writeln!(
        out,
        "7. Address           : Postal Zip Code {}",
        operator.zip
    );
    let _ = writeln!(out, "                     : {}", operator.address);
    let _ = writeln!(out, "    e-mail           : {}", operator.email);
    let _ = writeln!(out, "8. Name              : {}", operator.name);
    let _ = writeln!(out, "9. Licensed Class    : {}", operator.license_class);
    let _ = writeln!(out, "10.Output Power      : {}", operator.power);
    let _ = writeln!(out, "11.Result");
    let _ = writeln!(
        out,
        " (1)Confirmed QSO number of 3.5-28MHz       {:5} x 1 = {} points",
        log.hf_qsos, log.hf_qsos
    );
    let _ = writeln!(
        out,
        " (2)Confirmed QSO number of 50-430MHz       {:5} x 2 = {} points",
        log.vhf_qsos,
        log.vhf_qsos * 2
    );
    let _ = writeln!(
        out,
        " (3)Confirmed QSO number of 1200MHz&UP      {:5} x 3 = {} points",
        log.shf_qsos,
        log.shf_qsos * 3
    );
    let _ = writeln!(out, "\n");
    let _ = writeln!(
        out,
        "            Total QSO points : ({})+({})+({}) = {}",
        log.hf_qsos,
        log.vhf_qsos * 2,
        log.shf_qsos * 3,
        log.score
    );
    let _ = writeln!(out);
    let _ = writeln!(out, " (4)Total Areas of JA's                = {districts}");
    let _ = writeln!(out, " (5)Total Entity of DXCC except JA     = {entities}");
    let _ = writeln!(out, " (6)Total Worked Days(Maximum 10 days) = {days}");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "            Total Multi      : ({districts})+({entities})+({days}) = {multi}"
    );
    let _ = writeln!(out, "\n");
    let _ = writeln!(
        out,
        "   Total Points : {} x {} = {} points",
        log.score,
        multi,
        log.total()
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "I declare my honor that in this contest I have operated"
    );
    let _ = writeln!(
        out,
        "my station within the limitations of my license and have"
    );
    let _ = writeln!(
        out,
        "observed fully the rules and regulations of the contest."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "      {:04}.{:02}.{:02}",
        today.year(),
        today.month() as u8,
        today.day()
    );
    let _ = writeln!(out, "      Singnature _________________");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "***************************************************************************"
    );
    let _ = writeln!(out, "(Remarks)\n\n");
    let _ = writeln!(
        out,
        "---------------------------------------------------------------------------"
    );

    out
}

/// Renders the multiplier breakdown shown on the console, not submitted.
pub fn breakdown(log: &ContestLog) -> String {
    let mut out = String::new();

    let districts: Vec<String> = log.districts.iter().map(|d| format!("JA{d}")).collect();
    let _ = writeln!(
        out,
        "JA districts ({}): {}",
        districts.len(),
        districts.join(" ")
    );

    let entities: Vec<&str> = log.entities.iter().map(String::as_str).collect();
    let _ = writeln!(
        out,
        "DXCC entities ({}): {}",
        entities.len(),
        entities.join(" ")
    );

    let _ = writeln!(
        out,
        "Days operated: {} (multiplier {})",
        log.days.len(),
        log.day_multi()
    );

    for (index, qso) in log.unresolved() {
        if qso.multi == Multi::Unresolved {
            let _ = writeln!(
                out,
                "  unresolved multiplier at row {}: {}",
                index + 1,
                qso.call
            );
        }
    }

    out
}
