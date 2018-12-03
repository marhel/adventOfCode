(ns aoc3.core-test
  (:require [clojure.test :refer :all]
            [aoc3.core :refer :all]))

(def ex1 ["#1 @ 1,3: 4x4"
"#2 @ 3,1: 4x4"
"#3 @ 5,5: 2x2"])

(deftest a-test
  (testing "expansion"
    (is (= [[3 3] [3 4] [4 3] [4 4]] (cells [3 3 2 2])))
    (is (= {:id 10 :rect [793 662 19 17]} (parse-claim "#10 @ 793,662: 19x17")))
    (is (= {:id 10 :rect [3 3 2 2] :cells [[3 3] [3 4] [4 3] [4 4]]} 
        (add-cells {:id 10 :rect [3 3 2 2]})))
    (is (= 4 (part1 ex1)))
    (is (= 3 (part2 ex1)))
    (is (= 121163 (with-file "input-1.txt" part1)))
    (is (= 943 (with-file "input-1.txt" part2)))))
