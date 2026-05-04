mod data;

use std::path::Path;
use data::Run;

#[cfg(not(test))]
use std::fs::read_to_string;

#[cfg(test)]
fn read_to_string(_: impl AsRef<Path>) -> std::io::Result<String> {
    Ok(String::new())
}

pub fn read_run_file(file_path: &Path) -> std::io::Result<Run> {
    let contents = read_to_string(file_path)?;

    let run : Run = serde_json::from_str(&contents)?;

    Ok(run)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = read_run_file(Path::new("test"));

        assert!(!result.is_ok());
    }
}
