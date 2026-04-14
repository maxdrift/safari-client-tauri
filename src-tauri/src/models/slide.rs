use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Excluded,
    Fixed,
    Jury,
}

#[allow(dead_code)]
impl Category {
    pub fn from_u8(n: u8) -> Option<Self> {
        match n {
            0 => Some(Self::Excluded),
            1 => Some(Self::Fixed),
            2 => Some(Self::Jury),
            _ => None,
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            Self::Excluded => 0,
            Self::Fixed => 1,
            Self::Jury => 2,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailPaths {
    pub s350: String,
    pub s512: String,
    pub s1024: String,
}

fn default_exif_orientation() -> u8 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideDto {
    pub id: String,
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub category: Category,
    pub subject_id: u32,
    pub transform_id: u8,
    pub thumbnails: ThumbnailPaths,
    /// True until preview files exist; frontend generates them in the background.
    #[serde(default)]
    pub thumbnails_pending: bool,
    /// EXIF orientation tag 1–8 (`1` = normal). Used with `transform_id` for display and thumbnails.
    #[serde(default = "default_exif_orientation")]
    pub exif_orientation: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistedSlide {
    pub id: String,
    pub path: String,
    pub category: Category,
    pub subject_id: u32,
    pub transform_id: u8,
    #[serde(default)]
    pub exif_orientation: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    pub slides: Vec<PersistedSlide>,
}
