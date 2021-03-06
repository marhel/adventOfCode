(ns advent8
  (:use clojure.string))


(def code "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
")
(def code (slurp "../aoc8/input8.1.txt"))

(def ops {"inc" + "dec" -})
(def preds {">"  >
            "<"  <
            ">=" >=
            "<=" <=
            "!=" not=
            "==" =})

(defn evaluate [env [dst op change src pred lim]]
  (let [val (get env dst 0)
        src (get env src 0)
        change (Integer/parseInt change)
        lim (Integer/parseInt lim)]
    (if ((get preds pred =) src lim)
      (assoc env dst ((get ops op +) val change))
      env)))

(defn interpreter [code]
  (let [lines (split-lines code)]
    (->> lines
         (map (comp rest first (partial re-seq #"(\w+) (inc|dec) (-?\d+) if (\w+) (<|>|<=|>=|==|!=) (-?\d+)")))
         (reduce evaluate {})
         (apply max-key #(nth % 1))
         rest
         first)))

(defn interpreter-max [code]
  (let [lines (split-lines code)
        all-envs (->> lines
                      (map (comp rest first (partial re-seq #"(\w+) (inc|dec) (-?\d+) if (\w+) (<|>|<=|>=|==|!=) (-?\d+)")))
                      (reductions evaluate {})
                      (filter #(< 0 (count %))))
        env-max (fn [env] (apply max-key #(nth % 1) env))
        maxes (->> (map env-max all-envs)
                   (map (comp first rest)))]
    [(last maxes) (apply max maxes)]))

(repeatedly 3 #(time (interpreter code)))




