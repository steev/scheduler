use scheduler::Policy;
use std::{convert::TryFrom, str::FromStr};

#[derive(Debug, Error)]
pub enum RuleError {
    #[error(display = "invalid policy value: {}", _0)]
    InvalidPolicy(String),
    #[error(display = "invalid variant value: {}", _0)]
    InvalidBy(String),
}

/// A rule defines what parameters will be set when triggered on a process.
#[derive(Default)]
pub struct Rule {
    pub by:       By,
    pub priority: Option<i32>,
    pub policy:   Option<Policy>,
}

impl TryFrom<RawRule> for Rule {
    type Error = RuleError;

    fn try_from(raw_rule: RawRule) -> Result<Self, Self::Error> {
        Ok(Rule {
            by:       match raw_rule.by {
                Some(by) => By::from_str(&by).map_err(|_| RuleError::InvalidBy(by))?,
                None => By::Process,
            },
            priority: raw_rule.priority,
            policy:   match raw_rule.policy {
                Some(policy_str) => match policy_str.parse::<Policy>() {
                    Ok(policy) => Some(policy),
                    Err(_) => return Err(RuleError::InvalidPolicy(policy_str)),
                },
                None => None,
            },
        })
    }
}

/// The raw config parsed from the TOML file.
#[derive(Deserialize, Default)]
pub struct RawRule {
    #[serde(default)]
    by: Option<String>,

    #[serde(default)]
    priority: Option<i32>,

    #[serde(default)]
    policy: Option<String>,
}

/// Whom we will match a rule by.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum By {
    Process,
    Owner,
}

impl Default for By {
    fn default() -> Self { By::Process }
}

impl FromStr for By {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let variant = match input {
            "process" => By::Process,
            "owner" => By::Owner,
            _ => return Err(()),
        };

        Ok(variant)
    }
}
