(ns aoc.core-test
  (:require [clojure.test :refer :all]
            [aoc.core :refer :all]))

(deftest part1
  (testing "can sum"
    (is (= 6 (aoc1 [1 2 3])))
    (is (= 6 ((comp aoc1 to-numbers) ["1", "2", "3"])))
    (is (= 590 (with-file "input-1.txt" (comp aoc1 to-numbers))))
    ))

(deftest part2
  (testing "can find duplicates"
    (is (= 10 (first-duplicate [1 10 2 10 3])))
    (is (= 10 (aoc2 [3 3 4 -2 -4])))
    (is (= 0 (aoc2 [1 -1])))
    (is (= 83445 (with-file "input-1.txt" (comp aoc2 to-numbers))))))
