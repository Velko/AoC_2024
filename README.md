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

Day 4
=====

Part 1
------

Looking for an X and then look around at growing distance for M A S.

<pre>
S  S  S
 A A A
  MMM
SAMXMAS
  MMM
 A A A
S  S  S
</pre>

Luckily I already had a helper for neighboring cell selection in a grid. Had to improve it a bit to support distances larger than 1 and to
return None for invalid coordinates, instead of skipping them.


Part 2
------

Now we can look for an A, and check the corner cells of a dist=1 neighbours. There are only 4 valid patterns: MMSS, MSMS, SSMM and SMSM.

<pre>
0 2
 A
5 7
</pre>


Day 5
=====

Part 1
------

Implemented it the hard way, by iterating, comparing indices and whatnot.

After completing _Part 2_ I came back and re-worked it using is_sorted_by() and the same comparison.


Part 2
------

Could not come up with a viable algorithm at first. And then it hit me: it's just a sorting, using custom comparison function!


Day 6
=====

Part 1
------

A lighter version of [bouncing beams](https://adventofcode.com/2023/day/16)? Nothing fancy - just walk the path and count steps. No, keep track of visited tiles.


Part 2
------

I'm sure there is a smart solution, but after several unsuccessful attempts, I reverted to one of the dumbest. Check, what happens if I
place an obstacle on every tile in path from Part 1. At this stage it is still viable to force it, but the complexity level has started to
rise.

Came back with some optimizations:

* Converted the obstacle check to not rely on external state. Rewrote the invocation to functional style. Then it was straightforward
  change to `par_iter()` from [rayon](https://docs.rs/rayon/latest/rayon/index.html) crate. Bam! Instant parallelism!
* Converted loop tracker from HashSet of coordinates and directions to a 3D array of bools, indexed by coordinates and directions. HashSet
  is _supposed_ to be fast, but it is slow when compared to array indexing. Not sure if [ndarray](https://docs.rs/ndarray/latest/ndarray/)
  is the right choice, as it appears to be quite heavy, but I don't think that a hand-crafted one would make much difference.

Got the runtime down from 22s to around 1s. It is still not in the millisecond range as some brag on Reddit, but whatever.


Day 7
=====

Part 1
------

Just checking all possible combinations. Iterating from 0 to 2^num gives a nice bit pattern for operator selection.


Part 2
------

Ok, now it is base-3? Not in the mood for that! What about base4 then? Just ignore one of them (or map one of the operations
twice). Now we can iterate from 0 to 4^num and select 2-bit patterns.

A bit slower than I would like, but still "forceable". Parallelizing improved that from 3m9s to 51s. Probably I could
memoize something for greater improvement, but I'm good for now.


_Later:_ It appears that implementing this recursively makes a difference. Even the slowest and less optimized version
(intentionally worsened to power of 4) completes in 19s. Switching to power of 3 speeds it up to 1.5s, early termination
if result of operations exceeds expected - down to 0.9s, and finally applying parallelization gets it down to 0.38s.
Overall - an enormous improvement over my first implementation. Sill, for me it feels a lot harder to understand and debug.


Day 8
=====

Part 1
------

A puzzle that was hard to understand, but fairly easy to implement. Just add the same distance to the each end of a pair.


Part 2
------

A direct extension of Part 1. Instead of simply checking if the antinode is on the map, keep going until it isn't. And include the
starting positions.


Day 9
=====

Part 1
------

Seemed to be simple enough to just expand the disk map and do it as described in the puzzle.

I only get a feeling that as further I go, by code gets dirtier and dirtier. I'm writing more and more loops, instead of functional
stuff. Then I do not bother with error handling that much: throwing in some unwrap()s gets it done. Also I'm not building proper
structs anymore - now it's tuples, over vectors, over tuples.


Part 2
------

I had a suspicion during the Part 1, what the second part will be. At first I tried to think of a proper one, using indexed ranges
and whatnot, but it seemed to be a bit too complicated. Then I reverted to an updated version of the Part 1, just moving whole blocks,
instead of single fragments. A slight optimization there was to also store the block/space length with each item, so that start/end
of them could be calculated more quickly. In hopes that won't be necessary, I also left out coalescing of free space (I did not need it).

The end result was not particulary fast, but still got the answer. I will probably come back and write more optimal solution, but this
is fine for now.


Day 10
======

Part 1
------

Walking around recursively and counting the target cells that can be reached.

I misunderstood the assignment at first and counted number of ways, how those peaks could be reached. Spent quite some time to figure out
why my answers were nowhere near to the expected ones.


Part 2
------

Funny enough, my incorrect version for Part 1 was actually a solution for Part 2.


Day 11
======

Part 1
------

Seemed to be straightforward enough to just go for it. Build a new list by iterating the input according to the rules. Swap it into the input
and repeat 25 times.

Part 2
------

I sure tried to bruteforce it, but at around iteration 38 it became clear, that it won't work. Then I got an idea: the sequence of the pebbles
does not matter, there's no need to build a long list. Every pebble produces exactly the same "descendants" so I just need to count them.
