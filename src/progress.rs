const CLEAR: &str = "\x1B[2J\x1B[1;1H";

pub struct Unbounded;
pub struct Bounded {
    bound: usize,
    delimiters: Option<(char, char)>,
    progress_bar_style: ProgressBarStyle,
}

impl Bounded {
    pub fn with_progress_bar_style(mut self, style: ProgressBarStyle) -> Self {
        self.progress_bar_style = style;
        self
    }
}

pub struct ProgressBarStyle {
    filled_char: char,
    unfilled_char: char,
    total_length: usize,
}

impl Default for ProgressBarStyle {
    fn default() -> Self {
        Self {
            filled_char: '█',
            unfilled_char: ' ',
            total_length: 50,
        }
    }
}
pub struct Progress<Iter, Bound> {
    iter: Iter,
    count: usize,
    bound: Bound,
}

trait ProgressDisplay: Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}

impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("Iterations: {}", progress.count);
    }
}

impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        let percentage = (progress.count as f64 / self.bound as f64) * 100.0;
        let filled = (percentage as usize / 2) as usize; // assuming each filled_char represents 2%
        let unfilled = self.progress_bar_style.total_length - filled;

        println!(
            "{:3.0}% |{}{}| {}/{}",
            percentage,
            self.progress_bar_style.filled_char.to_string().repeat(filled),
            self.progress_bar_style.unfilled_char.to_string().repeat(unfilled),
            progress.count,
            self.bound
        );
    }
}

impl<Iter> Progress<Iter, Unbounded> {
    fn new(iter: Iter) -> Self {
        Self {
            iter,
            count: 0,
            bound: Unbounded,
        }
    }
}

impl<Iter> Progress<Iter, Unbounded>
where
    Iter: ExactSizeIterator,
{
    pub fn with_bound(self) -> Progress<Iter, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delimiters: None,
            progress_bar_style: ProgressBarStyle::default(),
        };

        Progress {
            iter: self.iter,
            count: self.count,
            bound,
        }
    }
}

impl<Iter> Progress<Iter, Bounded> {
    pub fn with_delimiters(mut self, delimiters: (char, char)) -> Self {
        self.bound.delimiters = Some(delimiters);
        self
    }
}

impl<Iter, Bound> Iterator for Progress<Iter, Bound>
where
    Iter: Iterator,
    Bound: ProgressDisplay,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR);

        self.bound.display(self);
        self.count += 1;

        self.iter.next()
    }
}

pub trait ProgressIteratorExtention: Sized {
    fn progress(self) -> Progress<Self, Unbounded>;
}

impl<Iter> ProgressIteratorExtention for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Self, Unbounded> {
        Progress::new(self)
    }
}
