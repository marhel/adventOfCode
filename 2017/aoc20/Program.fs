// Learn more about F# at http://fsharp.org

open System
open System.IO
open System.Text.RegularExpressions

[<EntryPoint>]
let main argv =
    let get c line = 
        let m = Regex.Match(line, c + "=<(-?\\d+?),(-?\\d+?),(-?\\d+?)>")
        (int(m.Groups.[1].ToString()), int(m.Groups.[2].ToString()), int(m.Groups.[3].ToString()))
    let dist (x, y, z) = abs(x) + abs(y) + abs(z)
    let grid = argv 
               |> List.ofSeq
               |> List.head
               |> File.ReadAllLines
               |> List.ofSeq
               |> Seq.map (get "a" >> dist)
               |> Seq.mapi (fun i d -> (i, d))
               |> Seq.minBy snd
               |> (fun (i, d) -> printfn "%d %d" i d)
             //|> Seq.iter (fun t -> printfn "%d %d %d = %d" <||| t <| (dist <||| t))
    0 // return an integer exit code
