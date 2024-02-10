const CLEAR: &str = "\x1B[2J\x1B[1;1H";

pub struct Unbounded;
pub struct Bounded {
    bound: usize,
    delimiters: Option<(char, char)>,
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
        println!("{}", "#".repeat(progress.count));
    }
}

impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!(
            "{}{}{}{}",
            self.delimiters.unwrap_or(('[', ']')).0,
            "#".repeat(progress.count),
            " ".repeat(self.bound - progress.count),
            self.delimiters.unwrap_or(('[', ']')).1
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
    Iter: Iterator, Bound: ProgressDisplay,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR);

        self.bound.display(&self);
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
