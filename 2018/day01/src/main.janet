#!/bin/env janet

(defn parse_input [lines]
    (map scan-number lines))

(defn first_part [lines]
    (def total (reduce + 0 (parse_input lines)))
    (print total))

(defn second_part [lines]
    (var total 0)
    (var twice -1)
    (def p_lines (parse_input lines))
    (def hashmap @{})
    (put hashmap 0 false)
    (while (<= twice -1)
        (loop [num :in p_lines :while (= false (get hashmap total))]
            (+= total num)
            (if (= true (get hashmap total))
                (do
                    (set twice total)
                    (put hashmap total true)))
            (put hashmap total false)))
    (print twice))

(def input "/home/grayjack/MySources/Advent-Of-Code/2018/day01/input/input.txt")
(def file (slurp input))
(def lines (string/split "\n" file))
(def lines_filtered (filter (complement empty?) lines))
(first_part lines_filtered)
(second_part lines_filtered)
