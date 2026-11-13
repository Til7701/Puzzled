use crate::app::components::polyform_drawing::{
    DrawingSettings, PolyformDrawingData, PolyformDrawingDataSpecial, PrototileDrawingMode,
    TRANSPARENT_PROTOTILE_ALPHA, fit_font_size,
};
use adw::gdk::cairo::Context;
use adw::gdk::pango;
use adw::gdk::pango::Alignment;
use adw::prelude::GdkCairoContextExt;
use puzzled_polyform::{HexCoord, Hexagon, Poly, Polyhex, Prototile};

pub fn draw_polyhex<const M: usize>(
    cr: &Context,
    pan: &pango::Layout,
    width: i32,
    height: i32,
    polyomino: &Polyhex<PolyformDrawingData>,
    drawing_settings: &DrawingSettings<M>,
) {
    let dim = polyomino.dim();
    let hexagons = polyomino.cells();

    let cell_width = width as f64 / dim.q() as f64;
    let cell_height = height as f64 / dim.r() as f64;
    pan.set_alignment(Alignment::Center);
    fit_font_size(
        pan,
        polyomino.cells().iter().map(|c| &c.data().text),
        cell_width,
        cell_height,
    );
    for cell in hexagons.iter() {
        draw_hexagon(cr, pan, cell_width, cell_height, cell, drawing_settings);
    }
}

fn draw_hexagon<const M: usize>(
    cr: &Context,
    pan: &pango::Layout,
    cell_width: f64,
    cell_height: f64,
    cell: &Hexagon<PolyformDrawingData, HexCoord>,
    drawing_settings: &DrawingSettings<M>,
) {
    let coord = cell.coord();
    let (x, y) = coord.to_pixel();
    println!("{} x {}", x, y);
    let cell_x = x * cell_width;
    let cell_y = y * cell_height;

    let drawing_data = cell.data();
    let special = match &drawing_data.special {
        PolyformDrawingDataSpecial::Polyhex(s) => s,
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
    cr.stroke().expect("Failed to fill");
}
