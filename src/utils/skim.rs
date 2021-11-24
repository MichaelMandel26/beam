use anyhow::Result;
use std::io::Cursor;

use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim,
};

pub fn skim(items: String) -> Result<String> {
    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(true)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(items));

    // `run_with` would read and show items from the stream
    let selected_item = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items.get(0).unwrap().text().to_string())
        .unwrap_or_else(|| "".to_string());

    Ok(selected_item)
}