defmodule Aoc8Test do
  use ExUnit.Case
  doctest Aoc8

  test "greets the world" do
    assert Aoc8.parse("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
    fdgacbe cefdb cefbgd gcbe") ==
             {[
                "be",
                "cfbegad",
                "cbdgef",
                "fgaecd",
                "cgeb",
                "fdcge",
                "agebfd",
                "fecdb",
                "fabcd",
                "edb"
              ], ["fdgacbe", "cefdb", "cefbgd", "gcbe"]}
  end

  test "can count" do
    assert Aoc8.count1478(["fdgacbe", "cefdb", "cefbgd", "gcbe"]) == 2
  end

  test "can solve simple case, part 1" do
    assert Aoc8.solve1("simple.txt") == 26
  end

  test "can solve real case, part 1" do
    assert Aoc8.solve1("input.txt") == 532
  end

  test "can deduce digits" do

    # 1 har seg cf
    # 4 bcdf
    # 7 acf 
    # 8 abcdefg
    # 6 abdefg är den med 6 segment som inte matchar båda från 1an (två-segmentaren)
    # 3 acdfg är den med 5 segment som matchar båda från 1an (två-segmentaren)
    # 2 acedg är den som saknar det segment som förekommer 9 ggr

    # 5 abdfg är den som har 5 segment, och inte är 2 eller 3
    #   eller saknar c-segmentet och inte är 6, 3, 1 eller 4
    # 0 abcefg är den med 6 segment och inte är 6 eller 9, 
    #   eller som saknar d-segmentet och inte är 1 eller 7,
    #   eller som har e-segmentet och inte är 2,6,8,
    # 9 abcdfg har inte e-segmentet och inte är 0 eller 7

    #  x har y segment
    #  1 => 2
    #  7 => 3
    #  4 => 4
    #  2 => 5 \
    #  3 => 5  \
    #  5 => 5  e/f skiljer 2/3, c/b skiljer 3/5, eller, 3 är den som matchar båda från 1an (två-segmentaren)
    #  0 => 6 \
    #  6 => 6  \
    #  9 => 6   c eller e skiljer, eller, 6 är den som inte matchar båda från 1an (två-segmentaren)
    #  8 => 7

    # a = seven-one
    # c andra 8an
    # occs[4,6,7,7,8,8,9]

    # *seg e förekommer 4 ggr 0268
    # *b 6 045689
    # d 7 ~017
    # g 7 ~147
    # a 8 ~14
    # c 8 ~56
    # *f 9 ~2

    assert Aoc8.deduce(
             Aoc8.parse(
               "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
             )
           ) == 5353
  end

  test "can solve simple case, part 2" do
    assert Aoc8.solve2("simple.txt") == 61229
  end

  test "can solve real case, part 2" do
    assert Aoc8.solve2("input.txt") == 1_011_284
  end
end
