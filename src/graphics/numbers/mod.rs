use std::collections::HashMap;
use macroquad::prelude::*;
use chrono::{NaiveDate, Local};
use chrono::Datelike;
use crate::calculations::HashDiagram;
use super::super::calculations;
//let mut diagram: HashMap<NaiveDate, HashDiagram> = HashMap::new();
pub fn draw_all_numbers(screen_w: f32, screen_h: f32, interval: f32, diagram: &HashMap<NaiveDate, HashDiagram>, sorted_diagram: &Vec<calculations::Diagram>) {
    //important variables
    let font_size: f32 = 22.0 / 30.0 * interval;
    let text_offset_y: f32 = -8.0;
    let text_offset_x: f32 = 1.0;

    let color = WHITE;
    //diana's color
    //let color = Color::from_hex(0x37cca4);

    //draw count numbers (vertical)
    if interval >= 30.0 { 
        draw_count_numbers(screen_h, interval, font_size, text_offset_x, text_offset_y, &color);
    }
    //draw days (horizontal)
    if interval >= 30.0 {
        draw_days(screen_w, screen_h, interval, font_size, text_offset_x, text_offset_y, sorted_diagram, &color);
    }

}
//draw count numbers (vertical)
pub fn draw_count_numbers(screen_h: f32, interval: f32, font_size: f32, text_offset_x: f32, text_offset_y: f32, color: &Color) {
    let mut n: f32 = 0.0;
    while interval * n < screen_h {
        draw_text(format!("{: >3}", n), 0.0 + text_offset_x, screen_h - interval * n + text_offset_y, font_size, *color);
        n += 1.0;
    }
}
//draw days (horizontal)
pub fn draw_days(screen_w: f32, screen_h: f32, interval: f32, font_size: f32, text_offset_x: f32, text_offset_y: f32, sorted_diagram: &Vec<calculations::Diagram>, color: &Color) {
    //simple draw days
    for (n, i) in sorted_diagram.iter().enumerate() {
        if i.date.day() != 1 {
            draw_text(format!("{}", i.date.day()), interval * n as f32 + interval + text_offset_x, screen_h + text_offset_y, font_size, *color);
        } else {
            draw_text(format!(".{:0>2}", i.date.month()), interval * n as f32 + interval + text_offset_x, screen_h + text_offset_y, font_size, *color);
        }
    }
}
