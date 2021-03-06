use std::fs;
use std::path::Path;

pub fn remove_files<P>(paths: &[P])
where
    P: AsRef<Path>,
{
    for path in paths {
        fs::remove_file(path).expect("destory file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    #[test]
    fn destroy_single_file() {
        let (_file, path) = get_new_tempfile_and_pathbuf();
        let paths: Vec<&Path> = vec![&path];
        remove_files(&paths);
        assert_path_deleted(&path);
    }

    #[test]
    fn destroy_multiple_files() {
        let multiplier = 10;
        let mut files = Vec::with_capacity(multiplier);
        let mut paths = Vec::with_capacity(multiplier);
        for _ in 0..multiplier {
            let (file, path) = get_new_tempfile_and_pathbuf();
            files.push(file);
            paths.push(path);
        }
        remove_files(&paths);
        assert_multi_paths_deleted(&paths);
    }

    fn get_new_tempfile_and_pathbuf() -> (NamedTempFile, PathBuf) {
        let tempfile = tempfile::NamedTempFile::new().expect("create tempfile");
        let temppath = PathBuf::from(tempfile.path());
        (tempfile, temppath)
    }

    fn assert_multi_paths_deleted(paths: &[PathBuf]) {
        for path in paths {
            assert_path_deleted(path);
        }
    }

    fn assert_path_deleted(path: &Path) {
        assert_eq!(false, path.exists());
    }
}
