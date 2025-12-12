defmodule Aoc2025.D01Test do
  use ExUnit.Case, async: true
  alias Aoc2025.D01

  doctest D01

  @sample_input "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
" |> String.trim()

  test "day 1" do
    assert D01.solve(@sample_input) == {3, 6}
  end
end
