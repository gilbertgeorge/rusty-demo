#![allow(unused)]

use std::io::{self, stdin};
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    // const GREETING: &str = "What is your name?";
    // println!("{}",GREETING);
    // let mut name = String::new();
    // io::stdin().read_line(&mut name).expect("Didn't receive input");
    // println!("Hello, {}", name.trim_end());

    const FIZZBUZZ: &str = "Enter fizzbuzz limit:";
    println!("{}",FIZZBUZZ);
    let mut fizz_value = String::new();
    io::stdin().read_line(&mut fizz_value).expect("Didn't receive input");
    fizz_buzz(fizz_value.trim().parse().expect("Input not an integer"));

    const FIB: &str = "Enter fibonacci term:";
    println!("{}",FIB);
    let mut fib_value = String::new();
    io::stdin().read_line(&mut fib_value).expect("Didn't receive input");

    let mut fib_result1 = fibonacci_hashmap(fib_value.trim().parse().expect("Input not an integer"));
    print!("The {}th term of the fibonacci sequence is {}\n", fib_value.trim_end(), fib_result1);

    let mut fib_result2 = fibonacci_memo(fib_value.trim().parse().expect("Input not an integer"));
    print!("The {}th term of the fibonacci sequence is {}\n", fib_value.trim_end(), fib_result2);    

    let mut fib_result3 = fibonacci(fib_value.trim().parse().expect("Input not an integer"));
    print!("The {}th term of the fibonacci sequence is {}\n", fib_value.trim_end(), fib_result3);
}

// The FizzBuzz problem is a classic test given in coding interviews. 
// The task is simple: Print integers 1 to N, but print “Fizz” if an 
// integer is divisible by 3, “Buzz” if an integer is divisible by 5,
// and “FizzBuzz” if an integer is divisible by both 3 and 5.
fn fizz_buzz(input: i32){
    let mut index = 1;
    loop{
        if index > input{
            break;
        }
        else{
            if (index % 3 == 0) && (index % 5 == 0){
                println!("FizzBuzz");
            }
            else if index % 3 == 0{
                println!("Fizz");
            }
            else if index % 5 == 0 {
                println!("Buzz");
            }
            else{
                print!("{}\n", index);
            }
        }
        index+=1;
    }
}

//recursive get fib number
fn fibonacci(input: usize) -> usize{
    if input == 0 {
        return 0;
    }
    else if input <= 1 {
        return 1;
    }
    else{
        return fibonacci(input-1) + fibonacci(input-2);
    }
}

//using hashmap memo
fn fibonacci_hashmap(input: usize) -> usize{
    struct Fib {
        memo: HashMap<usize, usize>,
    }
    impl Fib {
        fn new(num: usize) -> Fib{
            return Fib {
                memo: HashMap::with_capacity(num),
            };
        }
        fn get_fibonacci(&mut self, num:usize) -> usize{

            if num == 0 {
                return 0;
            }
            else if num <= 1 {
                return 1;
            }
            else{
                if self.memo.contains_key(&num){
                    return *self.memo.get(&num).unwrap();
                }
                else {
                    let one = self.get_fibonacci(num - 1);
                    let two = self.get_fibonacci(num - 2);
    
                    self.memo.entry(num).or_insert(one + two);
                    return *self.memo.get(&num).unwrap();
                }
            }
        }
    }

    let mut result = Fib::new(input);
    return result.get_fibonacci(input);
}

//using memo
fn fibonacci_memo(number: usize) -> usize {
    fn get_fibonacci(n: usize, memo: &mut [Option<usize>]) -> usize {
        memo[n].unwrap_or_else(|| {
            let result = {
                if n > 1 {
                    get_fibonacci(n - 1, memo) + get_fibonacci(n - 2, memo)
                } else if n ==0 {
                    0
                } else {
                    1
                }
            };
            memo[n] = Some(result);
            result
        })
    }

    get_fibonacci(number, &mut vec![None; number + 1])
}