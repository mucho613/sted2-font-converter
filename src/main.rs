use std::{fs::File, io::Read};
use image::{ImageBuffer};

fn main() {
    let mut file = File::open("./STED2.FON").expect("ファイルの読み込みに失敗しました");
    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer).expect("読み込みに失敗しました");
    let height = buffer.len() / 2;

    let img = ImageBuffer::from_fn(16, u32::try_from(height).unwrap(), |x: u32, y: u32| {
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

    let remapped_image = ImageBuffer::from_fn(256, 256, |x: u32, y: u32| {
        let source_glyph = (x / 16, y / 16);
        let source_glyph_px = (x % 16, y % 16);

        let target_x = source_glyph_px.0;
        let target_y = (source_glyph.0 * 16) + (source_glyph.1 * 16 * 16) + (source_glyph_px.1);

        if target_y >= u32::try_from(height).unwrap() {
            image::Luma([0u8])
        } else {
            *img.get_pixel(target_x, target_y)
        }
    });

    remapped_image.save("remapped.png").unwrap();
}
