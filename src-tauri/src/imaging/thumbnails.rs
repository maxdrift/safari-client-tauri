use std::fs;
use std::path::{Path, PathBuf};

use image::imageops::FilterType;
use image::DynamicImage;

use super::exif::{apply_exif_orientation, dimensions_after_exif, read_exif_orientation};
use crate::imaging::transform::apply_transform_id;
use crate::models::Category;
use crate::models::{SlideDto, ThumbnailPaths};

const WIDTHS: [u32; 3] = [350, 512, 1024];

fn resize_fit_inside(img: &DynamicImage, max_w: u32) -> DynamicImage {
    let (w, h) = (img.width(), img.height());
    if w <= max_w {
        return img.clone();
    }
    let nw = max_w;
    let nh = ((h as u64 * nw as u64) / w as u64).max(1) as u32;
    img.resize(nw, nh, FilterType::Triangle)
}

pub fn cache_dir() -> PathBuf {
    std::env::temp_dir().join("safari-client-previews")
}

fn ensure_cache() -> std::io::Result<()> {
    fs::create_dir_all(cache_dir())
}

pub fn thumb_paths_for_basename(basename: &str, ext: &str) -> ThumbnailPaths {
    let dir = cache_dir();
    let stem = Path::new(basename).file_stem().unwrap_or_default().to_string_lossy();
    let ext_clean = ext.trim_start_matches('.');
    ThumbnailPaths {
        s350: dir.join(format!("{}_350.{}", stem, ext_clean)).to_string_lossy().into_owned(),
        s512: dir.join(format!("{}_512.{}", stem, ext_clean)).to_string_lossy().into_owned(),
        s1024: dir.join(format!("{}_1024.{}", stem, ext_clean)).to_string_lossy().into_owned(),
    }
}

pub fn generate_thumbnails_for_slide(
    original_path: &Path,
    transform_id: u8,
) -> anyhow::Result<ThumbnailPaths> {
    ensure_cache()?;
    let ext = original_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg");
    let basename = original_path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "image".to_string());
    let paths = thumb_paths_for_basename(&basename, ext);

    let img = image::open(original_path)?;
    let exif_o = read_exif_orientation(original_path);
    let img = apply_exif_orientation(img, exif_o);
    let oriented = apply_transform_id(img, transform_id);

    for &w in &WIDTHS {
        let out_path = match w {
            350 => &paths.s350,
            512 => &paths.s512,
            1024 => &paths.s1024,
            _ => unreachable!(),
        };
        let resized = resize_fit_inside(&oriented, w);
        resized.save(out_path)?;
    }

    Ok(paths)
}

pub fn remove_thumbnails_for_basename(basename: &str, ext: &str) {
    let paths = thumb_paths_for_basename(basename, ext);
    for p in [&paths.s350, &paths.s512, &paths.s1024] {
        let _ = fs::remove_file(p);
    }
}

/// Fast path: reads dimensions only; thumbnail files are generated later (see `generate_thumbnails_for_slide`).
pub fn slide_dto_lazy_from_path(
    path: &Path,
    category: Category,
    subject_id: u32,
    transform_id: u8,
) -> anyhow::Result<SlideDto> {
    let exif_o = read_exif_orientation(path);
    let (w, h) = image::image_dimensions(path)?;
    let (w, h) = dimensions_after_exif(w, h, exif_o);
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg");
    let id = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let thumbnails = thumb_paths_for_basename(&id, ext);
    Ok(SlideDto {
        id: id.clone(),
        path: path.to_string_lossy().into_owned(),
        width: w,
        height: h,
        category,
        subject_id,
        transform_id,
        thumbnails,
        thumbnails_pending: true,
        exif_orientation: exif_o,
    })
}
