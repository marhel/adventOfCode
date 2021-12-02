defmodule Aoc2Part2Test do
  use ExUnit.Case
  doctest Aoc2Part2

  test "can parse a forward command" do
    assert Aoc2Part2.parse("forward 5") == {:forward, 5}
  end

  test "can parse a up command" do
    assert Aoc2Part2.parse("up 5") == -5
  end

  test "can parse a down command" do
    assert Aoc2Part2.parse("down 5") == 5
  end

  test "can move aim" do
    assert Aoc2Part2.move(5, {0, 0, 5}) == {0, 0, 10}
  end

  test "can move forward" do
    assert Aoc2Part2.move({:forward, 5}, {10, 10, 3}) == {15, 10 + 3 * 5, 3}
  end

  test "can solve the simple case" do
    assert Aoc2Part2.solve("simple.txt") == 900
  end

  test "can solve the real case" do
    assert Aoc2Part2.solve("input.txt") == 1_604_592_846
  end
end
