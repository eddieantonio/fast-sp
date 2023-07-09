// Copyright (C) 2023  Eddie Antonio Santos
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::env;
use std::process::Command;

fn main() {
    compile_c_library();
    generate_test_data();
}

fn compile_c_library() {
    let source = "src/count.c";
    cc::Build::new().file(source).compile("libcount.a");
    println!("cargo:rerun-if-changed={source}");
}

fn generate_test_data() {
    let script_name = "./generate-test-data.py";
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("python3")
        .args([script_name, "-C", &out_dir])
        .status()
        .unwrap();
    println!("cargo:rerun-if-changed={script_name}");
}
