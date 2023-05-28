use std::time::{Instant, Duration};

use sdl2::
{
    self,
    keyboard::{Keycode, Scancode},
    render::{Canvas, Texture},
    video,
    pixels::Color,
    rect::Rect,
    image::LoadTexture
};

mod tile;
use tile::tile::Tile;

mod initialization;
use initialization::init::init;

fn main()
{
    let (mut canvas, numbered_map, mut events) = init();
    let texture_creator = canvas.texture_creator();

    let mut tile_map: Vec<Vec<Tile>> = vec![];
    let mut counter = (0, 0);
    for array in numbered_map
    {
        let mut temp_vec = vec![];
        for item in array
        {
            match item
            {
                0 => temp_vec.push(Tile::new(texture_creator.load_texture("assets/grass_tile.png").unwrap(), counter)),
                1 => temp_vec.push(Tile::new(texture_creator.load_texture("assets/path_tile.png").unwrap(), counter)),
                2 => temp_vec.push(Tile::new(texture_creator.load_texture("assets/stone_tile.png").unwrap(), counter)),
                _ => ()
            }
            counter.0 += 1;
        }
        tile_map.push(temp_vec);
        counter.1 += 1;
        counter.0 = 0;
    }

    let player = Player
    {
        texture: texture_creator.load_texture("assets/player_states/player.png").unwrap(),
        position: (384, 193),
        size: (64, 64),
    };

    let frame_delay = 1000000/60;
    let game_time = Instant::now();
    let mut frames = 0.;

    'gameloop: loop
    {
        let start_instant = Instant::now();
        for event in events.poll_iter()
        {
            match event
            {
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Escape), ..}
                    | sdl2::event::Event::Quit { .. }
                    => break 'gameloop,

                _ => ()
            }
        }
        
        let keyboard_state = events.keyboard_state();
        let tile_map_len = tile_map.len();
        let array_len = tile_map[tile_map_len-1].len();

        if keyboard_state.is_scancode_pressed(Scancode::D) && tile_map[tile_map_len-1][array_len-1].position.0 - 64 + tile_map[tile_map_len-1][array_len-1].size.0 as i32 > 450 
        {
            move_pos(&mut tile_map, 0);
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) && tile_map[tile_map_len-1][array_len-1].position.1 - 64 + tile_map[tile_map_len-1][array_len-1].size.1 as i32> 260
        {
            move_pos(&mut tile_map, 1);
        }
        if keyboard_state.is_scancode_pressed(Scancode::A) && tile_map[0][0].position.0 + 64 < 385
        {
            move_pos(&mut tile_map, 2);
        }
        if keyboard_state.is_scancode_pressed(Scancode::W) && tile_map[0][0].position.1 + 64 < 190
        {
            move_pos(&mut tile_map, 3);
        }

        render(&mut canvas, &mut tile_map, &player);

        let end_instant = start_instant.elapsed().as_micros();
        if end_instant < frame_delay
        {
            std::thread::sleep(Duration::from_micros((frame_delay-end_instant-750) as u64));
        }
        frames += 1.;
    }
    println!("Average fps = {}", frames/game_time.elapsed().as_secs_f64());
}

fn render(canvas: &mut Canvas<video::Window>, tile_map: &mut Vec<Vec<Tile>>, player: &Player)
{
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();

    for array in tile_map
    {
        for tile in array
        {
            canvas.copy(&tile.texture,
                Rect::new(0, 0, 16, 16),
                Rect::new(tile.position.0, tile.position.1, tile.size.0, tile.size.1))
                .unwrap();
        }
    }
    canvas.copy(&player.texture,
        Rect::new(0, 0, 32, 32),
        Rect::new(player.position.0, player.position.1, player.size.0, player.size.1))
        .unwrap();

    canvas.present();
}

fn move_pos(tile_map: &mut Vec<Vec<Tile>>, dir: u8)
{
    for array in tile_map
    {
        for tile in array
        {
            tile.move_tile(dir);
        }
    }
}
struct Player<'a> {
    texture: Texture<'a>,
    position: (i32, i32),
    size: (u32, u32),
}