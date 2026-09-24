# jastagen

Generates the log sheet and summary sheet for the JASTA SSTV Activity Contest
from an ADIF log.

The contest rules name `MMJASTA.EXE` as the way to prepare the submission, so
both sheets reproduce its layout byte for byte. That program is no longer
distributed with MMSSTV and the JASTA site currently offers no template, which
is why this exists.

## Usage

```
jastagen <ADIF_FILE> --settings jasta.toml --cty cty.dat
```

Writes `<callsign>.txt` (log sheet) and `<callsign>.sum` (summary sheet) into
`--out-dir`. Both are CRLF, UTF-8 by default; pass `--encoding sjis` for the
Shift_JIS that MMJASTA wrote.

`--cty` takes an AD1C `cty.dat` country file, which resolves DXCC entities. A
recent copy ships with Turbo HAMLOG and with most contest loggers. The six WAE
entities it marks with `*`, such as Sicily and Shetland, are folded into their
parent DXCC entity because the contest counts DXCC.

`--year` defaults to the current year, or the previous one before August.
`--import-offset` defaults to UTC, which is what Wavelog exports.

Copy `jasta.example.toml` for the settings file. Everything in it is reproduced
on the summary sheet verbatim; the log supplies the rest. Keep the filled-in copy
under `assets/`, where `*.toml` is ignored, because it carries a postal address.

## What counts

A record enters the contest log when it is SSTV, on a contest band, inside
August of the contest year, and carries a sent contest number. Everything
dropped is reported on stderr with a reason.

When the log tags contest QSOs with `CONTEST_ID`, as Wavelog does, prefer
`--contest-id JASTA-SSTV`. It is stricter than inferring membership, because a
casual SSTV QSO made in August with a serial in it would otherwise be counted.
The tags present in the log are reported when the option is not given.

The sent number is read from `STX`, then `STX_STRING`, and the received one from
`SRX`, then `SRX_STRING`. A log that keeps the whole exchange in `RST_SENT`, such
as `595020`, is split instead. Serials below 1000 are padded to three digits.

A QSO scores unless the same station was already worked that UTC day, or no
contest number was received. Those are marked `*DUP*` and `*INV*` on the log
sheet, as MMJASTA marks them.

Points are 1 for 3.5 to 28 MHz, 2 for 50 to 430 MHz, and 3 for 1200 MHz and up.
The multiplier is the number of Japanese districts, plus DXCC entities other
than Japan, plus operating days capped at ten.

## Where this departs from MMJASTA

The 47th (2026) rules moved on from what MMJASTA implements:

- **WARC bands are excluded.** MMJASTA admits everything at 3.5 MHz and above,
  so it scores 10, 18, and 24 MHz. They are dropped here.
- **A QSO with a station that sent no self-portrait counts.** MMJASTA voids any
  QSO whose remarks contain `NOF` or `NOFACE`. That rule is gone, so remarks are
  not inspected at all.

Two MMJASTA behaviours are deliberately not reproduced:

- MMJASTA converts every timestamp from JST unconditionally, ignoring the log's
  own timezone setting. Here the offset is `--import-offset` and defaults to UTC.
- MMJASTA keys the duplicate check on the logged callsign, so `JA1ABC` and
  `JA1ABC/3` are two stations. Here portable designators are folded away, because
  the rules count the station.

The Japanese district comes from the callsign, with 7K through 7N treated as
district 1 as the rules state.
