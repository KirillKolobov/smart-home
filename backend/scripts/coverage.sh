#!/bin/bash

cargo llvm-cov nextest --html && xdg-open target/llvm-cov/html/index.html