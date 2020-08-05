(defn chars [str] (map string/from-bytes str))

(def input (string/trim (slurp "input")))
(def num-array (map |(if (= $ "(") 1 -1) (chars input)))

(def part1 (reduce + 0 num-array))
(print part1)

(def part2
  (do
    (var index 0)
    (var sum 0)
    (loop [num :in num-array]
      (if (= sum -1)
        (break)
        (do
          (+= sum num)
          (+= index 1))))
    index))

(print part2)
