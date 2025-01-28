fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);  // This is now valid since `vec` is mutable
    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);  // vec0 is moved into the function
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
