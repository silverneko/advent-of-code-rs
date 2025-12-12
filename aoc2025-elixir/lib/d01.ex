defmodule Aoc2025.D01 do
  @doc ~S"""
  iex> Aoc2025.D01.parse "L12\nR24"
  [{:L, 12}, {:R, 24}]
  """
  def parse(input) do
    lines = input |> String.trim() |> String.split("\n")

    for l <- lines do
      {d, x} = String.split_at(l, 1)
      {String.to_atom(d), String.to_integer(x)}
    end
  end

  def solve(input) do
    input
    |> parse()
    |> Enum.reduce({50, 0, 0}, fn {d, x}, {acc, sum1, sum2} ->
      m =
        case d do
          :L -> (100 - acc) |> Integer.mod(100)
          :R -> acc
        end

      sum2 = sum2 + div(x + m, 100)

      acc =
        case d do
          :L -> acc - x
          :R -> acc + x
        end
        |> Integer.mod(100)

      sum1 = sum1 + if acc == 0, do: 1, else: 0

      {acc, sum1, sum2}
    end)
    |> Tuple.delete_at(0)
  end

  def run(input_path \\ "input/d01.txt"), do: solve(File.read!(input_path))
end
