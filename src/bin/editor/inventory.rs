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

        let bg_border = 50.0; // TODO make 50 something calculated?
        let mut background = RectangleShape::new();
        background.set_position((bg_border, bg_border));
        background.set_size(Vector2f::new(
            screen_size.x as f32 - bg_border * 2.0,
            screen_size.y as f32 - bg_border * 2.0,
        ));
        background.set_fill_color(Color::rgba(44, 62, 80, 240));

        // Determinate item size by expecting a certain number of items per inventory row
        let item_per_row = 15;
        let item_size = background.size().x / item_per_row as f32;
        let item_border = item_size / 4.0;
        let mut items = Vec::new();
        let mut x = 0;
        let mut y = 0;

        let mut items_id = BTreeMap::new();
        for (i, (id, texture)) in textures.iter().enumerate() {
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

            items_id.insert(i as u32, *id);

            x += 1;
        }

        Inventory {
            background,
            items,
            items_id,
        }
    }

    /// Get the item located at given position
    /// this will returns None if no item are present at this position
    pub fn get_item_id<O: Into<Vector2f>>(&self, world_pos: O) -> Option<u32> {
        let world_pos = world_pos.into();

        for (i, item) in self.items.iter().enumerate() {
            if item.global_bounds().contains(world_pos) {
                return self.items_id.get(&(i as u32)).copied();
            }
        }

        None
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
        let textures = load_textures();

        let inventory = Inventory::new((1920, 1080), &textures);

        assert_eq!(inventory.background.position(), Vector2f::new(50.0, 50.0));
        assert_eq!(
            inventory.background.size(),
            Vector2f::new(1920.0 - 100.0, 1080.0 - 100.0)
        );
        assert_eq!(inventory.items.len(), 50);

        let size = inventory.items.get(0).unwrap().size();
        assert_eq!(size.x as u32, 121);
        assert_eq!(size.y as u32, 121);
        assert_eq!(*inventory.items_id.get(&0).unwrap(), 1);
        assert_eq!(*inventory.items_id.get(&1).unwrap(), 2);
        assert_eq!(*inventory.items_id.get(&2).unwrap(), 3);
    }

    #[test]
    fn test_inventory_get_item_id() {
        let textures = load_textures();

        let inventory = Inventory::new((1920, 1080), &textures);

        assert_eq!(inventory.get_item_id(Vector2f::new(0.0, 0.0)), None);
        assert_eq!(
            inventory.get_item_id(Vector2f::new(146.0, 98.0)).unwrap(),
            1
        );
        assert_eq!(
            inventory.get_item_id(Vector2f::new(246.0, 122.0)).unwrap(),
            2
        );
        assert_eq!(
            inventory.get_item_id(Vector2f::new(408.0, 162.0)).unwrap(),
            3
        );
        assert_eq!(
            inventory.get_item_id(Vector2f::new(250.0, 486.0)).unwrap(),
            24
        );
    }

    fn load_textures() -> BTreeMap<u32, SfBox<Texture>> {
        let mut textures = BTreeMap::new();

        for i in 0..50 {
            textures.insert(i + 1, Texture::new(16, 16).unwrap());
        }

        textures
    }
}
