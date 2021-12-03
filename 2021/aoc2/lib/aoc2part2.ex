defmodule Aoc2Part2 do
  def navigate("forward", amount), do: {:forward, amount}
  def navigate("down", amount), do: amount
  def navigate("up", amount), do: -amount
  def move({:forward, x}, {pos, depth, aim}), do: {pos + x, depth + aim * x, aim}
  def move(dAim, {pos, depth, aim}), do: {pos, depth, aim + dAim}
  def multiply({pos, depth, _}), do: pos * depth

  def parse(line) do
    [direction, amount] = String.split(String.trim(line), " ")
    navigate(direction, String.to_integer(amount))
  end

  def solve(name) do
    File.stream!(name)
    |> Enum.map(&parse/1)
    |> Enum.reduce({0, 0, 0}, &move/2)
    |> multiply
  end
end
