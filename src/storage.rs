use std::{collections::HashSet, fs::File, io::Read, iter::zip};

use inquire::{autocompletion::Replacement, Autocomplete, CustomUserError};
use serde::{Deserialize, Serialize};
use serde_json::to_writer;

use crate::error::Error;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct OnionLink {
    pub name: String,
    pub link: String,
}

impl std::hash::Hash for OnionLink {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.link.hash(state);
    }
}

impl OnionLink {
    pub fn new(name: &str, link: &str) -> Self {
        Self {
            name: name.to_string(),
            link: link.to_string(),
        }
    }
}

impl PartialEq for OnionLink {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for OnionLink {}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Storage {
    #[serde(skip)]
    input: String,

    #[serde(skip)]
    names_list: Vec<String>,

    #[serde(skip)]
    path: String,

    #[serde(skip)]
    lcp: String,

    pub links: HashSet<OnionLink>,
}

impl Storage {
    pub fn add_entry(&mut self, entry: OnionLink) -> Result<(), crate::error::Error> {
        match self.links.insert(entry.clone()) {
            true => {
                let mut file = File::create(&self.path)?;
                to_writer(&mut file, &self)?;

                Ok(())
            }
            false => Err(Error::from(format!(
                "Link with name {} already present\n",
                entry.name
            ))),
        }
    }

    pub fn update_storage_file(&mut self) -> Result<(), crate::error::Error> {
        let mut file = File::create(&self.path)?;
        to_writer(&mut file, &self)?;

        Ok(())
    }

    pub fn update_input(&mut self, pattern: &str) -> Result<(), CustomUserError> {
        let pattern = pattern.to_lowercase();
        if pattern == self.input {
            return Ok(());
        }

        self.input = pattern.to_owned();
        self.names_list.clear();

        let list: Vec<String> = self
            .links
            .iter()
            .filter(|&l| l.name.to_lowercase().contains(&pattern))
            .map(|l| l.name.clone().to_lowercase())
            .collect();

        self.names_list = list;

        Ok(())
    }

    pub fn longest_common_prefix(&self) -> String {
        let mut ret = String::new();

        let mut sorted = self.names_list.clone();
        sorted.sort();
        if sorted.is_empty() {
            return ret;
        }

        let first_word = sorted.first().unwrap().chars();
        let last_word = sorted.last().unwrap().chars();

        for (c1, c2) in zip(first_word, last_word) {
            if c1 == c2 {
                ret.push(c1);
            } else {
                return ret;
            }
        }

        ret
    }

}

impl Autocomplete for Storage {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        self.update_input(input)?;
        Ok(self.names_list.clone())
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<inquire::autocompletion::Replacement, CustomUserError> {
        self.update_input(input)?;

        Ok(match highlighted_suggestion {
            Some(sugg) => Replacement::Some(sugg),
            None => match self.lcp.is_empty() {
                true => Replacement::None,
                false => Replacement::Some(self.lcp.clone()),
            }
        })
    }
}

impl TryFrom<&str> for Storage {
    type Error = crate::error::Error;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let mut storage: Storage = serde_json::from_str(&data)?;
        storage.path = path.to_owned();
        Ok(storage)
    }
}
