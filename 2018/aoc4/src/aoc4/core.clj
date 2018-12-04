(ns aoc4.core
    (:require [clojure.set]))

(defn with-file [file per-line-fn]
  (with-open [rdr (clojure.java.io/reader file)]
     (let [lines (line-seq rdr)]
        (per-line-fn lines))))

(defn to-numbers [strs]
    (map #(Integer/parseInt %) strs))

(defn minute [time] (subs time 14 16))
(defn date [time] (subs time 0 10))
(defn minute-interval [start stop]
    (let [mins (vec (to-numbers [(minute start) (minute stop)]))]
        (if (= (date start) (date stop))
            mins
            (update-in mins [0] (fn [v] (- v 60))))))

(defn minute-range [start stop]
    (let [interval (minute-interval start stop)]
        (apply range interval)))

(defn identify [line]
    (let [[time what] (clojure.string/split line #"] ")
        [what who] (clojure.string/split what #"\s+")
        ]
        {:time (subs time 1)
         :what (condp = what
            "falls" :sleep
            "wakes" :wakes
            :guard)
         :who who
         }))

(defn span-from [{:keys [:who :start :stop]}]
    {who (minute-range start stop)})

(defn collect-from [acc]
    (update acc :spans conj (span-from acc)))

(defn detect-span [acc current]
    (condp = (:what current)
        :guard (merge (assoc acc :who (:who current)) {:start nil :stop nil})
        :sleep (assoc acc :start (:time current))
        :wakes (collect-from (assoc acc :stop (:time current)))))

(defn spans
  "parse the lines into sleep intervals"
  [lines]
  (->>
      (sort lines)
      (map identify)
      (reduce detect-span {})
      (:spans)
      ))

(defn mapval [m f]
    (into {}
        (for [[k v] m]
            [k (f v)])))

(defn guard-sleeps [lines]
    (let [found (spans lines)]
      (apply merge-with into found)))

(defn part1 [lines]
    (let [
      asleep            (guard-sleeps lines)
      sleep-time        (mapval asleep count)
      sleepiest         (key (apply max-key val sleep-time))
      schedule          (get asleep sleepiest)
      sleep-minutes     (frequencies schedule)
      candidate-minute  (first (apply max-key val sleep-minutes))
      sleepiest         (Integer/parseInt (subs sleepiest 1))
      ]
      (* sleepiest candidate-minute)))

(defn part2 [lines]
    (let [
      asleep            (guard-sleeps lines)
      sleep-minutes     (mapval asleep frequencies)
      max-minute-by-guard   (mapval sleep-minutes (partial apply max-key val))
      by-count          (comp second val)
      [sleepiest [max-minute count]]   (apply max-key by-count max-minute-by-guard)
      sleepiest         (Integer/parseInt (subs sleepiest 1))
      ]
      (* sleepiest max-minute)))