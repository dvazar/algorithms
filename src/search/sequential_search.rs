/// Sequential search.
/// Worst-case performance: O(n)
/// Average performance: O(n)
/// Best-case performance:	O(1)


pub fn sequential_search <T:Eq> (value: &T, collection: &Vec<T>) -> bool {
    for elem in collection.iter() {
        if elem == value {
            return true;
        }
    }
    return false;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequential_search_i32() {
        let array = vec![5, 2, 4, 6, 1, 3];

        assert_eq!(sequential_search(&1, &array), true);
        assert_eq!(sequential_search(&123, &array), false);
    }

    #[test]
    fn test_sequential_search_chars() {
        let array = vec!['b', 'f', 'c', 'a', 'e', 'd'];

        assert_eq!(sequential_search(&'d', &array), true);
        assert_eq!(sequential_search(&'z', &array), false);
    }

}
