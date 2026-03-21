use std::sync::LazyLock;

use serde::Deserialize;

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let config_path = format!("{}/ezffi.toml", crate::MANIFEST_DIR.as_str());
    let config = std::fs::read_to_string(&config_path).unwrap_or_default();
    toml::from_str(&config).unwrap_or_default()
});

#[derive(Deserialize, Clone, Copy)]
pub enum CaseStyle {
    SnakeCase,
    CamelCase,
    PascalCase,
    ScreamingSnakeCase,
    Raw,
}

impl CaseStyle {
    pub fn format(&self, name: &str) -> String {
        let name = name.replace("-", "_");

        let mut iter = name.chars();
        let mut output = String::with_capacity(name.len());

        match self {
            CaseStyle::SnakeCase => {
                if let Some(c) = iter.next() {
                    output.push(c.to_ascii_lowercase());
                }

                for c in iter {
                    if c.is_uppercase() {
                        output.push('_');
                    }
                    output.push(c.to_ascii_lowercase());
                }
            }
            CaseStyle::CamelCase => {
                if let Some(c) = iter.next() {
                    output.push(c.to_ascii_lowercase());
                }

                let mut make_uppercase = false;

                for mut c in iter {
                    if c == '_' {
                        make_uppercase = true;
                    } else {
                        if make_uppercase {
                            c = c.to_ascii_uppercase();
                            make_uppercase = false;
                        }

                        output.push(c);
                    }
                }
            }
            CaseStyle::PascalCase => {
                if let Some(c) = iter.next() {
                    output.push(c.to_ascii_uppercase());
                }

                let mut make_uppercase = false;

                for mut c in iter {
                    if c == '_' {
                        make_uppercase = true;
                    } else {
                        if make_uppercase {
                            c = c.to_ascii_uppercase();
                            make_uppercase = false;
                        }

                        output.push(c);
                    }
                }
            }
            CaseStyle::ScreamingSnakeCase => {
                if let Some(c) = iter.next() {
                    output.push(c.to_ascii_uppercase());
                }

                for c in iter {
                    if c.is_uppercase() {
                        output.push('_');
                    }
                    output.push(c.to_ascii_uppercase());
                }
            }
            CaseStyle::Raw => output = name.to_string(),
        }

        output
    }
}

#[derive(Deserialize)]
struct NamingSection {
    prefix: Option<String>,
    sufix: Option<String>,
    case_style: Option<CaseStyle>,
}

fn default_prefix() -> String {
    crate::PKG_NAME.to_string().to_string()
}

fn default_case_style() -> CaseStyle {
    CaseStyle::SnakeCase
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_prefix")]
    prefix: String,
    #[serde(default = "String::new")]
    sufix: String,
    #[serde(default = "default_case_style")]
    case_style: CaseStyle,

    types: Option<NamingSection>,
    fns: Option<NamingSection>,
    free_fns: Option<NamingSection>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prefix: default_prefix(),
            sufix: String::new(),
            case_style: default_case_style(),
            types: None,
            fns: None,
            free_fns: None,
        }
    }
}

impl Config {
    pub fn type_prefix(&self) -> &str {
        self.types
            .as_ref()
            .and_then(|s| s.prefix.as_deref())
            .unwrap_or(&self.prefix)
    }

    pub fn type_sufix(&self) -> &str {
        self.types
            .as_ref()
            .and_then(|s| s.sufix.as_deref())
            .unwrap_or(&self.sufix)
    }

    pub fn type_case_style(&self) -> CaseStyle {
        self.types
            .as_ref()
            .and_then(|s| s.case_style)
            .unwrap_or(self.case_style)
    }

    pub fn fns_prefix(&self) -> &str {
        self.fns
            .as_ref()
            .and_then(|s| s.prefix.as_deref())
            .unwrap_or(&self.prefix)
    }

    pub fn fns_sufix(&self) -> &str {
        self.fns
            .as_ref()
            .and_then(|s| s.sufix.as_deref())
            .unwrap_or(&self.sufix)
    }

    pub fn fns_case_style(&self) -> CaseStyle {
        self.fns
            .as_ref()
            .and_then(|s| s.case_style)
            .unwrap_or(self.case_style)
    }

    pub fn free_fns_prefix(&self) -> &str {
        self.free_fns
            .as_ref()
            .and_then(|s| s.prefix.as_deref())
            .unwrap_or(&self.prefix)
    }

    pub fn free_fns_sufix(&self) -> &str {
        self.free_fns
            .as_ref()
            .and_then(|s| s.sufix.as_deref())
            .unwrap_or(&self.sufix)
    }

    pub fn free_fns_case_style(&self) -> CaseStyle {
        self.free_fns
            .as_ref()
            .and_then(|s| s.case_style)
            .unwrap_or(self.case_style)
    }
}
