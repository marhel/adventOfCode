defmodule Aoc2Part1 do
  def navigate("forward", amount), do: {amount, 0}
  def navigate("down", amount), do: {0, amount}
  def navigate("up", amount), do: {0, -amount}
  def move({dPos, dDepth}, {pos, depth}), do: {pos + dPos, depth + dDepth}
  def multiply({pos, depth}), do: pos * depth

  def parse(line) do
    [direction, amount] = String.split(String.trim(line), " ")
    navigate(direction, Integer.parse(amount) |> elem(0))
  end

  def solve(name) do
    File.stream!(name)
    |> Enum.map(&parse/1)
    |> Enum.reduce({0, 0}, &move/2)
    |> multiply
  end
end
