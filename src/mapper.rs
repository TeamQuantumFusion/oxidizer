use crate::asset::Sprite;

pub struct Layout {
    tile_size: u32,
    tile_padding: u32,
    sprite: Sprite,
}

impl Layout {
    pub fn remap(&mut self, source: &Layout, from: (u32, u32), to: (u32, u32)) {
        let scale = source.tile_size as f32 / self.tile_size as f32;
        if scale != scale.round() {
            panic!("The quotient of the source by the scale is not equal to the target. ");
        }
        let scale = scale as u32;

        let source_offset = (source.tile_size + source.tile_padding);
        let from_pos = (from.0 * source_offset, from.1 * source_offset);

        let target_offset = (self.tile_size + self.tile_padding);
        let to_pos = (to.0 * target_offset, to.1 * target_offset);

        for y in 0..self.tile_size {
            for x in 0..self.tile_size {
                self.sprite.put_pixel(to_pos.0 + x, to_pos.1 + y, *source.sprite.get_pixel(from_pos.0 + (x * scale), from_pos.1 + (y * scale)));
            }
        }
    }
}

// x 0 1 2 3
// 0 cor u u
// 1 cor d d
// 2 v s l l
// 3 f h r r
//    flat ^

pub fn remap_tile(sprite: Sprite) -> Sprite {
    let source = Layout { tile_size: 16, tile_padding: 2, sprite };
    let mut target = Layout { tile_size: 8, tile_padding: 0, sprite: Sprite::new(96, 32) };

    // v = variant
    for v_raw in 0..3 {
        let v = v_raw as u32;
        let variant_offset = v * 4;
        // Corners. Top left \n top right \n bottom left \n bottom right
        // cor
        // cor
        target.remap(&source, (v * 2, 3),       (0 + variant_offset, 0));
        target.remap(&source, (v * 2, 4),       (0 + variant_offset, 1));
        target.remap(&source, ((v * 2) + 1, 3), (1 + variant_offset, 0));
        target.remap(&source, ((v * 2) + 1, 4), (1 + variant_offset, 1));

        // Left row
        // full \n vertical \n horizontal \n standalone
        target.remap(&source, (v + 1, 1), (0 + variant_offset, 3));
        target.remap(&source, (5, v),     (0 + variant_offset, 2));
        target.remap(&source, (6 + v, 4), (1 + variant_offset, 3));
        target.remap(&source, (9 + v, 3), (1 + variant_offset, 2));

        // Same block surrounds except direction.
        target.remap(&source, (v + 1, 0), (3 + variant_offset, 0));
        target.remap(&source, (v + 1, 2), (3 + variant_offset, 1));
        target.remap(&source, (0, v),     (3 + variant_offset, 2));
        target.remap(&source, (4, v),     (3 + variant_offset, 3));

        // Air block surrounds. Opposite is same block.
        target.remap(&source, (6 + v, 0), (2 + variant_offset, 0));
        target.remap(&source, (6 + v, 3), (2 + variant_offset, 1));
        target.remap(&source, (9, v),     (2 + variant_offset, 2));
        target.remap(&source, (12, v),    (2 + variant_offset, 3));
    };

    target.sprite
}

pub fn remap_wall(sprite: Sprite) -> Sprite {
    let source = Layout { tile_size: 32, tile_padding: 4, sprite };
    let mut target = Layout { tile_size: 16, tile_padding: 0, sprite: Sprite::new(8 * 6, 8 * 2) };
    for v_raw in 0..3 {
        let v = v_raw as u32;
        // 3x3 grid copy
        target.remap(&source, (9 + v, 3), (v, 0));
    };

    target.sprite
}


pub fn remap_wall_full(sprite: Sprite) -> Sprite {
    let source = Layout { tile_size: 32, tile_padding: 4, sprite };
    let mut target = Layout { tile_size: 16, tile_padding: 0, sprite: Sprite::new(32 * 3 * 2, 40 * 2) };

    // v = variant
    for v_raw in 0..3 {
        let v = v_raw as u32;
        let variant_offset = v * 4;
        target.remap(&source, (0, v), (0 + variant_offset, 3));
        target.remap(&source, (4, v), (0 + variant_offset, 4));
        target.remap(&source, (5, v), (2 + variant_offset, 0));
        target.remap(&source, (9, v), (1 + variant_offset, 3));
        target.remap(&source, (10, v), (2 + variant_offset, 3));
        target.remap(&source, (11, v), (2 + variant_offset, 4));
        target.remap(&source, (12, v), (1 + variant_offset, 4));
        target.remap(&source, (9 + v, 3), (1 + variant_offset, 0));
        target.remap(&source, (6 + v, 4), (3 + variant_offset, 0));
        target.remap(&source, (6 + v, 0), (1 + variant_offset, 1));
        target.remap(&source, (6 + v, 1), (2 + variant_offset, 1));
        target.remap(&source, (6 + v, 3), (1 + variant_offset, 2));
        target.remap(&source, (6 + v, 2), (2 + variant_offset, 2));
        target.remap(&source, (v + 1, 0), (0 + variant_offset, 1));
        target.remap(&source, (v + 1, 1), (0 + variant_offset, 0));
        target.remap(&source, (v + 1, 2), (0 + variant_offset, 2));
        target.remap(&source, (v * 2, 3), (3 + variant_offset, 1));
        target.remap(&source, (v * 2, 4), (3 + variant_offset, 2));
        target.remap(&source, ((v * 2) + 1, 3), (3 + variant_offset, 3));
        target.remap(&source, ((v * 2) + 1, 4), (3 + variant_offset, 4));
    };

    target.sprite
}
