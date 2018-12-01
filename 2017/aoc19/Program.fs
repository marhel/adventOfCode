// Learn more about F# at http://fsharp.org

open System
open System.IO

[<EntryPoint>]
let main argv =
    let grid = argv 
                |> List.ofSeq
                |> List.head
                |> File.ReadAllLines

    printfn "!%s!" grid.[0]
    grid
    |> List.ofSeq
    |> List.map (fun line -> printfn "!%d!" (String.length line))
    |> ignore
    0 // return an integer exit code
