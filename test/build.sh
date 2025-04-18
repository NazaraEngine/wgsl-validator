#!/bin/bash

clang valid_test.c -o valid ../target/release/libwgsl_validator.so;
clang invalid_test.c -o invalid ../target/release/libwgsl_validator.so;
