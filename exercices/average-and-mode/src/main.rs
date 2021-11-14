// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn main() {
    // use vec! macro to delcare new Vector from given array
    let grades = vec![12, 14, 14, 13, 15, 17, 20, 19, 14, 13, 11, 9];

// compute average:

    // sum grades
    let sum: i16 = grades.iter().sum();

    // compute average, casting the integers as floats for precision
    let average = f32::from(sum) / grades.len() as f32;

    // round the average to the second decimal
    // https://stackoverflow.com/questions/28655362/how-does-one-round-a-floating-point-number-to-a-specified-number-of-digits
    let average = (average * 100.0).round() / 100.0;

    // print the result
    println!("average grade is: {}", average);

// compute mode:
    // create a new HashMap;
    // let mut occurences = HashMap::new();
    // or with given types (not need as type can be infered from line 32):
    let mut occurences: HashMap<&i16, i16> = HashMap::new();

    // for each grade
    grades.iter().for_each(|grade|
        // check if grade is present in hashmap keys `occurences.entry(grade)`
        // if not, insert 0 as default value `[...].or_inset(0)`
        // deference the key `*occurences` so it can update the value with `+= 1`
        *occurences.entry(grade).or_insert(0) += 1
    );

    // raw print of occurences
    // {19: 1, 20: 1, 17: 1, 12: 1, 15: 1, 13: 2, 9: 1, 14: 3, 11: 1}
    println!("{:?}", occurences);

    // find max pair in hash_map, a tuple is returned: (14, 3)
    // (if unwrap() is not used, `Some((14, 2))` is returned)
    let max = occurences.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

    println!("grade {} appears {} times.", max.0, max.1);
}
