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
    let shape_path = drawable.shape_path();
    let text_box = drawable.text_box(TEXT_DEAD_ZONE);
    let converted_text_box = (
        convert_coords(text_box.0, x, y, size),
        convert_coords(text_box.1, 0.0, 0.0, size),
    );

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
    set_path(cr, &shape_path, x, y, size);
    cr.fill().expect("Failed to fill");

    // // Highlighting and Border
    // if drawing_data.drawing_mode != PrototileDrawingMode::Normal {
    //     let highlight_colors = drawing_settings.highlight_colors;
    //     if let Some(highlight_color) = highlight_colors(&drawing_data.drawing_mode) {
    //         draw_outline(
    //             cr,
    //             cell_x,
    //             cell_y,
    //             cell_width,
    //             cell_height,
    //             &highlight_color,
    //             cell_width * HIGHLIGHT_SIZE_RATIO,
    //         );
    //     }
    // } else if let Some(border_color) = drawing_settings.border_color {
    //     draw_outline(
    //         cr,
    //         cell_x,
    //         cell_y,
    //         cell_width,
    //         cell_height,
    //         &border_color,
    //         cell_width * BORDER_SIZE_RATIO,
    //     );
    // }

    // Text
    if let Some(t) = &drawing_data.text {
        cr.save().expect("Failed to save");

        cr.set_source_color(&drawing_settings.text_color);
        pan.set_text(t);
        pan.set_width(converted_text_box.1.0 as i32);
        pan.set_alignment(Alignment::Center);

        let (text_width, text_height) = pan.size();
        let text_width = text_width as f64 / SCALE as f64;
        let text_height = text_height as f64 / SCALE as f64;
        let x_offset = (size - text_width) / 2.0;
        let y_offset = (size - text_height) / 2.0;
        cr.move_to(x, y + y_offset);

        show_layout(cr, pan);

        cr.restore().expect("Failed to restore");
    }
}

fn set_path(cr: &Context, path: &[(f64, f64)], x: f64, y: f64, size: f64) {
    cr.new_path();
    let start = convert_coords(path[0], x, y, size);
    cr.move_to(start.0, start.1);
    for i in 1..path.len() {
        let converted = convert_coords(path[i], x, y, size);
        cr.line_to(converted.0, converted.1);
    }
    cr.close_path();
}

fn convert_coords(point: (f64, f64), x: f64, y: f64, size: f64) -> (f64, f64) {
    (x + (point.0 * size), y + (point.1 * size))
}
