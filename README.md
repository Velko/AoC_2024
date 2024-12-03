Advent of Code 2024
===================

These are my notes about solving the programming puzzles in [Advent of Code 2024](https://adventofcode.com/2024).

This time giving a try to [Rust](https://www.rust-lang.org/) programming language.

Had been practicing a bit during the year on exercises from 2015, so I got some helper functions ready. On the other
hand, Rust is not my everyday language, so my skills in it is a bit _rusty_ (not sure if pun intended).

Day 1
=====

Part 1
------

Quite simple stuff. Parse, then order the lists. Subtract the items, take an absolute value. Sum them all and voila!


Part 2
------

Process one list, counting the matching items in another. Multiply and sum them all.

Could made a little refactoring as there's some code repetition in solution for Part 2, but it is simple enough anyway.

Could come back and polish it some more, when in the mood.

Day 2
=====

Part 1
------

It could have been done using a loop, but [windows()](https://doc.rust-lang.org/std/primitive.slice.html#method.windows) function
allows to make it more elegant.


Part 2
------

A flashback to the [mirrors](https://adventofcode.com/2023/day/13). Solved it by simply checking all possibilities. I'm sure there might be a
smarter solution, but the number of records is small enough for a simple one.

Encountered a potential function for library extension: skip_nth() on Iterator. Will see if I will need it more than once.

It's just Day 2 - we're warming up :-)

Day 3
=====

Part 1
------

Just a simple regex.

Part 2
------

Ok, ok. Not that simple regex. And a state machine.
