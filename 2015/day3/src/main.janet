#!/bin/env janet
(defn chars [str] (map string/from-bytes str))

(defn distribute [map instructions]
  (var [x y] [0 0])
  (each char instructions
    (case char
      ">" (let [now-house-presents (get map [x y])
                next-house-presents (get map [(+ x 1) y])]
            (if (nil? now-house-presents)
              (put map [x y] 1)
              (put map [x y] (+ 1 now-house-presents)))
            (if (nil? next-house-presents)
              (put map [(+ x 1) y] 1)
              (put map [(+ x 1) y] (+ 1 next-house-presents)))
            (set x (+ x 1)))
      "<" (let [now-house-presents (get map [x y])
                next-house-presents (get map [(- x 1) y])]
            (if (nil? now-house-presents)
              (put map [x y] 1)
              (put map [x y] (+ 1 now-house-presents)))
            (if (nil? next-house-presents)
              (put map [(- x 1) y] 1)
              (put map [(- x 1) y] (+ 1 next-house-presents)))
            (set x (- x 1)))
      "^" (let [now-house-presents (get map [x y])
                next-house-presents (get map [x (+ y 1)])]
            (if (nil? now-house-presents)
              (put map [x y] 1)
              (put map [x y] (+ 1 now-house-presents)))
            (if (nil? next-house-presents)
              (put map [x (+ y 1)] 1)
              (put map [x (+ y 1)] (+ 1 next-house-presents)))
            (set y (+ y 1)))
      "v" (let [now-house-presents (get map [x y])
                next-house-presents (get map [x (- y 1)])]
            (if (nil? now-house-presents)
              (put map [x y] 1)
              (put map [x y] (+ 1 now-house-presents)))
            (if (nil? next-house-presents)
              (put map [x (- y 1)] 1)
              (put map [x (- y 1)] (+ 1 next-house-presents)))
            (set y (- y 1))))))

(defn split [instructions]
  (let [santa @[] robot @[]]
    (eachp [i instruction] instructions
      (if (even? i)
        (array/push santa instruction)
        (array/push robot instruction)))
    [santa robot]))

(def input (string/trim (slurp "input")))
(def instructions (chars input))

(def part1
  (let [map @{}]
    (distribute map instructions)
    (count |(not (nil? $)) map)))

(def part2
  (let [map @{} [santa-instructions robot-instructions] (split instructions)]
    (distribute map santa-instructions)
    (distribute map robot-instructions)
    (count |(not (nil? $)) map)))

(pp part1)
(pp part2)
