use macroquad::prelude::*;
mod game;
mod room;
mod world;
use room::Room;
use world::World;
use game::Game;

#[derive(Eq, PartialEq)]
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

    let mut wld = World::new(); // WORLD. i am a lazy typer
    wld.origin = vec2(screen_width() / 2., screen_height() / 2.); // Set the origin of the world to the middle of the screen

    // init game data
    let game = Game::new(&tilemap, 16, 32.);
    let tile_size = game._onscreen_size();

    let mut start_mpos: Vec2 = Vec2::ZERO;
    let mut mpos: Vec2;

    let mut rooms: [Option<Room>; 6] = Default::default();
    let mut mode = GameMode::LevelEdit;
    rooms[0] = Some(Room::new(0, vec2(-10., -10.)));
    let mut active_room = &rooms[0].clone();

    loop {
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
                        let size = tile_size * 32.;
                        let pos = vec2(mpos.x - (size / 2.), mpos.y - (size / 2.));
                        *room = Some(room::Room::new(id, -wld.to_screen(pos)));
                        break; // break for loop after initializing a room.
                               // we dont want to initialize every room in the
                               // list, just one :)
                    }
                }
                id += 1;
            }
        }

        match mouse_wheel() {
            (_x, y) if y != 0.0 => {
                // Normalize mouse wheel values is browser (chromium: 53, firefox: 3)
                #[cfg(target_arch = "wasm32")]
                let y = if y < 0.0 {
                    -1.0
                } else if y > 0.0 {
                    1.0
                } else {
                    0.0
                };
                if is_key_down(KeyCode::LeftShift) {
                    wld.scale *= 1.1f32.powf(y);
                }
            }
            _ => (),
        }

        clear_background(Color::from_rgba(40, 43, 46, 255));

        if mode == GameMode::LevelEdit {
            println!("level edit");
            match active_room {
                Some(room) => {
                    let size = tile_size * room.size;
                    println!("ok");
                    macroquad::prelude::draw_rectangle(
                        mpos.x - size / 2.,
                        mpos.y - size / 2.,
                        size,
                        size,
                        Color::from_rgba(57, 55, 64, 155),
                    );
                }
                None => {}
            }
        }

        if is_mouse_button_down(MouseButton::Left) && is_key_down(KeyCode::Space) {
            wld.origin += mpos - start_mpos;
            start_mpos = mpos;

            if debug {
                draw_text("panning!", 10., screen_height() - 20., 20., RED);
            }
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
                    // for x in 0..32 {
                    //     room.tiles.push((1, x, x));
                    // }

                    room.draw(&wld, &game);
                }

                None => {} // if room doesn't exist do nothing :P
            }
        }

        draw_text(
            "TEST_STAGE_0000",
            wld.origin.x + 20.,
            wld.origin.y - 20.,
            30.,
            GRAY,
        );
        let wld_col = Color::from_rgba(29, 28, 33, 255);
        draw_circle(wld.origin.x, wld.origin.y, 2., wld_col);
        draw_line(wld.origin.x, 0., wld.origin.x, screen_height(), 4., wld_col);
        draw_line(0., wld.origin.y, screen_width(), wld.origin.y, 4., wld_col);
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

        next_frame().await
    }
}
