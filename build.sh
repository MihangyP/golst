#!/bin/sh

set -x

rustc main.rs -Lraylib/lib -lraylib
