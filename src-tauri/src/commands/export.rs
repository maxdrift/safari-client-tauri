use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::commands::species::load_species_catalog;
use crate::models::Category;
use crate::models::SpeciesDto;
use crate::models::SlideDto;

pub const ERR_DUPLICATE_SPECIES: &str = "Sono presenti specie doppie.";
pub const ERR_MISSING_SPECIES: &str = "Non a tutte le slide è stata assegnata una specie.";

pub fn export_csv(slides: &[SlideDto], dest: &Path) -> Result<(), String> {
    let catalog = load_species_catalog()?;
    let by_id: std::collections::HashMap<u32, &SpeciesDto> =
        catalog.iter().map(|s| (s.id, s)).collect();

    let mut used_subj: HashSet<u32> = HashSet::new();
    let mut rows: Vec<Vec<String>> = Vec::new();

    for slide in slides {
        if slide.category == Category::Excluded {
            continue;
        }
        let sid = slide.subject_id;
        if sid == 0 {
            return Err(ERR_MISSING_SPECIES.to_string());
        }
        if used_subj.contains(&sid) {
            return Err(ERR_DUPLICATE_SPECIES.to_string());
        }
        used_subj.insert(sid);
        let sp = by_id.get(&sid).ok_or_else(|| format!("specie sconosciuta: {sid}"))?;
        let file_stem = stem_filename(&slide.id);
        let jury = if slide.category == Category::Jury {
            "X"
        } else {
            ""
        };
        rows.push(vec![
            file_stem,
            jury.to_string(),
            sp.id.to_string(),
            sp.common_name.clone(),
            format_coeff(sp.coefficient),
            sp.version.clone(),
            slide.transform_id.to_string(),
        ]);
    }

    let mut out = String::new();
    for r in rows {
        out.push_str(&r.join(";"));
        out.push('\n');
    }
    fs::write(dest, out.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

pub(crate) fn stem_filename(filename: &str) -> String {
    Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(filename)
        .to_string()
}

fn format_coeff(c: f64) -> String {
    if (c - c.round()).abs() < 1e-9 {
        format!("{}", c as i64)
    } else {
        format!("{c}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ThumbnailPaths;

    fn sample_slide(
        id: &str,
        cat: Category,
        subject_id: u32,
        transform_id: u8,
    ) -> SlideDto {
        SlideDto {
            id: id.to_string(),
            path: "/tmp/x".into(),
            width: 1,
            height: 1,
            category: cat,
            subject_id,
            transform_id,
            thumbnails: ThumbnailPaths {
                s350: "".into(),
                s512: "".into(),
                s1024: "".into(),
            },
            thumbnails_pending: false,
            exif_orientation: 1,
        }
    }

    #[test]
    fn duplicate_species_errors() {
        let slides = vec![
            sample_slide("a.jpg", Category::Jury, 1, 0),
            sample_slide("b.jpg", Category::Fixed, 1, 0),
        ];
        let r = export_csv(&slides, std::path::Path::new("/tmp/out.csv"));
        assert_eq!(r.unwrap_err(), ERR_DUPLICATE_SPECIES);
    }

    #[test]
    fn missing_species_errors() {
        let slides = vec![sample_slide("a.jpg", Category::Jury, 0, 0)];
        let r = export_csv(&slides, std::path::Path::new("/tmp/out.csv"));
        assert_eq!(r.unwrap_err(), ERR_MISSING_SPECIES);
    }
}
