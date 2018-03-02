use daemonize::Daemonize;

use std::fs::{remove_file, File};
use std::path::PathBuf;
use std::process::Command;

mod fs;

pub struct Service {
    pidfile: PathBuf,
}

impl Service {
    pub fn new(
        given_name: Option<String>,
        given_dir: Option<PathBuf>,
    ) -> Service {
        let dir = given_dir.unwrap_or(PathBuf::from("/tmp"));
        let name = given_name.unwrap_or("notectl".to_string());
        let mut pidfile = dir.clone();

        pidfile.push(format!("{}.pid", &name));

        Service { pidfile: pidfile }
    }

    pub fn empty() -> Service {
        Service::new(None, None)
    }

    pub fn id(&self) -> Option<usize> {
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

    pub fn start<F>(&self, on_start: F)
    where
        F: Fn() -> (),
    {
        if self.is_running() {
            println!("Notectl process already running.");
        } else {
            Daemonize::new()
                .working_directory("/tmp")
                .pid_file("/tmp/notectl.pid")
                .start()
                .unwrap();

            on_start()
        }
    }

    pub fn stop(&self) {
        if self.is_running() {
            Command::new("kill")
                .arg(self.id().expect("Unable to retrieve PID").to_string())
                .output()
                .expect("Couldn't kill notectl program");

            self.remove_pidfile();
        } else {
            println!("Unable to find running notectl process.");
        }
    }

    fn is_running(&self) -> bool {
        self.pidfile.exists()
    }

    fn remove_pidfile(&self) {
        if self.is_running() {
            remove_file(&self.pidfile).expect("Unable to remove pidfile");
        }
    }
}

#[cfg(test)]

mod test {
    use std::fs::File;
    use std::io::prelude::*;
    use super::*;

    fn service_named(name: &'static str) -> Service {
        Service::new(Some(name.to_string()), None)
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
        let service = service_named("notectl.id-when-present");
        let mut file = File::create(&service.pidfile).unwrap();
        let expectation: usize = 12345;
        let _write_outcome = write!(file, "{}", expectation);

        assert_eq!(service.id(), Some(expectation));

        service.remove_pidfile();
    }

    #[test]
    fn cleans_up_after_itself() {
        let service = service_named("notectl.cleans-up-after-itself");
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
        let service = service_named("notectl.is-running-when-file-exists");
        let mut file = File::create(&service.pidfile).unwrap();
        let expectation: usize = 12345;
        let _write_outcome = write!(file, "{}", expectation);

        assert_eq!(service.is_running(), true);

        service.remove_pidfile();
    }

}
