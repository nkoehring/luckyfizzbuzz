use std::fmt;

enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Lucky,
    Integer(i32),
}
impl fmt::Display for FizzBuzz {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FizzBuzz::Fizz => f.write_str("fizz"),
            FizzBuzz::Buzz => f.write_str("buzz"),
            FizzBuzz::FizzBuzz => f.write_str("fizzbuzz"),
            FizzBuzz::Lucky => f.write_str("lucky"),
            FizzBuzz::Integer(num) => write!(f, "{}", num),
        }
    }
}
impl FizzBuzz {
    fn single(i: i32, report: &mut Report) -> FizzBuzz {
        // there might be a mathematical solution to this
        // but this is much more readable
        if i.to_string().contains("3") {
            report.lucky += 1;
            FizzBuzz::Lucky
        } else if i % 15 == 0 {
            report.fizzbuzz += 1;
            FizzBuzz::FizzBuzz
        } else if i % 5 == 0 {
            report.buzz += 1;
            FizzBuzz::Buzz
        } else if i % 3 == 0 {
            report.fizz += 1;
            FizzBuzz::Fizz
        } else {
            report.integer += 1;
            FizzBuzz::Integer(i)
        }
    }

    fn range(a: i32, b: i32) -> String {
        let mut report = Report::new();
        let mut output = (a..b)
            .map(|n| format!("{}", FizzBuzz::single(n, &mut report)))
            .collect::<Vec<String>>()
            .join(" ");

        output.push_str(&format!("\n{}", report));
        output
    }
}

struct Report {
    fizz: i32,
    buzz: i32,
    fizzbuzz: i32,
    lucky: i32,
    integer: i32,
}
impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "fizz: {}\nbuzz: {}\nfizzbuzz: {}\nlucky: {}\ninteger: {}",
            self.fizz, self.buzz, self.fizzbuzz, self.lucky, self.integer
        )
    }
}
impl Report {
    fn new() -> Report {
        Report {fizz: 0, buzz: 0, fizzbuzz: 0, lucky: 0, integer: 0}
    }
}

fn main() {
    println!("{}", FizzBuzz::range(1, 21));
}

#[test]
fn test_fizz_buzz() {
    let desired_output =  "1 2 lucky 4 buzz fizz 7 8 fizz buzz 11 fizz lucky 14 fizzbuzz 16 17 fizz 19 buzz\nfizz: 4\nbuzz: 3\nfizzbuzz: 1\nlucky: 2\ninteger: 10";
    let actual_output = FizzBuzz::range(1, 21);

    assert_eq!(desired_output, actual_output);
}
