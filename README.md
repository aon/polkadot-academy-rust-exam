<p align="center">
<img alt="Polkadot Blockchain Academy" src="./.assets/polkadot.gif" style="width:350px;">
<h1 align="center">Blockchain Academy <br> Rust Entrance Exam</h1>
</p>

All students of the academy must be proficient in the [Rust programming language](https://rust-lang.org/).
To that end, we have created this Rust entrance exam in order to assess your competency.

> **_Keep all content of this repo private and do not share the content of the exam with anyone._**

## Before You Begin

We suggest you read the [Rust Book](https://doc.rust-lang.org/book/) completely, including appendixes, and complete all exercises.
At minimum, you will need to be familiar with chapters 1 - 11 as well as chapters 13 and 19 to complete this exam.

For those completely new to `git` and Github, please review [this video](https://www.youtube-nocookie.com/embed/10krMetDSWs) to learn the basics required for this exam.

### Setting Expectations

This exam is intended to be a challenge.

- Expect a minimum of 10 hours to complete it, and quite likely more if solutions are not immediately apparent to you.
  We don't expect you to complete this in a single sitting, but do be mindful that that time away from the exam does not violate the [honor code](./src/a_honor_code.rs).

- You are not required to complete everything on this exam, only to put in your best effort.

- Please leave in-line comments for graders about issues and questions you have, and ideas of how you might solve them but were unable to.

### Optional Problems

Some problems in the exam, especially in the later parts, are marked as optional. You are not required to complete these problems
and they will not affect your score on the exam whether you complete them or not.

These problems are provided to give you a sense of the level of Rust that you will encounter when working on Substrate and
to encourage you to continue studying and improving your Rust skills even if you do already meet the minimal requirements
to participate in the course.

### Honor Code

**Before starting any problems, review the [Honor Code](./src/a_honor_code.rs), as you will be required to abide by it while taking this exam, and attest that you have followed it to pass!**

## Working on this Exam

All problems are located in the `src` directory.

We suggest completing them in alphabetically prefixed order, however feel free to work on them in any order you like.
Be sure to read the comments in all files, as they include further instruction as well as required things `TODO`.

As you progress, push to the _private_ [Github Classroom](https://classroom.github.com/) generated repo's `main` branch.
We encourage you to push up all progress even if incomplete and a work in progress at frequent intervals.

Most problems ask you to write some original Rust code and allow referencing _some_ external resources. Some problems include starter code available and just ask you to complete it.
Others will describe what you need to write in English and leave all the coding to you.

**Make sure you use the exact function, type, and trait signatures provided as your code will be subjected to [automated grading](#automated-assessment) that depends on this.**

Even if logically "correct" these will not pass tests and will not earn any credit towards your grade.
For example, if you are asked to write a function `factorial(n: u32) -> u32`, don't write `calculate_factorial(n: u32) -> u64` which has both the wrong name and the wrong return type.

### Testing

We have included integration tests so you can minimally check your work. Passing all items in `/tests` is required to be
considered for further more rigorous grading. We recommend you write your own unit tests as you go to ensure your work is
correct. Our provided tests are intentionally light.

You can compile and run an [integration test](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests) for a specific problem by specifying the filename prefix, for example:

```sh
cargo t --test a # tests exclusively `src/a_some_file.rs`
cargo t --test b # tests exclusively `src/b_some_file.rs`
cargo t --test c # tests exclusively `src/c_some_file.rs`
```

**Do not modify the included integration tests, as grading requires them to remain unchanged.**

However, we encourage you to include unit tests in your solutions as you see fit.

## Submission and Grading

Only work that exists on Github at the deadline will be considered in grading.

### Deadline

The deadline for submission will be communicated when the exam is first sent to you, and the Github classroom invitation link mentions this explicitly as well.
After this deadline you may no longer able to push to your exam repo's `main` branch.
All grades will be assessed using the commit present on `main` at the time of the deadline.
All other branches, commits, comments on PRs, etc. will be ignored.

### Private Test Suite and Manual Grading

The primary way we will grade your work is through an automated testing suite that is kept private to the Academy team.
The private test suite is much more thorough than the included integration tests and will test many edge cases.

There are also some human-graded aspects such as:

- Ensuring that your code is of high quality and readability.
- Ensuring that your solutions are not plagiarized.
- Ensuring that you haven't imported a crate to do the heavy lifting of your code problem.
- Ensuring that you have written the actual algorithm requested in a particular problem as opposed to a different algorithm that is easier to code.
  Eg writing bubble sort when we asked for merge sort.
- Ensuring you have followed any specific directions for a problem.
  Eg used functional-style iterator methods as opposed to imperative-style loops.

## 🚀 Good luck! 🚀

Please reach out to the Academy team if you have any questions or concerns related to the exam.
