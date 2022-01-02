use anyhow::Result;
use std::io::Cursor;

use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim,
};

pub fn skim(items: String) -> Result<Option<String>> {
    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .delimiter(Some("\t"))
        .multi(true)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(items));

    let selected_item = Skim::run_with(&options, Some(items))
        .map(|out| {
            if !out.is_abort {
                out.selected_items
                    .first()
                    .map(|item| item.text().to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| None);
    Ok(selected_item)
}
