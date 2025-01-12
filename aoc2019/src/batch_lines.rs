use itertools::Itertools;

pub trait BatchLines: Iterator<Item = char> + Sized {
    fn batch_lines(self) -> impl Iterator<Item = String> {
        #[allow(clippy::manual_map)]
        self.peekable().batching(|it| match it.peek() {
            Some(_) => Some(it.take_while(|&c| c != '\n').collect()),
            None => None,
        })
    }
}

impl<T> BatchLines for T where T: Iterator<Item = char> + Sized {}
