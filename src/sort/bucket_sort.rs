/// Bucket sorting algorithm, O(n)
///
/// Input: <a1, a2, ..., an>.
/// Output: <a1', a2', ..., an'>, where a1' <= a2' <= ... <= an'.
///

use super::insertion_sort::insertion_sort;


pub fn bucket_sort <H> (array: &mut Vec<u32>, hash: H)
where
    H: Fn(&u32) -> u32,
{
    let mut blocks: Vec<Vec<u32>> = vec![vec![]; array.len()];

    for elem in array.iter() {
        blocks[hash(elem) as usize].push(*elem);
    }

    let mut i: usize = 0;
    for mut block in blocks {
        insertion_sort(&mut block);
        for elem in block {
            array[i] = elem;
            i += 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_sort_i32() {
        let mut array = vec![5, 2, 4, 6, 1, 3];
        let expected_array = vec![1, 2, 3, 4, 5, 6];

        bucket_sort(&mut array, |&x| x/3);

        assert_eq!(array, expected_array);
    }

}
