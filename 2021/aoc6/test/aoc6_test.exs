defmodule Aoc6Test do
  use ExUnit.Case
  doctest Aoc6

  test "fish state 1, 0-10" do
    fs = 0..10 |> Enum.map(&Aoc6.fish_state(1, &1))
    assert fs == [1, 0, 6, 5, 4, 3, 2, 1, 0, 6, 5]
  end

  test "fish state 2, 0-10" do
    fs = 0..10 |> Enum.map(&Aoc6.fish_state(2, &1))
    assert fs == [2, 1, 0, 6, 5, 4, 3, 2, 1, 0, 6]
  end

  test "fish state 1, 18" do
    assert Aoc6.fish_state(1, 18) == 4
  end

  test "fish state 2, 18" do
    assert Aoc6.fish_state(2, 18) == 5
  end

  test "fish state 3, 18" do
    assert Aoc6.fish_state(3, 18) == 6
  end

  test "fish state 4, 18" do
    assert Aoc6.fish_state(4, 18) == 0
  end

  test "fish offspring 1, 0-20" do
    fo = 0..20 |> Enum.map(&Aoc6.fish_offspring(1, &1))
    assert fo == [0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3]
  end

  test "fish offspring 2, 0-20" do
    fo = 0..20 |> Enum.map(&Aoc6.fish_offspring(2, &1))
    assert fo == [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3]
  end

  test "fish spawn days 1, 80" do
    # 1 får 12 barn
    assert Aoc6.fish_spawn_days(1, 80) == [2, 9, 16, 23, 30, 37, 44, 51, 58, 65, 72, 79]
  end

  test "new fish spawn days 2, 80" do
    # 
    assert Aoc6.fish_spawn_days(2, 80, :new) == [11, 18, 25, 32, 39, 46, 53, 60, 67, 74]
  end
  test "recursive fish spawn days 0, 60" do
    fs = Aoc6.spawn_fish(0, 60) |> Enum.group_by(fn i -> i end) |> IO.inspect(charlists: :as_lists)
    assert fs == [0]
  end
  test "recursive fish spawn count 0, 128" do
    fs = Aoc6.spawn_fish(0, 128) |> Enum.count
    assert fs == 0
  end

  @tag :skip
  test "recursive fish spawn days 0, 1-80" do
    fs = 1..80 |> Enum.zip(1..80 |> Enum.map(&Aoc6.spawn_fish(0, &1) |> Enum.count())) |> Enum.zip(List.duplicate(0, 80))
    assert fs == [0]
  end
  @tag :skip
  test "recursive fish spawn days 1, 1-80" do
    fs = 1..80 |> Enum.zip(1..80 |> Enum.map(&Aoc6.spawn_fish(1, &1) |> Enum.count())) |> Enum.zip(List.duplicate(1, 80))
    assert fs == [0]
  end
  @tag :skip
  test "recursive fish spawn days 2, 1-80" do
    fs = 1..80 |> Enum.zip(1..80 |> Enum.map(&Aoc6.spawn_fish(2, &1) |> Enum.count())) |> Enum.zip(List.duplicate(2, 80))
    assert fs == [0]
  end
  @tag :skip
  test "recursive fish spawn days 3, 1-80" do
    fs = 1..80 |> Enum.zip(1..80 |> Enum.map(&Aoc6.spawn_fish(3, &1) |> Enum.count())) |> Enum.zip(List.duplicate(3, 80))
    assert fs == [0]
  end
  @tag :only
  test "recursive fish spawn days 1, 256" do
    assert Enum.count(Aoc6.spawn_fish(1, 256)) == 550
  end

  test "recursive fish spawn days 1, 18" do
    # 
    assert Aoc6.spawn_fish(1, 18) == [1, 2, 9, 16, 11, 18, 18]
    #      1   2   3   4   5   6   7   8   9  10x
    # ---------------------------------------
    #  1:2, 9, 16, 23, 30, 37, 44, 51, 58, 65, 72, 79
    #  2: 11, 18, 25, 32, 39, 46, 53, 60, 67, 74   # 10 barn
    #  2.11: 20, 27, 34, 41, 48, 55, 62, 69, 76    #  9 barn
    #  2.11.20: 29, 36, 43, 50, 57, 64, 71, 78     #  8 barn
    #  2.11.20.29: 38, 45, 52, 59, 66, 73, 80      #  7 barn
    #  2.11.20.29.38: 47, 54, 61, 68, 75
    #  2.11.20.29.38.47: 56, 63, 70, 77
    #  2.11.20.29.38.47.56: 65, 72, 79
    #  2.11.20.29.38.47.56.65: 74, (81)
    #  2.11.20.29.38.47.56.65.74: (83)
    #  9: 18, 25, 32, 39, 46, 53, 60, 67, 74
    #  9.18: 27, 34, 41, 48, 55, 62, 69, 76
    #  9.18.27: 36, 43, 50, 57, 64, 71, 78
    #  9.18.27.36: 45, 52, 59, 66, 73, 80
    #  9.18.27.36.45: 54, 61, 68, 75, (82)
    #  9.18.27.36.45.54: 63, 70, 77 
    #  9.18.27.36.45.54.63: 72, 79
    #  9.18.27.36.45.54.63.72: (81)
    # 16: 25, 32, 39, 46, 53, 60, 67, 74
    # 16.25: 34, 41, 48, 55, 62, 69, 76
    # 16.25.34: 43, 50, 57, 64, 71, 78
    # 16.25.34.43: 52, 59, 66, 73, 80
    # 16.25.34.43.52: 61, 68, 75, (82)
    # 16.25.34.43.52.61: 70, 77
    # 16.25.34.43.52.61.70: 79
    # 23: 32, 39, 46, 53, 60, 67, 74
    # 30: 39, 46, 53, 60, 67, 74
    # 37: 46, 53, 60, 67, 74
    # 44: 53, 60, 67, 74
    # 51: 60, 67, 74
    # 58: 67, 74
    # 65: 74
    # ---- cut off at 80
    # 72: 81
  end

  test "fish spawn days 2, 20" do
    assert Aoc6.fish_spawn_days(2, 20) == [3, 10, 17]
  end

  test "can simulate the simple case after 18 days, part 1" do
    assert Aoc6.simulate([3, 4, 3, 1, 2], 18) == 26
  end

  test "can simulate the simple case after 80 days, part 1" do
    assert Aoc6.simulate([3, 4, 3, 1, 2], 80) == 5934
  end

  test "can solve the simple case after 80 days, part 1" do
    assert Aoc6.solve1("simple.txt") == 5934
  end

  test "can solve the real case after 80 days, part 1" do
    assert Aoc6.solve1("input.txt") == 352872
  end
end
