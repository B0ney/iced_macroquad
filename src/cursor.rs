use macroquad::miniquad::CursorIcon;

pub trait CursorSubscriber {
    fn update(&mut self, icon: CursorIcon);
    fn reset(&mut self) {
        self.update(CursorIcon::Default);
    }
}
