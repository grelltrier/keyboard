use std::collections::HashMap;

mod deserializer;
use deserializer::LayoutYamlParser;
mod deserialized_structs;
pub use deserialized_structs::{KeyAction, KeyDisplay, KeyEvent, Outline};
use deserialized_structs::{KeyDeserialized, KeyIds, LayoutDeserialized};

#[derive(Debug)]
pub struct KeyMeta {
    pub actions: HashMap<KeyEvent, Vec<KeyAction>>,
    pub key_display: KeyDisplay,
    pub outline: Outline,
    pub popup: Option<Vec<String>>,
    pub styles: Option<Vec<String>>,
}

impl KeyMeta {
    fn from(key_id: &str, key_deserialized: Option<&KeyDeserialized>) -> KeyMeta {
        let mut key_meta = KeyMeta::default(key_id);
        if let Some(key_deserialized) = key_deserialized {
            if let Some(deserialized_actions) = &key_deserialized.actions {
                key_meta.actions = deserialized_actions.clone();
            };
            if let Some(deserialized_key_display) = &key_deserialized.key_display {
                key_meta.key_display = deserialized_key_display.clone();
            };
            if let Some(deserialized_outline) = &key_deserialized.outline {
                key_meta.outline = *deserialized_outline;
            };
            if key_deserialized.popup.is_some() {
                key_meta.popup = key_deserialized.popup.clone();
            };
            if key_deserialized.styles.is_some() {
                key_meta.styles = key_deserialized.styles.clone();
            };
        }
        key_meta
    }

    fn default(key_id: &str) -> KeyMeta {
        let key_id = key_id.to_string();
        let mut actions = HashMap::new();
        actions.insert(
            KeyEvent::ShortPress,
            vec![KeyAction::EnterString(key_id.clone())],
        );
        let key_display = KeyDisplay::Text(key_id);
        let outline = Outline::Standard;
        let popup = None;
        let styles = None;

        KeyMeta {
            actions,
            key_display,
            outline,
            popup,
            styles,
        }
    }
}

#[derive(Debug)]
pub struct LayoutMeta {
    pub views: HashMap<String, KeyArrangement>,
    pub keys: HashMap<String, KeyMeta>,
}

impl LayoutMeta {
    pub fn new() -> HashMap<String, LayoutMeta> {
        let layout_deserialized = LayoutYamlParser::get_layouts();
        let mut layout_meta = HashMap::new();
        for (layout_name, layout_deserialized) in layout_deserialized {
            layout_meta.insert(layout_name, LayoutMeta::from(layout_deserialized));
        }
        layout_meta
    }

    // KeyMeta for all needed keys is created and the string of keyids is converted to a hashmap with the location and size of each key
    fn from(layout_deserialized: LayoutDeserialized) -> LayoutMeta {
        let mut views = HashMap::new();
        let mut keys = HashMap::new();
        for (view_name, key_arrangement) in layout_deserialized.views {
            let keys_for_view =
                LayoutMeta::get_key_meta_for_all_keys(&key_arrangement, &layout_deserialized.keys);
            for (key_id, key_meta) in keys_for_view {
                keys.insert(key_id, key_meta); // Could use map1.extend(map2); instead
            }
            let view = KeyArrangement::from(&key_arrangement, &keys);
            views.insert(view_name, view);
        }
        LayoutMeta { views, keys }
    }

    fn get_key_meta_for_all_keys(
        key_arrangement_deserialized: &[KeyIds],
        key_meta: &HashMap<String, KeyDeserialized>,
    ) -> HashMap<String, KeyMeta> {
        let mut keys = HashMap::new();
        for row in key_arrangement_deserialized {
            for key_id in row.split_ascii_whitespace() {
                let key_meta = KeyMeta::from(key_id, key_meta.get(key_id));
                keys.insert(key_id.to_string(), key_meta);
            }
        }
        keys
    }
}

#[derive(Debug)]
pub struct Location {
    pub coordinate: (i32, i32),
    pub size: (i32, i32),
}

#[derive(Debug)]
pub struct KeyArrangement {
    pub key_arrangement: HashMap<String, Location>,
}
impl KeyArrangement {
    pub fn from(
        key_arrangement_deserialized: &[KeyIds],
        key_meta: &HashMap<String, KeyMeta>,
    ) -> KeyArrangement {
        let (uncentered_key_arrangement, row_widths) =
            KeyArrangement::get_uncentered_key_arrangement(key_arrangement_deserialized, key_meta);
        let centered_key_arrangement =
            KeyArrangement::get_centered_key_arrangement(uncentered_key_arrangement, row_widths);
        KeyArrangement {
            key_arrangement: centered_key_arrangement,
        }
    }

    fn get_uncentered_key_arrangement(
        key_arrangement_deserialized: &[KeyIds],
        key_meta: &HashMap<String, KeyMeta>,
    ) -> (HashMap<String, Location>, Vec<i32>) {
        let mut key_arrangement = HashMap::new();
        let mut row_widths = Vec::new(); // Tracks width of the rows to later center the rows
        for (row_no, row) in key_arrangement_deserialized.iter().enumerate() {
            row_widths.insert(row_no, 0);
            for key_id in row.split_ascii_whitespace() {
                let coordinate = (row_widths[row_no], row_no as i32);
                let (width, height) = (
                    key_meta
                        .get(key_id)
                        .expect("KeyMeta should have been completed")
                        .outline as i32,
                    1,
                );
                let location = Location {
                    coordinate,
                    size: (width, height),
                };
                row_widths[row_no] += width;
                key_arrangement.insert(key_id.to_string(), location);
            }
        }
        (key_arrangement, row_widths)
    }

    fn get_centered_key_arrangement(
        uncentered_key_arrangement: HashMap<String, Location>,
        row_widths: Vec<i32>,
    ) -> HashMap<String, Location> {
        let max_width_rows = row_widths.iter().max().unwrap();
        let mut key_arrangement_centered = HashMap::new();
        for (
            key,
            Location {
                coordinate: (x, y),
                size,
            },
        ) in uncentered_key_arrangement
        {
            let new_x = (max_width_rows - row_widths[y as usize]) / 2 + x;
            let new_location = Location {
                coordinate: (new_x, y),
                size,
            };
            key_arrangement_centered.insert(key, new_location);
        }
        key_arrangement_centered
    }
}
