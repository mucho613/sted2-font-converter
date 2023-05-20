use std::{fs::File, io::Read};
use image::{ImageBuffer};

fn main() {
    const COUNT_X: u32 = 32;
    const COUNT_Y: u32 = 128;

    const GLYPH_SIZE_X: u32 = 16;
    const GLYPH_SIZE_Y: u32 = 16;

    let mut file: File = File::open("./STED2.FON").expect("ファイルの読み込みに失敗しました");
    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer).expect("読み込みに失敗しました");
    let height = buffer.len() / 2;

    let image = ImageBuffer::from_fn(GLYPH_SIZE_X, u32::try_from(height).unwrap(), |x, y| {
        let byte_offset: u32 = if x < 8 { 0 } else { 1 };
        let index: usize = usize::try_from(y * 2 + byte_offset).unwrap();
        let byte = buffer[index];

        let bit_shift_amount = x % 8;
        let pixel = (byte << bit_shift_amount & 0x80) >> 7;

        match pixel {
            0 => {
                image::Luma([0u8])
            },
            1 => {
                image::Luma([255u8])
            },
            _ => {
                panic!()
            }
        }

    });

    let remapped_image = ImageBuffer::from_fn(COUNT_X * GLYPH_SIZE_X, COUNT_Y * GLYPH_SIZE_Y, |x, y| {
        let source_glyph = (x / GLYPH_SIZE_X, y / GLYPH_SIZE_Y);
        let source_glyph_px = (x % GLYPH_SIZE_X, y % GLYPH_SIZE_Y);

        let target_x = source_glyph_px.0;
        let target_y = (source_glyph.0 * GLYPH_SIZE_X) + (source_glyph.1 * COUNT_X * GLYPH_SIZE_X) + (source_glyph_px.1);

        if target_y >= u32::try_from(height).unwrap() {
            image::Luma([0u8])
        } else {
            *image.get_pixel(target_x, target_y)
        }
    });

    remapped_image.save("output.png").expect("ファイルの保存に失敗しました");
}
