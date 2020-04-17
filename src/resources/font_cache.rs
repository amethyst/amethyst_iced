use amethyst::assets::Handle;
use amethyst::ui::FontAsset;

use glyph_brush::FontId;

use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct FontCache {
    pub(crate) map: HashMap<String, Handle<FontAsset>>, 
    pub(crate) load_list: HashSet<String>,
    pub(crate) map_id: HashMap<String, FontId>,
}

impl FontCache {
    pub fn insert(&mut self, font_name: String, font_handle: Handle<FontAsset>) {
        self.map.insert(font_name.clone(), font_handle);
        self.load_list.insert(font_name);
    } 

    pub(crate) fn get_handle(&self, font_name: &str) -> Option<&Handle<FontAsset>> {
        self.map.get(font_name)
    }

    pub(crate) fn get_id(&self, font_name: &str) -> Option<&FontId> {
        self.map_id.get(font_name)
    } 
}