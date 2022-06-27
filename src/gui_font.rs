use std::{collections::HashMap, num::NonZeroU32};

use rwge::{
    font::{
        font_atlas::{FontAtlas, FontCharLimit},
        font_characters::FontCharacters, font_load_gpu::FontDataLoad,
    },
};

pub fn load_default_font_data<'a>() -> [FontDataLoad<'a>; 3] {
    let font_data_1 = include_bytes!("../res/fonts/NeoSans/NeoSansStd-MediumItalic.otf");
    let font_data_2 = include_bytes!("../res/fonts/Lato/Lato-Bold.ttf");
    let font_data_3 = include_bytes!("../res/fonts/Lobster-Regular.ttf");

    return [
        FontDataLoad {
            name: "Neo_San_Medium_Italic",
            data: font_data_1,
            char_limit: FontCharLimit::All,
            character_size: 52.0,
            padding: 12,
        },
        FontDataLoad {
            name: "Lato_Bold",
            data: font_data_2,
            char_limit: FontCharLimit::All,
            character_size: 52.0,
            padding: 12,
        },
        FontDataLoad {
            name: "Lobster_Regular",
            data: font_data_3,
            char_limit: FontCharLimit::All,
            character_size: 40.0,
            padding: 8,
        },
    ];
}


