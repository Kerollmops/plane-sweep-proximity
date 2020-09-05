#![cfg_attr(feature = "nightly", feature(test))]

use std::iter::FromIterator;
use tinyvec::TinyVec;

pub type Size = u32;
pub type Position = u32;
pub type TinyVec12<T> = TinyVec<[T; 12]>;

/// Returns the list of best proximity found for these positions ordered by size.
///
/// Every keyword's positions list must be sorted.
pub fn near_proximity<I>(mut keywords: Vec<I>, output: &mut Vec<(Size, TinyVec12<Position>)>)
where I: Iterator<Item=Position>,
{
    output.clear();

    if keywords.len() < 2 {
        if let Some(keywords) = keywords.pop() {
            output.extend(keywords.map(|p| (0, TinyVec12::from_iter(Some(p)))));
        }
        return
    }

    // Pop top elements of each list.
    let mut current = TinyVec12::with_capacity(keywords.len());
    for (i, positions) in keywords.iter_mut().enumerate() {
        match positions.next() {
            Some(p) => current.push((i, p)),
            None => return,
        }
    }

    // Sort k elements by their positions.
    current.sort_unstable_by_key(|(_, p)| *p);

    // Find leftmost and rightmost keyword and their positions.
    let mut leftmost = *current.first().unwrap();
    let mut rightmost = *current.last().unwrap();

    loop {
        // Find the position p of the next elements of a list of the leftmost keyword.
        // If the list is empty, break the loop.
        let p = keywords[leftmost.0].next().map(|p| (leftmost.0, p));

        // let q be the position q of second keyword of the interval.
        let q = current[1];

        let mut leftmost_index = 0;

        // If p > r, then the interval [l, r] is minimal and
        // we insert it into the heap according to its size.
        if p.map_or(true, |p| p.1 > rightmost.1) {
            leftmost_index = current[0].0;
            current.sort_unstable_by_key(|(i, _)| *i);
            let path = current.iter().map(|(_, p)| *p).collect();
            let size = rightmost.1 - leftmost.1;
            output.push((size, path));
        }

        // TODO not sure about breaking here or when the p list is found empty.
        let p = match p {
            Some(p) => p,
            None => break,
        };

        // Remove the leftmost keyword P in the interval,
        // and pop the same keyword from a list.
        current[leftmost_index] = p;

        if p.1 > rightmost.1 {
            // if [l, r] is minimal, let r = p and l = q.
            rightmost = p;
            leftmost = q;
        } else {
            // Ohterwise, let l = min{p,q}.
            leftmost = if p.1 < q.1 { p } else { q };
        }

        // Then update the interval and order of keywords in the interval.
        current.sort_unstable_by_key(|(_, p)| *p);
    }

    // Sort the list according to the size and the positions.
    output.sort_unstable();
}

#[cfg(test)]
mod tests {
    use super::*;
    use tinyvec::tiny_vec;

    #[test]
    fn three_keywords() {
        let hello = vec![0, 1, 6, 10].into_iter();
        let kind =  vec![2, 11].into_iter();
        let world = vec![3, 7, 12].into_iter();
        let keywords = vec![hello, kind, world];
        let mut paths = Vec::new();
        near_proximity(keywords, &mut paths);

        let mut paths = paths.into_iter();
        assert_eq!(paths.next(), Some((2, tiny_vec![1, 2, 3])));
        assert_eq!(paths.next(), Some((2, tiny_vec![10, 11, 12])));
        assert_eq!(paths.next(), Some((4, tiny_vec![6, 2, 3])));
        assert_eq!(paths.next(), Some((4, tiny_vec![10, 11, 7])));
        assert_eq!(paths.next(), None);
    }

    #[test]
    fn three_keywords_bis() {
        let hello = vec![0, 5, 10].into_iter();
        let kind =  vec![1, 6, 11].into_iter();
        let world = vec![2, 7, 12].into_iter();
        let keywords = vec![hello, kind, world];
        let mut paths = Vec::new();
        near_proximity(keywords, &mut paths);

        let mut paths = paths.into_iter();
        assert_eq!(paths.next(), Some((2, tiny_vec![0, 1, 2])));
        assert_eq!(paths.next(), Some((2, tiny_vec![5, 6, 7])));
        assert_eq!(paths.next(), Some((2, tiny_vec![10, 11, 12])));
        assert_eq!(paths.next(), Some((4, tiny_vec![5, 1, 2])));
        assert_eq!(paths.next(), Some((4, tiny_vec![5, 6, 2])));
        assert_eq!(paths.next(), Some((4, tiny_vec![10, 6, 7])));
        assert_eq!(paths.next(), Some((4, tiny_vec![10, 11, 7])));
        assert_eq!(paths.next(), None);
    }

    #[test]
    fn three_keywords_ter() {
        let hello = vec![0, 4, 8, 12, 16, 20].into_iter();
        let kind =  vec![13].into_iter();
        let world = vec![14, 15].into_iter();
        let keywords = vec![hello, kind, world];
        let mut paths = Vec::new();
        near_proximity(keywords, &mut paths);

        let mut paths = paths.into_iter();
        assert_eq!(paths.next(), Some((2, tiny_vec![12, 13, 14])));
        assert_eq!(paths.next(), Some((3, tiny_vec![16, 13, 14])));
        assert_eq!(paths.next(), None);
    }

    #[test]
    fn three_keywords_quater() {
        let hello = vec![0, 4, 8, 12, 16, 20].into_iter();
        let kind =  vec![13, 23].into_iter();
        let world = vec![14, 15, 24].into_iter();
        let keywords = vec![hello, kind, world];
        let mut paths = Vec::new();
        near_proximity(keywords, &mut paths);

        let mut paths = paths.into_iter();
        assert_eq!(paths.next(), Some((2, tiny_vec![12, 13, 14])));
        assert_eq!(paths.next(), Some((3, tiny_vec![16, 13, 14])));
        assert_eq!(paths.next(), Some((4, tiny_vec![20, 23, 24])));
        assert_eq!(paths.next(), Some((8, tiny_vec![16, 23, 15])));
        assert_eq!(paths.next(), None);
    }

    #[test]
    fn empty() {
        use std::vec::IntoIter;
        let mut paths = Vec::new();
        near_proximity::<IntoIter<_>>(vec![], &mut paths);
        assert!(paths.is_empty());
    }

    #[test]
    fn one_keyword() {
        let hello = vec![0, 1, 2, 6, 10].into_iter();
        let keywords = vec![hello];
        let mut paths = Vec::new();
        near_proximity(keywords, &mut paths);

        let mut paths = paths.into_iter();
        assert_eq!(paths.next(), Some((0, tiny_vec![0])));
        assert_eq!(paths.next(), Some((0, tiny_vec![1])));
        assert_eq!(paths.next(), Some((0, tiny_vec![2])));
        assert_eq!(paths.next(), Some((0, tiny_vec![6])));
        assert_eq!(paths.next(), Some((0, tiny_vec![10])));
        assert_eq!(paths.next(), None);
    }

    #[test]
    fn two_keywords() {
        let hello = vec![0, 1, 6, 10].into_iter();
        let world = vec![2, 3, 7, 12].into_iter();
        let keywords = vec![hello, world];
        let mut paths = Vec::new();
        near_proximity(keywords, &mut paths);

        let mut paths = paths.into_iter();
        assert_eq!(paths.next(), Some((1, tiny_vec![1, 2])));
        assert_eq!(paths.next(), Some((1, tiny_vec![6, 7])));
        assert_eq!(paths.next(), Some((2, tiny_vec![10, 12])));
        assert_eq!(paths.next(), Some((3, tiny_vec![6, 3])));
        assert_eq!(paths.next(), Some((3, tiny_vec![10, 7])));
        assert_eq!(paths.next(), None);
    }
}

#[cfg(all(feature = "nightly", test))]
mod bench {
    extern crate test;
    use super::*;

    #[bench]
    fn three_keywords(b: &mut test::Bencher) {
        let hello = vec![0, 4, 8, 12, 16, 20].into_iter();
        let kind =  vec![13, 23].into_iter();
        let world = vec![14, 15, 24].into_iter();
        let keywords = vec![hello, kind, world];

        let mut paths = Vec::new();
        b.iter(|| near_proximity(keywords.clone(), &mut paths))
    }

    #[bench]
    fn three_long_keywords(b: &mut test::Bencher) {
        let hello = 0..100;
        let kind =  13..113;
        let world = 15..115;
        let keywords = vec![hello, kind, world];

        let mut paths = Vec::new();
        b.iter(|| near_proximity(keywords.clone(), &mut paths))
    }
}
