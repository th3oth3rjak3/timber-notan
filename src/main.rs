use notan::draw::*;
use notan::prelude::*;

use crate::state::State;

mod state;

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::new()
        .set_size(1920, 1080)
        .set_vsync(true)
        .set_title("Timber!!");

    notan::init_with(State::new)
        .add_config(win_config)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn update(app: &mut App, state: &mut State) {
    state.delta_time += app.timer.delta_f32();
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.image(&state.background).position(0.0, 0.0);

    gfx.render(&draw);
}
