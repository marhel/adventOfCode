defmodule Aoc1Test do
  use ExUnit.Case
  use ExUnit.Parameterized

  doctest Aoc1

  test_with_params "fuel calculations",
    fn (mass, expected_fuel) ->
      assert Aoc1.calculateFuel(mass) == expected_fuel
    end do
      [
        {12, 2},
        {14, 2},
        {1969, 654},
        {100756, 33583}
      ]
  end

  test_with_params "recursive fuel calculations",
    fn (mass, expected_fuel) ->
      assert Aoc1.calculateFuel2(mass) == expected_fuel
    end do
      [
        {12, 2},
        {14, 2},
        {1969, 966},
        {100756, 50346}
      ]
    end

    test "part1" do
      Aoc1.part1("input.txt")
    end
    
    test "part2" do
      Aoc1.part2("input.txt")
    end
end
