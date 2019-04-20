#![no_std]

mod glyph;

use core::slice;
use glyph::Glyph;
use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltRegion, GraphicsOutput};

pub fn print(gop: &mut GraphicsOutput, x: usize, y: usize, s: &str) -> usize {
    let unifont_bin = include_bytes!(concat!(env!("OUT_DIR"), "/unifont.bin"));
    let glyphs =
        unsafe { slice::from_raw_parts(unifont_bin.as_ptr() as *const Option<Glyph>, 0x10000) };

    let mut advanced = 0;

    for c in s.chars() {
        if let Some(glyph) = &glyphs[c as usize] {
            let buffer = glyph.buffer();
            let metrics = glyph.metrics();
            gop.blt(BltOp::BufferToVideo {
                buffer,
                src: BltRegion::SubRectangle {
                    coords: (0, 0),
                    px_stride: metrics.0,
                },
                dest: (x + advanced, y),
                dims: metrics,
            })
            .unwrap_success();
            advanced += metrics.0;
        }
    }

    advanced
}
