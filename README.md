# rust if branch benchmark
## Overview
I wanted to see if a hunch was right about if statements vs casting was correct. I was mostly wrong :). 

This was a valuable reminder in how we should trust the compiler to optimise, how every language is different and what you remember about hyper optimised code from a few years ago, is never a good substitute for proper *profiling* and *benchmarking* with the specific problem you have now! 

## Background: 
A few years ago I was helping optimise a GLES shader for a driver we were working on, one of the suggestions was to get rid of IF statements where possible as the "conditional branch" is slower then pure maths. This came up in a comment thread on this video https://www.youtube.com/watch?v=EumXak7TyQ0

I had decided to remember this as if statements = "conditional branches" = slower then simple maths. But of course like most things, the real truth is a lot more nuanced then this, see this great answer on Stack Overflow specifically about OpenGL shaders and if branches:
https://stackoverflow.com/questions/37827216/do-conditional-statements-slow-down-shaders

Which explains how conditional branching splits into three categories and then we get into specifics on how hardware can impact the outcome. 

## Caveat
I decided to run a basic benchmark test in rust. Partly because I couldn't do the direct casting I wanted in C#, but mostly because I wanted an excuse to actually play with Rust. So there is a high chance this is terribly written Rust code and ironically i couldnt do the direct casting in Rust either :p. 

## Actual results:

## Long timing
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

All in all neither was a clear winner and the difference between the two was often. 

## using averages

get_drinking_message_via_if average of 0.0000374ms
get_drinking_message_via_logical took  0.0000520ms

get_drinking_message_via_if average of 0.0000212ms
get_drinking_message_via_logical took 0.0000544ms

With this the logical is actually shown to be slower!

# using Rust Criterion
Using Rust criterion cargo for proper benchmarking
https://crates.io/crates/criterion

## with if version:
```
get_drinking_message    time:   [276.63 ps 284.54 ps 293.27 ps]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe
```

## with casting version:
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
