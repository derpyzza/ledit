use macroquad::prelude::*;

// local imports
mod tile;
mod room;
use room::Room;
use tile::Tile;

fn window_conf() -> Conf {
    Conf {
        // fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let tilemap = load_texture("assets/test_tileset16.png")
        .await
        .expect("tilemap not found");
    tilemap.set_filter(FilterMode::Nearest);
    let mut debug = false;
    show_mouse(false);

    let mut mpos: Vec2;
    let mut cursor_pos = Vec2::ZERO;
    let mut prev_cursor_pos = Vec2::ZERO;
    let offset = 48.;
    let mut room = Room::new(0);

    loop {
        // let offset: Vec2 = vec2((screen_width() % 48.).round(), (screen_height() % 48.).round());
        // println!("{}", offset);
        mpos = vec2(mouse_position().0, mouse_position().1);
        prev_cursor_pos = cursor_pos;
        cursor_pos = vec2(( mpos.x / 48. ).round() * 48., ( mpos.y / 48. ).round() * 48.)
                    .clamp(vec2(offset, offset),
                           vec2(screen_width() - offset*2., screen_height() - offset*2.));

        if is_key_pressed(KeyCode::O) {
            debug = !debug;
        }

        clear_background(Color::from_rgba(40, 43, 46, 255));
        // draw_circle(offset.x, offset.y, 4., RED);

        if debug {
            let mut c = RED;

            if is_mouse_button_down(MouseButton::Left) {
                c = GREEN;
            }
            draw_text(format!("{},{}", 
                              (cursor_pos.x/48.).round(), 
                              (cursor_pos.y/48.).round()).as_str(),
                              30., 50., 20., RED);
            draw_text(format!("{}", get_fps()).as_str(), 30., 30., 20., RED);
        }
        if is_mouse_button_down(MouseButton::Left) && prev_cursor_pos != cursor_pos {
            &room.push_tile(Tile(3, (cursor_pos.x/48.).round() as u8, 
                                    (cursor_pos.y/48.).round() as u8));
        }
        room.draw(tilemap);
        if !is_mouse_button_down(MouseButton::Left) {
            draw_texture_ex(
                tilemap,
                cursor_pos.x,
                cursor_pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(48., 48.)),

                    source: Some(Rect::new(
                        16.,
                        0.,
                        16.,
                        16.,
                    )),

                    ..Default::default()
                },
        );
        } else {
            draw_texture_ex(
                tilemap,
                cursor_pos.x,
                cursor_pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(48., 48.)),

                    source: Some(Rect::new(
                        32.,
                        0.,
                        16.,
                        16.,
                    )),

                    ..Default::default()
                },
            );
        }
        next_frame().await
    }
}
