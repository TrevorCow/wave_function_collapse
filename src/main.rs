mod constrait_solver;
mod piece;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::Path;
use std::process::exit;
use crate::constrait_solver::CellSolveState::{Solved, Unsolved};
use crate::constrait_solver::{PieceRotation, SolverState};
use crate::piece::{Cell, VisualCell};
use crate::piece::ConnectionType::{Double, NoConnection, Straight};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Canvas, Texture, TextureAccess, TextureCreator, WindowCanvas};
use std::time::{Duration, Instant};
use image::{EncodableLayout, Pixel};
use sdl2::video::WindowContext;

fn load_image_as_texture<P: AsRef<Path>>(binding: &'_ TextureCreator<WindowContext>, path: P) -> Result<Texture<'_>, ()> {
    println!("Loading image {}", path.as_ref().display());
    let image = image::open(path).unwrap();
    let image_width = image.width();
    let image_height = image.height();
    let mut texture = binding.create_texture(PixelFormatEnum::ABGR8888, TextureAccess::Static, image_width, image_height).unwrap();
    texture.update(None, image.into_rgba8().as_bytes(), (4 * image_width) as usize).unwrap();

    Ok(texture)
}

fn main() {
    let mut solver = SolverState::new();
    solver.current_grid_mut().place_piece(&piece::P10_P11, 0, 2);
    // solver.place_piece(piece::P5, 1, 1, PieceRotation::CCW0);
    // println!("Solver Valid: {}", solver.check());
    // println!("Solver(2, 1) == {:?}", solver.current_grid.grid[1][2]);
    // solver.do_constraint_propagation();
    // println!("Solver(2, 1) == {:?}", solver.current_grid.grid[1][2]);
    // println!("{:?}", solver);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let binding = canvas.texture_creator();

    let blit32_image = image::open("resources/blit32.png").unwrap();
    let mut blit32_texture = binding.create_texture(PixelFormatEnum::ABGR8888, TextureAccess::Static, 360, 360).unwrap();
    blit32_texture.update(None, blit32_image.as_bytes(), 4 * 360).unwrap();
    blit32_texture.set_blend_mode(BlendMode::Add);

    let mut texture_cache = HashMap::<&'static str, Texture>::new();

    fn image_for_visual_cell<'a, 'b>(binding: &'b TextureCreator<WindowContext>, visual_cell: VisualCell, texture_cache: &'a mut HashMap<&'static str, Texture<'b>>) -> &'a Texture<'b> {
        let image_key = visual_cell.get_image_path();
        let entry = texture_cache.entry(image_key);
        match entry {
            Entry::Occupied(texture) => {},
            Entry::Vacant(entry) => {
                let new_texture = load_image_as_texture(&binding, entry.key()).unwrap();
                entry.insert(new_texture);
            }
        }
        texture_cache.get(image_key).unwrap()
    };

    canvas.set_logical_size(192, 192).unwrap();

    let mut draw_solver = |canvas: &mut WindowCanvas, solver: &SolverState| {
        for y in 0..6 {
            for x in 0..6 {
                if let Solved(cell) = solver.current_grid().grid[y][x] {
                    // let (image_to_draw, angle) = image_for_cell(cell);
                    let visual_cell = solver.current_grid().visual_grid[y][x];
                    let image_to_draw = image_for_visual_cell(&binding, visual_cell, &mut texture_cache);
                    let angle = visual_cell.angle() as f64;
                    let draw_rect = Rect::new((x * 32) as i32, (y * 32) as i32, 32, 32);
                    canvas.copy_ex(image_to_draw, None, draw_rect, 360.0 - angle, None, false, false).unwrap();
                }else if let Unsolved(domain) = &solver.current_grid().grid[y][x] {
                    let mut len = domain.len();
                    let mut digits = vec![];
                    while len >= 10 {
                        let ones = len % 10;
                        digits.push(ones);
                        len /= 10;
                    }
                    digits.push(len);
                    digits.reverse();

                    for (i, digit) in digits.iter().enumerate() {
                        let digit_x = 24 + (24 * digit);
                        let letter_rect = Rect::new(digit_x as i32, 5 * 36, 24, 36);
                        let letter_scale = digits.len();
                        let letter_width = 24 / letter_scale;
                        let dest_rect = Rect::new((x * 32 + (i * letter_width)) as i32, (y * 32) as i32, (24 / letter_scale) as u32, (36 / letter_scale) as u32);
                        canvas.copy(&blit32_texture, letter_rect, dest_rect).unwrap();
                    }


                }
            }
        }
    };

    canvas.set_draw_color(Color::RGB(25, 25, 25));
    canvas.clear();
    canvas.present();

    canvas.clear();
    draw_solver(&mut canvas, &solver);
    canvas.present();

    let mut needs_redraw = false;
    let mut auto_mode = false;

    let mut ticks_in_last_second = 0;
    let mut last_tick_measure = Instant::now();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode, keymod, .. } => {
                    if keycode == Some(Keycode::Space) && !auto_mode {
                        solver.tick();
                        needs_redraw = true;
                    } else if keycode == Some (Keycode::P) {
                        solver.pop_state();
                        needs_redraw = true;
                    } else if keycode == Some (Keycode::A) {
                        auto_mode = !auto_mode;
                    }
                }
                _ => {}
            }
        }

        if auto_mode {
            solver.tick();
            // needs_redraw = true;
        }

        if needs_redraw {
            needs_redraw = false;
            canvas.clear();
            draw_solver(&mut canvas, &solver);
            canvas.present();
        }

        ticks_in_last_second += 1;
        if last_tick_measure.elapsed().as_secs() > 1 {
            println!("Tick rate: {:.2}/sec", ticks_in_last_second);
            last_tick_measure = Instant::now();
            ticks_in_last_second = 0;
        }
        // std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
