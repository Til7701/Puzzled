use crate::app::components::polyform_drawing::{
    BORDER_SIZE_RATIO, DrawingSettings, HIGHLIGHT_SIZE_RATIO, PolyformDrawingData,
    PolyformDrawingDataSpecial, PrototileDrawingMode, TRANSPARENT_PROTOTILE_ALPHA, fit_font_size,
};
use adw::gdk::cairo::Context;
use adw::gdk::pango::{Alignment, SCALE};
use adw::gdk::{RGBA, pango};
use adw::prelude::GdkCairoContextExt;
use pangocairo::functions::show_layout;
use puzzled_polyform::{Poly, Polyomino, Prototile, RegularCoord, Square};

pub fn draw_polyomino<const M: usize>(
    cr: &Context,
    pan: &pango::Layout,
    width: i32,
    height: i32,
    polyomino: &Polyomino<PolyformDrawingData>,
    drawing_settings: &DrawingSettings<M>,
) {
    let dim = polyomino.dim();
    let squares = polyomino.cells();

    let cell_width = width as f64 / dim.x() as f64;
    let cell_height = height as f64 / dim.y() as f64;
    pan.set_alignment(Alignment::Center);
    fit_font_size(
        pan,
        polyomino.cells().iter().map(|c| &c.data().text),
        cell_width,
        cell_height,
    );
    for cell in squares.iter() {
        draw_square(cr, pan, cell_width, cell_height, cell, drawing_settings);
    }
}

fn draw_square<const M: usize>(
    cr: &Context,
    pan: &pango::Layout,
    cell_width: f64,
    cell_height: f64,
    cell: &Square<PolyformDrawingData, RegularCoord>,
    drawing_settings: &DrawingSettings<M>,
) {
    let coord = cell.coord();
    let x = coord.x();
    let y = coord.y();
    let cell_x = x as f64 * cell_width;
    let cell_y = y as f64 * cell_height;

    let drawing_data = cell.data();
    let special = match &drawing_data.special {
        PolyformDrawingDataSpecial::Polyomino(s) => s,
        _ => unreachable!(),
    };

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
    cr.rectangle(cell_x, cell_y, cell_width, cell_height);
    cr.fill().expect("Failed to fill");
    // Due to floating point inaccuracies, there might be 2px gaps between cells, so
    // additional rectangles are drawn to fill those gaps if the adjacent cells are filled.
    // This only solves the problem, if the color is not transparent, otherwise there
    // would be visible lines between the cells of the tile.
    if color.alpha() == 1.0 {
        if special.x_pos_neighbour {
            cr.rectangle(cell_x + cell_width - 1.0, cell_y, 2.0, cell_height);
            cr.fill().expect("Failed to fill");
        }
        if special.y_pos_neighbour {
            cr.rectangle(cell_x, cell_y + cell_height - 1.0, cell_width, 2.0);
            cr.fill().expect("Failed to fill");
        }
    }

    // Highlighting and Border
    if drawing_data.drawing_mode != PrototileDrawingMode::Normal {
        let highlight_colors = drawing_settings.highlight_colors;
        if let Some(highlight_color) = highlight_colors(&drawing_data.drawing_mode) {
            draw_outline(
                cr,
                cell_x,
                cell_y,
                cell_width,
                cell_height,
                &highlight_color,
                cell_width * HIGHLIGHT_SIZE_RATIO,
            );
        }
    } else if let Some(border_color) = drawing_settings.border_color {
        draw_outline(
            cr,
            cell_x,
            cell_y,
            cell_width,
            cell_height,
            &border_color,
            cell_width * BORDER_SIZE_RATIO,
        );
    }

    // Text
    if let Some(t) = &drawing_data.text {
        cr.save().expect("Failed to save");

        cr.set_source_color(&drawing_settings.text_color);
        pan.set_text(t);

        let (_, text_height) = pan.size();
        let text_height = text_height as f64 / SCALE as f64;
        let y_offset = (cell_height - text_height) / 2.0;
        cr.move_to(cell_x, cell_y + y_offset);

        show_layout(cr, pan);

        cr.restore().expect("Failed to restore");
    }
}

fn draw_outline(
    cr: &Context,
    cell_x: f64,
    cell_y: f64,
    cell_width: f64,
    cell_height: f64,
    color: &RGBA,
    outline_width: f64,
) {
    let half_outline_width = outline_width / 2.0;
    cr.set_source_color(color);
    cr.set_line_width(outline_width);
    cr.rectangle(
        cell_x + half_outline_width,
        cell_y + half_outline_width,
        cell_width - outline_width,
        cell_height - outline_width,
    );
    cr.stroke().expect("Failed to stroke");
}
