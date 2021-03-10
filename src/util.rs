
/// Defines a fold over an iterator using a `Fn` instead of `FnMut`.
pub fn immutable_iterator_fold<'a, F, U, V, I>(f: F, acc: V, mut iterator: I) -> V 
    where F: Fn(V, U) -> V,
          U: 'a,
          I: Iterator<Item = U>,
{
    match iterator.next() {
        Some(data) => {
            let acc = f(acc, data);
            immutable_iterator_fold(f, acc, iterator)
        }
        None => acc
    }
}

mod test {
    use super::*;

    #[cfg(test)]
    fn test_immutable_iterator_fold() {
        let values = vec![1,2,3];
        let folding = |acc, value| value + acc;
        let result = immutable_iterator_fold(folding, 0, values.iter());
        assert_eq!(result, 6);
    }

}
