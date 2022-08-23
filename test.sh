#!/bin/bash

cargo build && \
(cd target/debug; cat ../../test.txt | ./jsonprsr)