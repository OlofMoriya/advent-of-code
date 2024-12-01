defmodule Ex do
  {left, right} =
    File.read!("../01")
    |> String.split("\n")
    |> Enum.reduce({[], []}, fn row, {left, right} ->
      case String.split(row) do
        [first, second] ->
          new_left =
            case Integer.parse(first) do
              {f, _} -> left ++ [f]
              :error -> left
            end

          new_right =
            case Integer.parse(second) do
              {f, _} -> right ++ [f]
              :error -> right
            end

          {new_left, new_right}

        _ ->
          {left, right}
      end
    end)

  IO.inspect(left, label: "Left")
  IO.inspect(right, label: "right")

  filtered =
    Enum.map(left, fn l ->
      Enum.filter(right, &(&1 == l))
      |> Enum.count()
    end)

  sum =
    Enum.zip(left, filtered)
    |> Enum.reduce(0, fn {l, f}, acc ->
      acc + l * f
    end)

  IO.inspect(filtered, label: "filtered")
  IO.inspect(sum, label: "sum")
end
