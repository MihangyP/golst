use std::ffi::{CString};
use std::os::raw::c_char;

#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[repr(C)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
enum KeyboardKey {
    KeyA = 65,
}

extern "C" {
    fn InitWindow(width: i32, height: i32, title: *const c_char);
    fn CloseWindow();
    fn WindowShouldClose() -> bool;
    fn BeginDrawing();
    fn EndDrawing();
    fn ClearBackground(color: Color);
    fn SetTargetFPS(fps: i32);
    fn DrawLine(start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: Color);
    fn IsMouseButtonPressed(button: i32) -> bool;
    fn GetMouseX() -> i32;
    fn GetMouseY() -> i32;
    fn DrawRectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color);
    fn IsKeyPressed(key: i32) -> bool;
    fn GetTime() -> f64;
}

const WIDTH: i32 = 600;
const HEIGHT: i32 = 600;
const NUMBER_OF_SQUARES: i32 = 30;
const SQUARE_SIZE: i32 = WIDTH / NUMBER_OF_SQUARES;

unsafe fn render(map: &[[i32; NUMBER_OF_SQUARES as usize]; NUMBER_OF_SQUARES as usize])
{
    for j in 0..NUMBER_OF_SQUARES {
        for i in 0..NUMBER_OF_SQUARES {
            if map[j as usize][i as usize] == 1 {
                let x = normalize(i);
                let y = normalize(j);
                DrawRectangle(x, y, SQUARE_SIZE, SQUARE_SIZE, Color {r: 255, g: 255, b: 255, a: 255});
            }
        }
    }
}

unsafe fn draw_grid()
{
    let mut x = 0;
    let mut y = 0;
    while x <= WIDTH {
        DrawLine(x, 0, x, HEIGHT, Color {r: 235, g: 235, b:235, a: 150});
        x += SQUARE_SIZE;
    }
    while y <= HEIGHT {
        DrawLine(0, y, WIDTH, y, Color {r: 235, g: 235, b:235, a: 150});
        y += SQUARE_SIZE;
    }
}

fn denormalize(n: i32) -> i32 {
    let mut n_tmp: f64 = n as f64 / SQUARE_SIZE as f64;
    n_tmp = n_tmp.floor();
    return n_tmp as i32;
}

fn normalize(n: i32) -> i32 {
    return n * SQUARE_SIZE;
}

fn main() {
    let tmp = CString::new("GOLST").unwrap();
    let title = tmp.as_ptr();
    let mut map = [[0; NUMBER_OF_SQUARES as usize]; NUMBER_OF_SQUARES as usize];
    let mut start_game = false;
    let mut last_update_time = 0.0;
    let update_interval = 0.1;

    unsafe {
        InitWindow(WIDTH, HEIGHT, title);
        SetTargetFPS(60);
        while !WindowShouldClose() {
            let curr_time = GetTime();

            if IsMouseButtonPressed(0) && !start_game {
                let x: i32 = denormalize(GetMouseX());
                let y: i32 = denormalize(GetMouseY());
                map[y as usize][x as usize] = 1
            }
            if IsKeyPressed(KeyboardKey::KeyA as i32) && !start_game {
                start_game = true;
                last_update_time = curr_time;
            }
            BeginDrawing();
            ClearBackground(Color {r: 24, g: 24, b: 24, a: 255});
            draw_grid();
            if !start_game { 
                for j in 0..NUMBER_OF_SQUARES {
                    for i in 0..NUMBER_OF_SQUARES {
                        if map[j as usize][i as usize] == 1 {
                            let pos_x = normalize(i);
                            let pos_y = normalize(j);
                            DrawRectangle(pos_x, pos_y, SQUARE_SIZE, SQUARE_SIZE, Color {r: 255, g: 255, b: 255, a: 255});
                        }
                    }
                }
            } else {

                if curr_time - last_update_time >= update_interval {
                    let mut next_map = map.clone();

                    let mut j = 0;
                    while j < NUMBER_OF_SQUARES {
                        let mut i = 0;
                        while i < NUMBER_OF_SQUARES {
                            let mut neighbors = [0; 8];
                            let top_j = j - 1;
                            let bottom_j = j + 1;
                            let left_i = i - 1;
                            let right_i = i + 1;

                            // top
                            if !(top_j < 0 || left_i < 0) {
                                neighbors[0] = map[top_j as usize][left_i as usize];
                            }
                            if !(top_j < 0) {
                                neighbors[1] = map[top_j as usize][i as usize];
                            }
                            if !(top_j < 0 || right_i >= NUMBER_OF_SQUARES) {
                                neighbors[2] = map[top_j as usize][right_i as usize];
                            }
                            // middle
                            if !(left_i < 0) {
                                neighbors[3] = map[j as usize][left_i as usize];
                            }
                            if !(right_i >= NUMBER_OF_SQUARES) {
                                neighbors[4] = map[j as usize][right_i as usize];
                            }
                            // bottom
                            if !(bottom_j >= NUMBER_OF_SQUARES || left_i < 0) {
                                neighbors[5] = map[bottom_j as usize][left_i as usize];
                            }
                            if !(bottom_j >= NUMBER_OF_SQUARES) {
                                neighbors[6] = map[bottom_j as usize][i as usize];
                            }
                            if !(bottom_j >= NUMBER_OF_SQUARES || right_i >= NUMBER_OF_SQUARES) {
                                neighbors[7] = map[bottom_j as usize][right_i as usize];
                            }

                            let n = neighbors.iter()
                                .filter(|&&x| x == 1)
                                .count();

                            // Update NEXT_MAP based on current MAP
                            if map[j as usize][i as usize] == 0 && n == 3 {
                                next_map[j as usize][i as usize] = 1;
                            } else if map[j as usize][i as usize] == 1 && (n < 2 || n > 3) {
                                next_map[j as usize][i as usize] = 0;
                            }

                            i += 1;
                        }
                        j += 1;
                    }
                    map = next_map;
                    last_update_time = curr_time;
                }
                render(&map);
            }
            EndDrawing();
        }
        CloseWindow();
    }
}
