use amethyst::ecs::{System, Write, WriteExpect, Read};
use amethyst::assets::AssetStorage;
use amethyst::ui::FontAsset;

use glyph_brush::FontId;

use crate::IcedGlyphBrush; 
use crate::resources::FontCache;

#[derive(Default)]
pub struct LoadFontToCacheSystem; 

impl<'a> System<'a> for LoadFontToCacheSystem {
    type SystemData = (
        WriteExpect<'a, IcedGlyphBrush>,
        Write<'a, FontCache>, 
        Read<'a, AssetStorage<FontAsset>>,
    );

    fn run(&mut self, (mut glyph_brush, mut font_cache, storage): Self::SystemData) {
        let imported: Vec<(String, FontId)> = font_cache
            .load_list
            .iter()
            .filter_map(|font_name| {
                if let Some(handle) = font_cache.get_handle(font_name) {
                    if let Some(asset) = storage.get(handle) {
                        // Could very likely be done better than cloning...
                        let font_id = glyph_brush.add_font(asset.0.clone());
                        return Some((font_name.to_string(), font_id));
                    }
                }
                None
            })
            .collect();

        imported
            .into_iter()
            .for_each(|(name, id)| {
                font_cache.load_list.remove(&name);
                font_cache.map_id.insert(name, id);
            })
    }
}