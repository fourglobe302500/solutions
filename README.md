# Solutions For Club On Rust

A github repo that holds the solutions made by Francisco Lima or Fourglobe in rust as a member of Clube de Programação Competitiva FCUL

## Solutions:

- [3n + 1](https://github.com/fourglobe302500/solutions/blob/master/src/problems/three_n_plus_one.rs) - [UV100](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=3&page=show_problem&problem=36) problem where if n is even $n = n/2$ and if n is odd $n = 3n+1$, and the cycle length of the starter number is how many iterations it takes for n to reach 0, it is supposed that any n will eventually reach 0 but it is not provem yet, even though it is proven that any n less than $2^{68} ≈ 2.95×1020$.

- [Minesweeper](https://github.com/fourglobe302500/solutions/blob/master/src/problems/minesweeper.rs) - [UV10189](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=29&page=show_problem&problem=1130) is a problem where given a field of $N×M$ with **'$`.`$'** as safe spaces and **'$`*`$'** as mines returns a populated field with the mine count of each cell.

- [Making Change](https://github.com/fourglobe302500/solutions/blob/master/src/problems/making_change.rs) - [UV166](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=3&page=show_problem&problem=102) is a problem where given a restricted amount of coins, return the minimum amount of coins exchanged.

- [Station Balance](https://github.com/fourglobe302500/solutions/blob/master/src/problems/station_balance.rs) - [UV410](https://onlinejudge.org/index.php?option=onlinejudge&Itemid=8&category=6&page=show_problem&problem=351) is a problem where given a number of chambers and weights place them in a way that the diference between the sum of the values in each chamber and the average weight on all chambers is minimal.

- [The Dragon of Loowater](https://github.com/fourglobe302500/solutions/blob/master/src/problems/loowater.rs) - [UV11292](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=2267) is a problem where given a amount of heads and knight and their diameters and heights, calculate if the dragon can be killed and when it can how many gold coins it's going to cost.

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

and it will run all solutions listed on the arguments in order
