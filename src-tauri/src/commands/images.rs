use std::path::{Path, PathBuf};

use crate::imaging::{
    compose_user_action, dimensions_after_exif, generate_thumbnails_for_slide,
    read_exif_orientation, remove_thumbnails_for_basename, slide_dto_lazy_from_path,
    thumb_paths_for_basename, UserAction,
};
use crate::models::Category;
use crate::models::{PersistedSlide, SlideDto};

pub fn load_slides_from_paths(paths: Vec<String>) -> Result<Vec<SlideDto>, String> {
    let mut out = Vec::new();
    for p in paths {
        let path = PathBuf::from(&p);
        if !path.is_file() {
            continue;
        }
        let slide = slide_dto_lazy_from_path(&path, Category::Excluded, 0, 0)
            .map_err(|e| e.to_string())?;
        out.push(slide);
    }
    Ok(out)
}

pub fn regenerate_thumbnails(path: String, transform_id: u8) -> Result<crate::models::ThumbnailPaths, String> {
    let path = PathBuf::from(path);
    generate_thumbnails_for_slide(&path, transform_id).map_err(|e| e.to_string())
}

pub fn remove_slide_cache(filename: String) {
    let path = Path::new(&filename);
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("jpg");
    let base = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| filename.clone());
    remove_thumbnails_for_basename(&base, ext);
}

pub fn apply_transform_action(current_id: u8, action: &str) -> Result<u8, String> {
    let a = match action {
        "rotateCw" => UserAction::RotateCw,
        "rotateCcw" => UserAction::RotateCcw,
        "flipH" => UserAction::FlipH,
        "flipV" => UserAction::FlipV,
        _ => return Err("azione sconosciuta".into()),
    };
    Ok(compose_user_action(current_id, a))
}

pub fn ensure_previews_for_persisted(slides: Vec<PersistedSlide>) -> Result<Vec<SlideDto>, String> {
    let mut out = Vec::new();
    for ps in slides {
        let path = PathBuf::from(&ps.path);
        if !path.is_file() {
            continue;
        }
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("jpg");
        let thumbnails = thumb_paths_for_basename(&ps.id, ext);
        let thumbnails_pending = !Path::new(&thumbnails.s350).exists()
            || !Path::new(&thumbnails.s512).exists()
            || !Path::new(&thumbnails.s1024).exists();
        let exif_o = ps
            .exif_orientation
            .filter(|&o| (1..=8).contains(&o))
            .unwrap_or_else(|| read_exif_orientation(&path));
        let (w, h) = image::image_dimensions(&path).map_err(|e| e.to_string())?;
        let (width, height) = dimensions_after_exif(w, h, exif_o);
        out.push(SlideDto {
            id: ps.id,
            path: ps.path,
            width,
            height,
            category: ps.category,
            subject_id: ps.subject_id,
            transform_id: ps.transform_id,
            thumbnails,
            thumbnails_pending,
            exif_orientation: exif_o,
        });
    }
    Ok(out)
}
