#[allow(dead_code)]
enum ProjectCategory {
    Major,
    OpenSource,
}

#[allow(dead_code)]
impl ProjectCategory {
    pub fn as_str(&self) -> &str {
        match self {
            ProjectCategory::Major => "Major",
            ProjectCategory::OpenSource => "Open Source",
        }
    }
}
