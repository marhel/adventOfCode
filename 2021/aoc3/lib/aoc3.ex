defmodule Aoc3 do
  use Bitwise

  def parse(line) do
    String.trim(line)
    |> String.split("", trim: true)
    |> Enum.map(&String.to_integer/1)
  end

  def numbers(name), do: File.stream!(name) |> Enum.map(&parse/1)
  def count(a, b), do: Enum.zip(a, b) |> Enum.map(&Tuple.sum/1)

  # This is too complicated, should use an existing bit-list to value converter
  def scale(nums, cut \\ 0, acc \\ 0, bitval \\ 1)
  def scale([], _, acc, _), do: acc
  def scale([first | rest], cut, acc, bitval) when first > cut,
    do: scale(rest, cut, acc + bitval, bitval <<< 1)
  def scale([_ | rest], cut, acc, bitval), do: scale(rest, cut, acc, bitval <<< 1)

  def calculate(name) do
    nums = numbers(name)
    counts = nums |> Enum.reduce(List.duplicate(0, 64), &count/2)
    len = Enum.count(nums)
    gamma = counts |> Enum.reverse() |> scale(len >>> 1)
    {gamma, Enum.count(counts)}
  end

  def solve(name) do
    {gamma, bits} = calculate(name)
    epsilon = band(bnot(gamma), (1 <<< bits) - 1)
    gamma * epsilon
  end

  def filter_report(_, f \\ &==/2, i \\ 0)
  def filter_report([item], _, _), do: item |> Enum.reverse() |> scale
  def filter_report(list, check, i) do
    countAt = list |> Enum.map(&Enum.at(&1, i)) |> Enum.sum
    pivot = Enum.count(list)

    bit = if countAt * 2 >= pivot do 1 else 0 end

    list
    |> Enum.filter(&check.(Enum.at(&1, i), bit))
    |> filter_report(check, i + 1)
  end

  def solve2(name) do
    nums = numbers(name)
    oxy = filter_report(nums, &==/2)
    co2 = filter_report(nums, &!=/2)
    oxy * co2
  end
end
