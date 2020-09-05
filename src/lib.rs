use std::vec::IntoIter;

pub type Size = u32;
pub type Position = u32;

/// Returns the list of best proximity found for these positions ordered by size.
///
/// Every keyword's positions list must be sorted.
pub fn near_proximity(mut keywords: Vec<IntoIter<Position>>) -> Vec<(Size, Vec<Position>)> {
    if keywords.len() < 2 {
        match keywords.pop() {
            Some(keywords) => return keywords.map(|p| (0, vec![p])).collect(),
            None => return vec![],
        }
    }

    // Pop top elements of each list.
    let mut heap = Vec::new();
    let mut current = Vec::with_capacity(keywords.len());
    for (i, positions) in keywords.iter_mut().enumerate() {
        match positions.next() {
            Some(p) => current.push((i, p)),
            None => return heap,
        }
    }

    // Sort k elements by their positions.
    current.sort_unstable_by_key(|(_, p)| *p);

    println!("current {:?}", current);

    // Find leftmost and rightmost keyword and their positions.
    let mut leftmost = *current.first().unwrap();
    let mut rightmost = *current.last().unwrap();

    loop {
        println!("leftmost {:?}", leftmost);
        println!("rightmost {:?}", rightmost);

        // Find the position p of the next elements of a list of the leftmost keyword.
        // If the list is empty, break the loop.
        let p = keywords[leftmost.0].next().map(|p| (leftmost.0, p));

        println!("p {:?}", p);

        // If p > r, then the interval [l, r] is minimal and
        // we insert it into the heap according to its size.
        if p.map_or(true, |p| p.1 > rightmost.1) {
            let mut tmp = current.clone();
            tmp.sort_unstable_by_key(|(i, _)| *i);
            let path = tmp.into_iter().map(|(_, p)| p).collect();
            let size = rightmost.1 - leftmost.1;
            println!("Found a new path {:?} of size {}", path, size);
            heap.push((size, path));
        }

        // TODO not sure about breaking here or when the p list is found empty.
        let p = match p {
            Some(p) => p,
            None => break,
        };

        let q = current[1];

        // Remove the leftmost keyword P in the interval,
        // and pop the same keyword from a list.
        *current.iter_mut().find(|(i, _)| *i == p.0).unwrap() = p;
        current.sort_unstable_by_key(|(_, p)| *p);

        println!("current {:?}", current);

        // if [l, r] is minimal.
        if p.1 > rightmost.1 {
            rightmost = p;
            leftmost = q;
        } else {
            leftmost = if p.1 < q.1 { p } else { q };
        }

        println!();
    }

    // Sort the list according to the size and the positions.
    heap.sort_unstable();
    heap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_keywords() {
        let hello = vec![0, 1, 6, 10].into_iter();
        let kind =  vec![2, 11].into_iter();
        let world = vec![3, 7, 12].into_iter();
        let keywords = vec![hello, kind, world];
        let paths = near_proximity(keywords);

        let mut paths = paths.into_iter();
        assert_eq!(paths.next(), Some((2, vec![1, 2, 3])));
        assert_eq!(paths.next(), Some((2, vec![10, 11, 12])));
        assert_eq!(paths.next(), Some((4, vec![6, 2, 3])));
        assert_eq!(paths.next(), Some((4, vec![10, 11, 7])));
        assert_eq!(paths.next(), None);
    }

    #[test]
    fn three_keywords_bis() {
        let hello = vec![0, 5, 10].into_iter();
        let kind =  vec![1, 6, 11].into_iter();
        let world = vec![2, 7, 12].into_iter();
        let keywords = vec![hello, kind, world];
        let paths = near_proximity(keywords);

        let mut paths = paths.into_iter();
        assert_eq!(paths.next(), Some((2, vec![0, 1, 2])));
        assert_eq!(paths.next(), Some((2, vec![5, 6, 7])));
        assert_eq!(paths.next(), Some((2, vec![10, 11, 12])));
        assert_eq!(paths.next(), Some((4, vec![5, 1, 2])));
        assert_eq!(paths.next(), Some((4, vec![5, 6, 2])));
        assert_eq!(paths.next(), Some((4, vec![10, 6, 7])));
        assert_eq!(paths.next(), Some((4, vec![10, 11, 7])));
        assert_eq!(paths.next(), None);
    }

    #[test]
    fn three_keywords_ter() {
        let hello = vec![0, 4, 8, 12, 16, 20].into_iter();
        let kind =  vec![13].into_iter();
        let world = vec![14, 15].into_iter();
        let keywords = vec![hello, kind, world];
        let paths = near_proximity(keywords);

        let mut paths = paths.into_iter();
        assert_eq!(paths.next(), Some((2, vec![12, 13, 14])));
        assert_eq!(paths.next(), Some((3, vec![16, 13, 14])));
        assert_eq!(paths.next(), None);
    }

    #[test]
    fn empty() {
        let paths = near_proximity(vec![]);
        assert!(paths.is_empty());
    }

    #[test]
    fn one_keyword() {
        let hello = vec![0, 1, 2, 6, 10].into_iter();
        let keywords = vec![hello];
        let paths = near_proximity(keywords);

        let mut paths = paths.into_iter();
        assert_eq!(paths.next(), Some((0, vec![0])));
        assert_eq!(paths.next(), Some((0, vec![1])));
        assert_eq!(paths.next(), Some((0, vec![2])));
        assert_eq!(paths.next(), Some((0, vec![6])));
        assert_eq!(paths.next(), Some((0, vec![10])));
        assert_eq!(paths.next(), None);
    }

    #[test]
    fn two_keywords() {
        let hello = vec![0, 1, 6, 10].into_iter();
        let world = vec![2, 3, 7, 12].into_iter();
        let keywords = vec![hello, world];
        let paths = near_proximity(keywords);

        let mut paths = paths.into_iter();
        assert_eq!(paths.next(), Some((1, vec![1, 2])));
        assert_eq!(paths.next(), Some((1, vec![6, 7])));
        assert_eq!(paths.next(), Some((2, vec![10, 12])));
        assert_eq!(paths.next(), Some((3, vec![6, 3])));
        assert_eq!(paths.next(), Some((3, vec![10, 7])));
        assert_eq!(paths.next(), None);
    }
}