defmodule Aoc4 do
  def nums(line, sep) do
    String.trim(line) |> String.split(sep, trim: true) |> Enum.map(&String.to_integer/1)
  end

  def parse("\n", state), do: state

  def parse(line, %{:state => :numbers} = state) do
    Map.put(state, :state, :cards)
    |> Map.put(:card, [])
    |> Map.put(:cards, [])
    |> Map.put(:numbers, nums(line, ","))
  end

  def parse(line, %{:state => :cards, :cards => cards, :card => card} = state) do
    new_card = [nums(line, " ") | card]

    if Enum.count(new_card) == 5 do
      Map.put(state, :cards, [Enum.reverse(new_card) | cards])
      |> Map.put(:card, [])
    else
      Map.put(state, :card, new_card)
    end
  end

  def parse_input(name), do: File.stream!(name) |> Enum.reduce(%{:state => :numbers}, &parse/2)
  def transpose(rows), do: Enum.zip(rows) |> Enum.map(&Tuple.to_list/1)
  def rows_and_columns(card_rows), do: Enum.concat(card_rows, transpose(card_rows))

  def add_columns(%{:cards => cards} = state),
    do: Map.put(state, :cards, Enum.map(cards, &rows_and_columns/1))

  def cross_out(rows_and_columns, n),
    do: rows_and_columns |> Enum.map(&Enum.filter(&1, fn i -> n != i end))

  def count_remaining(rows_and_columns), do: rows_and_columns |> Enum.map(&Enum.count/1)
  def best_remaining(rows_and_columns), do: rows_and_columns |> count_remaining |> Enum.min()

  def bingo(state, [], _), do: state

  def bingo(%{:cards => [winner | _], :called => called, :remains => [0 | _]}, _, _) do
    sum = winner |> List.flatten() |> Enum.uniq() |> Enum.sum()
    sum * called
  end

  def bingo(%{:cards => cards} = state, [called | numbers], strategy) do
    crossed_in_order =
      cards |> Enum.map(&cross_out(&1, called)) |> Enum.sort_by(&best_remaining/1, strategy)

    Map.put(state, :cards, crossed_in_order)
    |> Map.put(:remains, crossed_in_order |> Enum.map(&best_remaining/1))
    |> Map.put(:called, called)
    # visualize with |> IO.inspect(charlists: :as_lists)
    |> bingo(numbers, strategy)
  end

  def solve_with_strategy(name, strategy) do
    %{:cards => cards, :numbers => numbers} = parse_input(name)

    %{:cards => cards}
    |> add_columns
    |> bingo(numbers, strategy)
  end

  def solve1(name) do
    solve_with_strategy(name, :asc)
  end

  def solve2(name) do
    solve_with_strategy(name, :desc)
  end
end
