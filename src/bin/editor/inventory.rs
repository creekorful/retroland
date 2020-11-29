use sfml::graphics::{
    Color, Drawable, RectangleShape, RenderStates, RenderTarget, Shape, Transformable,
};
use sfml::system::{Vector2f, Vector2u};

pub struct Inventory<'s> {
    background: RectangleShape<'s>,
}

impl<'s> Inventory<'s> {
    pub fn new<T: Into<Vector2u>>(screen_size: T) -> Self {
        let screen_size = screen_size.into();

        let mut background = RectangleShape::new();
        background.set_position((50.0, 50.0));
        background.set_size(Vector2f::new(
            (screen_size.x - 50 * 2) as f32,
            (screen_size.y - 50 * 2) as f32,
        ));
        background.set_fill_color(Color::rgba(44, 62, 80, 240));

        Inventory { background }
    }
}

impl<'s> Drawable for Inventory<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw(&self.background);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_new() {
        let inventory = Inventory::new((1920, 1080));

        assert_eq!(inventory.background.position(), Vector2f::new(50.0, 50.0));
        assert_eq!(inventory.background.size(), Vector2f::new(1920.0 - 100.0, 1080.0 - 100.0));
    }
}