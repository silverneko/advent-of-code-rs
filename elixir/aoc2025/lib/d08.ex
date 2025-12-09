defmodule Aoc2025.D08 do
  @doc ~S"""
  iex> Aoc2025.D08.parse("1,2,3\n4,5,6")
  [{1, 2, 3}, {4, 5, 6}]
  """
  def parse(input) do
    input
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(fn line ->
      line
      |> String.split(",")
      |> Enum.map(&String.to_integer/1)
      |> List.to_tuple()
    end)
  end

  defmodule DisjointSet do
    def new(elems) do
      for e <- elems, do: {e, {:leader, 1}}, into: %{}
    end

    def lookup(djs, e) do
      case Map.get(djs, e) do
        {:leader, size} ->
          {djs, e, size}

        {:parent, parent} ->
          {djs, leader, size} = lookup(djs, parent)
          {%{djs | e => {:parent, leader}}, leader, size}
      end
    end

    def union(djs, e1, e2) do
      {djs, leader1, size1} = lookup(djs, e1)

      case lookup(djs, e2) do
        {djs, ^leader1, ^size1} ->
          djs

        {djs, leader2, size2} ->
          %{djs | leader1 => {:parent, leader2}, leader2 => {:leader, size1 + size2}}
      end
    end
  end

  def distance(v1, v2) do
    for {a, b} <- Enum.zip(Tuple.to_list(v1), Tuple.to_list(v2)) do
      (a - b) * (a - b)
    end
    |> Enum.sum()
  end

  def combinations([]), do: []
  def combinations([x | xs]), do: Stream.concat(for(y <- xs, do: {x, y}), combinations(xs))

  def solve(input, n) do
    elems = input |> parse

    djs = DisjointSet.new(elems)

    merges =
      combinations(elems)
      |> Enum.sort_by(&distance(elem(&1, 0), elem(&1, 1)))
      |> Stream.scan({djs, nil, nil}, fn {a, b}, {djs, _, _} ->
        {DisjointSet.union(djs, a, b), a, b}
      end)

    res1 =
      merges
      |> Enum.at(n - 1)
      |> elem(0)
      |> (&for({_, {:leader, s}} <- &1, do: s)).()
      |> Enum.sort(:desc)
      |> Enum.take(3)
      |> Enum.product()

    total_number_of_elems = length(elems)

    {_, a, b} =
      merges
      |> Enum.find(fn {djs, a, _} ->
        case DisjointSet.lookup(djs, a) do
          {_, _, ^total_number_of_elems} -> true
          _ -> false
        end
      end)

    res2 = elem(a, 0) * elem(b, 0)
    {res1, res2}
  end

  def run(input_path \\ "input/d08.txt") do
    input = File.read!(input_path)
    solve(input, 1000) |> IO.inspect()
  end
end
