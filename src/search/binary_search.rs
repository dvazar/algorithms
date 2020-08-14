/// Binary search.
/// Worst-case performance: O(log n)
/// Average performance: O(log n)
/// Best-case performance:	O(1)


pub fn binary_search <T:PartialOrd> (value: &T, collection: &Vec<T>) -> bool {
    let mut low: usize = 0;
    let mut high = collection.len() - 1;
    while low <= high {
        let mid = (low + high) / 2;
        if value < &collection[mid] {
            high = mid - 1;
        } else if value > &collection[mid] {
            low = mid + 1;
        } else {
            return true;
        }
    }
    return false;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seq_search_i32() {
        let array = vec![1, 2, 3, 4, 5, 6];

        assert_eq!(binary_search(&6, &array), true);
        assert_eq!(binary_search(&1, &array), true);
        assert_eq!(binary_search(&4, &array), true);
        assert_eq!(binary_search(&123, &array), false);
    }

    #[test]
    fn test_seq_search_chars() {
        let array = vec!['a', 'b', 'c', 'd', 'e', 'f'];

        assert_eq!(binary_search(&'a', &array), true);
        assert_eq!(binary_search(&'f', &array), true);
        assert_eq!(binary_search(&'d', &array), true);
        assert_eq!(binary_search(&'z', &array), false);
    }

}
