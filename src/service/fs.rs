use std::fs::File;
use std::io::prelude::*;

pub fn read_file(mut file: File) -> Result<String, String> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|error| error.to_string())?;
    Ok(contents)
}

pub fn parse_pidfile_contents(contents: String) -> Result<usize, String> {
    contents.parse::<usize>().map_err(|error| error.to_string())
}

#[cfg(test)]

mod test {
    use std::io::prelude::*;
    use std::fs::{remove_file, File};
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn reading_file_contents() {
        let pidfile = PathBuf::from("pidfile.example");
        let mut file = File::create(&pidfile).unwrap();
        let _write_outcome = write!(file, "Things");

        assert_eq!(
            File::open(&pidfile)
                .map_err(|error| error.to_string())
                .and_then(read_file),
            Ok("Things".to_string())
        );

        if pidfile.exists() {
            remove_file(pidfile).expect("Unable to remove pidfile");
        }
    }

    #[test]
    fn parsing_pidfile_contents() {
        assert_eq!(parse_pidfile_contents("12345".to_string()), Ok(12345))
    }

}
