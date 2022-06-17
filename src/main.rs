use macroquad::prelude::*;
mod game;
mod room;
mod world;

enum GameMode {
    LevelEdit,
    TileEdit,
    PlayLevel,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let tilemap = load_texture("assets/test_tileset16.png")
        .await
        .expect("tilemap not found");
    tilemap.set_filter(FilterMode::Nearest);
    let mut debug = false;

    let mut wld = world::World::new(); // WORLD. i am a lazy typer
    wld.origin = vec2(screen_width() / 2., screen_height() / 2.); // Set the origin of the world to the middle of the screen

    // init game data
    let game = game::Game::new(&tilemap, 16, 32.);

    let mut start_mpos: Vec2 = Vec2::ZERO;
    let mut mpos: Vec2;

    let mut rooms: [Option<room::Room>; 6] = Default::default();
    let room = &rooms[0];

    loop {
        rooms[0] = Some(room::Room::new(0, vec2(-10., -10.)));
        mpos = vec2(mouse_position().0, mouse_position().1);

        if is_key_pressed(KeyCode::O) {
            debug = !debug;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            start_mpos = mpos;
        }

        if is_key_pressed(KeyCode::L) {
            let mut id = 0;
            for room in &mut rooms {
                match room {
                    Some(room) => {} // if room actually exists, no need to initialize that;
                    None => {
                        *room = Some(room::Room::new(id, -wld.to_screen(mpos)));
                        break; // break for loop after initializing a room.
                               // we dont want to initialize every room in the
                               // list, just one :)
                    }
                }
                id += 1;
            }
        }

        clear_background(WHITE);

        if is_mouse_button_down(MouseButton::Left) && is_key_down(KeyCode::Space) {
            wld.origin += mpos - start_mpos;
            start_mpos = mpos;

            if debug {
                draw_text("panning!", 10., screen_height() - 20., 20., RED);
            }
        }

        if debug {
            let mut c = RED;

            if is_mouse_button_down(MouseButton::Left) {
                c = GREEN;
            }

            draw_line(mpos.x, 0., mpos.x, screen_height(), 2., c);
            draw_line(0., mpos.y, screen_width(), mpos.y, 2., c);
            draw_text(
                format!("({}:{})", mpos.x, mpos.y).as_str(),
                mpos.x + 20.,
                mpos.y - 10.,
                20.,
                c,
            );

            draw_text(format!("{}", get_fps()).as_str(), 30., 30., 20., RED);
            draw_line(wld.origin.x, wld.origin.y, mpos.x, mpos.y, 2., GREEN);
            draw_text(
                format!("{:?}", mpos - wld.origin).as_str(),
                mpos.x - 100.,
                mpos.y + 40.,
                20.,
                GREEN,
            );
        }

        for room in rooms.iter_mut() {
            match room {
                Some(room) => {
                    // if is_key_pressed(KeyCode::J) && room.is_hover(&game) {
                    //     start_mpos = mpos;
                    //
                    // }
                    //
                    // if is_key_down(KeyCode::J) && room.is_hover(&game) {
                    //
                    //     room.col = GREEN;
                    //     room.pos += mpos - start_mpos;
                    //     start_mpos = mpos;
                    //
                    // }
                    for x in 0..32 {
                        room.tiles.push((1, x, x));
                    }

                    room.draw(&wld, &game);
                }

                None => {} // if room doesn't exist do nothing :P
            }
        }

        draw_text(
            "TEST_STAGE_0000",
            wld.origin.x + 20.,
            wld.origin.y - 20.,
            20.,
            GRAY,
        );
        draw_circle(wld.origin.x, wld.origin.y, 2., GRAY);
        draw_line(wld.origin.x, 0., wld.origin.x, screen_height(), 2., GRAY);
        draw_line(0., wld.origin.y, screen_width(), wld.origin.y, 2., GRAY);

        next_frame().await
    }
}
