import gleeunit

import aoc2025

pub fn main() -> Nil {
  gleeunit.main()
}

// gleeunit test functions end in `_test`
pub fn hello_world_test() {
  let sample_input =
    "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"

  assert aoc2025.solve(sample_input) == #(50, 24)
}
