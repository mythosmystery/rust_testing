use std::{
    collections::VecDeque,
    iter::successors,
    vec::{self, Vec},
};

fn main() {
    // let tup = min_max((1, 2, 3, 4, 5));
    // println!("Min: {}, Max: {}", tup.0, tup.1);
    // let answer = persistence(999999999999999);
    // print!("Answer: {}", answer);
    // let time = queue_time(&[1, 2, 3, 4, 5], 1);
    // println!("Total time: {}", time)
    // let hello = "";
    // println!("{}", camel_case(hello));
}

// fn camel_case(str: &str) -> String {
//     str.split(' ')
//         .map(|w| {
//             w.get(..1).unwrap_or_default().to_ascii_uppercase()
//                 + &w.get(1..).unwrap_or_default().to_string()
//         })
//         .collect::<Vec<String>>()
//         .join("")
// }

// fn queue_time(customers: &[u32], n: u32) -> u32 {
//     let n = n.try_into().unwrap();
//     let mut minutes = 0;
//     let mut tills: Vec<u32> = vec![0; n];
//     let mut vec_customers = customers.to_vec();
//     loop {
//         for i in 0..n {
//             if tills[i] > 0 {
//                 tills[i] -= 1;
//             }
//             if vec_customers.len() > 0 && tills[i] == 0 {
//                 tills[i] = vec_customers.remove(0)
//             }
//         }
//         if tills.clone().into_iter().sum::<u32>() == 0 && vec_customers.len() == 0 {
//             break minutes;
//         }
//         minutes += 1;
//     }
// }

// fn min_max(list: &[i32]) -> (i32, i32) {
//    (*list.iter().min().unwrap(), *list.iter().max().unwrap())
// }

// fn persistence(num: u64) -> u64 {
//     fn digit_len(n: u64) -> u64 {
//         successors(Some(n), |&n| (n >= 10).then(|| n / 10)).count().try_into().unwrap()
//     }
//     fn to_digits(mut v: u64) -> Vec<u64> {
//         let mut digits: Vec<u64> = Vec::new();

//         while v > 0 {
//             let n = (v % 10) as u64;
//             v /= 10;
//             digits.push(n);
//         }
//         digits
//     }
//     fn multiply_digits(n: u64) -> u64 {
//         to_digits(n).into_iter().reduce(|acc, item| {acc * item}).unwrap()
//     }
//     if digit_len(num) == 1 {
//         return 0;
//     }
//     let mut accumulator = 1;
//     let mut result = multiply_digits(num);
//     loop {
//         println!("Result: {}", result);
//         if digit_len(result) == 1 {
//             break accumulator;
//         }
//         result = multiply_digits(result);
//         accumulator += 1;
//     }
// }
