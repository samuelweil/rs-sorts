pub fn bubble_sort<T: PartialOrd + Clone>(v: &Vec<T>) -> Vec<T> {
    let mut switched = true;
    let mut new_vec = v.to_vec();

    while switched {
        switched = false;

        for i in 1..v.len() {
            if new_vec[i - 1] > new_vec[i] {
                new_vec.swap(i, i - 1);
                switched = true;
            }
        }
    }

    new_vec
}

pub fn insert_sort<T: PartialOrd + Clone>(v: &Vec<T>) -> Vec<T> {
    let mut new_vec: Vec<T> = Vec::new();

    for x in v.iter() {
        let mut index = new_vec.len();
        for (i, y) in new_vec.iter().enumerate() {
            if x < y {
                index = i;
                break;
            }
        }

        new_vec.insert(index, x.clone());
    }

    new_vec
}

pub fn quick_sort<T: PartialOrd>(v: &mut Vec<T>, lo: usize, hi: usize) {
    if lo < hi {
        let pivot = partition(v, lo, hi);
        quick_sort(v, lo, pivot - 1);
        quick_sort(v, pivot + 1, hi);
    }
}

fn partition<T: PartialOrd>(v: &mut Vec<T>, lo: usize, hi: usize) -> usize {
    let pivot_pos: usize = (lo + hi) / 2;

    loop {
        let mut i = lo;
        let mut j = hi;
        {
            let pivot = &v[pivot_pos];

            while v[i] < *pivot {
                i += 1;
            }

            while v[j] > *pivot {
                j -= 1;
            }

            if i >= j {
                break j;
            }
        }

        v.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        let raw = vec![1, 7, 0, 4, 2, 6, 8];
        let sorted = vec![0, 1, 2, 4, 6, 7, 8];

        assert_eq!(sorted, bubble_sort(&raw));
    }

    #[test]
    fn test_insert_sort() {
        let raw = vec![1, 7, 0, 4, 2, 6, 8];
        let sorted = vec![0, 1, 2, 4, 6, 7, 8];

        assert_eq!(sorted, insert_sort(&raw));
    }

    #[test]
    fn test_quick_sort() {
        let mut raw = vec![1, 7, 0, 4, 2, 6, 8];
        let sorted = vec![0, 1, 2, 4, 6, 7, 8];

        let l = raw.len() - 1;
        quick_sort(&mut raw, 0, l);

        assert_eq!(sorted, raw);
    }
}
