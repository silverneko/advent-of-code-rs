defmodule Aoc2025.D07 do
  def parse(input) do
    input |> String.trim() |> String.split("\n") |> Enum.map(&String.to_charlist(&1))
  end

  def solve(input) do
    manifold = input |> parse()

    iv = {List.duplicate(0, length(hd(manifold))), 0}

    res =
      Enum.scan(manifold, iv, fn e, {acc, sum1} ->
        {dv, sv} =
          for {b, w} <- Enum.zip(e, acc) do
            case b do
              ?S -> {1, 0}
              ?. -> {w, 0}
              ?^ -> {0, w}
              _ -> {0, 0}
            end
          end
          |> Enum.unzip()

        acc =
          Enum.zip([dv, tl(sv) ++ [0], [0 | sv]])
          |> Enum.map(&Tuple.sum/1)

        sum1 = sum1 + Enum.count(sv, &(&1 != 0))
        {acc, sum1}
      end)

    graph =
      for {line, {acc, _}} <- Enum.zip(manifold, res) do
        for {c, w} <- Enum.zip(line, acc) do
          case {c, w} do
            {?S, _} -> ?S
            {c, 0} -> c
            _ -> ?|
          end
        end
      end
      |> Enum.join("\n")

    {acc, sum1} = List.last(res)
    sum2 = Enum.sum(acc)
    {graph, sum1, sum2}
  end

  def run(input_path \\ "input/d07.txt") do
    input = File.read!(input_path)
    solve(input) |> Tuple.delete_at(0)
  end
end
