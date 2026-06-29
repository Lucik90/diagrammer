use macroquad::prelude::*;
use crate::calculations::HashDiagram;
use crate::calculations;
use std::collections::HashMap;
use chrono::{NaiveDate, Local};
use super::draw_rounded_rectangle;
use super::super::API;

pub fn draw_GUI(
    scene: &API::Scene,
    sorted_diagram: &Vec<calculations::Diagram>,
    app_stats: &mut API::AppStats,
    users_input: &API::UsersInput,
    screen_size: &API::ScreenSize,
    users_input_text: &mut (String, String),
) -> Vec<API::Action> {

    let mut actions = vec![API::Action::Nothing];
    match scene {
        API::Scene::Diagram => {

            actions = draw_default_GUI(
                screen_size.screen_w, 
                screen_size.screen_h, 
                app_stats,
                users_input,
                app_stats.interval, 
                users_input.mouse_x, 
                users_input.mouse_y
            );

            clear_input_queue();
        },
        API::Scene::Menu => {
            actions = draw_menu_GUI(
                screen_size.screen_w, 
                screen_size.screen_h, 
                app_stats,
                app_stats.interval, 
                users_input, 
                users_input_text
            );
        },
    }
    actions
}
fn draw_default_GUI(
    screen_w: f32, 
    screen_h: f32, 
    app_stats: &mut API::AppStats,
    users_input: &API::UsersInput,
    interval: f32, 
    mouse_x: f32, 
    mouse_y: f32
    ) -> Vec<API::Action> {
    //init action
    let mut actions = vec![API::Action::Nothing];

    //draw grid?
    if users_input.key_g_pressed {
        app_stats.grid = !app_stats.grid;
    }
    //draw bars info?
    if users_input.mouse_button_left_down {
        actions[0] = API::Action::DrawBarInfo;
    }
    //menu button
    let menu_button = Rect::new(0.0, 0.0, 30.0, 30.0);
    if menu_button.contains(vec2(mouse_x, mouse_y)) && users_input.mouse_button_left_pressed {
        actions[0] = API::Action::ChangeScene(API::Scene::Menu);
    }
    draw_rectangle(0.0, 0.0, screen_w, 30.0, GRAY);
    draw_rectangle(menu_button.x, menu_button.y, menu_button.w, menu_button.h, GREEN);
    actions
}
fn draw_menu_GUI(
    screen_w: f32, 
    screen_h: f32, 
    app_stats: &mut API::AppStats,
    interval: f32, 
    users_input: &API::UsersInput,
    users_input_text: &mut (String, String)
    ) -> Vec<API::Action> {
    //init action
    let mut actions = vec![API::Action::Nothing];

    //mouse
    let (mouse_x, mouse_y) = (users_input.mouse_x, users_input.mouse_y);
    //set background color
    clear_background(GRAY);

    //exit button
    let exit_button = Rect::new(0.0, 0.0, 30.0, 30.0);
    draw_rectangle(exit_button.x, exit_button.y, exit_button.w, exit_button.h, RED);
    if exit_button.contains(vec2(mouse_x, mouse_y)) && users_input.mouse_button_left_pressed {
        actions[0] = API::Action::ChangeScene(API::Scene::Diagram);
    }

    //draw window
    let window = Rect::new(0.0, 0.0, 350.0, 300.0);
    let window = Rect::new(
        screen_w / 2.0 - window.w / 2.0,
        screen_h / 2.0 - window.h / 2.0,
        window.w,
        window.h 
    );
    draw_rounded_rectangle(window.x, window.y, window.w, window.h, 20.0, DARKGRAY);

    //count line
    let count_line = Rect::new(window.x + 10.0, window.y + 60.0 - 25.0, window.w - 20.0, 25.0);
    draw_rectangle(count_line.x, count_line.y, count_line.w, count_line.h, GRAY);
    if count_line.contains(vec2(mouse_x, mouse_y)) && users_input.mouse_button_left_pressed || app_stats.purpose == API::InputPurpose::Count {
        app_stats.purpose = API::InputPurpose::Count;
        //update user input
        if let Some(c) = get_char_pressed() {
            if c != '\x08' {
                users_input_text.0.push(c);
            } else {
                users_input_text.0.pop();
            }
        }
        draw_rectangle(count_line.x, count_line.y, count_line.w, count_line.h, BLUE);
    }

    //comment line
    let comment_line = Rect::new(window.x + 10.0, window.y + 90.0 - 25.0, window.w - 20.0, 25.0);
    draw_rectangle(comment_line.x, comment_line.y, comment_line.w, comment_line.h, GRAY);
    if comment_line.contains(vec2(mouse_x, mouse_y)) && users_input.mouse_button_left_pressed || app_stats.purpose == API::InputPurpose::Comment {
        app_stats.purpose = API::InputPurpose::Comment;
        //update user input
        if let Some(c) = get_char_pressed() {
            if c != '\x08' {
                users_input_text.1.push(c);
            } else {
                users_input_text.1.pop();
            }
        }

        draw_rectangle(comment_line.x, comment_line.y, comment_line.w, comment_line.h, BLUE);
    }

    //clear purpose
    if (!(count_line.contains(vec2(mouse_x, mouse_y))) && !(comment_line.contains(vec2(mouse_x, mouse_y)))) && users_input.mouse_button_left_pressed {

        app_stats.purpose = API::InputPurpose::Nothing;
    }

    //draw data
    draw_text("Day: TODAY", window.x + 10.0, window.y + 30.0, 30.0, WHITE);
    draw_text(format!("Count: {}", users_input_text.0), window.x + 10.0, window.y + 60.0, 30.0, WHITE);
    //correct comment
    loop {
        if users_input_text.1.len() > 15 {
            users_input_text.1 = users_input_text.1.chars().skip(1).collect::<String>();
        } else {
            break;
        }
    }
    draw_text(format!("Comment: {}", users_input_text.1), window.x + 10.0, window.y + 90.0, 30.0, WHITE);
    let color = if users_input_text.1.len() > 14 { Color::new(1.0, 0.0, 0.0, 1.0)} else { Color::new(1.0, 1.0, 1.0, 1.0)};
    draw_text(format!("{}/15", users_input_text.1.len()), window.x + 10.0, window.y + 120.0, 30.0, color);


    //new data button
    let new_data_button = Rect::new(0.0, 0.0, 175.0, 45.0);
    let new_data_button = Rect::new(
        (screen_w / 2.0 - new_data_button.w / 2.0) - 74.0,
        (screen_h / 2.0 - new_data_button.h / 2.0) + 115.0,
        new_data_button.w,
        new_data_button.h 
    );
    draw_rounded_rectangle(new_data_button.x, new_data_button.y, new_data_button.w, new_data_button.h, 11.0, GREEN);
    draw_text("Add", new_data_button.x + 65.0, new_data_button.y + 35.0, 35.0, WHITE);
    if new_data_button.contains(vec2(mouse_x, mouse_y)) 
        && users_input.mouse_button_left_pressed {
        let user_entered_count = match users_input_text.0.trim().parse::<u32>() {
            Ok(value) => value,
            Err(_) => {
                actions[0] = API::Action::PrintError("Please, input the number!".to_string());
                return actions;
            }

        };
        if users_input_text.1.len() > 20 {
            actions[0] = API::Action::PrintError("Your comment may be shorter than 20 letters!".to_string());
            return actions;
        }
        actions[0] = API::Action::AddBar(
            ( Local::now().date_naive(), 
              HashDiagram {
                  count: user_entered_count,
                  comment: users_input_text.1.clone(),
              }
            )
        );
        //clear input
        users_input_text.0.clear();
        users_input_text.1.clear();
        //exit from menu
        actions.push(API::Action::ChangeScene(API::Scene::Diagram));
    }

    //exit from menu
    let exit_button_2 = Rect::new(0.0, 0.0, 130.0, 45.0);
    let exit_button_2 = Rect::new(
        (screen_w / 2.0 - exit_button_2.w / 2.0) + 90.0,
        (screen_h / 2.0 - exit_button_2.h / 2.0) + 115.0,
        exit_button_2.w,
        exit_button_2.h 
    );
    draw_rounded_rectangle(exit_button_2.x, exit_button_2.y, exit_button_2.w, exit_button_2.h, 11.0, RED);
    draw_text("Exit", exit_button_2.x + 40.0, exit_button_2.y + 35.0, 35.0, WHITE);
    if exit_button_2.contains(vec2(mouse_x, mouse_y)) && users_input.mouse_button_left_pressed {
        actions[0] = API::Action::ChangeScene(API::Scene::Diagram);
    }

    actions
}
