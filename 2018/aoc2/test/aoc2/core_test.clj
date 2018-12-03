(ns aoc2.core-test
  (:require [clojure.test :refer :all]
            [aoc2.core :refer :all]))

(deftest a-test
  (testing "counts"
    (is (= [] (counts "abcdef")))
    (is (= [2 3] (counts "bababc")))
    (is (= [2] (counts "abbcde")))
    (is (= [3] (counts "abcccd")))
    (is (= [2] (counts "aabcdd")))
    (is (= [2] (counts "abcdee")))
    (is (= [3] (counts "ababab")))
    ))

(deftest checksum-test
  (testing "checksums"
    (is (= 12 (aoc2 ["abcdef"
                    "bababc"
                    "abbcde"
                    "abcccd"
                    "aabcdd"
                    "abcdee"
                    "ababab"])))
    (is (= 5704 (with-file "input-1.txt" aoc2)))))

(deftest part2
    (testing "diffs"
        (is (= "ace" (common ["abcde" "axcye"])))
        (is (= "fgij" (common ["fghij" "fguij"])))
        (is (= "fgij" (aoc2-2 ["abcde" "fghij" "klmno" "pqrst" "fguij" "axcye" "wvxyz"])))
        (is (= "umdryabviapkozistwcnihjqx" (with-file "input-1.txt" aoc2-2)))
        ))
