use std::fs::DirEntry;
use std::path::PathBuf;
use std::slice::Iter;

use indicatif::{ProgressBar, ProgressBarIter};

pub struct TerrariaFile {
    pub name: String,
}

pub fn seek_files<'a>(dir: &PathBuf, progress_bar: &'a mut ProgressBar, message: &'static str) -> FileIterator<'a> {
    let dir = std::fs::read_dir(dir).unwrap();
    let mut out = Vec::new();
    for x in dir {
        let entry = x.unwrap();
        out.push(TerrariaFile { name: entry.file_name().to_str().unwrap().trim_end_matches(".xnb").into() })
    };

    progress_bar.set_length(out.len() as u64);
    progress_bar.set_message(message);
    FileIterator::new(out, progress_bar)
}

pub struct FileIterator<'a> {
    progress: &'a ProgressBar,
    data: Vec<TerrariaFile>,
}

impl<'a> FileIterator<'a> {
    pub fn new(data: Vec<TerrariaFile>, progress_bar: &'a mut ProgressBar) -> FileIterator<'a> {
        Self {
            progress: progress_bar,
            data,
        }
    }
}

impl<'a> Iterator for FileIterator<'a> {
    type Item = TerrariaFile;

    fn next(&mut self) -> Option<Self::Item> {
        self.progress.inc(1);
        self.data.pop()
    }
}