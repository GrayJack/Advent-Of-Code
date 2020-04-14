(defn area [length width height]
    (+ (* 2 length width) (* 2 width height) (* 2 height length)))

(defn lesser-area [length width height]
    (min (* length width) (* length height) (* width height)))

(defn wrapping-paper [length width height]
    (+ (area length width height) (lesser-area length width height)))

(def input (string/trim (slurp "input")))
(def lines (string/split "\n" input))
(def lines-as-numbers  (map |(map scan-number (string/split "x" $)) lines))

(def part1
    (sum (map |(let [ [l w h] $ ] (wrapping-paper l w h)) lines-as-numbers)))
(pp part1)

(defn volume [length width height] (* length width height))

(defn lesser-perimeter [length width height]
    (min
        (+ length length width width)
        (+ width width height height)
        (+ length length height height)))

(defn ribbon [length width height]
    (+ (volume length width height) (lesser-perimeter length width height)))

(def part2
    (sum (map |(let [ [l w h] $ ] (ribbon l w h)) lines-as-numbers)))

(pp part2)
