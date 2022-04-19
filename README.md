# Rust if branch benchmark




## Overview
I wanted to see if a hunch was right about if statements vs maths (and casting) was correct. Turns out I was *mostly* wrong :). 

This was a valuable reminder in how we should trust the compiler to optimise, how every language is different and what you remember about hyper optimised code from a few years ago, is never a good substitute for proper **profiling** and **benchmarking** with the specific problem you have now! 

## Background: 
A few years ago I was helping optimise a GLES shader for a driver we were working on, one of the suggestions was to get rid of IF statements where possible as the "conditional branch" is slower then pure maths. 

Recently I came across this video https://www.youtube.com/watch?v=EumXak7TyQ0 and found myself recanting this pearl of wisdom on some tangental comment thread, as you do.

"The pearl of wisdom" as I had decided to remember it was 
> if statements = "conditional branches" = slower then simple maths. 

But of course like most things, the real truth is a lot more nuanced then this.

For example see this great answer on Stack Overflow specifically about OpenGL shaders (the hurdle i came across all those years ago) and how if statements can cause different kinds of conditional branches:
https://stackoverflow.com/questions/37827216/do-conditional-statements-slow-down-shaders

He explains how conditional branching splits into three categories:
"

 - **Compile-time static**. The conditional expression is entirely based off of compile-time constants. As such, you know from looking at the code which branches will be taken. Pretty much any compiler handles this as part of basic optimization.

- **Statically uniform branching**. The condition is based off of expressions involving things which are known at compile-time to be constant (specifically, constants and uniform values). But the value of the expression will not be known at compile-time. So the compiler can statically be certain that wavefronts will never be broken by this if, but the compiler cannot know which branch will be taken.

- **Dynamic branching**. The conditional expression contains terms other than constants and uniforms. Here, a compiler cannot tell a priority if a wavefront will be broken up or not. Whether that will need to happen depends on the runtime evaluation of the condition expression

"
He also discusses how the hardware can impact the outcome. Its worth a read in its entirity, especially if your into your shaders!

## What did I do
I decided to run a basic benchmark test in rust. 

Partly because I couldn't do the direct casting I wanted in C#, but mostly because I wanted an excuse to actually play with Rust. 

So *caveat* there is a high chance this is terribly written Rust code and ironically i couldn't do the direct casting in the way I wanted in Rust either :p. 

## How to run the code
If you havent already you will need to install rust: https://www.rust-lang.org/learn/get-started

and then from the hello-rust (yup I never renamed it) folder simply run:

> cargo run

Or to run the robust benchmark run
> cargo criterion

## Some actual results:

### Long timing
These were taken by just running the functions in a loop, using random ages, where the whole loop was timed. 
```
//using sensible numbers
over 100000
get_drinking_message_via_if took 22ms
get_drinking_message_via_logical took 24ms

over 1000000
get_drinking_message_via_if took 258ms
get_drinking_message_via_logical took 260ms

over 10000000
get_drinking_message_via_if took 2661ms
get_drinking_message_via_logical took 2380ms

//using silly big numbers
get_drinking_mess_age_via_if took 53553ms
get_drinking_mess_age_via_logical took 49576ms

get_drinking_message_via_if took 552648ms
get_drinking_message_via_logical took 529534ms
```
All in all neither was a clear winner and the difference between the two was often too small to notice. 

### Using averages

This time I didnt time the whole loop and instead timed the inside of the loop and took averages. Of course the numbers were so small, I had to add an inner loop so i didn't just get "0" as an answer. 
```
get_drinking_message_via_if average of 0.0000374ms
get_drinking_message_via_logical took  0.0000520ms

get_drinking_message_via_if average of 0.0000212ms
get_drinking_message_via_logical took 0.0000544ms

get_drinking_message_via_if average of 0.000012ms
get_drinking_message_via_logical took 0.00028ms
```
With this the logical is actually shown to be slower!

### Final Custom Benchmark version 
If you run this code as it is today, on your machine, you will see this result:

```
Basic benchmark
-------------------------------------
get_drinking_message_via_if average of 0.000176ms
get_drinking_message_via_logical took 0.000178ms
```

This version does away with the random numbers and tries to be slightly cleaner with the averages. 

Again with this the logical is actually shown to be slower!

# Using Rust Bench and Criterion
It was at this point I finally decided to use a proper benchmarking tool.

Rust comes with 
> cargo bench

Which works brilliantly and tells you some interesting stuff.

I also used Rust criterion cargo to get some nice reports:
https://crates.io/crates/criterion

> cargo criterion

## with if version:
When ran stand alone:

```
get_drinking_message    time:   [276.63 ps 284.54 ps 293.27 ps]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe
```

## with casting version:
When ran stand alone directly after:

```
get_drinking_message    time:   [262.24 ps 264.43 ps 266.96 ps]
                        change: [-6.6146% -4.6553% -2.7004%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
```
then for sanity, **changed back to the if version**:
```
get_drinking_message    time:   [274.56 ps 281.33 ps 289.58 ps]
                        change: [+2.2427% +4.5858% +7.0853%] (p = 0.00 < 0.05)
                        Performance has regressed.
```
So this implies that the logical version is in fact faster! Just by very very little.

## using a more robust benchmark
Using Criterion I generated a more robust report, which really brought home how variable the results can be when the functions are so close in actual performance. It makes sense that they vary based on parameter (probably why my custom benchmark shouldn't use random) however i'm not entirely sure I understand why it varies as it does. 

I very much expected the "IF" version to win out on the under 18s (as its a very early return) but actually the "logical operator" version only loses out on "17", "20" and "27" which...makes no sense to me.

EDIT
I realised the code was wrong, so I fixed it and ran it again. This time the logical version was slower on "16", "18", "25" and "28". No logical reason I can think of. So again i think its just when they are *that* close in performance, a bit of variance is expected.

You can see the reports by running the code yourself.

# Conclusion
This was a fun little dive into Rust and a nice way to challenge my own assumptions because I did expect the logical operator (the none if) version to be faster by a measurable amount. 

Turns out, its not... well not properly measurable anyway. Which leads me to believe the rust compiler is clever enough to optimise the if statement perfectly well and attempting this micro optimisation was a fools errand. Perhaps its still good to keep in mind for if I ever find myself working in GLES shaders again.... possibly :p 

But as always, premature optimisation is the route of all evil and you should always profile, benchmark and repeat! ( basically TDD it :p )

