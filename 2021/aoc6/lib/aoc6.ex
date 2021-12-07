defmodule Aoc6 do
  def fish_state(start, days) do
    rem(rem(start - days, 7) + 7, 7)
  end

  def fish_offspring(start, days) do
    div(days - start + 6, 7)
  end

  def fish_spawn_days(fish, days) do
    Enum.to_list((fish + 1)..days//7)
  end

  def fish_spawn_days(fish, days, :new) do
    Enum.to_list((fish + 9)..days//7)
  end

  def spawn_fish(fish, days) do
    offspring = fish_spawn_days(fish, days)
    spawn_more_fish(offspring, days, [fish])
  end

  def spawn_more_fish([], _days, acc), do: acc

  def spawn_more_fish(offspring, days, acc) do
    offspring
    # |> IO.inspect
    |> Enum.flat_map(&fish_spawn_days(&1, days, :new))
    |> spawn_more_fish(days, Enum.concat(acc, offspring))
  end

  def simulate(fishes, days) do
    fishes |> Enum.flat_map(&spawn_fish(&1, days)) |> Enum.count
  end

  def parse(line) do
    String.trim(line)
    |> String.split(",", trim: true)
    |> Enum.map(&String.to_integer/1)
  end

  def solve1(name) do
    File.stream!(name)
    |> Enum.map(&parse/1)
    |> Enum.at(0)
    |> IO.inspect
    |> simulate(80)
    |> IO.inspect
  end
end
