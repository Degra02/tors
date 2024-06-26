# Tors
<img src="./logo.png" width="100">
This is a command-line tool written in Rust for managing and searching locally stored associations between human-readable names and onion (.onion) website links.

Features
- Create associations: Add new name-link associations to your local storage.
- Search for links: Find onion links by their associated names.
- Simple and user-friendly: Interact with the tool through intuitive commands.
- Offline functionality: Works without an internet connection (searches local data).

Note: This tool does not access the dark web or retrieve information from onion links. It solely manages user-provided associations for easier access to bookmarked sites.

# Usage
The `storage.json` file must be stored inside `~/.config/tors/storage.json`, so a   
directory and the file have to be created in order for it to work.  
Copy the file [storage.json](./storage.json) to `~/.config/tors` as is.

This is the help message from the tool
```bash
Usage: tors [COMMAND]

Commands:
  search  Search for a .onion by name [default] [aliases: s]
  create  Create a new association [aliases: c]
  update  Update an existing association [aliases: u]
  delete  Delete an existing association [aliases: d]
  list    List all stored associations [aliases: l]
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Additional features
The `clipboard` feature adds the [arboard](https://github.com/1Password/arboard) crate,  
that copies the selected entry to the clipboard
