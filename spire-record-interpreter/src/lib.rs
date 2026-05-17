mod data;

use std::path::Path;
use data::Run;

use std::fs::read_to_string;
use std::fs::read_dir;

pub fn read_runs_from_directory(directory_path: &Path) -> std::io::Result<Vec<Run>> {
    let directory = read_dir(directory_path)?;
    let mut runs = Vec::<Run>::new();

    for directory_entry in directory {
        let path = directory_entry?.path();

        println!("Processing {}", path.display());

        let run = read_run_file(path.as_path())?;

        runs.push(run);
    }

    Ok(runs)
}

pub fn read_run_file(file_path: &Path) -> std::io::Result<Run> {
    let contents = read_to_string(file_path)?;

    let run : Run = serde_json::from_str(&contents)?;

    Ok(run)
}

pub fn read_test_file() -> std::io::Result<Run> {
    let contents = read_run_file(Path::new("spire-record-interpreter/test_files/1772846632.run"));

    contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = read_test_file();

        assert!(result.is_ok());
    }
}
