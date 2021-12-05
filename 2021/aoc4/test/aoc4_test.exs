defmodule Aoc4Test do
  use ExUnit.Case
  doctest Aoc4

  test "can parse the numbers sequence" do
    assert Aoc4.parse("1,2,3,4", %{:state => :numbers})[:numbers] == [1, 2, 3, 4]
  end

  test "can parse a partial bingo card, line 1" do
    assert Aoc4.parse("10 20 30 40 50", %{:state => :cards, cards: [], card: []})[:card] == [
             [10, 20, 30, 40, 50]
           ]
  end

  test "can parse the last line of the bingo card" do
    state =
      Aoc4.parse("11 21 31 41 51", %{:state => :cards, cards: [], card: [[4], [3], [2], [1]]})

    assert state[:cards] == [[[1], [2], [3], [4], [11, 21, 31, 41, 51]]]
    assert state[:card] == []
  end

  test "can transpose" do
    assert Aoc4.transpose([
             [14, 21, 17, 24, 4],
             [10, 16, 15, 9, 19],
             [18, 8, 23, 26, 20],
             [22, 11, 13, 6, 5],
             [2, 0, 12, 3, 7]
           ]) == [
             [14, 10, 18, 22, 2],
             [21, 16, 8, 11, 0],
             [17, 15, 23, 13, 12],
             [24, 9, 26, 6, 3],
             [4, 19, 20, 5, 7]
           ]
  end

  test "can add columns" do
    assert Aoc4.add_columns(%{
             cards: [
               [
                 [14, 21, 17, 24, 4],
                 [10, 16, 15, 9, 19],
                 [18, 8, 23, 26, 20],
                 [22, 11, 13, 6, 5],
                 [2, 0, 12, 3, 7]
               ]
             ]
           })[:cards] == [
             [
               [14, 21, 17, 24, 4],
               [10, 16, 15, 9, 19],
               [18, 8, 23, 26, 20],
               [22, 11, 13, 6, 5],
               [2, 0, 12, 3, 7],
               [14, 10, 18, 22, 2],
               [21, 16, 8, 11, 0],
               [17, 15, 23, 13, 12],
               [24, 9, 26, 6, 3],
               [4, 19, 20, 5, 7]
             ]
           ]
  end

  test "can cross out numbers" do
    assert Aoc4.cross_out(
             [
               [14, 21, 17, 24, 4],
               [10, 16, 15, 9, 19],
               [18, 8, 23, 26, 20],
               [22, 11, 13, 6, 5],
               [2, 0, 12, 3, 7],
               [14, 10, 18, 22, 2],
               [21, 16, 8, 11, 0],
               [17, 15, 23, 13, 12],
               [24, 9, 26, 6, 3],
               [4, 19, 20, 5, 7]
             ],
             7
           ) == [
             [14, 21, 17, 24, 4],
             [10, 16, 15, 9, 19],
             [18, 8, 23, 26, 20],
             [22, 11, 13, 6, 5],
             [2, 0, 12, 3],
             [14, 10, 18, 22, 2],
             [21, 16, 8, 11, 0],
             [17, 15, 23, 13, 12],
             [24, 9, 26, 6, 3],
             [4, 19, 20, 5]
           ]
  end

  test "can count remaining numbers" do
    assert Aoc4.count_remaining([
             [14, 21, 17, 24, 4],
             [10, 16],
             [18, 8, 23]
           ]) == [5, 2, 3]
  end

  test "can find best numbers" do
    assert Aoc4.best_remaining([
             [14, 21, 17, 24, 4],
             [10, 16],
             [18, 8, 23]
           ]) == 2
  end

  test "can parse the simple case fully" do
    state = Aoc4.parse_input("simple.txt")

    assert state[:numbers] == [
             7,
             4,
             9,
             5,
             11,
             17,
             23,
             2,
             0,
             14,
             21,
             24,
             10,
             16,
             13,
             6,
             15,
             25,
             12,
             22,
             18,
             20,
             8,
             19,
             3,
             26,
             1
           ]

    assert state[:card] == []

    assert state[:cards] == [
             [
               [14, 21, 17, 24, 4],
               [10, 16, 15, 9, 19],
               [18, 8, 23, 26, 20],
               [22, 11, 13, 6, 5],
               [2, 0, 12, 3, 7]
             ],
             [
               [3, 15, 0, 2, 22],
               [9, 18, 13, 17, 5],
               [19, 8, 7, 25, 23],
               [20, 11, 10, 24, 4],
               [14, 21, 16, 12, 6]
             ],
             [
               [22, 13, 17, 11, 0],
               [8, 2, 23, 4, 24],
               [21, 9, 14, 16, 7],
               [6, 10, 3, 18, 5],
               [1, 12, 20, 15, 19]
             ]
           ]
  end

  test "can solve part1, the simple case" do
    assert Aoc4.solve1("simple.txt") == 4512
  end

  test "can solve part1, the real case" do
    assert Aoc4.solve1("input.txt") == 29440
    # 31542 was too high because it was just looking for winners across rows
  end

  test "can solve part2, the simple case" do
    assert Aoc4.solve2("simple.txt") == 1924
  end

  test "can solve part2, the real case" do
    assert Aoc4.solve2("input.txt") == 13884
  end
end
