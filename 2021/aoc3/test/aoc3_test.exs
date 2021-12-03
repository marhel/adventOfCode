defmodule Aoc3Test do
  use ExUnit.Case
  doctest Aoc3

  test "scale" do
    assert Aoc3.scale([1, 0, 1]) == 5
  end

  test "scale with cut 500" do
    assert Aoc3.scale([548, 420, 611], 500) == 5
  end

  test "solves part1, simple case" do
    assert Aoc3.solve("simple.txt") == 198
  end

  test "solves part1, real case" do
    assert Aoc3.solve("input.txt") == 3_277_364
  end

  test "can filter 1-elemen report" do
    assert Aoc3.filter_report([[1]]) == 1
  end

  test "can filter 2-element report with equal number" do
    assert Aoc3.filter_report([[1], [0]]) == 1
  end

  test "can filter 3-element report with majority 1" do
    assert Aoc3.filter_report([[1, 0, 1, 1], [0, 0, 1, 1], [1, 1, 1, 1]]) == 15
  end

  test "can filter 3-digit report with majority 0" do
    assert Aoc3.filter_report([[1, 0], [0, 1], [0, 0]]) == 1
  end

  test "solves part2, simple case" do
    assert Aoc3.solve2("simple.txt") == 230
  end

  test "solves part2, real case" do
    assert Aoc3.solve2("input.txt") == 5_736_383
  end
end
