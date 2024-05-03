use std::{collections::{BTreeSet, HashSet}, fmt::{write, Display}, fs::File, io::Read};

use serde::{Deserialize, Serialize};
use serde_json::to_writer;
use yansi::Paint;

use crate::error::Error;


#[derive(Default, Debug, Serialize, Deserialize, Clone, Hash)]
pub struct OnionLink {
    name: String,
    link: String,
    rank: u32,
}

impl OnionLink {
    pub fn new(name: &str, link: &str) -> Self {
        Self {
            name: name.to_string(),
            link: link.to_string(),
            rank: 1
        }
    }
}

impl PartialEq for OnionLink {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for OnionLink {}

impl PartialOrd for OnionLink {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OnionLink {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl Display for OnionLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {}", self.name.bold(), self.link.italic())
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Storage {
    links: HashSet<OnionLink>
}

impl Storage {
    pub fn add_entry(&mut self, entry: OnionLink) -> Result<(), crate::error::Error> {
        match self.links.insert(entry.clone()) {
            true => {
                let mut file = File::create("storage")?;
                to_writer(&mut file, &self)?;

                Ok(())
            }
            false => Err(Error::from(format!("Link with name {} already present\n", entry.name))),
        }
    }

    pub fn search_entry(&self, pattern: &str) -> Vec<&OnionLink> {
        self.links.iter().filter(|&l| l.name.contains(pattern)).collect()
    }
}

impl TryFrom<&mut File> for Storage {
    type Error = crate::error::Error;

    fn try_from(file: &mut File) -> Result<Self, Self::Error> {
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        match serde_json::from_str(&data) {
            Ok(links) => Ok(Self {
                links,
            }),
            Err(err) => Err(Error::from(err)),
        }
    }
}
