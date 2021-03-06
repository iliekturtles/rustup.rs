use std::path::PathBuf;
use std::ffi::OsString;
use curl;
use url::Url;

error_chain! {
    types {
        Error, ErrorKind, ChainErr, Result;
    }

    links { }

    foreign_links { }

    errors {
        LocatingWorkingDir {
            description("could not locate working directory")
        }
        ReadingFile {
            name: &'static str,
            path: PathBuf,
        } {
            description("could not read file")
            display("could not read {} file: '{}'", name, path.display())
        }
        ReadingDirectory {
            name: &'static str,
            path: PathBuf,
        } {
            description("could not read directory")
            display("could not read {} directory: '{}'", name, path.display())
        }
        WritingFile {
            name: &'static str,
            path: PathBuf,
        } {
            description("could not write file")
            display("could not write {} file: '{}'", name, path.display())
        }
        CreatingDirectory {
            name: &'static str,
            path: PathBuf,
        } {
            description("could not create directory")
            display("could not crate {} directory: '{}'", name, path.display())
        }
        FilteringFile {
            name: &'static str,
            src: PathBuf,
            dest: PathBuf,
        } {
            description("could not copy  file")
            display("could not copy {} file from '{}' to '{}'", name, src.display(), dest.display())
        }
        RenamingFile {
            name: &'static str,
            src: PathBuf,
            dest: PathBuf,
        } {
            description("could not rename file")
            display("could not rename {} file from '{}' to '{}'", name, src.display(), dest.display())
        }
        RenamingDirectory {
            name: &'static str,
            src: PathBuf,
            dest: PathBuf,
        } {
            description("could not rename directory")
            display("could not rename {} directory from '{}' to '{}'", name, src.display(), dest.display())
        }
        HttpError(e: curl::Error) {
            description("http request did not succeed")
            display("http request returned failure: {}", e)
        }
        HttpStatus(e: u32) {
            description("http request returned an unsuccessful status code")
            display("http request returned an unsuccessful status code: {}", e)
        }
        DownloadingFile {
            url: Url,
            path: PathBuf,
        } {
            description("could not download file")
            display("could not download file from '{}' to '{}", url, path.display())
        }
        Download404 {
            url: Url,
            path: PathBuf,
        } {
            description("could not download file")
            display("could not download file from '{}' to '{}", url, path.display())
        }
        InvalidUrl {
            url: String,
        } {
            description("invalid url")
            display("invalid url: {}", url)
        }
        RunningCommand {
            name: OsString,
        } {
            description("command failed")
            display("command failed: '{}'", PathBuf::from(name).display())
        }
        NotAFile {
            path: PathBuf,
        } {
            description("not a file")
            display("not a file: '{}'", path.display())
        }
        NotADirectory {
            path: PathBuf,
        } {
            description("not a directory")
            display("not a directory: '{}'", path.display())
        }
        LinkingFile {
            src: PathBuf,
            dest: PathBuf,
        } {
            description("could not link file")
            display("could not create link from '{}' to '{}'", src.display(), dest.display())
        }
        LinkingDirectory {
            src: PathBuf,
            dest: PathBuf,
        } {
            description("could not symlink directory")
            display("could not create link from '{}' to '{}'", src.display(), dest.display())
        }
        CopyingDirectory {
            src: PathBuf,
            dest: PathBuf,
        } {
            description("could not copy directory")
            display("could not copy directory from '{}' to '{}'", src.display(), dest.display())
        }
        CopyingFile {
            src: PathBuf,
            dest: PathBuf,
        } {
            description("could not copy file")
            display("could not copy file from '{}' to '{}'", src.display(), dest.display())
        }
        RemovingFile {
            name: &'static str,
            path: PathBuf,
        } {
            description("could not remove file")
            display("could not remove '{}' file: '{}'", name, path.display())
        }
        RemovingDirectory {
            name: &'static str,
            path: PathBuf,
        } {
            description("could not remove directory")
            display("could not remove '{}' directory: '{}'", name, path.display())
        }
        SettingPermissions {
            path: PathBuf,
        } {
            description("failed to set permissions")
            display("failed to set permissions for '{}'", path.display())
        }
        CargoHome {
            description("couldn't find value of CARGO_HOME")
        }
        MultirustHome {
            description("couldn't find value of RUSTUP_HOME")
        }
    }
}
