pub mod constraint_solver;
pub mod piece;

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use crate::constraint_solver::CellSolveState::{Solved, Unsolved};
use crate::constraint_solver::{Grid, SolverState};
use crate::piece::VisualCell;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Texture, TextureAccess, TextureCreator, WindowCanvas};
use std::time::{Duration, Instant};
use image::EncodableLayout;
use sdl2::EventPump;
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

fn run_sync(mut canvas: WindowCanvas, mut texture_cache: TextureCache, mut event_pump: EventPump) {
    let mut solver = SolverState::new();

    canvas.clear();
    draw_grid(&mut canvas, &mut texture_cache, solver.current_grid());
    canvas.present();

    let mut needs_redraw = false;
    let mut auto_mode = false;

    let mut frames_in_last_second = 0;
    let mut last_frames_measure = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode,  .. } => {
                    if keycode == Some(Keycode::Space) {
                        auto_mode = false;
                        let solver_result = solver.step_propagate();
                        if solver_result.is_err() {
                            println!("Failed to tick solver");
                        }
                        needs_redraw = true;
                    } else if keycode == Some (Keycode::A) {
                        auto_mode = !auto_mode;
                    }
                }
                _ => {}
            }
        }

        if auto_mode {
            let solver_result = solver.step_propagate();
            if solver_result.is_err() {
                auto_mode = false;
            }
            needs_redraw = true;
        }

        if needs_redraw {
            needs_redraw = false;
            canvas.clear();
            draw_grid(&mut canvas, &mut texture_cache, solver.current_grid());
            canvas.present();
        }

        frames_in_last_second += 1;
        if last_frames_measure.elapsed().as_secs() > 1 {
            println!("Frame rate: {:.2}/sec", frames_in_last_second);
            last_frames_measure = Instant::now();
            frames_in_last_second = 0;
        }
        // std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_grid(canvas: &mut WindowCanvas, texture_cache: &mut TextureCache, grid: &Grid){
    for y in 0..6 {
        for x in 0..6 {
            if let Solved(cell) = grid.grid[y][x] {
                // let (image_to_draw, angle) = image_for_cell(cell);
                let visual_cell = grid.visual_grid[y][x];
                let image_to_draw = texture_cache.get_texture_for_visual_cell(visual_cell);
                let angle = visual_cell.angle() as f64;
                let draw_rect = Rect::new((x * 32) as i32, (y * 32) as i32, 32, 32);
                canvas.copy_ex(image_to_draw, None, draw_rect, 360.0 - angle, None, false, false).unwrap();
            }else if let Unsolved(domain) = &grid.grid[y][x] {
                let mut len = domain.len();
                let mut digits = vec![];
                while len >= 10 {
                    let ones = len % 10;
                    digits.push(ones);
                    len /= 10;
                }
                digits.push(len);
                digits.reverse();

                let blit32_texture = texture_cache.get_or_load_texture("resources/blit32.png");
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
}

pub struct TextureCache<'texture_creator: 'textures, 'textures>{
    texture_creator: &'texture_creator TextureCreator<WindowContext>,
    texture_cache: HashMap<&'static str, Texture<'textures>>,
}

impl <'texture_creator: 'textures, 'textures> TextureCache<'texture_creator, 'textures> {
    pub fn new(texture_creator: &'texture_creator TextureCreator<WindowContext>) -> TextureCache<'texture_creator, 'textures> {
        TextureCache {
            texture_creator,
            texture_cache: HashMap::new(),
        }
    }

    pub fn get_or_load_texture(&mut self, name: &'static str) -> &mut Texture<'textures> {
        if !self.texture_cache.contains_key(name) {
            let new_texture = load_image_as_texture(&self.texture_creator, name).unwrap();
            self.texture_cache.insert(name, new_texture);
        }
        self.texture_cache.get_mut(name).unwrap()
    }

    pub fn get_texture_for_visual_cell(&mut self, visual_cell: VisualCell) -> &Texture<'textures> {
        let image_key = visual_cell.get_image_path();
        self.get_or_load_texture(&image_key)
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Puzzle Wave Function Collapse", 800, 600).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let binding = canvas.texture_creator();
    let mut texture_cache = TextureCache::new(&binding);

    let blit32_texture = texture_cache.get_or_load_texture("resources/blit32.png");
    blit32_texture.set_blend_mode(BlendMode::Blend);

    canvas.set_logical_size(192, 192).unwrap();
    canvas.set_draw_color(Color::RGB(25, 25, 25));
    canvas.clear();
    canvas.present();


    let event_pump = sdl_context.event_pump().unwrap();
    run_sync(canvas, texture_cache, event_pump);




}
