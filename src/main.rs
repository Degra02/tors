use clap::Parser;
use cli::Cli;
use core::panic;
use error::Error;
use inquire::{
    ui::{Attributes, Color, RenderConfig, StyleSheet, Styled},
    Confirm, Text,
};
use storage::{OnionLink, Storage};
use yansi::Color as TColor;
use yansi::Paint;

mod cli;
mod error;
mod storage;

#[cfg(test)]
mod tests;

fn main() -> Result<(), Error> {
    inquire::set_global_render_config(get_render_config());

    let args = Cli::parse();
    let mut home_dir = dirs::home_dir().unwrap();
    home_dir.push(".config/tors/storage.json");

    let path = home_dir.to_str().unwrap();

    let storage = match Storage::try_from(path) {
        Ok(st) => st,
        Err(_) => {
            panic!("storage.json file not found!");
        }
    };

    match args.command {
        Some(command) => match command {
            cli::Command::Search { name }=> search_prompt(storage, name)?,
            cli::Command::Create => create_prompt(storage)?,
            cli::Command::Update => update_prompt(storage)?,
            cli::Command::Delete => delete_prompt(storage)?,
            cli::Command::List => list_links(&storage),
        },
        None => search_prompt(storage, None)?,
    }

    Ok(())
}

fn search_prompt(storage: Storage, name_opt: Option<String>) -> Result<(), Error> {
    let storage_clone = storage.clone();

    let query = if let Some(name) = name_opt {
        name
    } else {
        Text::new(format!("{}:", "Name | Tags".bold().fg(TColor::Magenta)).as_str())
            .with_autocomplete(storage)
            .with_page_size(10)
            .prompt()?
    };

    let (name, link) = storage_clone
        .links
        .iter()
        .find(|l| {
            l.name.to_lowercase() == query
                || if let Some(tags) = &l.tags {
                    tags.iter().any(|t| *t == query)
                } else {
                    false
                }
        })
        .map(|l| (l.name.clone(), l.link.clone()))
        .unwrap_or((" ".to_string(), " ".to_string()));

    if !query.is_empty() {
        #[cfg(feature = "clipboard")]
        copy_to_clipboard(&link)?;

        println!(
            "{} \n{}",
            name.bold().fg(TColor::BrightGreen),
            link.italic()
        );
    }

    Ok(())
}

fn create_prompt(mut storage: Storage) -> Result<(), Error> {
    let new_name = Text::new(format!("{}:", "Name".bold().fg(TColor::Magenta)).as_str())
        .prompt()?
        .to_lowercase();

    match storage
        .links
        .iter()
        .any(|ol| ol.name.to_lowercase() == new_name)
    {
        true => return Err(Error::Other("Name already present".to_string())),
        false => {}
    }

    let link = Text::new(format!("{}:", "Link".bold().fg(TColor::Magenta)).as_str())
        .prompt()?
        .to_lowercase();

    let tags: Vec<String> = Text::new(format!("{}:", "Tags".bold().fg(TColor::Magenta)).as_str())
        .with_help_message("write multiple tags separated by spaces")
        .prompt()?
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let tags = if tags.is_empty() { None } else { Some(tags) };

    let ol = OnionLink::new(&new_name, &link, tags);
    if storage.add_entry(ol).is_ok() {
        println!("{} created!", new_name.bold().fg(TColor::BrightGreen));
    } else {
        println!("{} Could not create the entry!", "".bold().fg(TColor::Red));
    }

    Ok(())
}

fn update_prompt(mut storage: Storage) -> Result<(), Error> {
    let name = Text::new(format!("{}:", "Name".bold().fg(TColor::Magenta)).as_str())
        .with_autocomplete(storage.clone())
        .prompt()?
        .to_lowercase();
    let new_link = Text::new(format!("{}:", "New Link".bold().fg(TColor::Magenta)).as_str())
        .with_help_message("Leave empty if unchanged")
        .prompt()?;
    let new_tags: Vec<String> =
        Text::new(format!("{}:", "New Tags".bold().fg(TColor::Magenta)).as_str())
            .with_help_message("Write multiple tags separated by spaces, leave empty if unchanged")
            .prompt()?
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

    storage.links.clone().iter().for_each(|onion_link| {
        if onion_link.name.to_lowercase() == name {
            let _ = storage.links.take(onion_link).expect("Not found!");
            let new_onion = OnionLink::new(
                &name,
                {
                    if new_link.is_empty() {
                        &onion_link.link
                    } else {
                        &new_link
                    }
                },
                {
                    if new_tags.is_empty() {
                        onion_link.tags.clone()
                    } else {
                        Some(new_tags.clone())
                    }
                },
            );
            if storage.add_entry(new_onion).is_ok() {
                println!(
                    "{} {} updated!",
                    "".bold().fg(TColor::Green),
                    name.bold().fg(TColor::BrightGreen)
                );
            } else {
                println!("{} Could not update the entry!", "".bold().fg(TColor::Red));
            }
        }
    });

    Ok(())
}

fn delete_prompt(mut storage: Storage) -> Result<(), Error> {
    let name = Text::new(format!("{}:", "Name".bold().fg(TColor::Magenta)).as_str())
        .with_autocomplete(storage.clone())
        .prompt()?
        .to_lowercase();

    storage.links.clone().iter().for_each(|onion_link| {
        if onion_link.name == name {
            let decision = Confirm::new("Confirm deletion?")
                .with_default(false)
                .prompt();

            if let Ok(true) = decision {
                storage.links.remove(onion_link);
                if storage.update_storage_file().is_ok() {
                    println!(
                        "{} {} deleted!",
                        "".bold().fg(TColor::Green),
                        name.bold().fg(TColor::BrightGreen)
                    )
                }
            }
        }
    });

    Ok(())
}

fn list_links(storage: &Storage) {
    for (i, onion_link) in storage.links.iter().enumerate() {
        println!(
            "[{i}] {} 󰁕 {}",
            onion_link.name.bold().fg(TColor::BrightGreen),
            onion_link.link.italic()
        );
        if let Some(tags) = &onion_link.tags {
            print!("    {}: ", "Tags".bold().fg(TColor::Magenta));
            tags.iter().for_each(|t| {
                print!("{} ", t.bold());
            });
            println!();
        }
        println!()
    }
}

#[cfg(feature = "clipboard")]
fn copy_to_clipboard(link: &str) -> Result<(), Error> {
    let mut clipboard = arboard::Clipboard::new()?; 
    clipboard.set_text(link)?;

    println!("Copied to clipboard!");

    Ok(())
}

fn get_render_config() -> RenderConfig<'static> {
    let mut render_config = RenderConfig::default();
    render_config.prompt_prefix = Styled::new("$").with_fg(Color::LightBlue);
    render_config.highlighted_option_prefix = Styled::new("󰁕").with_fg(Color::LightYellow);
    render_config.scroll_up_prefix = Styled::new("");
    render_config.scroll_down_prefix = Styled::new("");

    render_config.selected_option = Some(
        StyleSheet::new()
            .with_attr(Attributes::BOLD)
            .with_fg(Color::DarkYellow),
    );

    render_config.option = StyleSheet::new().with_attr(Attributes::ITALIC);

    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("❌").with_fg(Color::LightRed));
    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::LightYellow);

    render_config
}
