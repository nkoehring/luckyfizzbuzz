Lucky Fizz Buzz
===============

A FizzBuzz with some extras, implemented in Rust.

This repository is meant as a starter and inspiration to learn Rust because some otherwise obvious things are handled a bit different in this language.

simple FizzBuzz
---------------

The simple FizzBuzz is about implementing a function, class or program that for any continouus range of integers produces output with the following conditions:

* is multiple of 3: "fizz"
* is multiple of 5: "buzz"
* is multiple of 3 and 5: "fizzbuzz"
* otherwise the number itself
* all outputs are separated by space

Running the program for an inclusive range from 1 to 20 should yield following output:
`1 2 fizz 4 buzz fizz 7 8 fizz buzz 11 fizz 13 14 fizzbuzz 16 17 fizz 19 buzz`

some extras
-----------

To spice it up a little, following additional criteria will be added:

* if the number contains a 3, the output is replaced with "lucky"
* additionally a report is printed at the end, which states the amount of values printed per value type

Now the output should look like:
```
1 2 lucky 4 buzz fizz 7 8 fizz buzz 11 fizz lucky 14 fizzbuzz 16 17 fizz 19 buzz
fizz: 4
buzz: 3
fizzbuzz: 1
lucky: 2
integer: 10
```


