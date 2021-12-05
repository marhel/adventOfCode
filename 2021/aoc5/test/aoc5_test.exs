defmodule Aoc5Test do
  use ExUnit.Case
  doctest Aoc5

  test "can parse line" do
    assert Aoc5.parse("123,234 -> 345,456") == [{123, 234}, {345, 456}]
  end

  test "can detect slanted line" do
    assert Aoc5.horiz_or_vertic([{123, 234}, {345, 456}]) == false
  end

  test "can detect vertical line" do
    assert Aoc5.horiz_or_vertic([{123, 234}, {345, 234}]) == true
  end

  test "can detect horizontal line" do
    assert Aoc5.horiz_or_vertic([{123, 234}, {123, 456}]) == true
  end

  test "can generate points along horizontal line" do
    assert Aoc5.points([{1, 1}, {1, 3}]) == [{1, 1}, {1, 2}, {1, 3}]
  end

  test "can generate points along vertical line" do
    assert Aoc5.points([{9, 7}, {7, 7}]) == [{9, 7}, {8, 7}, {7, 7}]
  end

  test "can find busy intersections without overlap" do
    assert Aoc5.busy_intersections([{9, 7}, {8, 7}], 0) == 2
  end

  test "can find busy intersections with overlap" do
    assert Aoc5.busy_intersections([{9, 7}, {8, 7}, {9, 7}], 1) == 1
  end

  test "can find busy intersections with not enough overlap" do
    assert Aoc5.busy_intersections([{9, 7}, {8, 7}, {9, 7}], 2) == 0
  end

  test "can solve the simple case, part 1" do
    assert Aoc5.solve1("simple.txt") == 5
  end

  test "can solve the real case, part 1" do
    assert Aoc5.solve1("input.txt") == 6572
  end

  test "can generate points along diagonal line, case 1" do
    assert Aoc5.points([{1, 1}, {3, 3}]) == [{1, 1}, {2, 2}, {3, 3}]
  end
  test "can generate points along diagonal line, case 2" do
    assert Aoc5.points([{9, 7}, {7, 9}]) == [{9, 7}, {8, 8}, {7, 9}]
  end

  test "can solve the simple case, part 2" do
    assert Aoc5.solve2("simple.txt") == 12
  end

  test "can solve the real case, part 2" do
    assert Aoc5.solve2("input.txt") == 21466
  end
end
