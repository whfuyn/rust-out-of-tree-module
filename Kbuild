# SPDX-License-Identifier: GPL-2.0

obj-m := rust_out_of_tree.o
rustflags-y := --extern add -L $(src)/add/target/target/debug/deps
rust_out_of_tree-y := rust_out_of_tree_main.o add.o
