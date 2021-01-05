use std::collections::HashSet;

fn main() {

    let k:usize = 12;
    let mut array: Vec<usize> = vec![10, 15, 3, 7];
    array.sort();

    let result = |array:Vec<usize>, k: usize| -> bool {
        let hs: HashSet<usize> = array.clone().into_iter().collect();

        for i in 0..array.len() - 1 {
            println!("len: {} - i: {} - k: {} - array[i]: {} - array[i+1]: {}", array.len(), i, k, array[i], array[i+1]);

            if hs.contains(&(k - array[i])) {
                return true;
            }
        }

        false
    };



    println!("S: {:?}", result(array, k));
}
