(ns aoc.core-test
  (:require [clojure.test :refer :all]
            [aoc.core :refer :all]))

(deftest part1
  (testing "can sum"
    (is (= 6 (part1 [1 2 3])))
    (is (= 6 ((comp part1 to-numbers) ["1", "2", "3"])))
    (is (= 590 (with-file "input-1.txt" (comp part1 to-numbers))))
    ))

(deftest part2
  (testing "can find duplicates"
    (is (= 10 (first-duplicate [1 10 2 10 3])))
    (is (= 10 (part2 [3 3 4 -2 -4])))
    (is (= 0 (part2 [1 -1])))
    (is (= 83445 (with-file "input-1.txt" (comp part2 to-numbers))))))
