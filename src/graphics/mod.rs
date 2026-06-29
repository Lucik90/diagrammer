use std::collections::HashMap;
use macroquad::prelude::*;
use chrono::{NaiveDate, Local};
use chrono::Datelike;   
use crate::calculations::HashDiagram;
use super::API;

pub fn update_screen(
    scene: &API::Scene,
    diagram: &mut HashMap<NaiveDate, HashDiagram>,
    sorted_diagram: &Vec<super::calculations::Diagram>, 
    app_stats: &mut API::AppStats,
    users_input: &API::UsersInput,
    screen_size: &API::ScreenSize,
    users_input_text: &mut (String, String)
    ) -> Vec<API::Action> {
    //let actions
    let mut actions = vec![API::Action::Nothing];

    //let blocked_input
    let mut blocked_input = users_input.clone();
    if let Some(_) = app_stats.printing_error {
        blocked_input.mouse_x = 0.0;
        blocked_input.mouse_y = 0.0;
        blocked_input.mouse_button_left_pressed = false;
        blocked_input.mouse_button_right_pressed = false;
        blocked_input.mouse_button_left_down = false;
        blocked_input.mouse_button_right_down = false;
        blocked_input.key_g_pressed = false;
        blocked_input.key_up_pressed = false;
        blocked_input.key_down_pressed = false;
    }

    //variables
    //get super important variables
    let (mouse_x, mouse_y) = (blocked_input.mouse_x, blocked_input.mouse_y);
    //get importand variables
    let background_color = Color::from_hex(0x282a36);
    //diana's color
    //let background_color = Color::from_hex(0xc93888);
    //end
    if users_input.key_up_pressed {
        app_stats.interval += 10.0;
    }
    if users_input.key_down_pressed {
        app_stats.interval += -10.0;
    }
    //minimal limit
    app_stats.interval = app_stats.interval.max(10.0);
    //maximal limit
    app_stats.interval = app_stats.interval.min(100.0);

    let interval = app_stats.interval;

    if let API::Scene::Diagram = *scene {
        //clear background
        clear_background(background_color);

        //draw grid
        if app_stats.grid {
            borders::draw_2D_grid(screen_size.screen_w, screen_size.screen_h, interval);
        }

        //draw borders
        borders::draw_borders(screen_size.screen_w, screen_size.screen_h, interval);

        //draw numbers
        numbers::draw_all_numbers(screen_size.screen_w, screen_size.screen_h, interval, diagram, sorted_diagram);

        //draw bars
        bars::draw_chart_bars(screen_size.screen_w, screen_size.screen_h, interval, sorted_diagram);
    }
    //draw GUI
    actions = GUI::draw_GUI(
        scene, 
        sorted_diagram,
        app_stats,
        &blocked_input,
        screen_size, 
        users_input_text,
    );
    //printing error window
    if app_stats.printing_error.is_some() {
        printing_error::draw_error_window(app_stats, users_input, screen_size);
    }
    match actions[0] {
        API::Action::DrawBarInfo => {
            bars::draw_info_about_bars(&sorted_diagram, &diagram, app_stats.interval);
            actions[0] = API::Action::Nothing;
        },
        _ => (),
    }
    actions
}

mod borders;
mod numbers;
pub mod bars;
pub mod GUI;
pub mod printing_error;
pub fn draw_rounded_rectangle(x: f32, y: f32, w: f32, h: f32, cornerRadius: f32, color: Color) {
    let cornerRadius = cornerRadius.min(w / 2.0);
    let cornerRadius = cornerRadius.min(h / 2.0);
    let cornerRadius = cornerRadius.max(0.0);
    //all needed
    draw_circle(x + cornerRadius, y + cornerRadius, cornerRadius, color);
    draw_circle(x + w - cornerRadius, y + cornerRadius, cornerRadius, color);
    draw_circle(x + cornerRadius, y + h - cornerRadius, cornerRadius, color);
    draw_circle(x + w - cornerRadius, y + h - cornerRadius, cornerRadius, color);

    draw_rectangle(x, y + cornerRadius, w, h - cornerRadius * 2.0, color);
    draw_rectangle(x + cornerRadius, y, w - cornerRadius * 2.0, h, color);
}
