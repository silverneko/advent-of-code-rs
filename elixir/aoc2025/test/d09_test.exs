defmodule Aoc2025.D09Test do
  use ExUnit.Case, async: true
  alias Aoc2025.D09

  doctest D09

  @sample_input """
  7,1
  11,1
  11,7
  9,7
  9,5
  2,5
  2,3
  7,3
  """

  describe "day 9" do
    test "sample input" do
      assert D09.solve(@sample_input) == {50, 24}
    end
  end
end
