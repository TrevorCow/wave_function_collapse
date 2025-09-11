use image::{ColorType, GenericImageView};

fn main() {
    let puzzle_sheet = image::open("resources/PuzzlePieces.png").unwrap();

    for y in 0..6 {
        for x in 0..6 {
            let piece = puzzle_sheet.view(x * 32, y * 32, 32, 32);
            let path = format!("resources/P{}{}.png", x, y);
            image::save_buffer(&path, piece.to_image().as_raw(), 32, 32, ColorType::Rgba8).unwrap()
        }
    }
}