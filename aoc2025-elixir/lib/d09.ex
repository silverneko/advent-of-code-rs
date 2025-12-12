defmodule Aoc2025.D09 do
  @doc ~S"""
  iex> Aoc2025.D09.parse("\n7,1\n11,1\n")
  [{7, 1}, {11, 1}]
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

  def combinations([]), do: []
  def combinations([x | xs]), do: Stream.concat(for(y <- xs, do: {x, y}), combinations(xs))

  def area({x1, y1}, {x2, y2}), do: (abs(x1 - x2) + 1) * (abs(y1 - y2) + 1)
  def area({p1, p2}), do: area(p1, p2)

  @doc ~S"""
  iex> Aoc2025.D09.discretize([5, 10, 10, 100])
  {
    %{5 => 0, 10 => 1, 100 => 2},
    %{0 => 5, 1 => 10, 2 => 100}
  }
  """
  def discretize(enumerable) do
    v_to_i =
      enumerable
      |> Enum.sort()
      |> Stream.dedup()
      |> Stream.zip(Stream.from_index())
      |> Map.new()

    i_to_v = Map.new(v_to_i, fn {k, v} -> {v, k} end)
    {v_to_i, i_to_v}
  end

  def flood_fill(graph, {x, y} = s) do
    case graph do
      %{^s => :uninit} ->
        graph = %{graph | s => :empty}

        for {dx, dy} <- [{0, 1}, {0, -1}, {1, 0}, {-1, 0}], reduce: graph do
          graph -> flood_fill(graph, {x + dx, y + dy})
        end

      _ ->
        graph
    end
  end

  def make_graph(points) do
    {x_s, y_s} = Enum.unzip(points)
    max_x = Enum.max(x_s)
    max_y = Enum.max(y_s)

    edges =
      Stream.zip(points, Stream.cycle(points) |> Stream.drop(1))
      |> Stream.flat_map(fn {{x1, y1}, {x2, y2}} ->
        cond do
          x1 == x2 ->
            [y1, y2] = Enum.sort([y1, y2])
            for y <- y1..y2, do: {x1, y}

          y1 == y2 ->
            [x1, x2] = Enum.sort([x1, x2])
            for x <- x1..x2, do: {x, y1}
        end
      end)
      |> Map.new(fn k -> {k, :tile} end)

    graph =
      for(x <- -1..(max_x + 1), y <- -1..(max_y + 1), into: %{}, do: {{x, y}, :uninit})
      |> Map.merge(edges)
      |> flood_fill({0, 0})

    for {k, v} <- graph, into: %{} do
      case v do
        :uninit -> {k, :tile}
        _ -> {k, v}
      end
    end
  end

  def solve(input) do
    points = input |> parse
    res1 = combinations(points) |> Stream.map(&area/1) |> Enum.max()

    {x_s, y_s} = Enum.unzip(points)
    {x_to_i, i_to_x} = discretize(x_s)
    {y_to_i, i_to_y} = discretize(y_s)

    dpoints = Enum.map(points, fn {x, y} -> {x_to_i[x], y_to_i[y]} end)

    graph = make_graph(dpoints)

    res2 =
      combinations(dpoints)
      |> Stream.filter(fn {{x1, y1}, {x2, y2}} ->
        [x1, x2] = Enum.sort([x1, x2])
        [y1, y2] = Enum.sort([y1, y2])
        for(x <- x1..x2, y <- y1..y2, do: {x, y}) |> Enum.all?(&(graph[&1] === :tile))
      end)
      |> Stream.map(fn {{x1, y1}, {x2, y2}} ->
        [x1, x2] = Enum.map([x1, x2], &i_to_x[&1])
        [y1, y2] = Enum.map([y1, y2], &i_to_y[&1])
        area({x1, y1}, {x2, y2})
      end)
      |> Enum.max()

    {res1, res2}
  end

  def run(input_path \\ "input/d09.txt") do
    input = File.read!(input_path)
    solve(input) |> IO.inspect()
  end
end
