use sfml::graphics::{
    Color, Drawable, RectangleShape, RenderStates, RenderTarget, Shape, Texture, Transformable,
};
use sfml::system::{SfBox, Vector2f, Vector2u};
use std::collections::BTreeMap;

pub struct Inventory<'s> {
    background: RectangleShape<'s>,
    items: Vec<RectangleShape<'s>>,
    items_id: BTreeMap<u32, u32>,
}

impl<'s> Inventory<'s> {
    pub fn new<T: Into<Vector2u>>(
        screen_size: T,
        textures: &'s BTreeMap<u32, SfBox<Texture>>,
    ) -> Self {
        let screen_size = screen_size.into();

        let bg_border = 50.0; // TODO make 50 something calculated
        let mut background = RectangleShape::new();
        background.set_position((bg_border, bg_border));
        background.set_size(Vector2f::new(
            screen_size.x as f32 - bg_border * 2.0,
            screen_size.y as f32 - bg_border * 2.0,
        ));
        background.set_fill_color(Color::rgba(44, 62, 80, 240));

        let item_border = 25.0; // TODO make 25 something calculated
        let item_size = 100.0; // TODO make 100 something calculated
        let mut items = Vec::new();
        let mut x = 0;
        let mut y = 0;

        let mut items_id = BTreeMap::new();
        for (id, texture) in textures {
            // Determinate if x position will overlaps background and therefore
            // need to do a new line
            if x as f32 * (item_size + item_border) + item_border + item_size
                >= background.size().x - item_border
            {
                x = 0;
                y += 1;
            }

            let mut item = RectangleShape::new();
            item.set_position(Vector2f::new(
                background.position().x + (x as f32 * (item_size + item_border) + item_border),
                background.position().y + (y as f32 * (item_size + item_border) + item_border),
            ));
            item.set_size((item_size, item_size));
            item.set_texture(texture, true);
            items.push(item);

            items_id.insert(x + y, *id);

            x += 1;
        }

        Inventory {
            background,
            items,
            items_id,
        }
    }
}

impl<'s> Drawable for Inventory<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_with_renderstates(&self.background, states);

        for item in &self.items {
            target.draw_with_renderstates(item, states);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_new() {
        let mut textures = BTreeMap::new();
        textures.insert(10, Texture::new(16, 16).unwrap());
        textures.insert(22, Texture::new(16, 16).unwrap());
        textures.insert(34, Texture::new(16, 16).unwrap());

        let inventory = Inventory::new((1920, 1080), &textures);

        assert_eq!(inventory.background.position(), Vector2f::new(50.0, 50.0));
        assert_eq!(
            inventory.background.size(),
            Vector2f::new(1920.0 - 100.0, 1080.0 - 100.0)
        );
        assert_eq!(inventory.items.len(), 3);
        assert_eq!(
            inventory.items.get(0).unwrap().size(),
            Vector2f::new(100.0, 100.0)
        );
        assert_eq!(*inventory.items_id.get(&0).unwrap(), 10);
        assert_eq!(*inventory.items_id.get(&1).unwrap(), 22);
        assert_eq!(*inventory.items_id.get(&2).unwrap(), 34);
    }
}
