use rayon::{prelude::ParallelIterator, slice::ParallelSlice};

pub type Color = (u8, u8, u8);

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ColorBlock {
    pub top: Color,
    pub bottom: Color,
}

pub fn to_colors(img: &[u8], width: usize, height: usize) -> Vec<ColorBlock> {
    assert_eq!(
        img.len(),
        width * height * 4,
        "Image dimensions do not match data length"
    );

    let indexed: Vec<(u8, u8, u8)> = img
        .par_chunks(4)
        .map(|chunk| {
            let [r, g, b, _]: [u8; 4] = chunk.try_into().unwrap();
            (r, g, b)
        })
        .collect();

    let mut colors = Vec::with_capacity(width * height);
    for line in 0..height / 2 {
        let base = (line * 2) * width;
        for i in 0..width {
            let top = indexed[base + i];
            let bottom = indexed[base + width + i];
            colors.push(ColorBlock { top, bottom });
        }
    }

    colors
}
