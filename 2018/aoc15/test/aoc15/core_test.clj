(ns aoc15.core-test
  (:require [clojure.string :as str]
            [clojure.test :refer :all]
            [aoc15.core :refer :all]))

; Targets:      In range:     Reachable:    Nearest:      Chosen:
; #######       #######       #######       #######       #######
; #E..G.#       #E.?G?#       #E.@G.#       #E.!G.#       #E.+G.#
; #...#.#  -->  #.?.#?#  -->  #.@.#.#  -->  #.!.#.#  -->  #...#.#
; #.G.#G#       #?G?#G#       #@G@#G#       #!G.#G#       #.G.#G#
; #######       #######       #######       #######       #######

(def smallboard
  "#######
#E..G.#
#...#.#
#.G.#G#
#######")

; [[:wall :wall :wall :wall :wall :wall :wall] [:wall {:type :unit, :unit :elf, :pos [1 1]} :space :space {:type :unit, :unit :goblin, :pos [1 4]} :space :wall] [:wall :space :space :space :wall :space :wall] [:wall :space {:type :unit, :unit :goblin, :pos [3 2]} :space :wall {:type :unit, :unit :goblin, :pos [3 5]} :wall] [:wall :wall :wall :wall :wall :wall :wall]]

(def grid (parse smallboard))
(def theunits (get-unit-atoms grid))
(def elf1 (nth theunits 0))
(def gob1 (nth theunits 1))
(def gob2 (nth theunits 2))
(def gob3 (nth theunits 3))

; (route adjacent dist (partial dist [1 1]) [1 1] [1 3])
(def larger-example (map parse ["#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########", "#########
#.G...G.#
#...G...#
#...E..G#
#.G.....#
#.......#
#G..G..G#
#.......#
#########", "#########
#..G.G..#
#...G...#
#.G.E.G.#
#.......#
#G..G..G#
#.......#
#.......#
#########", "#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########"]))

(def sample-combat-grid (map parse ["#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######", "#######
#..G..#
#...G.#
#.#G#G#
#...#E#
#.....#
#######"]))

(def le1-grid (nth larger-example 0))
(def le1-units (get-unit-atoms le1-grid))
(defn get-unit [units id type]
  (first (filter #(= (:id (deref %)) id) units)))

(defn anonymize
  "equalize all unit ids and hp to 0 for easier comparison"
  [grid]
   (let [units (get-unit-atoms grid)
         d-units (map deref units)]
    (reduce (fn [grid unit] 
      (update-in grid (:pos unit) (fn [unit-atom] (merge (deref unit-atom) {:id 0 :hp 0})))) grid d-units)))

(defn xid [elem]
  (condp = elem
    :wall \#
    :space \space
    :route \u26AC
    :start \+
    (subs (str/upper-case (str (or (:unit (deref elem)) "???"))) 1 2)))

(defn show 
  ([grid] (show grid -1))
  ([grid ndx]
    (let [units (get-unit-atoms grid)
          d-units (map deref units)
          stats (str/join \newline (map #(str/join " " ((juxt :unit :id :pos :hp) %)) d-units))]
      (str/join \newline [(format "after turn %d" ndx) stats (str/join \newline (map #(str/join (map xid %)) grid))]))))

(defn show-path [start goal grid]
  (let [route (route-to start goal grid)
        grid (update-in grid start (constantly :start))]
    (show (reduce (fn [grid pos] (update-in grid pos (constantly :route))) grid route))
    ))

(def obstacle-course (parse 
; 123456789 
"###########
 #.........#
 #.........#
 #....#....#
 #....#....#
 #..#####..#
 #....#....#
 #....#....#
 #.........#
 #.........# 
 ###########"))
; 123456789 

(deftest obstacle-test
  (testing "obstacles"
    ; route should handle obstacles and still traverse in reading order 
    (is (= [[1 2] [1 1] [2 1] [3 1] [4 1] [4 2] [4 3] [5 3] [5 4] [5 5]] (route-to [1 3] [5 5] (nth sample-combat-grid 1))))
  ))

(deftest route-test
  (testing "routing"
    (is (= [[3 4] [2 4] [2 3] [1 3] [1 2]] (route-to [4 4] [1 2] le1-grid)))
    (is (= [[3 4] [2 4] [2 5] [1 5] [1 6]] (route-to [4 4] [1 6] le1-grid)))
    (is (= [[4 3] [4 2] [5 2] [5 1] [6 1]] (route-to [4 4] [6 1] le1-grid)))
    (is (= [[4 5] [4 6] [5 6] [5 7] [6 7]] (route-to [4 4] [6 7] le1-grid)))

    (is (= (reverse [[3 4] [2 4] [2 3] [1 3] [1 2]]) (route-to [1 1] [3 4] le1-grid)))
    (is (= (reverse [[3 4] [2 4] [2 5] [1 5] [1 6]]) (route-to [1 7] [3 4] le1-grid)))
    (is (= (reverse [[4 3] [4 2] [5 2] [5 1] [6 1]]) (route-to [7 1] [4 3] le1-grid)))
    (is (= (reverse [[4 5] [4 6] [5 6] [5 7] [6 7]]) (route-to [7 7] [4 5] le1-grid)))
  )
  (testing "obstacle course"
    (is (=                [[1 4] [1 3] [1 2] [2 2] [3 2] [4 2] [5 2] [6 2] [6 3] [6 4] [7 4] [8 4] [8 5] [9 5]] (route-to [1 5] [9 5] obstacle-course)))
    (is (= (reverse [[1 5] [1 4] [1 3] [1 2] [2 2] [3 2] [4 2] [5 2] [6 2] [6 3] [6 4] [7 4] [8 4] [8 5]]) (route-to [9 5] [1 5] obstacle-course)))

    (is (=                [[4 1] [3 1] [2 1] [2 2] [2 3] [2 4] [2 5] [2 6] [2 7] [2 8] [2 9] [3 9] [4 9] [5 9]] (route-to [5 1] [5 9] obstacle-course)))
    (is (= (reverse [[5 1] [4 1] [3 1] [2 1] [2 2] [2 3] [2 4] [2 5] [2 6] [2 7] [2 8] [2 9] [3 9] [4 9]]) (route-to [5 9] [5 1] obstacle-course)))
    (is (= nil (println (show-path [5 1] [5 9] obstacle-course))))
    (is (= nil (println (show-path [5 9] [5 1] obstacle-course))))
    (is (= nil (println (show-path [1 9] [9 1] obstacle-course))))
    (is (= nil (println (show-path [9 1] [1 9] obstacle-course))))
    (is (= nil (println (show-path [1 1] [9 9] obstacle-course))))
    (is (= nil (println (show-path [9 9] [1 1] obstacle-course))))
    (is (= nil (println (show-path [1 5] [9 5] obstacle-course))))
    (is (= nil (println (show-path [9 5] [1 5] obstacle-course))))
    ))

(deftest range-test
  (testing "support functions"
    (is (= [[1 3] [1 5]] (in-range-of [1 1] [1 4] grid)))
    (is (= [[2 5]] (in-range-of [1 1] [3 5] grid)))
    (is (= [[2 2] [3 1] [3 3]] (in-range-of [1 1] [3 2] grid)))
    (is (= [[2 2] [3 1] [3 3]] (in-range-of [1 1] [3 2] grid)))
    (is (= [[1 3] [1 5] [2 2] [2 5] [3 1] [3 3]] (in-range [1 1] (targets :goblin theunits) grid)))
))

(defn unatomize-row [row ugrid] 
  (->> [:grid row]
    (get-in ugrid)
    (map deref-or-identity)))

(deftest support-test
  (testing "support functions"
    (is (= (map deref theunits) (map deref (get-unit-atoms (parse smallboard)))))
    (is (= [{:id 1 :type :unit :unit :elf :pos [1 1] :hp 200}] (map deref (targets :elf theunits))))
    ;(is (= (update-in (update-in grid [1 1] (constantly :space)) [1 2] (constantly (assoc elf1 :pos [1 2]))) (move-to elf1 [[1 2]] grid)))
    (is (= [[3 4] [2 4]] (route-to-nearest-enemy (get-unit le1-units 5 :elf) {:grid le1-grid :unit-atoms le1-units})))
    (is (= [1 3] (nearest elf1 {:grid grid :unit-atoms theunits})))
    (is (= [:wall :space {:id 1, :type :unit, :unit :elf, :pos [1 2] :hp 200} :space {:id 2, :type :unit, :unit :goblin, :pos [1 4] :hp 200} :space :wall]
           (unatomize-row 1 (take-turn {:grid grid :unit-atoms theunits} elf1))))))

(defn compare-grids [expected actual]
  (let [expected (anonymize expected)
        actual (anonymize actual)]
    (->> (map list expected actual)
         (reduce (fn [acc [er ar]]
                   (if (= er ar)
                     acc
                     (conj acc [er ar]))) []))))

(defn play-game [grid]
  (let [turns (take-while (complement reduced?) (iterate turn grid))]
    (map #(println (show %1 %2)) turns (range))
    ))

(def sample-game [(parse "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########") 20 937])

(deftest board-test
  (testing "board handling"
    ;(is (= (anonymize (nth larger-example 1)) (anonymize (turn le1-grid))))
    ;(is (= [] (compare-grids (nth larger-example 2) (turn (turn le1-grid)))))
    ;(is (= nil (println (show (nth (iterate turn le1-grid) 5)))))
    ;(is (= nil (println (take 5 (map show (iterate turn (nth sample-combat-grid 0)) (range 0 1000))))))
    ;(is (= nil (println (play-game (parse board)))))
    (is (= nil (println (play-game (first sample-game)))))
    ;(is (= nil (println (play-game (nth sample-combat-grid 0)))))
    #_(is (= [{:id 2 :type :unit :unit :goblin :pos [1 4] :hp 200}
            {:id 3 :type :unit :unit :goblin :pos [3 2] :hp 200}
            {:id 4 :type :unit :unit :goblin :pos [3 5] :hp 200}] (map deref (targets :goblin theunits))))))
