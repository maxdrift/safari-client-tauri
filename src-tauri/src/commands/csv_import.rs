//! Merge rows from an exported Safari Client CSV back onto slides matched by file stem.
use std::collections::HashSet;
use std::fs;
use std::path::Path;

use serde::Serialize;

use crate::commands::export::stem_filename;
use crate::commands::species::load_species_catalog;
use crate::models::{Category, SlideDto};

pub const ERR_CSV_UNKNOWN_SPECIES: &str = "Il CSV contiene un ID specie non presente nel catalogo.";

#[derive(Serialize)]
pub struct CsvImportOutcome {
    pub slides: Vec<SlideDto>,
    pub matched: usize,
}

/// Normalized key for matching CSV `file` column to slide basename (case-insensitive).
fn stem_match_key(cell: &str) -> String {
    stem_filename(cell).to_lowercase()
}

/// Full Safari export: `file;jury;subj_id;name;coeff;version;transform_id` (7+ columns).
/// Also accepts shorter rows (e.g. external sheets): 5 columns = no version/transform, 6 = version only, transform defaults to 0.
fn parse_data_row(parts: &[&str]) -> Option<(u32, u8, bool)> {
    let n = parts.len();
    if n < 5 {
        return None;
    }
    let sid: u32 = parts[2].parse().ok()?;
    let tid: u8 = if n >= 7 {
        parts.get(6).and_then(|s| s.parse::<u8>().ok()).unwrap_or(0) % 8
    } else {
        // 5 or 6 columns: no `transform_id` column (keep current orientation in app).
        0
    };
    let jury_x = parts
        .get(1)
        .map(|s| s.eq_ignore_ascii_case("x"))
        .unwrap_or(false);
    Some((sid, tid, jury_x))
}

pub fn merge_csv_into_slides(path: &Path, mut slides: Vec<SlideDto>) -> Result<CsvImportOutcome, String> {
    let catalog = load_species_catalog()?;
    let valid_ids: HashSet<u32> = catalog.iter().map(|s| s.id).collect();

    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut matched = 0usize;

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(';').map(|s| s.trim()).collect();
        let Some((sid, tid, jury_x)) = parse_data_row(&parts) else {
            continue;
        };
        if sid != 0 && !valid_ids.contains(&sid) {
            return Err(ERR_CSV_UNKNOWN_SPECIES.to_string());
        }
        let category = if jury_x {
            Category::Jury
        } else {
            Category::Fixed
        };

        let csv_key = stem_match_key(parts[0]);
        let has_full_row = parts.len() >= 7;
        for slide in slides.iter_mut() {
            if stem_match_key(&slide.id) == csv_key {
                slide.category = category;
                slide.subject_id = sid;
                if has_full_row {
                    slide.transform_id = tid;
                    slide.thumbnails_pending = true;
                }
                matched += 1;
                break;
            }
        }
    }

    Ok(CsvImportOutcome { slides, matched })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ThumbnailPaths;

    fn thumb() -> ThumbnailPaths {
        ThumbnailPaths {
            s350: String::new(),
            s512: String::new(),
            s1024: String::new(),
        }
    }

    #[test]
    fn merges_row_by_stem_without_extension_in_csv() {
        let dir = tempfile::tempdir().unwrap();
        let csv_path = dir.path().join("scheda.csv");
        fs::write(&csv_path, "fish1;;1;ACCIUGA;6;v0;3\n").unwrap();

        let slides = vec![SlideDto {
            id: "fish1.jpg".into(),
            path: "/tmp/fish1.jpg".into(),
            width: 100,
            height: 100,
            category: Category::Excluded,
            subject_id: 0,
            transform_id: 0,
            thumbnails: thumb(),
            thumbnails_pending: false,
            exif_orientation: 1,
        }];

        let out = merge_csv_into_slides(&csv_path, slides).unwrap();
        assert_eq!(out.matched, 1);
        assert_eq!(out.slides[0].subject_id, 1);
        assert_eq!(out.slides[0].transform_id, 3);
        assert_eq!(out.slides[0].category, Category::Fixed);
        assert!(out.slides[0].thumbnails_pending);
    }

    #[test]
    fn merges_row_when_csv_includes_extension() {
        let dir = tempfile::tempdir().unwrap();
        let csv_path = dir.path().join("scheda.csv");
        fs::write(&csv_path, "fish1.jpg;;1;ACCIUGA;6;v0;2\n").unwrap();

        let slides = vec![SlideDto {
            id: "fish1.jpg".into(),
            path: "/tmp/fish1.jpg".into(),
            width: 100,
            height: 100,
            category: Category::Excluded,
            subject_id: 0,
            transform_id: 0,
            thumbnails: thumb(),
            thumbnails_pending: false,
            exif_orientation: 1,
        }];

        let out = merge_csv_into_slides(&csv_path, slides).unwrap();
        assert_eq!(out.matched, 1);
        assert_eq!(out.slides[0].transform_id, 2);
    }

    /// Five columns (no version / transform) — common in truncated or hand-made CSVs.
    #[test]
    fn merges_five_column_row_like_external_sheet() {
        let dir = tempfile::tempdir().unwrap();
        let csv_path = dir.path().join("scheda.csv");
        fs::write(&csv_path, "DSC_3325;;65;LATTERINO;6\n").unwrap();

        let slides = vec![SlideDto {
            id: "DSC_3325.JPG".into(),
            path: "/tmp/DSC_3325.JPG".into(),
            width: 100,
            height: 100,
            category: Category::Excluded,
            subject_id: 0,
            transform_id: 4,
            thumbnails: thumb(),
            thumbnails_pending: false,
            exif_orientation: 1,
        }];

        let out = merge_csv_into_slides(&csv_path, slides).unwrap();
        assert_eq!(out.matched, 1);
        assert_eq!(out.slides[0].subject_id, 65);
        assert_eq!(out.slides[0].transform_id, 4, "short row must not reset transform");
        assert!(!out.slides[0].thumbnails_pending);
    }

    #[test]
    fn stem_match_is_case_insensitive() {
        let dir = tempfile::tempdir().unwrap();
        let csv_path = dir.path().join("scheda.csv");
        fs::write(&csv_path, "dsc_3325;;1;ACCIUGA;6;v0;0\n").unwrap();

        let slides = vec![SlideDto {
            id: "DSC_3325.JPG".into(),
            path: "/tmp/DSC_3325.JPG".into(),
            width: 100,
            height: 100,
            category: Category::Excluded,
            subject_id: 0,
            transform_id: 0,
            thumbnails: thumb(),
            thumbnails_pending: false,
            exif_orientation: 1,
        }];

        let out = merge_csv_into_slides(&csv_path, slides).unwrap();
        assert_eq!(out.matched, 1);
        assert_eq!(out.slides[0].subject_id, 1);
    }
}
