use indicatif::ProgressBar;
use std::path::Path;

pub struct TerrariaFile {
    pub name: String,
}

pub fn seek_files<'a>(
    dir: &Path,
    progress_bar: &'a mut ProgressBar,
    message: &'static str,
) -> FileIterator<'a> {
    let dir = std::fs::read_dir(dir).unwrap();
    let files = dir
        .map(|entry| {
            let entry = entry.unwrap();
            TerrariaFile {
                name: entry.path().file_stem().unwrap().to_str().unwrap().into(),
            }
        })
        .collect::<Vec<_>>();

    progress_bar.set_length(files.len() as u64);
    progress_bar.set_message(message);
    FileIterator {
        files,
        progress_bar,
    }
}

pub struct FileIterator<'a> {
    progress_bar: &'a ProgressBar,
    files: Vec<TerrariaFile>,
}

impl<'a> Iterator for FileIterator<'a> {
    type Item = TerrariaFile;

    fn next(&mut self) -> Option<Self::Item> {
        self.progress_bar.inc(1);
        self.files.pop()
    }
}
