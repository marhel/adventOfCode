defmodule Aoc5 do
  def nums(line, sep) do
    String.trim(line)
    |> String.split(sep, trim: true)
    |> Enum.map(&String.to_integer/1)
    |> List.to_tuple()
  end

  def parse(line) do
      String.trim(line)
      |> String.split("->", trim: true)
      |> Enum.map(&nums(&1, ","))
  end

  def horiz_or_vertic([{x1, y1}, {x2, y2}]), do: x1 == x2 || y1 == y2

  def points([{x1, y1}, {x1, y2}]) do
    ys = Enum.to_list(y1..y2)
    List.duplicate(x1, Enum.count(ys)) |> Enum.zip(ys)
  end

  def points([{x1, y1}, {x2, y1}]) do
    xs = Enum.to_list(x1..x2)
    xs |> Enum.zip(List.duplicate(y1, Enum.count(xs)))
  end

  def points([{x1, y1}, {x2, y2}]) do
    xs = Enum.to_list(x1..x2)
    ys = Enum.to_list(y1..y2)
    xs |> Enum.zip(ys)
  end

  def busy_intersections(points, limit) do
    points
    |> Enum.group_by(&Function.identity/1)
    |> Map.values()
    |> Enum.map(&Enum.count/1)
    |> Enum.count(fn c -> c > limit end)
  end

  def solve_with_filter(name, filter) do
    File.stream!(name)
    |> Enum.map(&parse/1)
    |> Enum.filter(filter)
    |> Enum.flat_map(&points/1)
    |> busy_intersections(1)
  end

  def solve1(name) do
    solve_with_filter(name, &horiz_or_vertic/1)
  end

  def solve2(name) do
    solve_with_filter(name, fn _ -> true end)
  end
end
