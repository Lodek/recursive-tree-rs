
/// Defines a fold over an iterator using a `Fn` instead of `FnMut`.
pub fn imutable_iterator_fold<F, U, V, I>(f: F, acc: V, mut iterator: I) -> V 
    where F: Fn(V, U) -> V,
          I: Iterator<Item = U>
{
    match iterator.next() {
        Some(data) => {
            let acc = f(acc, data);
            imutable_iterator_fold(f, acc, iterator)
        }
        None => acc
    }
}

mod test {

    #[cfg(test)]
    fn test_imutable_iterator_fold() {
        let vales = vec![1,2,3];
        let folding = |acc, value| value + acc;
        let result = imutable_iterator_fold(folding, 0, values.iter());
        assert_eq!(result, 6);
    }

}
