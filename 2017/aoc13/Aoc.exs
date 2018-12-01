defmodule Aoc do
    def severity(_, slot, depth) when depth < 2, do: {:caught, slot * depth}
    def severity(time, slot, depth) do
        if rem(time, (depth * 2 - 2)) == 0 do
            {:caught, slot * depth}
        else
            {:cleared, 0}
        end
    end

    def report_back(sentinels, total, delay, hivemind, status) do
        what = if status == :caught do
            "got caught"
        else
            "made it!"
        end
        # IO.puts ~s{Travelled past #{sentinels} sentinels, #{what} (severity #{total})\n}
        # IO.inspect hivemind, label: "report_back to hivemind"
        send hivemind, {status, delay}
    end

    def traveller(state) do
        %{hivemind: hivemind, sentinels: sentinels, status: status} = state
        receive do
            {:cleared, slot, _, delay} ->
                state = Map.update state, :replies, 1, &(&1 + 1)
                %{total: new_total, replies: replies} = state
                # IO.puts ~s{Sentinel #{slot} cleared @#{delay} (#{replies}/#{sentinels})\n}
                if replies < sentinels do
                    traveller(state)
                else
                    report_back(sentinels, new_total, delay, hivemind, status)
                end
            {:caught, slot, score, delay} ->
                state = Map.update state, :replies, 1, &(&1 + 1)
                state = Map.update state, :total, score, &(&1 + score)
                state = Map.put state, :status, :caught
                %{total: new_total, replies: replies} = state
                # IO.puts ~s{Sentinel #{slot} caught @#{delay} severity #{score}, total: #{new_total} (#{replies}/#{sentinels})\n}
                report_back(sentinels, new_total, delay, hivemind, :caught)
        end
    end

    def spawn_traveller(hivemind, delay, state) do
        traveller = spawn fn -> traveller(%{hivemind: hivemind, delay: delay, total: 0, status: :cleared, sentinels: Map.size(state)}) end
        Map.keys(state)
        |> Enum.map(fn slot -> send state[slot], {traveller, delay + slot} end)
    end

    def hivemind(delay, state) do
        spawn_traveller(self(), delay, state)
        # IO.puts ~s{hivemind waiting...}
        # IO.inspect self(), label: "hivemind self"

        receive do
            {:cleared, delay} ->
                IO.puts ~s{Travelled past all sentinels with delay #{delay}\n}
            {:caught, delay} ->
                if rem(delay, 1000) == 0 do
                    IO.puts ~s{Travelling with delay #{delay} wasn't undetected\n}
                end
                hivemind(delay + 1, state)
        end
    end

    def sentinel([slot, depth]) do
        receive do
            {sender, time} ->
                # IO.puts ~s{Sentinel #{slot} reacts @#{time}\n}
                {res, score} = severity(time, slot, depth)
                send sender, {res, slot, score, time - slot}
                sentinel([slot, depth])
        end
    end

    def parse(line) do
        [slot, depth] = String.split(String.trim(line), ": ")
        [elem(Integer.parse(slot), 0), elem(Integer.parse(depth), 0)]
    end

    def start_sentinels(name), do:
        File.stream!(name)
        |> Enum.map(&parse/1)
        |> Enum.reduce(%{}, &start_sentinel/2)

    def start_sentinel([slot, depth], state) do
        s = spawn fn -> sentinel [slot, depth] end
        Map.put state, slot, s
    end

    def setup(name) do
        state = start_sentinels(name)
        spawn fn -> hivemind(0, state) end
    end
end
