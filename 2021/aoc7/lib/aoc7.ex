defmodule Aoc7 do
  def parse(line) do
    String.trim(line)
    |> String.split(",", trim: true)
    |> Enum.map(&String.to_integer/1)
  end

  def fit(nums, pivot) do
    nums |> Enum.map(fn i -> abs(i - pivot) end) |> Enum.sum()
  end

  def best_fit(nums) do
    median = nums |> Statistics.median()
    fit(nums, floor(median))
  end

  def solve1(name) do
    File.read!(name) |> parse |> best_fit
  end

  def fit2(nums, pivot) do
    nums |> Enum.map(fn i -> Enum.sum(0..abs(i - pivot)) end) |> Enum.sum()
  end

  def solve2(name) do
    nums = File.read!(name) |> parse
    {min, max} = nums |> Enum.min_max()
    # yes, this is brute force, so what? Live with it.
    best = min..max |> Enum.min_by(fn i -> fit2(nums, i) end)
    nums |> fit2(best)
  end
end
