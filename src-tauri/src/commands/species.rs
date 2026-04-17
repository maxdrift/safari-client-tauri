use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::commands::persistence::safari_client_dir;
use crate::models::SpeciesDto;

const DEFAULT_CATALOG: &str = include_str!("../../resources/elenco_pesci_2026.csv");

const CUSTOM_FILENAME: &str = "elenco_pesci_custom.csv";

pub const ERR_CATALOG_DUPLICATE_ID: &str = "Sono presenti ID specie duplicati nel catalogo.";
pub const ERR_CATALOG_EMPTY: &str = "Il catalogo non contiene specie valide.";
pub const ERR_CATALOG_LINE: &str =
    "Riga catalogo non valida (attesi: id;nome;scientifico;coeff;versione).";

fn custom_catalog_path() -> Result<std::path::PathBuf, String> {
    Ok(safari_client_dir()?.join(CUSTOM_FILENAME))
}

/// Loads custom CSV from app data if present and valid; otherwise the embedded default.
pub fn load_species_catalog() -> Result<Vec<SpeciesDto>, String> {
    if let Ok(path) = custom_catalog_path() {
        if path.exists() {
            let data = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            if !data.trim().is_empty() {
                match parse_catalog_strict(&data).and_then(ensure_unique_ids) {
                    Ok(v) => return Ok(v),
                    Err(_) => {
                        // Corrupt custom file: ignore and fall back so the app still starts.
                    }
                }
            }
        }
    }
    parse_catalog_strict(DEFAULT_CATALOG).and_then(ensure_unique_ids)
}

fn parse_catalog_strict(data: &str) -> Result<Vec<SpeciesDto>, String> {
    let mut out = Vec::new();
    for line in data.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(';').map(|s| s.trim()).collect();
        if parts.len() < 5 {
            return Err(ERR_CATALOG_LINE.to_string());
        }
        let id: u32 = parts[0].parse().map_err(|e| format!("id: {e}"))?;
        let coeff: f64 = parts[3].parse().map_err(|e| format!("coeff: {e}"))?;
        out.push(SpeciesDto {
            id,
            common_name: parts[1].to_string(),
            scientific_name: parts[2].to_string(),
            coefficient: coeff,
            version: parts[4].to_string(),
        });
    }
    if out.is_empty() {
        return Err(ERR_CATALOG_EMPTY.to_string());
    }
    Ok(out)
}

fn ensure_unique_ids(rows: Vec<SpeciesDto>) -> Result<Vec<SpeciesDto>, String> {
    let mut seen = HashSet::new();
    for r in &rows {
        if !seen.insert(r.id) {
            return Err(ERR_CATALOG_DUPLICATE_ID.to_string());
        }
    }
    Ok(rows)
}

/// Validates, persists to app data, and returns the catalog.
pub fn import_species_catalog(src: &Path) -> Result<Vec<SpeciesDto>, String> {
    let data = fs::read_to_string(src).map_err(|e| e.to_string())?;
    let rows = parse_catalog_strict(&data).and_then(ensure_unique_ids)?;
    let dest = custom_catalog_path()?;
    fs::write(&dest, data.as_bytes()).map_err(|e| e.to_string())?;
    Ok(rows)
}

pub fn restore_default_species_catalog() -> Result<Vec<SpeciesDto>, String> {
    if let Ok(path) = custom_catalog_path() {
        if path.exists() {
            fs::remove_file(&path).map_err(|e| e.to_string())?;
        }
    }
    parse_catalog_strict(DEFAULT_CATALOG).and_then(ensure_unique_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_first_line() {
        let v = parse_catalog_strict("1;ACCIUGA;Engraulis encrasicolus;6;v0").unwrap();
        assert_eq!(v[0].id, 1);
        assert_eq!(v[0].common_name, "ACCIUGA");
    }

    #[test]
    fn rejects_duplicate_ids() {
        let err =
            parse_catalog_strict("1;A;B;1;v0\n2;C;D;1;v0\n1;E;F;1;v0").and_then(ensure_unique_ids);
        assert!(err.is_err());
    }

    #[test]
    fn rejects_empty_catalog() {
        assert!(parse_catalog_strict("").is_err());
    }
}
