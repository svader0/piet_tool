# Piet_Tool

## Description

Piet_Tool is an interpreter for the [Piet programming language](https://www.dangermouse.net/esoteric/piet.html) written in Rust. It has support for translating Piet programs into equivalent Forth code, which can then be run on a Forth interpreter or system.

## Limitations

Piet_Tool is a work in progress and has some limitations. The main limitations are currently:

- Some programs will run indefinitely. This is due to a bug in the way Piet_Tool handles termination conditions. This is a known issue and will be fixed in a future release.
- The Forth translation is not very good. Some Piet programs will not translate correctly to Forth, or will just be translated in a way that is verbose and stupid. The translation feature is a work in progress.
- The interpreter is not very fast. Piet programs can be slow to run, especially if they are large or complex. Not like anybody picks Piet for its speed though...

## Usage

To use Piet_Tool, you need to have Rust installed. You can install Rust by following the instructions on the [Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can clone this repository and run `make install` to build the Piet_Tool executable. You can then run Piet_Tool with the following command:

```bash
./piet_tool <image path>
```

`<image path>` is the path to a Piet program image file. Piet_Tool will then interpret the Piet program and output the result to the console.

You can provide a codel size to Piet_Tool with the `-c` flag. For example, to run a Piet program with a codel size of 10, you can use the following command:

```bash
./piet_tool -c 10 <image path>
```

You can also use the `-f` flag to output the Forth translation of the Piet program. For example, to output the Forth translation of a Piet program with a codel size of 10, you can use the following command:

```bash
./piet_tool -c 10 -f <image path>
```

This will output the Forth translation of the Piet to a file called (by default) `out.f`. You can specify a different output file with the `-o` flag.

The max execution steps can be set with the `-s` flag.

The debug level can be set with the `-d` flag. It takes values from 0 to 3, with 0 being no debug output and 3 being the most verbose (a full program trace).

## Examples

The `examples` directory contains some example Piet programs that you can run with Piet_Tool. For example, you can run the `hello_world.png` program with the following command:

```bash
./target/release/piet_tool examples/hello_world.png
```

This will output `Hello, World!` to the console.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
The exception is the `examples` directory, which contains Piet programs that I found online. The only example I wrote is 'Fogarty.png', the rest are from the [Piet website](http://www.dangermouse.net/esoteric/piet/samples.html) or other sources.
