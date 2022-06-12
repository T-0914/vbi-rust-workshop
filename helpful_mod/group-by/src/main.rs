use std::collections::BTreeMap;

fn main() {
    let mut result = Vec::new();
    let mut offset = 0;
    let mut duplications: BTreeMap<i32, usize> = BTreeMap::new();
    let mut sample_array = vec![1, 2, 1, 5, 5, 10, 11, 4, 2, 2, 3, 3];
    sample_array.sort();

    for x in sample_array.clone() {
        *duplications.entry(x).or_insert(0) += 1;
    }

    for key in duplications.keys() {
        result.push(&sample_array[offset..(offset + duplications[key])]);
        offset = offset + duplications[key]
    }
    println!("result: {:#?}", result);
}
