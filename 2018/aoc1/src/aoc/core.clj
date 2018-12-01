(ns aoc.core)

(defn with-file [file per-line-fn]
  (with-open [rdr (clojure.java.io/reader file)]
         (let [lines (line-seq rdr)]
            (per-line-fn lines))))

(defn to-numbers [strs]
    (map #(Integer/parseInt %) strs))

(defn aoc1
  "I don't do a whole lot."
  [numbers]
  (reduce + numbers))

(defn first-duplicate [coll]
  (reduce (fn [seen x]
            (if (contains? seen x)
              (reduced x)
              (conj seen x)))
          #{0} coll))

(defn aoc2
  "I do a whole lot."
  [numbers]
  (first-duplicate (reductions + (cycle numbers))))

#_(defn benchit [fun]
    (with-file "input-1.txt" (fn [lines]
                (let [numbers (to-numbers lines)]
                    (bench (fun numbers) :verbose)))))
