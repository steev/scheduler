mod rules;

use indexmap::IndexMap;
use regex::Regex;
use std::{convert::TryFrom, fs, io, path::Path, str::FromStr, time::SystemTime};
use users::User;

pub use self::rules::*;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error(display = "failed to read config: {}", _0)]
    Read(io::Error),
    #[error(display = "faild to parse config: {}", _0)]
    Parse(toml::de::Error),
    #[error(display = "error parsing rule: {}", _0)]
    RuleError(RuleError),
    #[error(display = "error creating regular expression: {}", _0)]
    Regex(regex::Error),
}

/// Stores the user-defined settings for managing processes by a set of rules.
pub struct Config {
    pub entities: Vec<Entity>,
    pub rules:    Vec<Rule>,
    pub mtime:    Option<SystemTime>,
}

/// The key to a rule, which contains the regular expression that the rule was built from,
/// the target by which the rule will be matched against, and the location of the rule that
/// this entity points to.
pub struct Entity {
    by:   By,
    expr: Regex,
    id:   u32,
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let input = fs::read_to_string(path).map_err(ConfigError::Read)?;

        Self::from_str(&input)
    }

    /// Given a process name and its owner's UID, this method will return an iterator containing
    /// each rule that is applicable for this process.
    pub fn find_rules<'a>(
        &'a self,
        process: &'a str,
        process_owner: u32,
        users: &'a [User],
    ) -> impl Iterator<Item = &'a Rule> + 'a {
        let &Self { ref entities, ref rules, .. } = self;

        entities
            .iter()
            .filter(move |entity| match entity.by {
                // Match the name of the process.
                By::Process => entity.expr.is_match(process),
                // Match the owner of the process.
                By::Owner => users.iter().any(|user| {
                    if user.uid() == process_owner as libc::uid_t {
                        if let Some(name) = user.name().to_str() {
                            return entity.expr.is_match(name);
                        }
                    }

                    false
                }),
                // TODO: Match by group
            })
            .map(move |entity| &rules[entity.id as usize])
    }
}

impl FromStr for Config {
    type Err = ConfigError;

    fn from_str(input: &str) -> Result<Self, ConfigError> {
        let raw: RawConfig = toml::from_str(input).map_err(ConfigError::Parse)?;

        let mut rules = Vec::with_capacity(raw.data.len());
        let mut entities = Vec::with_capacity(raw.data.len());

        let mut add_rule = |expr: &str, raw_rule| -> Result<(), ConfigError> {
            let rule = Rule::try_from(raw_rule).map_err(ConfigError::RuleError)?;
            let id = rules.len();

            entities.push(Entity {
                by:   rule.by,
                expr: Regex::new(&expr).map_err(ConfigError::Regex)?,
                id:   id as u32,
            });

            rules.push(rule);

            Ok(())
        };

        for (expr, raw_rule) in raw.data {
            add_rule(&expr, raw_rule)?;
        }

        rules.shrink_to_fit();

        Ok(Config { entities, rules, mtime: None })
    }
}

/// The raw data parsed directly from the TOML config, which is to be further parsed and checked.
#[derive(Deserialize)]
#[serde(transparent)]
struct RawConfig {
    data: IndexMap<String, RawRule>,
}
