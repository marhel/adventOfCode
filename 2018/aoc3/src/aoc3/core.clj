(ns aoc3.core
    (:require [clojure.set])
    (:require [clojure.math.combinatorics :as combo]))

(defn with-file [file per-line-fn]
  (with-open [rdr (clojure.java.io/reader file)]
         (let [lines (line-seq rdr)]
            (per-line-fn lines))))
 
(defn to-numbers [strs]
    (map #(Integer/parseInt %) strs))

(defn cells
  "I don't do a whole lot."
  [[x y w h]]
  (combo/cartesian-product (range x (+ x w)) (range y (+ y h))))

(defn add-cells
    [claim]
    (assoc claim :cells (cells (:rect claim))))

(defn parse-claim [line]
    (let [items (clojure.string/split line #"[#@, :x]")
        [id x y w h] (to-numbers (filter #(< 0 (count %)) items))]
        {:id id :rect [x y w h]}))

(defn claims-with-overlap [lines]
    (let [claims (map parse-claim lines)
        claims (map add-cells claims)
        cells (mapcat :cells claims)]
    {
        :claims claims 
        :overlap (->> cells
            (frequencies)
            (filter (fn [[k v]] (> v 1)))
            (keys))
    }))

(defn part1 [lines]
    (count (:overlap (claims-with-overlap lines))))

(defn overlaps [overlap claim]
    (let [cells (set (:cells claim))]
        ; uses the fact that sets are also membership predicates
        (some overlap cells)))

(defn single [x] {:pre [(nil? (next x))]} (first x))

(defn part2 [lines]
    (let [claims (claims-with-overlap lines)
        overlap (set (:overlap claims))]
    (->> (:claims claims)
        (remove (partial overlaps overlap))
        (single)
        (:id)
        )))
