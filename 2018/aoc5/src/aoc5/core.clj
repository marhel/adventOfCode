(ns aoc5.core
    (:require [clojure.set]))

(defn with-file [file per-line-fn]
    (with-open [rdr (clojure.java.io/reader file)]
        (let [lines (line-seq rdr)]
            (per-line-fn lines))))

(defn xor [x y] (if x (not y) (boolean y)))
(defn same-char? [x y] (apply = (map clojure.string/upper-case [x y])))
(defn annihilates? [x y]
    (let [
        xup (Character/isUpperCase x)
        yup (Character/isUpperCase y)
        same-char (same-char? x y)
        diff-case (xor xup yup)
        ]
        (and same-char diff-case)
        ))

(defn collapse [[head & tail :as acc] char]
    (if (annihilates? (or head \?) char)
        tail
        (conj acc char)))

(defn part1
  "I don't do a whole lot."
  [polymer]
  (count (reduce collapse (list) polymer)))

(defn remove-unit [char polymer]
    (clojure.string/join (remove (partial same-char? char) polymer)))

(defn reduced-length [char polymer]
    (->>
        (remove-unit char polymer)
        (part1)))

(defn key-pair [f] (fn [k] [k (f k)]))

(defn part2
  "I don't do a whole lot."
  [polymer]
  (let [
    chars (set (map clojure.string/upper-case polymer))
    len-without #(reduced-length % polymer)
    count-by-char (into {} (map (key-pair len-without) chars))]
        (second (apply min-key val count-by-char))))
