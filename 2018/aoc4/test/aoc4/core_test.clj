(ns aoc4.core-test
  (:require [clojure.test :refer :all]
            [aoc4.core :refer :all]))

(deftest a-test
  (testing "guard sleep schedules"
    (is (= {:time "1518-11-01 00:00" :what :guard :who "#10"} (identify "[1518-11-01 00:00] Guard #10 begins shift")))
    (is (= {:time "1518-11-01 00:55" :what :wakes :who "up"} (identify "[1518-11-01 00:55] wakes up")))
    (is (= {:time "1518-11-01 00:30" :what :sleep :who "asleep"} (identify "[1518-11-01 00:30] falls asleep")))
    (is (= (range 30 55) (minute-range "1518-11-01 00:30" "1518-11-01 00:55")))
    (is (= (range -2 5) (minute-range "1518-11-01 23:58" "1518-11-02 00:05")))
    (is (= 6 (count (with-file "simple-input.txt" spans))))
    (is (= {"#99" (range 45 55)} (first (with-file "simple-input.txt" spans))))
    (is (= {"#10" (range 5 25)} (last (with-file "simple-input.txt" spans))))
    (is (= 240 (with-file "simple-input.txt" part1)))
    (is (= 4455 (with-file "simple-input.txt" part2)))
    (is (= 48680 (with-file "input.txt" part1)))
    (is (= 94826 (with-file "input.txt" part2)))))
