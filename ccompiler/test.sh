#!/bin/bash

assert() {
  expected="$1"
  input="$2"
  
  ./mycc "$input" > tmp.s
  gcc tmp.s -o tmp.out
  ./tmp.out
  actual="$?"
  rm tmp.s
  rm tmp.out

  if [ "$actual" = "$expected" ]; then
    echo "$input" -> "$actual"
  else
    echo "$input" -> "$expected" expected, but got "$actual"
    exit 1
  fi
}

assert 0 0 
assert 42 42
assert 41 " 12 + 34 - 5 "

echo OK
