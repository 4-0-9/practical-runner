use std::{error::Error, fs, path::Path};

pub fn get_executables() -> Result<Vec<String>, Box<dyn Error>> {
    let mut executables: Vec<String> = Vec::new();

    get_files(Path::new("/usr/local"), &mut executables)?;
    get_files(Path::new("/bin"), &mut executables)?;

    Ok(executables)
}

pub fn get_files(path: &Path, files: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
    let dirs = fs::read_dir(path)?;

    for e in dirs {
        let entry = e.unwrap();
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            get_files(&entry.path(), files)?;
            continue;
        }
        match entry.file_name().into_string() {
            Ok(file_name) => {
                if !files.contains(&file_name) {
                    files.push(file_name)
                }
            }
            Err(_) => {}
        }
    }

    Ok(())
}
