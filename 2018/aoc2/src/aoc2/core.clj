(ns aoc2.core
    (:require [clojure.set :as set])
    (:require [clojure.string :as str])
    (:require [clojure.math.combinatorics :as combo]))

(defn with-file [file per-line-fn]
  (with-open [rdr (clojure.java.io/reader file)]
         (let [lines (line-seq rdr)]
            (per-line-fn lines))))

(defn incnil [old] (inc (or old 0)))
(defn count-letter [acc letter] (update acc letter incnil))

(defn counts
  "Indicate if letters occur twice, trice or both"
  [boxid]
  (-> 
    (reduce count-letter {} boxid)
    (set/map-invert)
    (select-keys [2 3])
    (keys)
    (vec)
  ))

(defn aoc2 [lines]
    (->> 
        lines
        (map counts)
        (flatten)
        (frequencies)
        (vals)
        (apply *)))

(defn equal-pair [[a b]] (= a b))

(defn common [[w1 w2]]
    (->> 
        (map list w1 w2)
        (filter equal-pair)
        (map first)
        (str/join)))

(defn aoc2-2 [boxids]
    (let [box-pairs (combo/combinations boxids 2)
          commonalities (map common box-pairs)]
        (apply max-key count commonalities)))
