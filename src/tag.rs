use color_eyre::Result;
use std::str::FromStr;

#[allow(dead_code)]
enum Tag {
    Emacs,
    Kubernetes,
    Linux,
    Meta,
    Neovim,
    Python,
    Rust,
}

#[allow(dead_code)]
impl Tag {
    pub fn as_str(&self) -> &str {
        match self {
            Tag::Emacs => "emacs",
            Tag::Kubernetes => "kubernetes",
            Tag::Linux => "linux",
            Tag::Meta => "meta",
            Tag::Neovim => "neovim",
            Tag::Python => "python",
            Tag::Rust => "rust",
        }
    }
}

impl FromStr for Tag {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "emacs" => Ok(Tag::Emacs),
            "kubernetes" => Ok(Tag::Kubernetes),
            "linux" => Ok(Tag::Linux),
            "meta" => Ok(Tag::Meta),
            "neovim" => Ok(Tag::Neovim),
            "python" => Ok(Tag::Python),
            "rust" => Ok(Tag::Rust),
            _ => Err(format!("Unhandled Tag: {}", s)),
        }
    }
}
