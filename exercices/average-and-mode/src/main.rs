// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

fn main() {
    // use vec! macro to delcare new Vector from given array
    let grades = vec![12, 14, 13, 15, 17, 20, 19, 14, 13, 11, 9];

    // sum all the grades
    let sum: i16 = grades.iter().sum();

    // compute average, casting the integers as floats for precision
    let average = f32::from(sum) / grades.len() as f32;

    // round the average to the second decimal
    // https://stackoverflow.com/questions/28655362/how-does-one-round-a-floating-point-number-to-a-specified-number-of-digits
    let average = (average * 100.0).round() / 100.0;

    // print the result
    println!("average grade is: {}", average);
}
