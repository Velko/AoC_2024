Advent of Code 2024
===================

These are my notes about solving the programming puzzles in [Advent of Code 2024](https://adventofcode.com/2024).

This time giving a try to [Rust](https://www.rust-lang.org/) programming language.

Had been practicing a bit during the year on exercises from 2015, so I got some helper functions ready. On the other
hand, Rust is not my everyday language, so my skills in it is a bit _rusty_ (not sure if pun intended).


**SPOILER ALERT: Do not scroll down and read the rest of the README if you do not want to see spoilers**


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


Day 12
======

Part 1
------

It all starts with flood-filling the regions. After that, calculating areas is straightforward. Getting the perimeter was a bit more complicated,
but not by much. Just need to look around each plot.

Part 2
------

While it did not seem so at a first glance, it was quite challenging to come up with an idea, how to calculate the number of sides.

I ended up by scanning each row and marking if each plot begins a new side on the top edge. Then, the same on the bottom edge. Repeat that with
scanning columns and checking their left and right edges. Implementing that _correctly_ was another challenge.

I mainly focused on getting the E-shaped example right. That solved the whole thing.


Day 13
======

Part 1
------

At first I got a little scare when red about the hexagonal tiles. Fortunately that did not show up in the puzzle. Phew!

Quite an effort to parse the input. Then, for the solution I had to decide if I want to go the *proper* way or the lazy one. Opted for a lazy one
with looping.

Part 2
------

Ok, now it's time for the *proper* solution - using an equation system. Had to bring out the [Gauss eliminator](https://en.wikipedia.org/wiki/Gaussian_elimination)
from [last year](https://adventofcode.com/2023/day/24). It took a bit to port from C to Rust. Then a little fight to check and convert floating point
values back to integers.

Day 14
======

Part 1
------

Seemed to be simple enough. As the robots wrapped around the map (instead of bouncing off the edges), there was no need to go step by step. Just
multiply and take the modulo.


Part 2
------

My first reaction was: Dafuq? I even stated to look for clues elsewhere - like: there was a Christmas tree on AoC 2015 calendar? Should I try to
produce that? And I thought that each puzzle is self-contained.

Thinking of it more - it is not necessarily to know what image has to be formed, as long as it matches some criteria it should be fine.

My next thought: Ok, maybe robots travel only over some of the lines. Then there might be a cell that is only visited by small number (ideally -
just single) robot, while others are more popular. Then I could solve similarly to a Sudoku puzzle. Nope. Every robot visits every cell.

What's next? Surely robots return to their initial positions. What is their cycle length? Probably will need to calculate some Least Common Multiple
or something. Nope. All robots return to their starting positions after visiting every position on a map once.

Thinking back - the fact that map height and width were a prime numbers should have been a clue.

Ok, how about I just step through it step by step and look how many robots occupy adjacent cells? When they form a picture, that number should stay out.

Hey, there's one larger number! Let's write a code to print out the map at that moment of time. What's that? A tree? Eureka!

It appears that I was overthinking it, but I did not expect the resulting number to be that low. I was expecting that to be in several billions, not
around 6500.


Later I learned that it is quite easy to generate an image in BPM format. Here's another idea for solution:

* generate a huge 10201x10609 ( 101<sup>2</sup>x103<sup>2</sup> ) image, where every possible state is arranged in a 101x103 grid. (I tried another way around for a square, but found subsequent calculations too confusing)
* open the file in a image viewer/editor that allows zooming and shows cursor position. The tree stands out almost immediately.
* note the approximate coordinates of the tree.
* divide x by 101, to get column and y by 103 to get row
* row * 101 + column is the answer


Day 15
======

Part 1
------

This is almost [Sokoban](https://en.wikipedia.org/wiki/Sokoban), with the difference that multiple boxes can be moved.

Wasn't that hard, with the exception that I had a lot of external distractions this time. Probably the hardest part was that all my parsing routines
was not designed to read multi-part inputs.


Part 2
------

I'm getting [Tetris/Jenga](https://adventofcode.com/2023/day/22) vibes here. Solved it in similar way: by having a list of boxes and their projection
on the map. Collect all the movable boxes recursively and move them in one go, or bail out if wall was encountered.


Day 16
======

Part 1
------

This doesn't look too hard - just a standard BFS search. Ok, I got a tiny mistake, because turning and then moving costs 1001 in total, instead of 1000.


Part 2
------

And here we go, a hard one! After few unsuccessful attempts to modify my BFS for the first part, I tried to implement it as a basic recursive DFS. Worked
right for the sample, but was too slow for the real thing. What was I even hoping for?!

I ended up modifying by BFS to add a direction to the visited state and loop it until the nodes I visit start to appear further than the shortest found path
to the target. Collecting all the visited states, complete with the distances.

This way I got a list of all visited cells that can be reached with the "best" cost. Then I started to recursively search backwards within this list until
I get to the starting point. Collect all cells visited on the way, their number is the answer.


Day 17
======

Part 1
------

This is my kind of exercise! I love [custom processors](https://github.com/Velko/8-bit-CPU)!

Funny enough, I struggled with input parsing: could not come up with a regex, that splits the program part in one go. Gave up and parsed the string old-fashined way.

Implementing the CPU from the description was quite straightforward, with exception that I messed up one instruction.

My testing infrastructure, however, broke down. It was designed to check for a single numeric value, not a string. Luckily, none of the outputs started with zeros,
so I could simply convert it into a number, by concatenating all the digits.

Part 2
------

Now what? Sure, I tried out the brute-force to check if the example really behaves as expected, but wasn't crazy enough to believe that would work with the real
input.

It appears that I have to more or less get the idea on what that program is doing. Improved the interpreter to print out the instructions it is executing.

Ok, so it is a loop, where it takes 3 lowest bits from A, calculates some "mumbo-jumbo" from it and _few more bits_. Outputs the result, discards those 3 bits and continue until
register A becomes 0.

Without understanding the "mumbo-jumbo" I can not really tell what is needed to get the desired output...

Hey, what if I start from the end? The "few more" bits then should be all zeros. Then I can walk back to the beginning, with those "few more" already calculated. Bingo!

Day 18
======

Part 1
------

The puzzle looked too easy - basically the same as 2 days ago, only now one has to build the grid, and there's no turning costs.

Implemented it, left out the direction from state as it is not important anymore. It runs perfectly on sample data, but the real input takes too long. So there was a catch!

As the grid was somewhat sparsely populated, the queue of future cells grew in an enormous rate, compared to consumption from it. It appears, however, that we're pushing
same cells over and over again on the queue. What about some deduplication?


Part 2
------

Seeing the description of Part 1, I was afraid that there's something crazy to come. Not this time! I think it would have been sufficient with a simple loop, but I felt
fancy today and implemented a binary search.


Day 19
======

Part 1
------

Got little scared on hot springs, that was a puzzle from last year, where I spent lots of time. With some anticipation for what's to come,
wrote the "matcher" in a recursive way. A little debugging was involved, but got my result pretty quickly.

Part 2
------

Changed the return type from bool to integer and counted the number of arrangements instead. Obviously that worked fine for example, but was too
slow for the real input. Since this was a recursive solution, adding memoization was not complicated.


Day 20
------

Part 1
------

Another pathfinding? This time it's more like enumerating the grid cells with distances from start. I did not trully trust the input, so I still checked
if there are no "parallel" distance cells.

Now if a cell 2 places up, down, left or right is reachable faster than by normal means, it must be a cheat. Out with off-by-one errors, welcome off-by-2
errors.


Part 2
------

Not that different from the previous one. Just have to check if any cell within Manhattan distance up to 20 could be reached faster than by normal means.

Day 21

Part 1
------

From the description it did not seem that complicated. A bit of emulation, then take that in reverse. When simple heuristics failed, I resorted to searching
all the possibilities. Still, the brute-force looked like "waitable". I only pruned off the most obvious cases.

After submitting the result, the beginnings for P2 made me come back and write a proper solution, recording required transitions between numpad keys.

Part 2
------

Ok, waiting for it won't work. Now I have to think on a proper solution. This is the first time this year, when I have to postpone the solution till
later.

I have an idea - let's work with _transitions_: a pair of buttons, robots have to move from one to another. It I calculate, what it takes to move from
one digit to another, or from one directional button to another, I should be able to calculate the length of a command stream without actually building
it whole.

**The end of second day working on this problem:** It now calculates the answers to Part 1 blazingly fast. It even has a loop for the intermediate robots.

Unfortunately, when I change the length of the loop to 25, the answer I get is too high. It, however, is not a off-by one error, because if I change the
loop to 24 iterations, I get a "too low" answer. At least now I know that the number must be between these two, but that is not helping much.

Hopefully I'm missing something stupid, but if my main assumption, that there is "one true way" for each transition between buttons breaks down when the
number of levels increases, then it might be very bad. Or haven't I only identified the best ones?


_This year I have gotten this far without looking up any hints on Reddit (I only studied an old StackOverflow post about multiple best paths). Should I admit defeat and get some help? I'd really like to figure it out myself, but that might take a while..._

**Sudden win:** My "desperate" changes to re-arrange the sequence how the paths are generated suddenly yielded a number that was between the two. Tried it out and, **bingo, two stars!** It was something about preferring one direction over others. At least - I was not able to identify
the "one true" paths properly. The whole idea was sound after all. _I'm still worhty!_

I should probably study later more carefully, but this was a lucky accident.

Day 22
------

Part 1
------

This seems to be suspiciously easy. I immediately recognized bit-twiddling operations. Wondering what's coming next.


Part 2
------

Calculate prices and their changes, build an index of price for each first occurrence of a sequence and its associated prices.
Sum it all together and pick the highest.

This indeed appears to be a relief after yesterday.
