#[path = "src/glyph.rs"]
mod glyph;

use glyph::Glyph;
use std::env;
use std::fs;
use std::iter;
use std::mem;
use std::path::Path;
use std::slice;
use std::str;
use uefi::proto::console::gop::BltPixel;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let unifont_bin = Path::new(&out_dir).join("unifont.bin");
    let unifont_hex = include_str!("unifont-12.0.01.hex");

    let mut glyphs = Vec::new();
    glyphs.resize(0x10000, None);

    for line in unifont_hex[..unifont_hex.len() - 1].lines() {
        let codepoint = usize_from_hex_str(&line[..4]);
        let buffer = bits_from_hex_str(&line[5..])
            .map(|bit| {
                let v = u8::max_value() * bit as u8;
                BltPixel::new(v, v, v)
            })
            .collect::<Vec<_>>();

        glyphs[codepoint] = Some(Glyph::from_pixels(&buffer).unwrap());
    }

    fs::write(unifont_bin, unsafe {
        slice::from_raw_parts(
            glyphs.as_slice() as *const _ as *const u8,
            mem::size_of::<Option<Glyph>>() * glyphs.len(),
        )
    })
    .unwrap();
}

fn usize_from_hex_str(b: &str) -> usize {
    usize::from_str_radix(b, 16).unwrap()
}

fn bits_from_hex_str<'a>(bytes: &'a str) -> impl Iterator<Item = bool> + 'a {
    bytes
        .as_bytes()
        .chunks(2)
        .map(|chunk| u8::from_str_radix(str::from_utf8(&chunk).unwrap(), 16).unwrap())
        .flat_map(|byte| {
            iter::successors(Some(0b10000000u8), |mask| Some(mask >> 1))
                .map(move |mask| 0 < byte & mask)
                .take(8)
        })
}
