# Solutions For Club On Rust

A github repo that holds the solutions made by Francisco Lima or Fourglobe in rust as a member of Clube de Programação Competitiva FCUL

## Solutions:

- [three n plus one](https://github.com/fourglobe302500/solutions/blob/master/src/problems/three_n_plus_one.rs) - [$3n + 1$](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=3&page=show_problem&problem=36) problem where if n is even $n = n/2$ and if n is odd $n = 3n+1$, and the cycle length of the starter number is how many iterations it takes for n to reach 0, it is supposed that any n will eventually reach 0 but it is not provem yet, even though it is proven that any n less than $2^{68} ≈ 2.95×1020$.

- [minesweeper](https://github.com/fourglobe302500/solutions/blob/master/src/problems/minesweeper.rs) - [minesweeper](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=29&page=show_problem&problem=1130) is a problem where given a field of $N×M$ with __'$.$'__ as safe spaces and __'$*$'__ as mines returns a populated field with the mine count of each cell. 

## Installation and usage:

### With Rust:

- Have Rust installed on your machine, if you do not have follow the guide on [rust home page](https://www.rust-lang.org/tools/install)
- Clone the repo

```
git clone http://github.com/fourglobe302500/solutions.git
```

- run

```
cargo run --list
```

to get all solutions and then

```
cargo run <problem 1> <problem 2> ...
```

and it will run all solutions listed on the arguments in order

### Without Rust:

- Install the solutions with your preferred method from the releases

- On a command line run

```
solutions.exe --list
```

to get all solutions and then

```
solutions.exe  <problem 1> <problem 2> ...
```
