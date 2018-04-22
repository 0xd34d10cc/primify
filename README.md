# primify 
Generate prime number that in binary form looks like provided  image

# Usage
```
USAGE:
    primify [OPTIONS] --input <INPUT>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --edge <EDGE>            The color edge between 0 and 1 (0-255)
    -i, --input <INPUT>          Input file name
    -n, --witnesses <ITER>       Number of witnesses
    -h, --height <MAX_HEIGTH>    Max image height
    -w, --width <MAX_WIDTH>      Max image width
    -o, --output <OUTPUT>        Output file name
    -s, --sieve <SIEVE>          Sieve upper bound
```

# Example
See  [example](example) directory

# Performance of backends
With  image from example and default arguments

`num-bigint` backend:
```
real    0m55.793s
user    7m14.313s
sys     0m0.181s
```

`ramp` backend:
```
real    0m20.399s
user    2m34.344s
sys     0m0.076s
```

`gmp` backend:
```
real    0m7.656s
user    0m59.536s
sys     0m0.025s
```
