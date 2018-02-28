use std::fs::{remove_file, File};
use std::path::PathBuf;

mod fs;

pub struct Service {
    dir: PathBuf,
    name: String,
    pidfile: PathBuf,
}

impl Service {
    fn new(given_name: Option<String>, given_dir: Option<PathBuf>) -> Service {
        let dir = given_dir.unwrap_or(PathBuf::from("/tmp"));
        let name = given_name.unwrap_or("notectl".to_string());
        let mut pidfile = dir.clone();

        pidfile.push(format!("{}.pid", &name));

        Service {
            dir: dir,
            name: name,
            pidfile: pidfile,
        }
    }

    fn named(name: &'static str) -> Service {
        Service::new(Some(name.to_string()), None)
    }

    fn empty() -> Service {
        Service::new(None, None)
    }

    fn is_running(&self) -> bool {
        self.pidfile.exists()
    }

    fn remove_pidfile(&self) {
        if self.is_running() {
            remove_file(&self.pidfile).expect("Unable to remove pidfile");
        }
    }

    fn id(&self) -> Option<usize> {
        if self.pidfile.exists() {
            File::open(&self.pidfile)
                .map_err(|error| error.to_string())
                .and_then(fs::read_file)
                .and_then(fs::parse_pidfile_contents)
                .ok()
        } else {
            None
        }
    }
}

#[cfg(test)]

mod test {
    use std::fs::File;
    use std::io::prelude::*;
    use super::*;

    #[test]
    fn name_base_case() {
        let service = Service::empty();

        assert_eq!(service.name, "notectl".to_string());
    }

    #[test]
    fn arbitrary_name() {
        let name = "note-control".to_string();
        let service = Service::new(Some(name.clone()), None);

        assert_eq!(service.name, name);
    }

    #[test]
    fn dir_base_case() {
        let service = Service::empty();

        assert_eq!(service.dir, PathBuf::from("/tmp"));
    }

    #[test]
    fn arbitrary_dir() {
        let dir = PathBuf::from("/home");
        let service = Service::new(None, Some(dir.clone()));

        assert_eq!(service.dir, dir);
    }

    #[test]
    fn pidfile_base_case() {
        let service = Service::empty();

        assert_eq!(service.pidfile, PathBuf::from("/tmp/notectl.pid"));
    }

    #[test]
    fn id_base_case() {
        let service = Service::empty();

        assert_eq!(service.id(), None);
    }

    #[test]
    fn id_when_present() {
        let service = Service::named("notectl.id-when-present");
        let mut file = File::create(&service.pidfile).unwrap();
        let expectation: usize = 12345;
        let _write_outcome = write!(file, "{}", expectation);

        assert_eq!(service.id(), Some(expectation));

        service.remove_pidfile();
    }

    #[test]
    fn cleans_up_after_itself() {
        let service = Service::named("notectl.cleans-up-after-itself");
        let mut file = File::create(&service.pidfile).unwrap();
        let expectation: usize = 12345;
        let _write_outcome = write!(file, "{}", expectation);

        assert_eq!(service.is_running(), true);

        service.remove_pidfile();

        assert_eq!(service.pidfile.exists(), false);
    }

    #[test]
    fn is_running_base_case() {
        let service = Service::empty();

        assert_eq!(service.is_running(), false);
    }

    #[test]
    fn is_running_when_file_exists() {
        let service = Service::named("notectl.is-running-when-file-exists");
        let mut file = File::create(&service.pidfile).unwrap();
        let expectation: usize = 12345;
        let _write_outcome = write!(file, "{}", expectation);

        assert_eq!(service.is_running(), true);

        service.remove_pidfile();
    }

}
