//! JPEG/TIFF orientation tag (values 1–8) applied to match pixel order before D₄ user transforms.
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use exif::{In, Reader, Tag};
use image::DynamicImage;

use super::transform::{classify_image, compose_transform_ids, test_pattern_image};

/// Read EXIF Orientation (0x0112). Returns `1`–`8`, or `1` if missing / unsupported.
pub fn read_exif_orientation(path: &Path) -> u8 {
    let Ok(file) = File::open(path) else {
        return 1;
    };
    let Ok(exif) = Reader::new().read_from_container(&mut BufReader::new(file)) else {
        return 1;
    };
    let Some(field) = exif.get_field(Tag::Orientation, In::PRIMARY) else {
        return 1;
    };
    match field.value.get_uint(0) {
        Some(v) if (1..=8).contains(&v) => v as u8,
        _ => 1,
    }
}

/// Apply standard EXIF orientation fix (pixel reordering) before user `transform_id`.
pub fn apply_exif_orientation(img: DynamicImage, o: u8) -> DynamicImage {
    match o {
        1 => img,
        2 => img.fliph(),
        3 => img.rotate180(),
        4 => img.flipv(),
        5 => img.rotate90().fliph(),
        6 => img.rotate90(),
        7 => img.rotate270().fliph(),
        8 => img.rotate270(),
        _ => img,
    }
}

/// Map EXIF orientation to our D₄ `transform_id` by classifying the EXIF fix on the test pattern.
#[cfg_attr(not(test), allow(dead_code))]
pub fn exif_orientation_to_d4(orientation: u8) -> u8 {
    let o = orientation.clamp(1, 8);
    let orig = test_pattern_image();
    let fixed = apply_exif_orientation(orig, o);
    classify_image(&fixed)
}

/// Display transform = user ∘ exif (EXIF applied to file pixels first, then user adjustments).
#[cfg_attr(not(test), allow(dead_code))]
pub fn display_transform_id(user_id: u8, exif_orientation: u8) -> u8 {
    let exif_d4 = exif_orientation_to_d4(exif_orientation);
    compose_transform_ids(user_id, exif_d4)
}

/// Logical dimensions after EXIF (for layout); width/height from `image::image_dimensions` are raw storage order.
pub fn dimensions_after_exif(width: u32, height: u32, orientation: u8) -> (u32, u32) {
    match orientation {
        5..=8 => (height, width),
        _ => (width, height),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imaging::transform::apply_transform_id;

    #[test]
    #[ignore]
    fn dump_tables_for_typescript() {
        println!("EXIF_TO_D4:");
        for o in 1..=8 {
            println!("  {o} -> {}", exif_orientation_to_d4(o));
        }
        println!("COMPOSE[outer][inner]:");
        for outer in 0..8 {
            let row: Vec<u8> = (0..8)
                .map(|inner| compose_transform_ids(outer, inner))
                .collect();
            println!("  {outer}: {row:?}");
        }
    }

    #[test]
    fn exif_fix_matches_classified_d4() {
        for o in 1u8..=8 {
            let orig = test_pattern_image();
            let via_exif = apply_exif_orientation(orig.clone(), o);
            let d4 = exif_orientation_to_d4(o);
            let via_d4 = apply_transform_id(test_pattern_image(), d4);
            assert_eq!(via_exif.to_rgba8(), via_d4.to_rgba8(), "orientation {o}");
        }
    }

    #[test]
    fn display_id_matches_pixel_pipeline() {
        for exif_o in 1u8..=8 {
            for uid in 0u8..8 {
                let orig = test_pattern_image();
                let after_exif = apply_exif_orientation(orig, exif_o);
                let expected = apply_transform_id(after_exif, uid);
                let disp = display_transform_id(uid, exif_o);
                let got = apply_transform_id(test_pattern_image(), disp);
                assert_eq!(got.to_rgba8(), expected.to_rgba8(), "exif={exif_o} uid={uid}");
            }
        }
    }
}
