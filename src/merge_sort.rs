/// Merge sorting algorithm, O(n log n)
/// 
/// Input: <a1, a2, ..., an>.
/// Output: <a1', a2', ..., an'>, where a1' <= a2' <= ... <= an'.
/// 
use std::mem::swap;


pub fn merge_sort (array: &mut Vec<i32>) {
    let mut a: Vec<_> = array.drain(..).collect();
    let mut b = a.split_off(a.len() / 2);
    
    if a.len() > 1 {
        merge_sort(&mut a);
    }
    if b.len() > 1 {
        merge_sort(&mut b);
    }
    let (mut a, mut b) = (a.drain(..), b.drain(..));
    
    let mut previous_value = a.next().unwrap();
    loop {
        match b.next() {
            Some(value) => {
                if value <= previous_value {
                    array.push(value);
                } else {
                    array.push(previous_value);
                    previous_value = value;
                    swap(&mut a, &mut b);
                }
            },
            None => match a.next() {
                Some(value) => {
                    if value <= previous_value {
                        array.push(value);
                    } else {
                        array.push(previous_value);
                        previous_value = value;
                    }
                },
                None => break,
            },
        };
    }
    array.push(previous_value);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut array = vec![5, 2, 4, 6, 1, 3];
        let expected_array = vec![1, 2, 3, 4, 5, 6];
        
        merge_sort(&mut array);
        
        assert_eq!(array, expected_array);
    }

}
