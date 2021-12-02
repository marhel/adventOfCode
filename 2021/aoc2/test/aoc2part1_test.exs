defmodule Aoc2Part1Test do
  use ExUnit.Case
  doctest Aoc2Part1

  test "can parse a forward command" do
    assert Aoc2Part1.parse("forward 5") == {5, 0}
  end

  test "can parse a up command" do
    assert Aoc2Part1.parse("up 5") == {0, -5}
  end

  test "can parse a down command" do
    assert Aoc2Part1.parse("down 5") == {0, 5}
  end

  test "can move" do
    assert Aoc2Part1.move({5, 2}, {10, 10}) == {15, 12}
  end

  test "can solve the simple case" do
    assert Aoc2Part1.solve("simple.txt") == 15 * 10
  end

  test "can solve the real case" do
    assert Aoc2Part1.solve("input.txt") == 1_660_158
  end
end
