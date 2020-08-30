#!/bin/bash

assert() {
    expected="$1"
    input="$2"

    ./target/debug/compiler_rs "$input" > products/product.s
    
    cd products
    cc product.s -o product
    chmod u+x product
    ./product
    result="$?"

    cd ..

    if [ "$result" = "$expected" ]; then
        echo "$input => $result"
    else
        echo "$input => $expected expected, but result was $result"
        exit 1
    fi
}

cargo build

assert 0 0
assert 42 42
assert 41 " 12 + 34 - 5 "
echo OK
