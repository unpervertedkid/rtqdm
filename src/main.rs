use rtqdm::progress::ProgressIteratorExtention;
use std::thread::sleep;

fn main() {
    let vector = vec![1, 2, 3];

    for number in vector
        .iter()
        .progress()
        .with_bound()
        .with_delimiters(('*', '*'))
    {
        expensive_calculation(&number);
    }

    for number in (1..).progress() {
        expensive_calculation(&number);
    }
}

fn expensive_calculation<T>(_n: &T) {
    sleep(std::time::Duration::from_secs(1));
}
