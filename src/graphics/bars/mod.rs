use macroquad::prelude::*;
use super::super::calculations;
use crate::calculations::HashDiagram;
use std::collections::HashMap;
use chrono::{NaiveDate, Local};
//fn for drawing chart bars
pub fn draw_chart_bars(screen_w: f32, screen_h: f32, interval: f32, sorted_diagram: &Vec<calculations::Diagram>) {
    //init some colors
    let bar_color = Color::from_hex(0xbd93f9);
    //diana's color
    //let bar_color = Color::from_hex(0x8df9d9);
    let outline_color = Color::from_hex(0x6272a4);
    //diana's color
    //let outline_color = Color::from_hex(0xa0ff00);
    for (n, bar) in sorted_diagram.iter().enumerate() {
        draw_rectangle(interval * n as f32 + interval, screen_h - interval, interval, -interval * bar.count as f32, bar_color);
        draw_rectangle_lines(interval * n as f32 + interval, screen_h - interval, interval, -interval * bar.count as f32, 4.0, outline_color);
    }
}

pub fn draw_info_about_bars(sorted_diagram: &Vec<calculations::Diagram>, diagram: &HashMap<NaiveDate, HashDiagram>, interval: f32) {
    //init colors
    let outline_color = Color::from_hex(0x6272a4);
    let background_color = Color::from_hex(0x282a36);
    //init vars
    let (mouse_pos_x, mouse_pos_y) = mouse_position();
    let (mut x, mut y) = (mouse_pos_x, mouse_pos_y);
    x += -200.0/2.0;
    x = x.max(5.0);
    y += -(90.0 + 30.0);
    let bar = (mouse_pos_x / interval - 1.0) as usize;
    //draw_text(format!("{}", bar), 30.0, 30.0, 30.0, RED);
    //frame
    //outline
    draw_rectangle(0.0 + x - 5.0, 0.0 + y - 5.0, 200.0 + 5.0 * 2.0, 90.0 + 5.0 * 2.0, outline_color);
    //original
    draw_rectangle(0.0 + x, 0.0 + y, 200.0, 90.0, background_color);
    draw_text(format!("full date: {}", match sorted_diagram.get(bar){Some(value) => format!("{:?}", value.date), None => "future".to_string()}), 0.0 + x, 20.0 + y, 20.0, WHITE);
    draw_text(format!("count: {}", match sorted_diagram.get(bar){Some(value) => value.count.to_string(), None => "unknown".to_string()}), 0.0 + x, 40.0 + y, 20.0, WHITE);
    draw_text("comment:", 0.0 + x, 60.0 + y, 20.0, WHITE);
    //get comment
    let mut comment: String = String::new();
    match sorted_diagram.get(bar) {
        Some(value) => {
            comment = match diagram.get(&value.date) {
                Some(value) => value.comment.clone(),
                None => "Nothing!".to_string(),
            }
        },
        None => {
            comment = " ".to_string();
        },
    }
    draw_text(format!("{}", comment), 0.0 + x, 80.0 + y, 20.0, WHITE);
}
