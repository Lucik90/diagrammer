use macroquad::prelude::*;
pub fn draw_2D_grid(screen_w: f32, screen_h: f32, interval: f32) {
    draw_horizontal_lines(screen_w, screen_h, interval);
    draw_vertical_lines(screen_w, screen_h, interval);
}
pub fn draw_horizontal_lines(screen_w: f32, screen_h: f32, interval: f32) {
    let mut n: f32 = 0.0;
    while interval * n < screen_h {
        draw_line(0.0, screen_h - interval * n, screen_w, screen_h - interval * n, 2.0, GRAY);
        n += 1.0;
    }
}
pub fn draw_vertical_lines(screen_w: f32, screen_h: f32, interval: f32) {
    let mut n: f32 = 0.0;
    while interval * n < screen_w {
        draw_line(interval * n, screen_h, interval * n, 0.0, 2.0, GRAY);
        n += 1.0;
    }
}
pub fn draw_borders(screen_w: f32, screen_h: f32, interval: f32){
    draw_rectangle_lines(interval, screen_h - interval, screen_w, -screen_h, 4.0, WHITE);
    draw_rectangle_lines(interval, screen_h - interval, -interval, interval + 4.0, 4.0, WHITE);
}
