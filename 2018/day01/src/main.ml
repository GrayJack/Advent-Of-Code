open Core

module SetInt = Set.Make(Int)

val first_part : string -> int
let first_part input =
    input
        |> List.map Int.of_string |> List.map (Option.get_or ~default:0)
        |> List.fold_left (+) 0

(* let second_part input = *)


let input = IO.(with_in "input.txt" read_lines_l)

let part_one = first_part input

let () =
    Printf.printf "Total: %d\n" part_one
