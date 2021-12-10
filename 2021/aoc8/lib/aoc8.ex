defmodule Aoc8 do
  def segs(line, sep) do
    String.trim(line)
    |> String.split(sep, trim: true)
  end

  def parse(line) do
    String.trim(line)
    |> String.split("|", trim: true)
    |> Enum.map(&segs(&1, " "))
    |> List.to_tuple()
  end

  def count1478(segs) do
    segs
    |> Enum.map(&String.length/1)
    |> Enum.filter(fn i -> i == 2 || i == 3 || i == 4 || i == 7 end)
    |> Enum.count()
  end

  def solve1(name) do
    File.stream!(name)
    |> Enum.map(&parse/1)
    |> Enum.map(&elem(&1, 1))
    |> Enum.map(&count1478/1)
    |> Enum.sum()
  end

  def to_set([elem]), do: to_set(elem)

  def to_set(elem) do
    elem |> String.graphemes() |> MapSet.new()
  end

  def except(segs, sets) do
    segs |> Enum.filter(&(!Enum.any?(sets, fn i -> i == to_set(&1) end)))
  end

  def not_(f), do: &(!f.(&1))
  def all_segs_of(set), do: fn s -> Enum.all?(set, &String.contains?(s, &1)) end
  def with_seg_that([char]), do: &String.contains?(&1, char)
  def matching(list, where), do: list |> Enum.filter(where)

  def deduce({digits, output}) do
    occurs =
      digits
      |> Enum.flat_map(&String.graphemes/1)
      |> Enum.group_by(&Function.identity/1)
      |> Enum.group_by(fn {k, v} -> Enum.count(v) end, fn {k, v} -> k end)

    with_num_segs = digits |> Enum.group_by(&String.length/1)

    _1 = with_num_segs[2] |> to_set
    _7 = with_num_segs[3] |> to_set
    _4 = with_num_segs[4] |> to_set
    _8 = with_num_segs[7] |> to_set
    _6 = with_num_segs[6] |> matching(not_(all_segs_of(_1))) |> to_set
    _3 = with_num_segs[5] |> matching(all_segs_of(_1)) |> to_set
    _2 = digits |> matching(not_(with_seg_that(occurs[9]))) |> to_set
    _5 = with_num_segs[5] |> except([_2, _3]) |> to_set
    _0 = digits |> matching(with_seg_that(occurs[4])) |> except([_2, _6, _8]) |> to_set
    _9 = with_num_segs[6] |> except([_0, _6]) |> to_set

    digits =
      [_0, _1, _2, _3, _4, _5, _6, _7, _8, _9]
      |> Enum.zip(0..9)
      |> Map.new()

    output |> Enum.map(&to_set/1) |> Enum.map(&digits[&1]) |> Integer.undigits()
  end

  def solve2(name) do
    File.stream!(name)
    |> Enum.map(&parse/1)
    |> Enum.map(&deduce/1)
    |> Enum.sum()
  end
end
