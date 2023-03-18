# Tiny hash table

Exploration of different techniques to optimize (constant) hash tables. Inspired by this [blog post](https://orlp.net/blog/worlds-smallest-hash-table/).

## About

To analyze different techniques for optimizing a hash table an example will be used.
The table below shows the mapping between the desired inputs and outputs (or key-value-pairs).

| Input     | Output |
| --------- | ------ |
| 0xa582041 | 4      |
| 0xa592041 | 8      |
| 0xa5a2041 | 3      |
| 0xa582042 | 1      |
| 0xa592042 | 5      |
| 0xa5a2042 | 9      |
| 0xa582043 | 7      |
| 0xa592043 | 2      |

3 different methods are implemented and compared:

1. [HashMap](hashtable_dictionary.rs): use the built-in HashMap
2. [Array](hashtable_array_modulo.rs): build a lookup table and index it using modulo arithmetic
3. [Constant](hashtable_constant.rs): build and encode a lookup table in a constant and index it using shifts

The development of these methods is done in jupyter notebooks.

## Results

The different methods are compared by accessing the function 1M times using a for loop.

| Algorithm | Time      |
| --------- | --------- |
| HashMap   | 145.54 ms |
| Array     | 6.16 ms   |
| Constant  | 7.77 ms   |

Compiled with `rustc 1.68.0 (2c8cc3432 2023-03-06)`.
