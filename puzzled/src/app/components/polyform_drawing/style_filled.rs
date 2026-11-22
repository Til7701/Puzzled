use crate::app::components::polyform_drawing::drawable::Drawable;
use crate::app::components::polyform_drawing::{
    DrawingSettings, PrototileDrawingMode, TEXT_DEAD_ZONE,
};
use adw::gdk::cairo::Context;
use adw::gdk::pango;
use adw::gdk::pango::Alignment;
use adw::prelude::GdkCairoContextExt;
use pangocairo::functions::show_layout;
use pangocairo::pango::SCALE;

const TRANSPARENT_PROTOTILE_ALPHA: f32 = 0.5;
const RELATIVE_BORDER_SIZE: f64 = 0.03;
const RELATIVE_HIGHLIGHT_SIZE: f64 = 0.08;

pub fn draw_drawable<const M: usize>(
    cr: &Context,
    pan: &pango::Layout,
    x: f64,
    y: f64,
    size: f64,
    drawable: &impl Drawable,
    drawing_settings: &DrawingSettings<M>,
) {
    let drawing_data = drawable.drawing_data();

    // Main Area
    let color = drawing_settings.main_colors
        [drawing_data.main_color_index % drawing_settings.main_colors.len()];
    let color = match drawing_data.drawing_mode {
        PrototileDrawingMode::Normal => color,
        PrototileDrawingMode::Overlapping => color.with_alpha(TRANSPARENT_PROTOTILE_ALPHA),
        PrototileDrawingMode::OutOfBounds => color.with_alpha(TRANSPARENT_PROTOTILE_ALPHA),
        PrototileDrawingMode::Highlighted => color,
    };
    cr.set_source_color(&color);
    // Due to floating point inaccuracies, there might be 2px gaps between cells, so
    // additional rectangles are drawn to fill those gaps if the adjacent cells are filled.
    // This only solves the problem, if the color is not transparent, otherwise there
    // would be visible lines between the cells of the tile.
    if color.alpha() == 1.0 {
        for path in drawable.pixel_line_paths(size) {
            set_path(cr, &path, x, y);
            cr.fill().expect("Failed to fill");
        }
    }
    let shape_path = drawable.normalized_shape_path();
    set_normalized_path(cr, &shape_path, x, y, size);
    cr.clip_preserve();
    cr.fill_preserve().expect("Failed to fill");

    // // Highlighting and Border
    if drawing_data.drawing_mode != PrototileDrawingMode::Normal {
        let highlight_colors = drawing_settings.highlight_colors;
        if let Some(highlight_color) = highlight_colors(&drawing_data.drawing_mode) {
            cr.set_source_color(&highlight_color);
            cr.set_line_width(RELATIVE_HIGHLIGHT_SIZE * size * 2.0);
            cr.stroke().expect("Failed to draw highlight");
        }
    } else if let Some(border_color) = drawing_settings.border_color {
        cr.set_source_color(&border_color);
        cr.set_line_width(RELATIVE_BORDER_SIZE * size * 2.0);
        cr.stroke().expect("Failed to draw border");
    } else {
        cr.new_path();
    }

    // Text
    if let Some(t) = &drawing_data.text {
        cr.save().expect("Failed to save");

        let text_box = drawable.normalized_text_box(TEXT_DEAD_ZONE);
        let converted_text_box = (
            convert_coords(text_box.0, x, y, size),
            convert_coords(text_box.1, 0.0, 0.0, size),
        );

        cr.set_source_color(&drawing_settings.text_color);
        pan.set_text(t);
        pan.set_width(converted_text_box.1.0 as i32);
        pan.set_alignment(Alignment::Center);

        let (_, text_height) = pan.size();
        let text_height = text_height as f64 / SCALE as f64;
        let x_offset = size / 2.0;
        let y_offset = (size - text_height) / 2.0;
        cr.move_to(x + x_offset, y + y_offset);

        show_layout(cr, pan);

        cr.restore().expect("Failed to restore");
    }
    cr.reset_clip();
}

fn set_normalized_path(cr: &Context, path: &[(f64, f64)], x: f64, y: f64, size: f64) {
    cr.new_path();
    let start = convert_coords(path[0], x, y, size);
    cr.move_to(start.0, start.1);
    for v in path.iter().skip(1) {
        let converted = convert_coords(*v, x, y, size);
        cr.line_to(converted.0, converted.1);
    }
    cr.close_path();
}

fn set_path(cr: &Context, path: &[(f64, f64)], x: f64, y: f64) {
    cr.new_path();
    let start = path[0];
    cr.move_to(x + start.0, y + start.1);
    for v in path.iter().skip(1) {
        cr.line_to(x + v.0, y + v.1);
    }
    cr.close_path();
}

fn convert_coords(point: (f64, f64), x: f64, y: f64, size: f64) -> (f64, f64) {
    (x + (point.0 * size), y + (point.1 * size))
}
