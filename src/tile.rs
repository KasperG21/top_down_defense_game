pub mod tile
{
    use sdl2::{render::{Texture}};

    pub struct Tile<'a>
    {
        pub texture: Texture<'a>,
        pub position: (i32, i32),
        pub size: (u32, u32),
    }

    impl<'a> Tile<'a>
    {
        pub fn new(texture: Texture<'a>, raw_pos: (i32, i32)) -> Self
        {
            Tile
            {
                texture: texture,
                position: (raw_pos.0 * 64, raw_pos.1 * 64),
                size: (64, 64),
            }
        }

        pub fn move_tile(&mut self, dir: u8)
        {
            match dir
            {
                0 => self.position.0 += -5,
                1 => self.position.1 += -5,
                2 => self.position.0 += 5,
                3 => self.position.1 += 5,
                _ => (),
            }
        }
    }
}