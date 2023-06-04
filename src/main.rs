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

    let mut player = Player
    {
        texture: texture_creator.load_texture("assets/player_states/player_v1.png").unwrap(),
        position: (384, 193),
        size: (40, 64),
    };

    let george = Npc::spawn(texture_creator.load_texture("assets/NPCs/George.png").unwrap(), "George");

    let mut npcs = vec![george];

    let mut in_dialogue = false;

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

        let mut test = true;
        let mut test_2 = true;
        let mut test_3 = true;
        let mut test_4 = true;
        for n in npcs.iter_mut()
        {
            //seamingly random numbers added for the sake of smoothness

            if player.position.0 + player.size.0 as i32 >= n.position.0 &&
                player.position.0 + player.size.0 as i32 <= n.position.0 + n.size.0 as i32 &&
                    player.position.1 + player.size.1 as i32 >= n.position.1 &&
                    player.position.1 <= n.position.1 + n.size.1 as i32
                    {
                        test = false;
                    }
            if player.position.0 <= n.position.0 + n.size.0 as i32&&
                player.position.0 >= n.position.0 &&
                    player.position.1 + player.size.1 as i32 >= n.position.1 &&
                    player.position.1 <= n.position.1 + n.size.1 as i32
                    {
                        test_3 = false;
                    }
            if player.position.0 + 10 >= n.position.0 &&
                player.position.0 <= n.position.0 + n.size.0 as i32 - 5 &&
                    player.position.1 + player.size.1 as i32 + 10 > n.position.1 &&
                    player.position.1 < n.position.1 + n.size.1 as i32
                    {
                        test_2 = false;
                    }
            if player.position.0 + 10 >= n.position.0 &&
                player.position.0 <= n.position.0 + n.size.0 as i32 &&
                    player.position.1 < n.position.1 + n.size.1 as i32 + 10 &&
                    player.position.1 > n.position.1 + n.size.1 as i32 - 5
                    {
                        test_4 = false;
                    }
            //checking if "E" is pressed and if the player is in the range of one of the npcs
            if keyboard_state.is_scancode_pressed(Scancode::E) 
            {
                let mut count = 0;
                if test { count += 1 }
                if test_2 { count += 1 }
                if test_3 { count += 1 }
                if test_4 { count += 1 }
                
                if count < 4
                {
                    n.start_dialogue(&mut player, &mut in_dialogue);
                }
            }
        }

        if keyboard_state.is_scancode_pressed(Scancode::D) && !in_dialogue && test &&tile_map[tile_map_len-1][array_len-1].position.0 - 40 + tile_map[tile_map_len-1][array_len-1].size.0 as i32 > 450 
        {
            move_pos(&mut tile_map, &mut npcs, 0);
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) && !in_dialogue && test_2 &&tile_map[tile_map_len-1][array_len-1].position.1 - 64 + tile_map[tile_map_len-1][array_len-1].size.1 as i32> 260
        {
            move_pos(&mut tile_map, &mut npcs, 1);
        }
        if keyboard_state.is_scancode_pressed(Scancode::A) && !in_dialogue&& test_3 && tile_map[0][0].position.0 + 64 < 385
        {
            move_pos(&mut tile_map, &mut npcs, 2);
        }
        if keyboard_state.is_scancode_pressed(Scancode::W) && !in_dialogue&& test_4 && tile_map[0][0].position.1 + 64 < 190
        {
            move_pos(&mut tile_map, &mut npcs, 3);
        }

        if in_dialogue
        {
            if keyboard_state.is_scancode_pressed(Scancode::Return)
            {
                in_dialogue = false; 
            }
            render(&mut canvas, &mut tile_map, &player, &npcs, Some(&texture_creator.load_texture("assets/dialogue_bg.png").unwrap()));
        }
        else
        {
            render(&mut canvas, &mut tile_map, &player, &npcs, None); 
        }

        let end_instant = start_instant.elapsed().as_micros();
        if end_instant < frame_delay
        {
            std::thread::sleep(Duration::from_micros((frame_delay-end_instant-750) as u64));
        }
        frames += 1.;
    }
    println!("Average fps = {}", frames/game_time.elapsed().as_secs_f64());
}

fn render(canvas: &mut Canvas<video::Window>, tile_map: &mut Vec<Vec<Tile>>, player: &Player, npcs: &Vec<Npc>, in_dialogue: Option<&Texture>)
{
    canvas.set_draw_color(Color::BLACK);
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

    for n in npcs
    {
        canvas.copy(&n.texture,
                    Rect::new(0, 0, 32, 32),
                    Rect::new(n.position.0, n.position.1, n.size.0, n.size.1))
            .unwrap();
    }

    canvas.copy(&player.texture,
                Rect::new(0, 0, 12, 25),
                Rect::new(player.position.0, player.position.1, player.size.0, player.size.1))
        .unwrap();

    if let Some(x) = in_dialogue
    {
        canvas.copy(
            x,
            Rect::new(0, 0, 800, 250),
            Rect::new(0, 200, 800, 250))
            .unwrap();
    }

    canvas.present();
}

fn move_pos(tile_map: &mut Vec<Vec<Tile>>, npcs: &mut Vec<Npc>, dir: u8)
{
    for array in tile_map
    {
        for tile in array
        {
            tile.move_tile(dir);
        }
    }
    
    for n in npcs
    { 
        n.move_npc(dir) 
    }
}
struct Player<'a> {
    texture: Texture<'a>,
    position: (i32, i32),
    size: (u32, u32),
}

struct Npc<'a>
{
    texture: Texture<'a>, 
    position: (i32, i32),
    size: (u32, u32),
    name: &'a str,
}

impl<'a> Npc<'a>
{
    fn spawn(texture: Texture<'a>, name: &'a str) -> Self
    {
        match name
        {
            "George" => {
                Npc
                {
                    texture,
                    position: (1280, 192),
                    size: (64, 64),
                    name,
                }
            }
            _ => Npc::spawn(texture, "George"),
        }
    }

    fn move_npc(&mut self, dir: u8)
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
    
    fn start_dialogue(&self, player: &mut Player, in_dialogue: &mut bool)
    {
        if !*in_dialogue
        {
            *in_dialogue = true; 
            println!("In dialogue with {}", self.name);
        }
    }
}
