use std::fmt;

enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(i32),
}

impl fmt::Display for FizzBuzz {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FizzBuzz::Fizz => f.write_str("fizz"),
            FizzBuzz::Buzz => f.write_str("buzz"),
            FizzBuzz::FizzBuzz => f.write_str("fizzbuzz"),
            FizzBuzz::Number(num) => write!(f, "{}", num),
        }
    }
}

impl FizzBuzz {
    fn single(i: i32) -> FizzBuzz {
        if i % 15 == 0 {
            FizzBuzz::FizzBuzz
        } else if i % 5 == 0 {
            FizzBuzz::Buzz
        } else if i % 3 == 0 {
            FizzBuzz::Fizz
        } else {
            FizzBuzz::Number(i)
        }
    }

    fn range(a: i32, b: i32) -> String {
        (a..b)
        .map(|n| format!("{}", FizzBuzz::single(n)))
        .collect::<Vec<String>>()
        .join(" ")
    }
}

fn main() {
    println!("{}", FizzBuzz::range(1, 21));
}

#[test]
fn test_fizz_buzz() {
    let desired_output = "1 2 fizz 4 buzz fizz 7 8 fizz buzz 11 fizz 13 14 fizzbuzz 16 17 fizz 19 buzz";
    let actual_output = FizzBuzz::range(1, 21);

    assert_eq!(desired_output, actual_output);
}
