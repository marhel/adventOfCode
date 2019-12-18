(ns aoc6.core-test
  (:require [clojure.test :refer :all]
            [aoc6.core :refer :all]))

(deftest a-test
  (testing "provided example"
    (is (= 42 (orbits "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L")))
  (testing "part1"
  ; 2279 is too low
    (is (= 42 (orbits input))))
  (testing "part2-ex" 
    (let [rk (pk-tree "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN")
          you (path-to-root rk "YOU" [])
          san (path-to-root rk "SAN" [])
          v (strip-prefix you san)
          _ (print rk you san v)]
      (is (= 4 (- (count v) 3)))
    ))
  (testing "part2-ex" 
    (let [rk (pk-tree input)
          you (path-to-root rk "YOU" [])
          san (path-to-root rk "SAN" [])
          v (strip-prefix you san)
          _ (print rk you san v)]
      (is (= 4 (- (count v) 3)))
    ))
    ))
