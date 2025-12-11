import gleam/dict.{type Dict}
import gleam/int
import gleam/list
import gleam/string
import gleam/yielder

import simplifile

fn parse(input: String) -> List(#(Int, Int)) {
  let lines = input |> string.trim |> string.split("\n")
  use line <- list.map(lines)
  let assert Ok(t) = string.split_once(line, ",")
  let assert Ok(x) = int.parse(t.0)
  let assert Ok(y) = int.parse(t.1)
  #(x, y)
}

fn area(p1, p2) {
  let #(x1, y1) = p1
  let #(x2, y2) = p2
  { int.absolute_value(x1 - x2) + 1 } * { int.absolute_value(y1 - y2) + 1 }
}

fn discretize(nums) {
  let v_to_i =
    nums
    |> list.sort(int.compare)
    |> list.unique
    |> list.index_map(fn(v, i) { #(v, i) })
    |> dict.from_list
  let i_to_v =
    v_to_i
    |> dict.to_list
    |> list.map(fn(t) { #(t.1, t.0) })
    |> dict.from_list

  #(v_to_i, i_to_v)
}

type Node {
  Uninit
  Empty
  Tile
}

fn flood_fill(graph, s: #(Int, Int)) -> Dict(#(Int, Int), Node) {
  case dict.get(graph, s) {
    Ok(Uninit) -> {
      let graph = dict.insert(graph, s, Empty)

      [#(0, 1), #(0, -1), #(1, 0), #(-1, 0)]
      |> list.fold(graph, fn(graph, d) {
        flood_fill(graph, #(s.0 + d.0, s.1 + d.1))
      })
    }
    _ -> graph
  }
}

fn make_graph(points) {
  let #(x_s, y_s) = list.unzip(points)
  let assert Ok(max_x) = list.max(x_s, int.compare)
  let assert Ok(max_y) = list.max(y_s, int.compare)

  let py = yielder.from_list(points)

  let edges =
    yielder.zip(py, py |> yielder.cycle |> yielder.drop(1))
    |> yielder.to_list
    |> list.flat_map(fn(p) {
      let #(x1, y1) = p.0
      let #(x2, y2) = p.1
      case x1 == x2 {
        True -> {
          use y <- list.map(list.range(y1, y2))
          #(x1, y)
        }
        _ -> {
          use x <- list.map(list.range(x1, x2))
          #(x, y1)
        }
      }
    })
    |> list.map(fn(k) { #(k, Tile) })
    |> dict.from_list

  let graph =
    {
      use x <- list.map(list.range(-1, max_x + 1))
      use y <- list.map(list.range(-1, max_y + 1))
      #(#(x, y), Uninit)
    }
    |> list.flatten
    |> dict.from_list
    |> dict.merge(edges)
    |> flood_fill(#(0, 0))

  graph
  |> dict.map_values(fn(_, v) {
    case v {
      Uninit -> Tile
      v -> v
    }
  })
}

pub fn solve(input) {
  let points = parse(input)
  let assert Ok(res1) =
    points
    |> list.combination_pairs
    |> list.map(fn(ps) { area(ps.0, ps.1) })
    |> list.max(int.compare)

  let #(x_s, y_s) = list.unzip(points)
  let #(x_to_i, i_to_x) = discretize(x_s)
  let #(y_to_i, i_to_y) = discretize(y_s)

  let dpoints =
    points
    |> list.map(fn(p) {
      let assert Ok(x) = dict.get(x_to_i, p.0)
      let assert Ok(y) = dict.get(y_to_i, p.1)
      #(x, y)
    })

  let graph = make_graph(dpoints)

  let assert Ok(res2) =
    dpoints
    |> list.combination_pairs
    |> list.filter_map(fn(p) {
      let #(x1, y1) = p.0
      let #(x2, y2) = p.1
      case
        {
          use x <- list.map(list.range(x1, x2))
          use y <- list.map(list.range(y1, y2))
          #(x, y)
        }
        |> list.flatten
        |> list.all(fn(p) { dict.get(graph, p) == Ok(Tile) })
      {
        False -> Error(Nil)
        True -> {
          let assert [x1, x2] = list.filter_map([x1, x2], dict.get(i_to_x, _))
          let assert [y1, y2] = list.filter_map([y1, y2], dict.get(i_to_y, _))
          Ok(area(#(x1, y1), #(x2, y2)))
        }
      }
    })
    |> list.max(int.compare)

  #(res1, res2)
}

pub fn main() -> Nil {
  let assert Ok(input) = simplifile.read("/dev/stdin")
  echo solve(input)
  Nil
}
