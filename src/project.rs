use crate::markdown::ProjectMetadata;
// use crate::tag::Tag;
use color_eyre::Result;
use edmondburnett_com::markdown::Markdown;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub html: String,
}

impl Project {
    pub fn new(id: &str) -> Result<Self> {
        let markdown = Markdown::<ProjectMetadata>::from_file("projects", id, true)?;

        // let tags = self.load_tags(markdown.metadata().tags.clone());

        Ok(Self {
            id: id.to_string(),
            name: markdown.metadata().name.clone(),
            description: markdown.metadata().description.clone(),
            tags: markdown.metadata().tags.clone(),
            html: markdown.html().to_string(),
        })
    }

    pub fn list() -> Result<Vec<Project>> {
        let mut projects = Vec::new();

        for entry in std::fs::read_dir("projects")? {
            let path = entry?.path();

            if path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }

            if let Some(id) = path.file_stem().and_then(|s| s.to_str()) {
                let markdown = Markdown::<ProjectMetadata>::from_file("projects", id, false)?;

                projects.push(Project {
                    id: id.to_string(),
                    name: markdown.metadata().name.clone(),
                    description: markdown.metadata().description.clone(),
                    tags: markdown.metadata().tags.clone(),
                    html: markdown.html().to_string(),
                });
            }
        }
        Ok(projects)
    }

    // fn load_tags(&self, string_tags: &Vec<String>) -> Result<Vec<Tag>> {
    //     let mut tags = Vec::<Tag>::new();
    //     todo!()
    // }
}
