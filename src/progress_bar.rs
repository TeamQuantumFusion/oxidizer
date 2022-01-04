
use console::{Color, style};

use indicatif::{ProgressBar, ProgressStyle};

pub struct Progress {
    progress_bar: ProgressBar,
}

impl Progress {
    pub fn new() -> Self {
        let header = "{msg} [{prefix}".to_owned() + "{percent}% ";
        let footer = "{eta} {pos}".to_owned() + &*style("/").fg(Color::White).to_string() + "{len}";

        let format = header + "{wide_bar:.cyan}\x1b[m] " + &*footer;
        let progress_bar = ProgressBar::new(1).with_style(
            ProgressStyle::default_bar()
                .template(&*format)
                .progress_chars("##-"),
        );


        Self {
            progress_bar
        }
    }

    pub fn update(&mut self, tasks: usize, task: &'static str) {
        self.progress_bar.set_position(0);
        self.progress_bar.set_length(tasks as u64);
        self.progress_bar.set_message(task);
    }

    pub fn iter<'a, I: Iterator>(&mut self, tasks: I, task: &'static str) -> ProgressIter<I> {
        self.progress_bar.set_position(0);
        self.progress_bar.set_length(tasks.size_hint().1.unwrap_or(0) as u64);
        self.progress_bar.set_message(task);
        ProgressIter { progress_bar: self.progress_bar.clone(), iterator: tasks }
    }

    pub fn iter_unsized<'a, I: Iterator>(&mut self, tasks: I, amount: usize, task: &'static str) -> ProgressIter<I> {
        self.progress_bar.set_position(0);
        self.progress_bar.set_length(amount as u64);
        self.progress_bar.set_message(task);
        ProgressIter { progress_bar: self.progress_bar.clone(), iterator: tasks }
    }

    pub fn print<I: AsRef<str>>(&mut self, message: I) {
        self.progress_bar.println(message);
    }

    pub fn next(&mut self) {
        self.progress_bar.inc(1);
    }
}

pub struct ProgressIter<I: Iterator> {
    progress_bar: ProgressBar,
    iterator: I,
}

impl<'a, I: Iterator> Iterator for ProgressIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.progress_bar.inc(1);
        self.iterator.next()
    }
}
