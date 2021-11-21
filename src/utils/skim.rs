use std::io::Cursor;

use skim::{Skim, prelude::{SkimItemReader, SkimOptionsBuilder}};

pub fn skim(items: String){
  let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(true)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(items));

    // `run_with` would read and show items from the stream
    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        print!("{}{}", item.output(), "\n");
    }
}