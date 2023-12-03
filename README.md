Advent of Code 2023
===================

Behold my solutions to [Advent of Code 2023](https://adventofcode.com).

They are written in Rust and I make no promises that the code is well-written.
I hope that it's functional; I want to solve the puzzles, but well, this year
is off to a great start.

Implementation Note
-------------------

The creator of AoC has asked that the input files to each of the puzzles is not
shared, so that people can't just steal them from the website. As such, my
input files have been put in an input directory that has been ignored by Git.
(As a bonus, this also means that none of my code assumes that it's running on
specifically _my_ input files). However, if you want to run my solutions
yourself, you're going to need to organize your repository the way I do.

Input files live in a directory called `src/input`. Input files live in text
documents titled `dayXX.txt` where `XX` is the day of the puzzle, with a
leading zero if necessary for proper sorting. The examples given for each of
the puzzles lives in `dayXX-test` where `XX` is once again the day of the
puzzle.

All of the solutions assume that `cargo run` is being performed _from the repo
root_ because I haven't found a better solution for that yet. I may refactor
the code so that it does not rely on this assumption but I'll get back to you
on that one.
