use engine::core::GameCode;
use log::info;

#[allow(dead_code)]
pub struct SimpleOneFrameGame {}

impl GameCode for SimpleOneFrameGame {
    fn update(&mut self, _delta: f32) {
        info!(target: "SimpleOneFrameGame", "in game code update");
    }

    fn draw_frame(&mut self) {}

    fn should_exit(&self) -> bool {
        true
    }
}
