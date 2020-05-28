# radixheap

## Description

`radixheap` is a Rust library implementing the *radix heap* data structure.

A *radix heap* is a monotone priority queue with provides extremely fast
(priority) element access. It is suitable only for very special use cases,
though.

The data structure consists of a set of *buckets* with exponentially increasing
size. Each data element is pushed onto the heap as *tuple* by associating a
natural number with the data as their *key*. The key is used to determine the
bucket the data has to be put into. The respective *priority element* (the one
with the lowest key) can later be popped off the heap in minimal time.
`radixheap` only supports *unsigned 32-bit intergers* (`u32`) as keys.


There is already a radix heap library at [crates.io](https://crates.io) which
is a bit more mature and performs a lot better.


## Compilation and Execution

The use of the Rust build tool `cargo` is highly recommended. To build
`radixheap`, `cargo` and the Rust compiler `rustc` are needed.

Running the *unit test* is as simple as:

	```
	$ cargo test
	```

An example code (`basic.rs`) using the library is provided under `examples/`.
The "example" also looks much like a unit test. It can be run using the command:

	```
	$ cargo run --example basic
	```

To build the optimized library `libradixheap.rlib` for use in production the
following can be executed:

	```
	$ cargo build --release
	```


## Copyright

Copyright &copy; 2019, 2020 Daniel Haase

`radixheap` is licensed under the **GNU Lesser General Public License**,
version 3.



## License disclaimer

```
radixheap - Radix heap data structure library
Copyright (C) 2019, 2020 Daniel Haase

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public
License along with this program.
If not, see <https://www.gnu.org/licenses/lgpl-3.0.txt>.
```

&lt;[https://www.gnu.org/licenses/lgpl-3.0.txt](https://www.gnu.org/licenses/lgpl-3.0.txt)&gt;
