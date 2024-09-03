pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_test() {
        let a1 = [100, 200, 300];
        let a2 = [100, 200, 300];
        assert_eq!(a1, a2);
    }

    #[test]
    fn vec_test() {
        let v1= vec!["apple", "banana", "mango"];
        let mut v2 = Vec::new();

        v2.push("apple");
        v2.push("banana");
        v2.push("mango");

        assert_eq!(v1, v2);
    }
}
