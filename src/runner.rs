use std::process::Command;
use crate::models::ProjectType;

pub fn run_project(project_type: &ProjectType, project_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
    match project_type {
        ProjectType::Rust => {
            let output = Command::new("docker")
                .args([
                    "run",
                    "--rm",
                    "--user",
                    "1001:1001",
                    "-v",
                    &format!("{}:/workspace", project_path),
                    "-w",
                    "/workspace",
                    "rust",
                    "cargo",
                    "test",
                ])
                .output()?;

            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr));

            Ok(output.status.success())

        }

        ProjectType::Node => {
            let output = Command::new("docker")
                .args([
                    "run",
                    "--rm",
                    "--user",
                    "1001:1001",
                    "-v",
                    &format!("{}:/workspace", project_path),
                    "-w",
                    "/workspace",
                    "node",
                    "sh",
                    "-c",
                    "npm install && npm test",
                ])
                .output()?;

            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr));

            Ok(output.status.success())
        }

        ProjectType::Python => {
            let output = Command::new("docker")
                .args([
                    "run",
                    "--rm",
                    "--user",
                    "1001:1001",
                    "-v",
                    &format!("{}:/workspace", project_path),
                    "-w",
                    "/workspace",
                    "python",
                    "sh",
                    "-c",
                    "pip install --target=/tmp/python-packages pytest && PYTHONPATH=/tmp/python-packages python -m pytest",
                ])
                .output()?;

        println!("{}", String::from_utf8_lossy(&output.stdout));
        println!("{}", String::from_utf8_lossy(&output.stderr));

            Ok(output.status.success())
        }
    }
}
