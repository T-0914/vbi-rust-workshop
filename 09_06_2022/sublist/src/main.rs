/**
 * ## Bài tập 1
 * Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ?
 * Ví dụ :
 *          let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
 *          let sub_arr = [6, 8, 10];
*/

/**
 * SOLUTION
 * We will try to divide the longer array into chucks that have the smaller array's length
 * Something like: vec![1, 2, 3, 5, 6, 8, 10, 11] and vec![6, 8, 10] => vec![1, 2, 3], vec![2, 3, 5], vec![3, 5, 6], .. and so on
 * And then we simply use equal operator (==) to compare them together.
 */
fn main() {
    let first_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let second_arr = [6, 8, 10];

    let result = is_sublist(&second_arr, &first_arr);

    println!("result: {:#?}", result);
}

fn is_sublist(array: &[i32], other_array: &[i32]) -> bool {
    if array.len() == 0 {
        return true;
    }

    for frame in other_array.windows(array.len()) {
        if frame == array {
            return true;
        }
    }

    return false;
}
