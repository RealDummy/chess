# Chess
A pretty basic chess game made in rust. I am not saying this is a fast implementation of chess, no bitboards or whatever are being used, but it is playable.

## future additions
I would like to figure out all this chess programing stuff and make a real contender for a fast, efficient chess game, and also I would like to impliment a basic
board evaluation feature to create a simple chess bot. That would be fun.

## Why Rust
I feel like any time one wishes to write code in an uncommon language, the first question that will be asked is "Why?" With good reason too. The tooling is often worse, the community less matured. Maybe most importantly, there will be less people that understand the code. These are all good points. Below are some reasons I chose to learn Rust to make chess, instead of C++.

### Performance
It has always been my hope that I could use this to power a chess bot one day, and fast chess bots need to be running on good stuff. Interpreted languages are magical, but not very fast. Compiled languages are for sure the way to go, at least in my opinion.

### Less (bad) features
"Within C++, there is a much smaller and cleaner language struggling to get out" -Bjarne Stroustrup
C++ has been around for a very long time. We have learned a lot of things about programming since then. C++ is not only used to solve the problems of today, but also the problems of years ago. Rust feels like that smaller and cleaner language trying to escape.

### Super Iterators
Rust has access to iterators that blow C++ iterators out of the water. Working with a game like chess, having access to chainable iterator functions makes the coding process much cleaner and more fool proof. Out of bounds errors are practically impossible with Rust's iterator functions, while C++ iterators will happily give you an iterator to unallocated memory.

### Powerful Enums
Enumerations in C++ are a disaster. They are numbers, but worse. When the enum simply needs to be flags for function, they work fine, but as soon as anything more complicated comes up, the limitations start to show. Rust enums are on a new level. They are type safe, can't be turned into integers unless explicitly allowed, and can be used with the powerful match tool to represent data in a very clear and understandable nature. This allows for less headaches.

### Chainsaw Monkey
One of my professors said coding in C was like a monkey with a chainsaw. C++, while in practice is better than C, still allows you to code yourself into a very bad situation. Most of the things that make C++ less dangerous are guidelines that the compiler can't enforce. Rust's const correctness and borrowing concept together cover a very large portion of things that can go wrong in C++.
Variables are const in Rust by default, which catches more potential bugs than I imagined. For example, I was borrowing a vector of chess pieces, iterating through them, and doing some things to each piece. The whole process was supposed to be const, but the compiler said I needed to make a variable mutable in order for my code to work. That didn't seem right to me, and it turns out one of the operations I was doing to the pieces was mutating them. This would have been a tricky bug to catch in C++ if I wasn't counting everything, and the Rust compiler caught it for me.
One of the biggest C++ headaches is passing by value, reference or r-value reference. Rust avoids all this headache by never implicitly copying variables. Ever. If that isn't exciting to you, I don't know what to tell you. Variables are moved by default, and then instead of just slapping a null in the old variable and calling it good, Rust will not allow you to use the old variable again. What exactly happens is a little complicated, but at least the complexity can't result in copying a string 100 times.

### Summary
I feel for any small project that doesn't have legacy code, Rust offers several distinct advantages over C++. The newness of the language can be frustrating. If you can get over that, Rust is a very powerful language that I feel works very well for my chess coding needs.

