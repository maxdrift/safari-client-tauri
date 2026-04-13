use crate::models::SpeciesDto;

const CATALOG: &str = include_str!("../../resources/elenco_pesci_2019.csv");

pub fn load_species_catalog() -> Result<Vec<SpeciesDto>, String> {
    parse_catalog(CATALOG)
}

fn parse_catalog(data: &str) -> Result<Vec<SpeciesDto>, String> {
    let mut out = Vec::new();
    for line in data.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(';').map(|s| s.trim()).collect();
        if parts.len() < 5 {
            continue;
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
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_first_line() {
        let v = parse_catalog("1;ACCIUGA;Engraulis encrasicolus;6;v0").unwrap();
        assert_eq!(v[0].id, 1);
        assert_eq!(v[0].common_name, "ACCIUGA");
    }
}
