use bytes::Bytes;
use lib_ot::core::TextDelta;
use lib_ot::{core::DeltaBuilder, rich_text::RichTextDelta};
use serde::{Deserialize, Serialize};
#[inline]
pub fn initial_quill_delta() -> RichTextDelta {
    DeltaBuilder::new().insert("\n").build()
}

#[inline]
pub fn initial_quill_delta_string() -> String {
    initial_quill_delta().json_str()
}

#[inline]
pub fn initial_read_me() -> RichTextDelta {
    let json = include_str!("READ_ME.json");
    RichTextDelta::from_json(json).unwrap()
}

#[derive(Serialize, Deserialize)]
pub struct BuildInGrid {
    grid_delta: String,
    rows_delta: String,
}

impl BuildInGrid {
    pub fn json_str(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn from_bytes(bytes: Bytes) -> Self {
        serde_json::from_slice::<BuildInGrid>(bytes.as_ref()).unwrap()
    }

    pub fn split(self) -> (Bytes, Bytes) {
        (Bytes::from(self.grid_delta), Bytes::from(self.rows_delta))
    }
}

#[inline]
pub fn initial_board_read_me() -> BuildInGrid {
    let grid_json = include_str!("board/grid.json");
    let block_json = include_str!("board/rows.json");
    let grid_delta = TextDelta::from_json(grid_json).unwrap().json_str();
    let rows_delta = TextDelta::from_json(block_json).unwrap().json_str();
    BuildInGrid { grid_delta, rows_delta }
}

#[cfg(test)]
mod tests {
    use crate::client_document::default::{initial_board_read_me, initial_read_me, BuildInGrid};
    use bytes::Bytes;

    #[test]
    fn load_read_me() {
        println!("{}", initial_read_me().json_str());
    }

    #[test]
    fn load_board_read_me() {
        let s = initial_board_read_me().json_str();
        let _ = BuildInGrid::from_bytes(Bytes::from(s));
    }
}
