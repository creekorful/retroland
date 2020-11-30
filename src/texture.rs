use sfml::graphics::{IntRect, Texture};
use sfml::system::SfBox;
use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;

pub fn load<P: AsRef<Path>>(
    assets_dir: P,
) -> Result<BTreeMap<u32, SfBox<Texture>>, Box<dyn Error>> {
    let mut textures = BTreeMap::new();

    let mut idx = 1;
    for x in 0..5 {
        textures.insert(
            idx,
            Texture::from_file_with_rect(
                &format!("{}/grass.png", assets_dir.as_ref().display()),
                &IntRect::new(x * 16, 0, 16, 16),
            )
            .ok_or_else(|| "unable to load texture".to_string())?,
        );
        idx += 1;
    }

    // Start of layer 1 blocks
    idx = 50;
    for y in 2..4 {
        for x in 0..3 {
            textures.insert(
                idx,
                Texture::from_file_with_rect(
                    &format!("{}/houses.png", assets_dir.as_ref().display()),
                    &IntRect::new(x * 16, y * 16, 16, 16),
                )
                .ok_or_else(|| "unable to load texture".to_string())?,
            );
            idx += 1;
        }
    }
    for x in 0..3 {
        textures.insert(
            idx,
            Texture::from_file_with_rect(
                &format!("{}/markets.png", assets_dir.as_ref().display()),
                &IntRect::new(x * 16, 2 * 16, 16, 16),
            )
            .ok_or_else(|| "unable to load texture".to_string())?,
        );
        idx += 1;
    }
    for y in 0..4 {
        textures.insert(
            idx,
            Texture::from_file_with_rect(
                &format!("{}/resources.png", assets_dir.as_ref().display()),
                &IntRect::new(0, y * 16, 16, 16),
            )
            .ok_or_else(|| "unable to load texture".to_string())?,
        );
        idx += 1;
    }
    for x in 0..2 {
        for y in 0..2 {
            textures.insert(
                idx,
                Texture::from_file_with_rect(
                    &format!("{}/towers.png", assets_dir.as_ref().display()),
                    &IntRect::new(x * 16, y * 16 + 16, 16, 16),
                )
                .ok_or_else(|| "unable to load texture".to_string())?,
            );
            idx += 1;
        }
    }
    for x in 0..4 {
        textures.insert(
            idx,
            Texture::from_file_with_rect(
                &format!("{}/wheatfields.png", assets_dir.as_ref().display()),
                &IntRect::new(x * 16, 0, 16, 16),
            )
            .ok_or_else(|| "unable to load texture".to_string())?,
        );
        idx += 1;
    }
    for x in 1..4 {
        textures.insert(
            idx,
            Texture::from_file_with_rect(
                &format!("{}/trees.png", assets_dir.as_ref().display()),
                &IntRect::new(x * 16, 0, 16, 16),
            )
            .ok_or_else(|| "unable to load texture".to_string())?,
        );
        idx += 1;
    }

    Ok(textures)
}
