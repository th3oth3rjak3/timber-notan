use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
pub struct State {
    pub delta_time: f32,
    pub rng: Random,

    // Assets
    pub font: Font,
    pub background: Texture,
}

impl State {
    pub fn new(gfx: &mut Graphics) -> Self {
        let rng = Random::default();
        let font = gfx
            .create_font(include_bytes!("../assets/fonts/KOMIKAP_.ttf"))
            .unwrap();

        let background = gfx
            .create_texture()
            .from_image(include_bytes!("../assets/graphics/background.png"))
            .build()
            .unwrap();

        Self {
            delta_time: 0.0,
            rng,
            font,
            background,
        }
    }
}
