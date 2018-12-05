(ns aoc5.core-test
  (:require [clojure.test :refer :all]
            [aoc5.core :refer :all]))

(deftest a-test
  (testing "FIXME, I fail."
    (is (= true (annihilates? \a \A)))
    (is (= true (annihilates? \A \a)))
    (is (= false (annihilates? \A \b)))
    (is (= false (annihilates? \b \A)))
    (is (= false (annihilates? \a \a)))
    (is (= false (annihilates? \A \A)))
    (is (= 10 (part1 "dabAcCaCBAcCcaDA")))
    (is (= 1 (part1 "bXaaaaAAaAAAx")))
    (is (= 9822 (with-file "input.txt" (comp part1 first))))
    (is (= "dbcCCBcCcD" (remove-unit "A" "dabAcCaCBAcCcaDA")))
    (is (= 6 (reduced-length "A" "dabAcCaCBAcCcaDA")))
    (is (= "dabAaBAaDA" (remove-unit "C" "dabAcCaCBAcCcaDA")))
    (is (= 4 (reduced-length "C" "dabAcCaCBAcCcaDA")))
    (is (= 4 (part2 "dabAcCaCBAcCcaDA")))
    (is (= 5726 (with-file "input.txt" (comp part2 first))))
    ))
