extern crate good_web_game as ggez;

mod deck;
mod game;
mod render;
mod resources;
mod table;

mod prelude {
    pub use crate::deck::*;
    pub use crate::game::*;
    pub use crate::render::*;
    pub use crate::resources::*;
    pub use crate::table::*;
    pub use cgmath::prelude::*;
    pub use ggez::event::{self, MouseButton};
    pub use ggez::graphics::{self, Color, Rect};
    pub use ggez::timer;
    pub use ggez::{Context, GameResult};
    pub use quad_rand as qrand;
    pub use std::env;
    pub use std::path;
    pub use std::time::{Duration, SystemTime};
    pub use std::{thread, time};
}

use prelude::*;

pub const STATUS_BAR: f32 = 20.0;

struct GameState {
    render: TRender,
    mouse_down: bool,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let (w, h) = graphics::drawable_size(ctx);
        let mut render = TRender::new_render(ctx);
        render.card_width = w / COLUMNS as f32;
        render.card_height = (h - STATUS_BAR) / ROWS as f32;
        render.table.card_width = w / COLUMNS as f32;
        render.table.card_height = (h - STATUS_BAR) / ROWS as f32;

        let s = GameState {
            render,
            mouse_down: false,
        };

        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // This runs MAX_FPS time per second

        const INTERVAL: u32 = 1;

        while timer::check_update_time(ctx, INTERVAL) {
            if !self.render.table.game.eog_flag {
                self.render.table.game.duration += 1;
                if self.render.table.game.duration % 3 == 0 {
                    self.render.table.game.win_check();
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // This runs MAX_FPS time per second

        graphics::clear(ctx, Color::BLUE);

        self.render.display(ctx).unwrap();

        graphics::present(ctx)?;

        Ok(())
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.mouse_down = true;
        self.render.table.process_mouse_down(x, y);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        self.mouse_down = false;
        self.render.table.process_mouse_up(x, y);
    }
    fn resize_event(&mut self, ctx: &mut Context, w: f32, h: f32) {
        self.render.card_width = w / COLUMNS as f32;
        self.render.card_height = (h - STATUS_BAR) / ROWS as f32;
        self.render.table.card_width = w / COLUMNS as f32;
        self.render.table.card_height = (h - STATUS_BAR) / ROWS as f32;
        let coordinates = graphics::Rect::new(0., 0.0, w, h);

        graphics::set_screen_coordinates(ctx, coordinates).expect("Can't resize the window");
    }
}

pub fn main() -> GameResult {
    //let state = GameState::new(&mut context).unwrap();

    let conf = ggez::conf::Conf::default()
        .cache(miniquad::conf::Cache::Tar(include_bytes!(
            "../resources/resources.tar"
        )))
        .physical_root_dir(Some(path::PathBuf::from("../resources/")))
        .window_title("Memory v1.0.0, 2021".to_string());

    ggez::start(conf, |mut context| {
        Box::new(GameState::new(&mut context).unwrap())
    })
}
