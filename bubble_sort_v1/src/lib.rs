use std::fmt::Debug;

pub fn bubble_sort<T: PartialOrd + Debug> (v: &mut [T]) {
    for _ in 0..v.len() {
        println!("{:?}", v);
        let mut sorted = true;
        for i in 0..(v.len() - 1) {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return; 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut v = vec![2, 5, 3, 8, 21, 15, 18, 12];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 5, 8, 12, 15, 18, 21]);
    }
}
