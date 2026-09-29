use std::path::Path;
use std::fs;
use crate::models::ProjectType;

pub fn detect_projects(workspace: &str)-> Vec<(ProjectType, String)> {
    let mut projects = Vec::new();

    scan_directory(Path::new(&workspace), &mut projects);

    projects
}

fn scan_directory(
    directory: &Path,
    projects: &mut Vec<(ProjectType, String)>
){
    let entries = match fs::read_dir(directory){
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten(){
        let path = entry.path();

        if path.is_dir(){
            scan_directory(&path, projects);
        } else{
            match path.file_name().and_then(|name| name.to_str()){
                Some("Cargo.toml")=> {
                    projects.push((
                            ProjectType::Rust,
                            path
                            .parent()
                            .unwrap()
                            .to_string_lossy()
                            .into_owned(),
                    ));
                }

                Some("package.json") => {
                    projects.push((
                            ProjectType::Node,
                            path
                            .parent()
                            .unwrap()
                            .to_string_lossy()
                            .into_owned(),
                    ));
                }

                Some("pyproject.toml") => {
                    projects.push((
                            ProjectType::Python,
                            path
                            .parent()
                            .unwrap()
                            .to_string_lossy()
                            .into_owned(),
                    ));
                }
                _ => {}
            }
        }
    }
}
