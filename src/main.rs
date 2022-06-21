use macroquad::prelude::*;

mod room;
use room::Room;

#[macroquad::main("BasicShapes")]
async fn main() {
    let tilemap = load_texture("assets/test_tileset16.png")
        .await
        .expect("tilemap not found");
    tilemap.set_filter(FilterMode::Nearest);
    let mut debug = false;
    show_mouse(false);

    let mut start_mpos: Vec2 = Vec2::ZERO;
    let mut mpos: Vec2;
    
    loop {
        if is_key_pressed(KeyCode::O) {
            debug = !debug;
        }

        clear_background(Color::from_rgba(40, 43, 46, 255));

        if debug {
            let mut c = RED;

            if is_mouse_button_down(MouseButton::Left) {
                c = GREEN;
            }

            draw_text(format!("{}", get_fps()).as_str(), 30., 30., 20., RED);
        }
        if !is_mouse_button_down(MouseButton::Left) {
            draw_texture_ex(
                tilemap,
                mouse_position().0,
                mouse_position().1,
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
                mouse_position().0,
                mouse_position().1,
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
