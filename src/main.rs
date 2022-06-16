use macroquad::prelude::*;
mod world;
mod room;
mod game;

type i32 = f32;

#[macroquad::main("BasicShapes")]
async fn main() {
    let tilemap = load_texture("/home/derpyzza/projects/ledit/assets/test_tileset.png").await.expect("tilemap not found");
    tilemap.set_filter(FilterMode::Nearest);
    let mut DEBUG = false;
    let mut room_select = true;
    let (tile , tile_size) = (1, 8);
    let mut wld  = world::World::new(); // WORLD. i am a lazy typer
    wld.origin = vec2(screen_width() / 2., screen_height() / 2.);
    let game = game::Game::new(&tilemap, 8);
    let mut start_mpos = Vec2::ZERO;
    let mut mpos = Vec2::ZERO;
    let mut rooms: [Option<room::Room>; 6 ] = Default::default();
    let mut room = &rooms[0];
    loop {
        rooms[0] = Some(room::Room::new(0, vec2(-10., -10.)));
        mpos = vec2(mouse_position().0, mouse_position().1);
        if is_key_pressed(KeyCode::O) {
            DEBUG = !DEBUG;
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            start_mpos = mpos;
        }

        if is_mouse_button_down(MouseButton::Left) &&
           is_key_down(KeyCode::Space) {
                wld.origin += mpos - start_mpos;
                start_mpos = mpos;
        }

        if is_key_pressed(KeyCode::L) {
            let mut id = 0;
            for room in &mut rooms {
                match room {
                    Some(room) => {},
                    None => {
                        // println!("mpos is {}, {}, {}", mpos, wld.origin, mpos - wld.origin);
                        *room = Some(room::Room::new(id, -wld.to_screen(mpos)));
                        // println!("mpos is {}, {}, {}", mpos, wld.origin, mpos - wld.origin);
                        // println!("created room at {:?}", mpos);
                        break;
                    },
                }
                id += 1;
            }
        }

        clear_background(WHITE);
        if DEBUG {
            let mut c = RED;
            if is_mouse_button_down(MouseButton::Left) {
                c = GREEN;
            }
            draw_line(mpos.x, 0., mpos.x, screen_height(), 2., c);
            draw_line(0., mpos.y, screen_width(), mpos.y, 2., c);
            draw_text(format!("({}:{})", mpos.x, mpos.y).as_str(), mpos.x + 20., mpos.y - 10., 20., c);
            draw_text(format!("{}", get_fps()).as_str(), 30., 30., 20., RED);
            // draw_line(0., 0., wld.origin.x, wld.origin.y, 4., GREEN);
            draw_line(wld.origin.x, wld.origin.y, mpos.x, mpos.y, 2., GREEN);
            draw_text(format!("{:?}", mpos - wld.origin).as_str(), 
                mpos.x - 100., mpos.y + 40., 20., GREEN);
        }

        let pos = wld.to_world(vec2(30., 20.));
        // draw_texture_ex(tilemap, pos.x, pos.y, WHITE,
        // DrawTextureParams
        // {
        //     dest_size: Some(vec2(32.0, 32.0)),
        //     source: Some(Rect::new((tile*tile_size) as f32, 0., 8., 8.)),
        //     ..Default::default()
        // });

        for room in rooms.iter_mut() {
            match room {
                Some(room) => {
                    // room.update(&wld);
                    if is_key_pressed(KeyCode::J) && room.is_hover() {
                        start_mpos = mpos;
                    }

                    if is_key_down(KeyCode::J) && room.is_hover() {
                        room.col = GREEN;
                        room.pos += mpos - start_mpos;
                        start_mpos = mpos;
                    }
                    room.tiles.push( ( 2, 8, 0 ) );
                    room.draw(&wld, &game);
                },
                None => {
                    
                }
            }
        }


        draw_text("TEST_STAGE_0000", wld.origin.x + 20., wld.origin.y - 20., 20., GRAY);
        draw_circle(wld.origin.x, wld.origin.y, 2., GRAY);
        draw_line(wld.origin.x, 0., wld.origin.x, screen_height(), 2., GRAY);
        draw_line(0., wld.origin.y, screen_width(), wld.origin.y, 2., GRAY);

        next_frame().await
    }
}
