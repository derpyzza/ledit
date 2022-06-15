use macroquad::prelude::*;
mod room;
mod world;
mod game;

#[macroquad::main("BasicShapes")]
async fn main() {
    let tilemap = load_texture("/home/derpyzza/projects/ledit/assets/test_tileset.png").await.expect("tilemap not found");
    tilemap.set_filter(FilterMode::Nearest);
    let mut DEBUG = false;
    let game = game::Game::new(&tilemap, 8);
    let tile:u8 = 1; // temp
    let mut id: u8 = 1;
    let mut world = world::World::new();
    world.init();
    // let mut room = room::Room::new( world.to_screen( vec2( 20., 30. ) ) );
    let mut start_mpos = Vec2::ZERO;
    let mut mpos = Vec2::ZERO;
    world.offset = vec2( screen_width() / 2., screen_height() / 2. );
    loop {

        // room.pos = world.to_screen( vec2( 20., 30. ) );
        mpos = vec2(mouse_position().0, mouse_position().1);
        let coords = world.to_screen(vec2(0., 0.));
        let w_origin = vec2(0., 0.) + world.offset;

        if is_mouse_button_pressed(MouseButton::Left) {
            start_mpos = mpos;
        }

        if is_mouse_button_down(MouseButton::Left) &&
           is_key_down(KeyCode::Space) {
            world.offset += mpos - start_mpos;
            start_mpos = mpos;
        }

        if is_key_pressed(KeyCode::L) {
            world.room_init(&mut id, mpos);
            println!("{:?}", world.to_world(mpos));
        }

        if is_key_pressed(KeyCode::O) {
            DEBUG = !DEBUG;
        }


        // println!("{:?}", coords);
        clear_background(WHITE);
        if DEBUG {
            let mut c = RED;
            if is_mouse_button_down(MouseButton::Left) {
                c = GREEN;
            }
            draw_line(mpos.x, 0., mpos.x, screen_height(), 2., c);
            draw_line(0., mpos.y, screen_width(), mpos.y, 2., c);
            draw_line(mpos.x, mpos.y, coords.x, coords.y, 2., c);
            draw_text(format!("({}:{})", mpos.x, mpos.y).as_str(), mpos.x + 20., mpos.y - 10., 20., c);
            draw_text(format!("({}:{})", world.to_world(mpos).x, world.to_world(mpos).y).as_str(), mpos.x + 20., mpos.y + 20., 20., c);
            draw_text(format!("({}, {})", w_origin.x, w_origin.y).as_str(),
                w_origin.x - 30., w_origin.y - 30., 20., GRAY);

            draw_text(format!("{}", get_fps()).as_str(), 30., 30., 20., RED);
        }

        for room in world.rooms.iter() {
            match room {
                Some(room) => {
                    room.draw(&game, world.offset);
                },
                None => {
                    
                }
            }
        }
        // room.draw(&game);
        // draw_texture_ex(tilemap, coords.x, coords.y, WHITE, 
        // DrawTextureParams
        // {
        //     dest_size: Some(vec2(32.0, 32.0)),
        //     source: Some(Rect::new((tile*game.tile_size) as f32, 0., 8., 8.)),
        //     ..Default::default()
        // });
        draw_line(w_origin.x, 0., w_origin.x, screen_height(), 2., GRAY);
        draw_line(0., w_origin.y, screen_width(), w_origin.y, 2., GRAY);

        next_frame().await
    }
}
