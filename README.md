# mental-calc
A small CLI app which helps you to practise mental calculation.

Mental-calc let you practise mental calculation. Just run the executable and follow the on screen instructions.

You can decide what mathematical operations you want to practise and the size of the numbers. You can select multiple mathematical operations and let the program randomly select one at each question.

### Features

* Count right answers and measure time.
* Let you assemble a set of mathematical operations. The operations will then be shuffeled randomly.
* Let you decide the size of the numbers.
* Supports addition, subtraction and multiplication, more mathematical operations will hopefully be added in the future.
* Supports negative numbers.

### Build

Mental-calc is written in Rust, so you'll need Rust's package manager Cargo in order to compile it. The easiest way is to visit [rustup.rs](https://rustup.rs) and download rustup. Mental-calc compiles with the latest stable release of Rust.

To build mental-calc:

```
$ git clone https://github.com/tage64/mental-calc
$ cd mental-calc
$ cargo build --release
$ ./target/release/mcalc  # And follow the on screen instructions.
```


### License

GPL-3.0 (or any later version at your option).
See LICENSE.txt for the full license text.