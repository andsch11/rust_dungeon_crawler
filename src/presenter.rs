use crate::model::Model;
use crate::view::View;
use bracket_lib::prelude::{BTerm, GameState};

pub struct Presenter {
    model: Model,
    view: View,
}

impl Presenter {
    pub fn new(model: Model, view: View) -> Self {
        Self { model, view }
    }
}

impl GameState for Presenter {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.view.cls(ctx);
        let movement = self.view.get_keycode_movement(ctx);
        self.model.game_logic_update(movement);
        self.view.render_map(ctx, &self.model);
        self.view.render_player(ctx, &self.model);
    }
}
