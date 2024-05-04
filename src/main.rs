use core::panic;
use std::env;

use clap::Parser;
use cli::Cli;
use dotenv::dotenv;
use inquire::{error::InquireResult, ui::{Attributes, Color, RenderConfig, StyleSheet, Styled}, Text};
use storage::{OnionLink, Storage};

mod storage;
mod error;
mod cli;

#[cfg(test)]
mod tests;

fn main() {
    dotenv().ok();

    let args = Cli::parse();

    let path: String = env::var("STORAGE_FILE_PATH").unwrap();
    let mut storage = match Storage::try_from(path.as_str()) {
        Ok(st) => st,
        Err(_) => {
            panic!("storage.json file not found!");
        }
    };

    if args.create {
        let _ = create_prompt(&mut storage);
    } else {
        let _ = search_prompt(storage);
    }

}

pub fn search_prompt(storage: Storage) -> InquireResult<()> {
    inquire::set_global_render_config(get_render_config());
    let storage_clone = storage.clone();

    let link_res = Text::new("Name:")
        .with_autocomplete(storage)
        .prompt();

    match link_res {
        Ok(link_name) => {
            let link = storage_clone.links.iter().find(|l| l.name.to_lowercase() == link_name).map(|l| l.link.clone()).unwrap_or(" ".to_string());
            println!("{} 󰁕 {}", link_name, link);
        }
        Err(err) => println!("Error with storage: {err:?}")
    }

    Ok(())
}

pub fn create_prompt(storage: &mut Storage) -> InquireResult<()> {
    inquire::set_global_render_config(get_render_config());

    let new_name = Text::new("Name:")
        .prompt()?.to_lowercase();

    match storage.links.iter().any(|ol| ol.name.to_lowercase() == new_name) {
        true => panic!("Name already present"),
        false => {}
    }

    let link = Text::new("Link:").prompt()?.to_lowercase();

    let ol = OnionLink::new(&new_name, &link);
    storage.add_entry(ol);

    Ok(())
}


fn get_render_config() -> RenderConfig<'static> {
    let mut render_config = RenderConfig::default();
    render_config.prompt_prefix = Styled::new("$").with_fg(Color::LightBlue);
    render_config.highlighted_option_prefix = Styled::new("󰁕").with_fg(Color::LightYellow);
    render_config.scroll_up_prefix = Styled::new("⇞");
    render_config.scroll_down_prefix = Styled::new("⇟");

    render_config.error_message = render_config.error_message.with_prefix(Styled::new("❌").with_fg(Color::LightRed));
    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::LightYellow);

    render_config
}
