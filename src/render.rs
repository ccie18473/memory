use crate::prelude::*;

pub type Point2 = cgmath::Point2<f32>;
pub type Vector2 = cgmath::Vector2<f32>;

pub const CARD_WIDTH: f32 = 96.0;
pub const CARD_HEIGHT: f32 = 144.0;
pub const COLUMNS: usize = 13;
pub const ROWS: usize = 4;
pub const RUST_IMG: usize = 52;
pub const FERRIS_IMG: usize = 60;

pub struct TRender {
    pub table: TTable,
    pub resources: TResources,
    pub card_width: f32,
    pub card_height: f32,
}

impl TRender {
    pub fn new_render(ctx: &mut Context) -> TRender {
        let s = Self {
            table: TTable::new_table(),
            resources: TResources::new_resources(ctx),
            card_width: 0.0,
            card_height: 0.0,
        };
        s
    }
    pub fn game_render(&mut self, ctx: &mut Context) -> GameResult {
        let mut i = 0;
        let scale = Vector2::new(
            (self.card_width / CARD_WIDTH) as f32,
            (self.card_height / CARD_HEIGHT) as f32,
        );

        for card in self.table.game.deck.cards.iter() {
            if i < COLUMNS {
                let p = graphics::DrawParam::new()
                    .dest(Point2::new((i % COLUMNS) as f32 * self.card_width, 0.0))
                    .scale(scale);
                graphics::draw(ctx, &self.resources.get_image(card.image), p)?;
            } else if i < 2 * COLUMNS {
                let p = graphics::DrawParam::new()
                    .dest(Point2::new(
                        (i % COLUMNS) as f32 * self.card_width,
                        (self.card_height) as f32,
                    ))
                    .scale(scale);
                graphics::draw(ctx, &self.resources.get_image(card.image), p)?;
            } else if i < 3 * COLUMNS {
                let p = graphics::DrawParam::new()
                    .dest(Point2::new(
                        (i % COLUMNS) as f32* self.card_width,
                        (2.0 * self.card_height) as f32,
                    ))
                    .scale(scale);
                graphics::draw(ctx, &self.resources.get_image(card.image), p)?;
            } else if i < 4 * COLUMNS {
                let p = graphics::DrawParam::new()
                    .dest(Point2::new(
                        (i % COLUMNS) as f32 * self.card_width,
                        (3.0 * self.card_height) as f32,
                    ))
                    .scale(scale);
                graphics::draw(ctx, &self.resources.get_image(card.image), p)?;
            }
            i += 1;
        }

        Ok(())
    }
    pub fn fps_render(&mut self, ctx: &mut Context) -> GameResult {
        let fps = timer::fps(ctx);
        let fps_display = graphics::Text::new(format!("FPS: {:.2}", fps));
        let p = cgmath::Point2::new(
            2.0 * self.card_width,
            ROWS as f32 * self.card_height,
        );
        graphics::draw(ctx, &fps_display, (p,))?;

        Ok(())
    }
    pub fn duration_render(&mut self, ctx: &mut Context) -> GameResult {
        let secs_display = graphics::Text::new(format!("Duration: {}", self.table.game.duration));
        let p = cgmath::Point2::new(
            6.0 * self.card_width,
            ROWS as f32 * self.card_height,
        );
        graphics::draw(ctx, &secs_display, (p,))?;

        Ok(())
    }
    pub fn moves_render(&mut self, ctx: &mut Context) -> GameResult {
        let moves_display = graphics::Text::new(format!("Moves: {}", self.table.game.moves));
        let p = cgmath::Point2::new(
            10.0 * self.card_width,
            ROWS as f32 * self.card_height,
        );
        graphics::draw(ctx, &moves_display, (p,))?;

        Ok(())
    }
    pub fn display(&mut self, ctx: &mut Context) -> GameResult {
        self.game_render(ctx).unwrap();
        self.fps_render(ctx).unwrap();
        self.duration_render(ctx).unwrap();
        self.moves_render(ctx).unwrap();

        Ok(())
    }
}
