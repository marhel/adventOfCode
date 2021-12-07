defmodule Aoc7Test do
  use ExUnit.Case
  doctest Aoc7

  @nums [16, 1, 2, 0, 4, 2, 7, 1, 2, 14]

  test "can parse" do
    assert Aoc7.parse("1,2,3,4,5") == [1, 2, 3, 4, 5]
  end

  test "fit 2" do
    assert Aoc7.fit(@nums, 2) == 37
  end

  test "fit 5" do
    assert Aoc7.fit(@nums, 5) == 45
  end

  test "fit 10" do
    assert Aoc7.fit(@nums, 10) == 71
  end

  test "best fit" do
    assert Aoc7.best_fit(@nums) == 37
  end

  test "solve simple case, part 1" do
    assert Aoc7.solve1("simple.txt") == 37
  end

  test "solve real case, part 1" do
    assert Aoc7.solve1("input.txt") == 340_056
  end

  test "fit2 2" do
    assert Aoc7.fit2(@nums, 2) == 206
  end

  test "fit2 5" do
    assert Aoc7.fit2(@nums, 5) == 168
  end

  test "solve simple case, part 2" do
    assert Aoc7.solve2("simple.txt") == 168
  end

  test "solve real case, part 2" do
    assert Aoc7.solve2("input.txt") == 340_056
  end
end
