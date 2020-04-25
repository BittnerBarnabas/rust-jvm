use itertools::Itertools;
//TODO Add tests
pub trait ResultIterator {
    fn collect_to_result<T, E>(&mut self) -> Result<Vec<T>, E>
    where
        Self: Iterator<Item = Result<T, E>>,
    {
        self.fold_results(Vec::new(), |mut vec, item| {
            vec.push(item);
            return vec;
        })
    }
}

impl<T: ?Sized> ResultIterator for T where T: Iterator {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
