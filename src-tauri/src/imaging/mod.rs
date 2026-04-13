pub mod transform;
pub mod thumbnails;

pub use transform::{compose_user_action, UserAction};
pub use thumbnails::{
    generate_thumbnails_for_slide, remove_thumbnails_for_basename, slide_dto_lazy_from_path,
    thumb_paths_for_basename,
};
