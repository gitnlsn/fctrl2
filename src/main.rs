use std::collections::HashMap;
use std::io;

struct LargeNumber {
    digits: HashMap<usize, usize>,
}

impl LargeNumber {
    fn new(mut num: usize) -> LargeNumber {
        let mut new_obj = LargeNumber {
            digits: HashMap::new(),
        };
        let mut index = 0;
        while num > 0 {
            new_obj.digits.insert(index, num % 10);
            index = index + 1;
            num = num / 10;
        }
        return new_obj;
    }

    fn multiply_primitive(&mut self, multiplier: usize) -> LargeNumber {
        let mut foo = LargeNumber::new(0);
        for index in self.digits.keys() {
            let multipled_digit = match self.digits.get(index) {
                Some(digit) => &digit,
                None => &0,
            };
            
            let mut this_result = multipled_digit * multiplier;
            let mut plus_one = 0;
            
            while this_result > 0 {
                let existing_digit = match foo.digits.get(&(index + plus_one)) {
                    Some(digit) => &digit,
                    None => &0,
                };
                this_result = this_result + existing_digit;
                
                foo.digits.insert(index + plus_one, this_result % 10);
                this_result = this_result / 10;
                plus_one = plus_one + 1;
            }
        }
        return foo;
    }

    // fn add(&mut self, other: &mut LargeNumber) -> LargeNumber {
    //     let mut ans = LargeNumber::new(0);
    //     let indexes = LargeNumber::get_indexes(self, other);
    //     for index in indexes.iter() {
    //         let self_digit = match self.digits.get(index) {
    //             Some(digit) => &digit,
    //             None => &0,
    //         };
    //         let other_digit = match other.digits.get(index) {
    //             Some(digit) => &digit,
    //             None => &0,
    //         };
    //         let existing_digit = match ans.digits.get(index) {
    //             Some(digit) => &digit,
    //             None => &0,
    //         };
            
    //         let mut sum = self_digit + other_digit + existing_digit;
    //         let mut plus_one = 0;
            
    //         while sum > 0 {
    //             ans.digits.insert(*index + plus_one, sum %10);
    //             sum = sum / 10;
    //             plus_one = plus_one + 1;
    //         }
    //     }
    //     return ans;
    // }
    
    fn as_string(&mut self) -> String {
        let max_size = match self.digits.keys().max(){
            Some(value) => &value,
            None => &0,
        } + 1;
        let printable: String = (0..max_size)
            .rev()
            .map(|index| {
                match self.digits.get(&index) {
                    Some(digit) => digit.to_string(),
                    None => "0".to_string(),
                }
            })
            .collect();
        return printable;
    }
    
    // pub fn get_indexes(a: &mut LargeNumber, b: &mut LargeNumber) -> Vec<usize> {
    //     let mut indexes: Vec<usize> = a
    //         .digits
    //         .keys()
    //         .cloned()
    //         .chain(b.digits.keys().cloned())
    //         .map(|x| x)
    //         .collect();

    //     indexes.sort();
    //     indexes.dedup();

    //     return indexes;
    // }
}

fn get_input() -> String {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed at read_line");
    return input.trim().parse().expect("Failed to parse");
}

fn factorial(mut n: usize) -> LargeNumber {
    let mut num = LargeNumber::new(1);
    while n > 0 {
        num = num.multiply_primitive(n);
        n = n - 1;
    }
    return num;
}

#[test]
fn test_constructor() {
    let num = LargeNumber::new(10);
    assert!(num.digits.get(&0).unwrap() == &0);
    assert!(num.digits.get(&1).unwrap() == &1);
    
    let num = LargeNumber::new(1010);
    assert!(num.digits.get(&0).unwrap() == &0);
    assert!(num.digits.get(&1).unwrap() == &1);
    assert!(num.digits.get(&2).unwrap() == &0);
    assert!(num.digits.get(&3).unwrap() == &1);
}

#[test]
fn test_printable() {
    let mut num = LargeNumber::new(123456);
    assert_eq!(num.as_string(), "123456");
}

#[test]
fn test_know_multiplication() {
    for i in 0..1002 {
        for j in 0..102 {
            assert_eq!(LargeNumber::new(i).multiply_primitive(j).as_string(), (i*j).to_string());
        }
    }
}

#[test]
fn test_multiplication() {
    assert_eq!(LargeNumber::new(1).multiply_primitive(2).as_string(), "2");
    assert_eq!(LargeNumber::new(1).multiply_primitive(100).as_string(), "100");
    assert_eq!(LargeNumber::new(1).multiply_primitive(2).as_string(), "2");
    assert_eq!(LargeNumber::new(100).multiply_primitive(1).as_string(), "100");
    assert_eq!(LargeNumber::new(2).multiply_primitive(2).as_string(), "4");
    assert_eq!(LargeNumber::new(22).multiply_primitive(2).as_string(), "44");
    assert_eq!(LargeNumber::new(11).multiply_primitive(11).as_string(), "121");
    assert_eq!(LargeNumber::new(111).multiply_primitive(11).as_string(), "1221");
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(0).as_string(), "1");
    assert_eq!(factorial(1).as_string(), "1");
    assert_eq!(factorial(2).as_string(), "2");
    assert_eq!(factorial(3).as_string(), "6");
    assert_eq!(factorial(4).as_string(), "24");
    assert_eq!(factorial(5).as_string(), "120");
    assert_eq!(factorial(6).as_string(), "720");
    assert_eq!(factorial(7).as_string(), "5040");
    assert_eq!(factorial(8).as_string(), "40320");
    assert_eq!(factorial(9).as_string(), "362880");
    assert_eq!(factorial(10).as_string(), "3628800");
    assert_eq!(factorial(11).as_string(), "39916800");
}

fn main() {
    let test_cases: isize = get_input().parse().expect("Failed to parse integer");

    for _index in 0..test_cases {
        let n = get_input().parse().expect("Failed to parse integer");
        println!("{}", factorial(n).as_string());
    }
}
