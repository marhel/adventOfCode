defmodule Aoc do
    def parse(line) do
        [slot, depth] = String.split(String.trim(line), ": ")
        [elem(Integer.parse(slot), 0), elem(Integer.parse(depth), 0)]
    end

    def severity(_, slot, depth) when depth < 2, do: {:caught, slot * depth}
    def severity(time, slot, depth) do
        if rem(time, (depth * 2 - 2)) == 0 do
            {:caught, slot * depth}
        else
            {:missed, 0}
        end
    end

    def startSentinel([slot, depth], state) do
        sentinel = spawn fn -> sentinel [slot, depth] end
        Map.put state, slot, sentinel
    end

    def travel(state) do
        receive do
            {:missed, slot, _, delay} -> 
                state = Map.update state, :replies, 1, &(&1 + 1)
                %{total: new_total, replies: replies, sentinels: sentinels} = state
                IO.puts ~s{Sentinel #{slot} missed @#{delay} (#{replies}/#{sentinels})\n}
                if replies < sentinels do
                    travel(state)
                else
                    what = if new_total > 0 do "got caught" else "made it!" end
                    IO.puts ~s{Travelled past #{sentinels} sentinels, #{what}\n}
                end
            {:caught, slot, score, delay} ->
                state = Map.update state, :replies, 1, &(&1 + 1)
                state = Map.update state, :total, score, &(&1 + score)
                %{total: new_total, replies: replies, sentinels: sentinels} = state
                IO.puts ~s{Sentinel #{slot} caught @#{delay} severity #{score}, total: #{new_total} (#{replies}/#{sentinels})\n}
                if replies < sentinels do
                    travel(state)
                else
                    what = if new_total > 0 do "got caught" else "made it!" end
                    IO.puts ~s{Travelled past #{sentinels} sentinels, #{what}\n}
                end
        end
    end

    def sentinel([slot, depth]) do
        receive do
            {sender, time} ->
                IO.puts ~s{Sentinel #{slot} reacts @#{time}\n}
                {res, score} = severity(time, slot, depth)
                send sender, {res, slot, score, time - slot}
        end
    end

    def load(name), do:
        File.stream!(name)
        |> Enum.map(&parse/1)
        |> Enum.reduce(%{}, &startSentinel/2)

    def setup(name, delay) do
        state = load(name)
        traveller = spawn fn -> travel(%{sentinels: Map.size(state)}) end
        Map.keys(state)
        |> Enum.map(fn slot -> send state[slot], {traveller, delay + slot} end)
    end

end
