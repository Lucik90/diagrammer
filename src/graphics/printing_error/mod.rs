use macroquad::prelude::*;
use crate::API;
use super::draw_rounded_rectangle;
use crate::calculations;
pub fn draw_error_window(
    app_stats: &mut API::AppStats,
    users_input: &API::UsersInput,
    screen_size: &API::ScreenSize, 
    ) {
    //let vars
    let (screen_w, screen_h) = (screen_size.screen_w, screen_size.screen_h);
    let (mouse_x, mouse_y) = (users_input.mouse_x, users_input.mouse_y);
    //init error_text
    let error_text = app_stats.printing_error.as_ref().unwrap();
    //tint all
    draw_rectangle(0.0, 0.0, screen_size.screen_w, screen_size.screen_h, Color::new(0.0, 0.0, 0.0, 0.75));
    //draw error window
    let error_window = Rect::new(0.0, 0.0, 500.0, 200.0);
    let error_window = Rect::new(
        screen_w / 2.0 - error_window.w / 2.0,
        screen_h / 2.0 - error_window.h / 2.0,
        error_window.w,
        error_window.h 
    );
    draw_rounded_rectangle(error_window.x, error_window.y, error_window.w, error_window.h, 20.0, DARKGRAY);
    draw_text("Error:", error_window.x + 40.0, error_window.y + 30.0 as f32, 30.0, WHITE);
    //calculate error_strings
    let error_strings: Vec<String> = calculations::chunk_string_by_chars(error_text, 35);
    for (ind, error_text) in error_strings.iter().enumerate() {
        draw_text(format!("{}", error_text), error_window.x + 20.0, error_window.y + 30.0 + (30.0 * (ind + 1) as f32), 30.0, WHITE);
    }
    //exit from error window
    let exit_button = Rect::new(0.0, 0.0, 20.0, 20.0);
    let exit_button = Rect::new(
        error_window.x + 10.0,
        error_window.y + 10.0,
        exit_button.w,
        exit_button.h 
    );
    draw_rounded_rectangle(exit_button.x, exit_button.y, exit_button.w, exit_button.h, 11.0, RED);
    if exit_button.contains(vec2(mouse_x, mouse_y)) && users_input.mouse_button_left_pressed {
        app_stats.printing_error = None;
    }
}
