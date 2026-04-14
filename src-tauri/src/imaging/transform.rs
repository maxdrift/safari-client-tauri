//! D₄ transforms per PRD §8.6: net transform is H^f ∘ R^k (apply R^k, then H^f).
use image::DynamicImage;

/// Apply canonical transform `transform_id` (0–7) to `img` (original bitmap as loaded).
pub fn apply_transform_id(img: DynamicImage, transform_id: u8) -> DynamicImage {
    let id = transform_id % 8;
    let f = id / 4;
    let k = id % 4;
    let mut out = img;
    for _ in 0..k {
        out = out.rotate90();
    }
    if f == 1 {
        out = out.fliph();
    }
    out
}

#[derive(Debug, Clone, Copy)]
pub enum UserAction {
    RotateCw,
    RotateCcw,
    FlipH,
    FlipV,
}

/// Compose: new transform = T_action ∘ T_current (user action applied to already-transformed view).
pub fn compose_user_action(current_id: u8, action: UserAction) -> u8 {
    let table = compose_table();
    let a = match action {
        UserAction::RotateCw => table.rotate_cw,
        UserAction::RotateCcw => table.rotate_ccw,
        UserAction::FlipH => table.flip_h,
        UserAction::FlipV => table.flip_v,
    };
    a[current_id as usize]
}

struct ComposeTable {
    rotate_cw: [u8; 8],
    rotate_ccw: [u8; 8],
    flip_h: [u8; 8],
    flip_v: [u8; 8],
}

fn compose_table() -> &'static ComposeTable {
    static TABLE: once_cell::sync::OnceCell<ComposeTable> = once_cell::sync::OnceCell::new();
    TABLE.get_or_init(build_compose_table)
}

fn build_compose_table() -> ComposeTable {
    let mut rotate_cw = [0u8; 8];
    let mut rotate_ccw = [0u8; 8];
    let mut flip_h = [0u8; 8];
    let mut flip_v = [0u8; 8];
    for s in 0u8..8 {
        rotate_cw[s as usize] = compose_elementary(s, UserAction::RotateCw);
        rotate_ccw[s as usize] = compose_elementary(s, UserAction::RotateCcw);
        flip_h[s as usize] = compose_elementary(s, UserAction::FlipH);
        flip_v[s as usize] = compose_elementary(s, UserAction::FlipV);
    }
    ComposeTable {
        rotate_cw,
        rotate_ccw,
        flip_h,
        flip_v,
    }
}

fn compose_elementary(state: u8, action: UserAction) -> u8 {
    let orig = test_pattern();
    let after_state = apply_transform_id(orig, state);
    let after_action = match action {
        UserAction::RotateCw => after_state.rotate90(),
        UserAction::RotateCcw => after_state.rotate270(),
        UserAction::FlipH => after_state.fliph(),
        UserAction::FlipV => after_state.flipv(),
    };
    classify(&after_action)
}

fn test_pattern() -> DynamicImage {
    use image::{ImageBuffer, Rgba};
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(2, 2);
    *img.get_pixel_mut(0, 0) = Rgba([1, 0, 0, 255]);
    *img.get_pixel_mut(1, 0) = Rgba([2, 0, 0, 255]);
    *img.get_pixel_mut(0, 1) = Rgba([3, 0, 0, 255]);
    *img.get_pixel_mut(1, 1) = Rgba([4, 0, 0, 255]);
    DynamicImage::ImageRgba8(img)
}

/// Compose pixel transforms: result is `outer ∘ inner` (apply `inner` first, then `outer`).
/// Matches `apply_transform_id(apply_transform_id(img, inner), outer)`.
#[cfg_attr(not(test), allow(dead_code))]
pub fn compose_transform_ids(outer: u8, inner: u8) -> u8 {
    let orig = test_pattern();
    let mid = apply_transform_id(orig, inner);
    let out = apply_transform_id(mid, outer);
    classify(&out)
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn classify_image(img: &DynamicImage) -> u8 {
    classify(img)
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn test_pattern_image() -> DynamicImage {
    test_pattern()
}

fn classify(img: &DynamicImage) -> u8 {
    for id in 0u8..8 {
        let orig = test_pattern();
        let expected = apply_transform_id(orig, id);
        if rgba_equal(img, &expected) {
            return id;
        }
    }
    0
}

fn rgba_equal(a: &DynamicImage, b: &DynamicImage) -> bool {
    let a = a.to_rgba8();
    let b = b.to_rgba8();
    a.pixels().zip(b.pixels()).all(|(p, q)| p == q)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_rotations_is_identity() {
        let mut id = 3u8;
        for _ in 0..4 {
            id = compose_user_action(id, UserAction::RotateCw);
        }
        assert_eq!(id, 3);
        let mut id = 0u8;
        for _ in 0..4 {
            id = compose_user_action(id, UserAction::RotateCw);
        }
        assert_eq!(id, 0);
    }

    #[test]
    fn flip_h_twice_identity() {
        let mut id = 5u8;
        id = compose_user_action(id, UserAction::FlipH);
        id = compose_user_action(id, UserAction::FlipH);
        assert_eq!(id, 5);
    }
}
