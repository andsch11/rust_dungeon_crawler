use crate::model::{GetPlayerPosition, Model};
use crate::shared_types::CameraView;
use crate::view::View;
use bracket_lib::prelude::{BTerm, GameState, Point};

pub struct Presenter {
    model: Model,
    view: View,
    camera: Camera,
}

impl Presenter {
    pub fn new(model: Model, view: View) -> Self {
        let camera = Camera::new(model.get_player_position(), view.width, view.height);
        Self {
            model,
            view,
            camera,
        }
    }
}

impl GameState for Presenter {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.view.cls(ctx);
        let movement = self.view.get_keycode_movement(ctx);
        self.model.game_logic_update(movement);
        self.camera.on_player_move(self.model.get_player_position());
        self.view
            .render_map(ctx, &self.model, &self.camera.camera_view, &self.model);
        self.view
            .render_player(ctx, &self.model, &self.camera.camera_view);
    }
}

struct Camera {
    camera_view: CameraView,
    display_width: usize,
    display_height: usize,
}

impl Camera {
    pub fn new(player_position: Point, display_width: usize, display_height: usize) -> Self {
        let camera_view = CameraView {
            left_x: Self::get_left_x(player_position, display_width),
            right_x: Self::get_right_x(player_position, display_width),
            top_y: Self::get_top_y(player_position, display_height),
            bottom_y: Self::get_bottom_y(player_position, display_height),
        };
        Self {
            camera_view,
            display_width,
            display_height,
        }
    }

    pub fn on_player_move(&mut self, player_position: Point) {
        self.camera_view.left_x = Self::get_left_x(player_position, self.display_width);
        self.camera_view.right_x = Self::get_right_x(player_position, self.display_width);
        self.camera_view.top_y = Self::get_top_y(player_position, self.display_height);
        self.camera_view.bottom_y = Self::get_bottom_y(player_position, self.display_height);
    }
    fn get_left_x(player_position: Point, display_width: usize) -> i32 {
        player_position.x - display_width as i32 / 2
    }
    fn get_right_x(player_position: Point, display_width: usize) -> i32 {
        player_position.x + display_width as i32 / 2
    }
    fn get_bottom_y(player_position: Point, display_height: usize) -> i32 {
        player_position.y + display_height as i32 / 2
    }
    fn get_top_y(player_position: Point, display_height: usize) -> i32 {
        player_position.y - display_height as i32 / 2
    }
}
