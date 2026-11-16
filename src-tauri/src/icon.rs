//! The icon next to the clock: a chat bubble with a bar inside. The bar is
//! drawn anew whenever the number changes, so the icon itself shows how full
//! the limit is.
//!
//! On macOS the picture is a "template": only its shape matters and the
//! system paints it in the same grey as the other menu bar icons, light or
//! dark as needed. On Windows it is drawn in colour.

use tauri::image::Image;
use tiny_skia::{
    BlendMode, Color, FillRule, LineCap, LineJoin, Paint, Path, PathBuilder, Pixmap, Stroke, Transform,
};

/// Side of the picture in pixels; the system scales it down to the bar height.
const SIZE: u32 = 72;
/// The drawing is designed on a 1024 box; this part of it is the icon.
const BOX_LEFT: f32 = 142.0;
const BOX_TOP: f32 = 157.0;
const BOX_SIDE: f32 = 740.0;
/// How far a quarter-circle's control points sit from its ends.
const ROUND: f32 = 0.5523;

/// The bubble outline: a rounded rectangle with a tail at the lower left.
fn bubble() -> Option<Path> {
    let (left, top, right, bottom, r) = (182.0, 222.0, 842.0, 692.0, 130.0);
    let k = r * (1.0 - ROUND);
    let mut p = PathBuilder::new();
    p.move_to(left + r, top);
    p.line_to(right - r, top);
    p.cubic_to(right - k, top, right, top + k, right, top + r);
    p.line_to(right, bottom - r);
    p.cubic_to(right, bottom - k, right - k, bottom, right - r, bottom);
    p.line_to(506.0, bottom);
    p.line_to(352.0, 832.0);
    p.line_to(352.0, bottom);
    p.line_to(left + r, bottom);
    p.cubic_to(left + k, bottom, left, bottom - k, left, bottom - r);
    p.line_to(left, top + r);
    p.cubic_to(left, top + k, left + k, top, left + r, top);
    p.close();
    p.finish()
}

/// A bar with fully rounded ends.
fn pill(x: f32, y: f32, width: f32, height: f32) -> Option<Path> {
    let r = height / 2.0;
    let width = width.max(height); // never thinner than a dot
    let k = r * (1.0 - ROUND);
    let mut p = PathBuilder::new();
    p.move_to(x + r, y);
    p.line_to(x + width - r, y);
    p.cubic_to(x + width - k, y, x + width, y + k, x + width, y + r);
    p.cubic_to(x + width, y + height - k, x + width - k, y + height, x + width - r, y + height);
    p.line_to(x + r, y + height);
    p.cubic_to(x + k, y + height, x, y + height - k, x, y + r);
    p.cubic_to(x, y + k, x + k, y, x + r, y);
    p.close();
    p.finish()
}

fn paint(color: Color) -> Paint<'static> {
    let mut paint = Paint::default();
    paint.set_color(color);
    paint.anti_alias = true;
    paint
}

/// Draw the icon. `percent` (0..100) is how much of the bar is filled;
/// None leaves the bar empty. `template` asks for the one-colour version.
pub fn tray_image(percent: Option<f64>, template: bool) -> Image<'static> {
    let mut canvas = Pixmap::new(SIZE, SIZE).expect("icon canvas");
    let scale = SIZE as f32 / BOX_SIDE;
    let place = Transform::from_translate(-BOX_LEFT, -BOX_TOP).post_scale(scale, scale);

    let (ink, track, fill) = if template {
        // Only the opacity matters; macOS supplies the colour.
        (
            Color::from_rgba8(0, 0, 0, 255),
            Color::from_rgba8(0, 0, 0, 80),
            Color::from_rgba8(0, 0, 0, 255),
        )
    } else {
        (
            Color::from_rgba8(242, 244, 248, 255),
            Color::from_rgba8(242, 244, 248, 80),
            Color::from_rgba8(217, 119, 87, 255),
        )
    };

    if let Some(outline) = bubble() {
        let stroke = Stroke {
            width: 62.0,
            line_join: LineJoin::Round,
            line_cap: LineCap::Round,
            ..Stroke::default()
        };
        canvas.stroke_path(&outline, &paint(ink), &stroke, place, None);
    }
    // The bar is much thicker than in the big logo so its level reads at 18 points.
    let (x, y, width, height) = (282.0, 375.0, 460.0, 164.0);
    if let Some(rail) = pill(x, y, width, height) {
        canvas.fill_path(&rail, &paint(track), FillRule::Winding, place, None);
    }
    let share = (percent.unwrap_or(0.0) / 100.0).clamp(0.0, 1.0) as f32;
    if share > 0.0 {
        if let Some(bar) = pill(x, y, width * share, height) {
            canvas.fill_path(&bar, &paint(fill), FillRule::Winding, place, None);
        }
    }

    // tiny-skia keeps colours pre-multiplied by opacity; the system wants them plain.
    let mut rgba = Vec::with_capacity((SIZE * SIZE * 4) as usize);
    for pixel in canvas.pixels() {
        let plain = pixel.demultiply();
        rgba.extend_from_slice(&[plain.red(), plain.green(), plain.blue(), plain.alpha()]);
    }
    Image::new_owned(rgba, SIZE, SIZE)
}

/// "Kvadrik", the square fellow in sunglasses, for the macOS menu bar.
/// One colour (a template): the glasses, smile and charge cells are cut out
/// of the body, so the menu bar shows through them.
pub fn menu_bar_character() -> Image<'static> {
    let (width, height) = (66u32, 72u32);
    let mut canvas = Pixmap::new(width, height).expect("icon canvas");
    // Drawn on a 1024 box (x 244..780, y 214..798); fit it into the picture.
    let scale = 68.0 / 584.0;
    let place = Transform::from_translate(-244.0, -214.0)
        .post_scale(scale, scale)
        .post_translate(1.8, 2.0);
    let solid = paint(Color::from_rgba8(0, 0, 0, 255));
    let mut cut = paint(Color::from_rgba8(0, 0, 0, 255));
    cut.blend_mode = BlendMode::Clear;

    let rect = |x: f32, y: f32, w: f32, h: f32| -> Option<Path> {
        let mut p = PathBuilder::new();
        p.move_to(x, y);
        p.line_to(x + w, y);
        p.line_to(x + w, y + h);
        p.line_to(x, y + h);
        p.close();
        p.finish()
    };
    let polygon = |points: &[(f32, f32)]| -> Option<Path> {
        let mut p = PathBuilder::new();
        for (index, (x, y)) in points.iter().enumerate() {
            if index == 0 { p.move_to(*x, *y) } else { p.line_to(*x, *y) }
        }
        p.close();
        p.finish()
    };
    let line = |from: (f32, f32), to: (f32, f32)| -> Option<Path> {
        let mut p = PathBuilder::new();
        p.move_to(from.0, from.1);
        p.line_to(to.0, to.1);
        p.finish()
    };
    let round_stroke = |width: f32| Stroke {
        width,
        line_cap: LineCap::Round,
        line_join: LineJoin::Round,
        ..Stroke::default()
    };

    // Body, antenna, arms and legs.
    for piece in [
        rect(292.0, 330.0, 440.0, 380.0),
        rect(492.0, 262.0, 40.0, 72.0),
        rect(468.0, 214.0, 88.0, 54.0),
        rect(352.0, 706.0, 72.0, 92.0),
        rect(600.0, 706.0, 72.0, 92.0),
        rect(244.0, 548.0, 52.0, 74.0),
        rect(728.0, 548.0, 52.0, 74.0),
    ]
    .into_iter()
    .flatten()
    {
        canvas.fill_path(&piece, &solid, FillRule::Winding, place, None);
    }

    // Sunglasses: the bridge and two lenses are holes...
    for hole in [
        rect(312.0, 418.0, 400.0, 28.0),
        polygon(&[(312.0, 426.0), (494.0, 426.0), (478.0, 534.0), (344.0, 534.0)]),
        polygon(&[(712.0, 426.0), (530.0, 426.0), (546.0, 534.0), (680.0, 534.0)]),
    ]
    .into_iter()
    .flatten()
    {
        canvas.fill_path(&hole, &cut, FillRule::Winding, place, None);
    }
    // ...with a glint left in each lens.
    for glint in [line((396.0, 506.0), (426.0, 452.0)), line((612.0, 506.0), (642.0, 452.0))]
        .into_iter()
        .flatten()
    {
        canvas.stroke_path(&glint, &solid, &round_stroke(22.0), place, None);
    }

    // The smile.
    let mut smile = PathBuilder::new();
    smile.move_to(446.0, 584.0);
    smile.quad_to(518.0, 630.0, 590.0, 576.0);
    if let Some(smile) = smile.finish() {
        canvas.stroke_path(&smile, &cut, &round_stroke(26.0), place, None);
    }

    // Three charge cells on the belly; the fourth is spent.
    for index in 0..3 {
        if let Some(cell) = rect(372.0 + index as f32 * 74.0, 636.0, 58.0, 36.0) {
            canvas.fill_path(&cell, &cut, FillRule::Winding, place, None);
        }
    }

    let mut rgba = Vec::with_capacity((width * height * 4) as usize);
    for pixel in canvas.pixels() {
        let plain = pixel.demultiply();
        rgba.extend_from_slice(&[plain.red(), plain.green(), plain.blue(), plain.alpha()]);
    }
    Image::new_owned(rgba, width, height)
}

/// The letters "AI" for the macOS menu bar, drawn bold. Kept for going back.
/// The system's own
/// text there cannot be made heavier, so the name is a picture (a template,
/// painted by macOS in the menu bar colour) and only the number is text.
#[allow(dead_code)]
pub fn menu_bar_name() -> Image<'static> {
    // Four pixels per point: the bar is 18 points high, capitals about 10.5.
    let (width, height) = (66u32, 72u32);
    let (top, bottom) = (15.0f32, 57.0f32);
    let mut canvas = Pixmap::new(width, height).expect("icon canvas");
    let black = paint(Color::from_rgba8(0, 0, 0, 255));

    // "A": the outer shape with the feet cut flat, then the triangle inside removed.
    let mut a = PathBuilder::new();
    for (index, (x, y)) in [
        (1.0, bottom), (17.0, top), (28.0, top), (44.0, bottom),
        (33.2, bottom), (30.4, 48.5), (14.6, 48.5), (11.8, bottom),
    ]
    .into_iter()
    .enumerate()
    {
        if index == 0 { a.move_to(x, y) } else { a.line_to(x, y) }
    }
    a.close();
    a.move_to(17.4, 40.0);
    a.line_to(27.6, 40.0);
    a.line_to(22.5, 24.6);
    a.close();
    if let Some(shape) = a.finish() {
        canvas.fill_path(&shape, &black, FillRule::EvenOdd, Transform::identity(), None);
    }

    // "I": one upright bar.
    let mut i = PathBuilder::new();
    i.move_to(50.0, top);
    i.line_to(60.5, top);
    i.line_to(60.5, bottom);
    i.line_to(50.0, bottom);
    i.close();
    if let Some(shape) = i.finish() {
        canvas.fill_path(&shape, &black, FillRule::Winding, Transform::identity(), None);
    }

    let mut rgba = Vec::with_capacity((width * height * 4) as usize);
    for pixel in canvas.pixels() {
        let plain = pixel.demultiply();
        rgba.extend_from_slice(&[plain.red(), plain.green(), plain.blue(), plain.alpha()]);
    }
    Image::new_owned(rgba, width, height)
}

/// Kvadrik in colour for the Windows tray, where no text can stand next to
/// the icon. The four cells on his belly are the charge: each lit cell is a
/// quarter of the limit still left. `left` is None while nothing is known,
/// then all cells stay dark.
pub fn tray_character(left: Option<f64>) -> Image<'static> {
    let side = 64u32;
    let mut canvas = Pixmap::new(side, side).expect("icon canvas");
    // Drawn on a 1024 box (x 232..792, y 202..810 with the outline); fit it in.
    let scale = 63.0 / 608.0;
    let place = Transform::from_translate(-512.0, -506.0)
        .post_scale(scale, scale)
        .post_translate(side as f32 / 2.0, side as f32 / 2.0);

    let outline = paint(Color::from_rgba8(27, 27, 31, 255));
    let body = paint(Color::from_rgba8(217, 119, 87, 255));
    let ink = paint(Color::from_rgba8(27, 27, 31, 255));
    let white = paint(Color::from_rgba8(255, 255, 255, 255));
    let lit = paint(Color::from_rgba8(246, 240, 228, 255));
    let spent = paint(Color::from_rgba8(150, 70, 48, 255));

    let rect = |x: f32, y: f32, w: f32, h: f32| -> Option<Path> {
        let mut p = PathBuilder::new();
        p.move_to(x, y);
        p.line_to(x + w, y);
        p.line_to(x + w, y + h);
        p.line_to(x, y + h);
        p.close();
        p.finish()
    };
    let polygon = |points: &[(f32, f32)]| -> Option<Path> {
        let mut p = PathBuilder::new();
        for (index, (x, y)) in points.iter().enumerate() {
            if index == 0 { p.move_to(*x, *y) } else { p.line_to(*x, *y) }
        }
        p.close();
        p.finish()
    };
    let stroke = |width: f32| Stroke {
        width,
        line_cap: LineCap::Round,
        line_join: LineJoin::Round,
        ..Stroke::default()
    };

    let pieces: Vec<Path> = [
        rect(292.0, 330.0, 440.0, 380.0),
        rect(492.0, 262.0, 40.0, 72.0),
        rect(468.0, 214.0, 88.0, 54.0),
        rect(352.0, 706.0, 72.0, 92.0),
        rect(600.0, 706.0, 72.0, 92.0),
        rect(244.0, 548.0, 52.0, 74.0),
        rect(728.0, 548.0, 52.0, 74.0),
    ]
    .into_iter()
    .flatten()
    .collect();
    // A dark rim first, so he stands out on a light taskbar too.
    for piece in &pieces {
        canvas.stroke_path(piece, &outline, &stroke(30.0), place, None);
    }
    for piece in &pieces {
        canvas.fill_path(piece, &body, FillRule::Winding, place, None);
    }

    // Sunglasses with a glint.
    for glass in [
        rect(312.0, 418.0, 400.0, 28.0),
        polygon(&[(312.0, 426.0), (494.0, 426.0), (478.0, 534.0), (344.0, 534.0)]),
        polygon(&[(712.0, 426.0), (530.0, 426.0), (546.0, 534.0), (680.0, 534.0)]),
    ]
    .into_iter()
    .flatten()
    {
        canvas.fill_path(&glass, &ink, FillRule::Winding, place, None);
    }
    for (from, to) in [((396.0, 506.0), (426.0, 452.0)), ((612.0, 506.0), (642.0, 452.0))] {
        let mut p = PathBuilder::new();
        p.move_to(from.0, from.1);
        p.line_to(to.0, to.1);
        if let Some(glint) = p.finish() {
            canvas.stroke_path(&glint, &white, &stroke(18.0), place, None);
        }
    }

    // A small smile, higher than in the big logo to leave room for the cells.
    let mut smile = PathBuilder::new();
    smile.move_to(462.0, 566.0);
    smile.quad_to(518.0, 600.0, 574.0, 562.0);
    if let Some(smile) = smile.finish() {
        canvas.stroke_path(&smile, &ink, &stroke(20.0), place, None);
    }

    // The charge: four big cells, lit from the left for what is still left.
    let quarters = match left {
        Some(left) if left > 0.0 => ((left / 25.0).ceil() as usize).min(4),
        _ => 0,
    };
    for index in 0..4 {
        let fill = if index < quarters { &lit } else { &spent };
        if let Some(cell) = rect(318.0 + index as f32 * 100.0, 616.0, 88.0, 74.0) {
            canvas.fill_path(&cell, fill, FillRule::Winding, place, None);
        }
    }

    let mut rgba = Vec::with_capacity((side * side * 4) as usize);
    for pixel in canvas.pixels() {
        let plain = pixel.demultiply();
        rgba.extend_from_slice(&[plain.red(), plain.green(), plain.blue(), plain.alpha()]);
    }
    Image::new_owned(rgba, side, side)
}
