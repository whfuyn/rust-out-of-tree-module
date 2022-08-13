// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample

use kernel::prelude::*;
use add::add;

module! {
    type: RustOutOfTree,
    name: "rust_out_of_tree",
    author: "Rust for Linux Contributors",
    description: "Rust out-of-tree sample",
    license: "GPL",
}

struct RustOutOfTree {
    numbers: Vec<i32>,
}

impl kernel::Module for RustOutOfTree {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust out-of-tree sample (init)\n");
        pr_info!("1 + 2 = {}\n", add(1, 2));


        let mut numbers = Vec::new();
        numbers.try_push(72)?;
        numbers.try_push(108)?;
        numbers.try_push(200)?;

        Ok(RustOutOfTree { numbers })
    }
}

impl Drop for RustOutOfTree {
    fn drop(&mut self) {
        pr_info!("My numbers are {:?}\n", self.numbers);
        pr_info!("Rust out-of-tree sample (exit)\n");
    }
}
