#!/bin/sh

set -x

rustc golst.rs -o golst -Lraylib/lib -lraylib
