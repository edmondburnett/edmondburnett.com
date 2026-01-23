use std::str::FromStr;

#[allow(dead_code)]
enum Tag {
    Linux,
    Kubernetes,
    Rust,
    Python,
    Neovim,
    Emacs,
}

#[allow(dead_code)]
impl Tag {
    pub fn as_str(&self) -> &str {
        match self {
            Tag::Linux => "linux",
            Tag::Kubernetes => "kubernetes",
            Tag::Rust => "rust",
            Tag::Python => "python",
            Tag::Neovim => "neovim",
            Tag::Emacs => "emacs",
        }
    }
}

impl FromStr for Tag {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "linux" => Ok(Tag::Linux),
            "kubernetes" => Ok(Tag::Kubernetes),
            "rust" => Ok(Tag::Rust),
            "python" => Ok(Tag::Python),
            "neovim" => Ok(Tag::Neovim),
            "emacs" => Ok(Tag::Emacs),
            _ => Err(format!("Unhandled Tag: {}", s)),
        }
    }
}
