defmodule Aoc1 do
  @moduledoc """
  Documentation for Aoc1.
  """

  @doc """
  Hello world.

  ## Examples

      iex> Aoc1.calculateFuel(14)
      2

  """
  def calculateFuel(mass) do
    mass / 3
    |> floor
    |> Kernel.-(2)
  end

  def calculateFuel2(mass) do
    case calculateFuel(mass) do
      v when v < 0 -> 0
      v -> v + calculateFuel2(v)
    end
  end

  def part1(path) do
    solve(1, path, &calculateFuel/1)
  end

  def part2(path) do
    solve(2, path, &calculateFuel2/1)
  end

  def solve(part, path, solver) do
    res = path
    |> File.read!
    |> String.split
    |> Enum.map(&String.to_integer/1)
    |> Enum.map(solver)
    |> Enum.sum
    IO.puts "part#{part} (#{path}): #{res}\n" 
  end
end
