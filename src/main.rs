use std::collections::HashMap;
use macroquad::prelude::*;
use chrono::{NaiveDate, Local};
use chrono::Datelike;
use crate::calculations::HashDiagram;
#[macroquad::main("diagrammer")]
async fn main() {

    //universal variable for graphic
    let mut app_stats = API::AppStats {
        //print error window?
        printing_error: None,
        //draw grid?
        grid: false,
        //interval variable 
        interval: 30.0,
        //input purpose
        purpose: API::InputPurpose::Nothing,
    };

    //init actions
    let mut actions = vec![API::Action::Nothing];
    //init diagram
    let mut diagram: HashMap<NaiveDate, HashDiagram> = HashMap::new();
    diagram.insert(
        NaiveDate::from_ymd_opt(5000, 6, 15).unwrap(),
        HashDiagram {
            count: 8,
            comment: "Some comment".to_string(),
        },
    );
    //load diagram from file
    let mut diagram: HashMap<NaiveDate, HashDiagram> = match work_with_fs::load() {
        Ok(value) => value,
        Err(e) => {
            app_stats.printing_error = Some("Load error! File diagram_neo.ron not found. Using empty diagram. Add data via menu to save.".to_string());
            HashMap::new()
        },
    };

    let mut sorted_diagram: Vec<calculations::Diagram> = vec![];

    //user's text input
    let mut users_text_input: (String, String) = (String::new(), String::new());

    let mut scene = API::Scene::default();

    //calculate sorted diagram
    sorted_diagram = calculations::transformer(&diagram);

    loop {

        //init screen_size
        let screen_size = API::ScreenSize::new();
        //init users input
        let users_input = API::UsersInput::new();
        //clear input if 
        if let API::InputPurpose::Nothing = app_stats.purpose {
            clear_input_queue();
        }

        actions = graphics::update_screen(
            &scene,
            &mut diagram,
            &sorted_diagram, 
            &mut app_stats,
            &users_input,
            &screen_size,
            &mut users_text_input
        );

        //handling action
        let mut do_not_clear_actions = false;
        for action in actions {
            match action {
                API::Action::PrintError(error) => {
                    println!("{}", error);
                    app_stats.printing_error = Some(error);
                },
                API::Action::AddBar((date, data)) => {
                    diagram.entry(date).or_insert(data);
                    sorted_diagram = calculations::transformer(&diagram);
                    crate::work_with_fs::save(&diagram);
                },
                API::Action::ChangeScene(change_to) => {
                    scene = change_to;
                },
                API::Action::CaptureInput => {
                    do_not_clear_actions = true;
                }
                _ => (),
            }
        }
        if !do_not_clear_actions {
            actions = vec![API::Action::Nothing];
        }

        next_frame().await;
    }
}
pub mod graphics;
pub mod calculations;
pub mod work_with_fs;
pub mod API;
