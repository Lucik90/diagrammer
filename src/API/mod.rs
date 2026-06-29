use macroquad::prelude::*;
use crate::calculations::HashDiagram;
use chrono::{NaiveDate, Local};
//API things

//appstats for graphic fns
pub struct AppStats {
    pub printing_error: Option<String>,
    pub interval: f32,
    pub grid: bool,
    pub purpose: InputPurpose,
}

//user's input for graphic fns
#[derive(Debug, Clone)]
pub struct UsersInput {
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_button_left_pressed: bool,
    pub mouse_button_right_pressed: bool,
    pub mouse_button_left_down: bool,
    pub mouse_button_right_down: bool,
    pub key_g_pressed: bool,
    pub key_up_pressed: bool,
    pub key_down_pressed: bool,
} impl UsersInput {
    pub fn new() -> Self {
        Self {
            mouse_x: mouse_position().0,
            mouse_y: mouse_position().1,
            mouse_button_left_pressed: is_mouse_button_pressed(MouseButton::Left),
            mouse_button_right_pressed: is_mouse_button_pressed(MouseButton::Right),
            mouse_button_left_down: is_mouse_button_down(MouseButton::Left),
            mouse_button_right_down: is_mouse_button_down(MouseButton::Right),
            key_g_pressed: is_key_pressed(KeyCode::G),
            key_up_pressed: is_key_pressed(KeyCode::Up),
            key_down_pressed: is_key_pressed(KeyCode::Down),
        }
    }
}
pub struct ScreenSize {
    pub screen_w: f32,
    pub screen_h: f32,
} impl ScreenSize {
    pub fn new() -> Self {
        Self {
            screen_w: screen_width(),
            screen_h: screen_height(),
        }
    }
}
#[derive(Debug)]
pub enum Scene {
    Diagram,
    Menu,
} 
impl Default for Scene {
    fn default() -> Self {
        Self::Diagram
    }
}

#[derive(Debug)]
pub enum Action {
    Nothing,
    ChangeScene(Scene),
    SaveFile,
    DrawBarInfo,
    AddBar( (NaiveDate, HashDiagram) ),
    RemoveBar,
    UpdateBar,
    CaptureInput,
    PrintError(String),
    Exit,
}
#[derive(Debug, PartialEq)]
pub enum InputPurpose {
    Nothing,
    Count,
    Comment,
}
