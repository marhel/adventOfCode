(ns aoc15.core
  (:require [clojure.string :as str]
            [clojure.data.priority-map :refer [priority-map-keyfn]]
            [clojure.set :as set]))

#_(defn with-file [file per-line-fn]
    (with-open [rdr (clojure.java.io/reader file)]
      (let [lines (line-seq rdr)]
        (per-line-fn lines))))

(defn ^:private generate-route [node came-from]
  (loop [route '()
         node node]
    (if (came-from node)
      (recur (cons node route) (came-from node))
      route)))

(defn route
  "Finds the shortest route from start to goal in a graph.
  Graph is a function (eg. a map) from nodes to a collection of adjacent nodes.
  Dist is a function from two nodes to the distance (as a number) from the first node to the second.
  H is a function from a node to the heuristic distance from that node to the goal. It should never overestimate the distance.
  Start and goal are two nodes.
  Returns a list of nodes on the route, excluding the start node and including the goal node. If a route can't be found, returns nil."
  [graph dist h start goal]
  (loop [visited {}
         queue (priority-map-keyfn first start [0 0 nil])]
    (when (seq queue)
      (let [[current [_ current-score previous]] (peek queue)
            visited (assoc visited current previous)]
        (if (= current goal)
          (generate-route goal visited)
          (recur visited (reduce (fn [queue node]
                                   (let [score (+ current-score (dist current node))]
                                     (if (and (not (contains? visited node))
                                              (or (not (contains? queue node))
                                                  (< score (get-in queue [node 1]))))
                                       (assoc queue node [(+ score (h node)) score current])
                                       queue)))
                                 (pop queue)
                                 (graph current))))))))

(def board "################################
####.#######..G..########.....##
##...........G#..#######.......#
#...#...G.....#######..#......##
########.......######..##.E...##
########......G..####..###....##
#...###.#.....##..G##.....#...##
##....#.G#....####..##........##
##..#....#..#######...........##
#####...G.G..#######...G......##
#########.GG..G####...###......#
#########.G....EG.....###.....##
########......#####...##########
#########....#######..##########
#########G..#########.##########
#########...#########.##########
######...G..#########.##########
#G###......G#########.##########
#.##.....G..#########..#########
#............#######...#########
#...#.........#####....#########
#####.G..................#######
####.....................#######
####.........E..........########
#####..........E....E....#######
####....#.......#...#....#######
####.......##.....E.#E...#######
#####..E...####.......##########
########....###.E..E############
#########.....##################
#############.##################
################################")

; Targets:      In range:     Reachable:    Nearest:      Chosen:
; #######       #######       #######       #######       #######
; #E..G.#       #E.?G?#       #E.@G.#       #E.!G.#       #E.+G.#
; #...#.#  -->  #.?.#?#  -->  #.@.#.#  -->  #.!.#.#  -->  #...#.#
; #.G.#G#       #?G?#G#       #@G@#G#       #!G.#G#       #.G.#G#
; #######       #######       #######       #######       #######

(defn new-counter []
  (let [mem (atom 0)]
    (fn [] (swap! mem inc))))

(defn id [sym [rn cn] gen-id]
  (condp = sym
    \E (atom {:id (gen-id) :type :unit :unit :elf :pos [rn cn] :hp 200})
    \G (atom {:id (gen-id) :type :unit :unit :goblin :pos [rn cn] :hp 200})
    \. :space
    :wall))

(defn inspect [label x]
  (println label "<" x ">")
  x)

(defn parse [board]
  (let [gen-id (new-counter)
        rows (map str/trim (str/split board #"\n"))
        rowno (range 0 100)
        colno (range 0 100)]
    (vec (map (fn [row rn]
                (vec (map #(id %1 [rn %2] gen-id) row colno))) rows rowno))))

(defn adjacent-to [pos] {:pre [(seq pos)]}
  (let [[r c] pos
        above [(- r 1) c]
        below [(+ r 1) c]
        right [r (+ c 1)]
        left [r (- c 1)]]
    [above left right below]))

(defn in-range-of [pos target grid] {:pre [(seq pos) (seq target)]}
  (let [dir (adjacent-to target)
        loc (map #(get-in grid %) dir)]
    (->> (zipmap dir loc)
         (filter #(or (= :space (second %)) (= pos (first %))))
         (map first))))

#_(defn get-units [grid]
  (filter #(= :unit (:type %)) (flatten grid)))

(defn get-unit-atoms [grid]
  (filter #(instance? clojure.lang.Atom %) (flatten grid)))

(defn targets [enemy unit-atoms]
  (filter #(= enemy (:unit (deref %))) unit-atoms))

(defn in-range [pos enemies grid]
  (->>
   enemies
   (map deref)
   (map :pos)
   (mapcat #(in-range-of pos % grid))
   (sort)))

(defn dist [p1 p2]
  (* 1024 (reduce + (map (comp #(Math/abs %) -) p1 p2))))

(defn enemy-of [unit] (unit {:elf :goblin :goblin :elf}))

(defn in-reading-order
  "makes nodes on the top left (in reading order)
  appear closer to the target than other
  nodes having same actual distance"
  [target node]
  (let [[r c] node]
    (- (dist node target) (- 1024 (+ (* r 32) c)))))

(defn simple-route-to [source target grid]
  (let [adjacent (fn [node] (in-range-of source node grid))

        ;_ (print pos enemy target-positions destinations)
        ]
    (route adjacent dist (partial dist target) source target)))

(defn route-to-correct [source target grid] {:pre [(seq source) (seq target)]}
  (loop [step source
         route nil
         visited {source true}
         ]
    (if (or (< 0 (count route)) (= step target))
      (reverse route)
      (let [
            first-steps (in-range-of [-1 -1] step grid)
            first-steps (remove #(contains? visited %) first-steps)
            route-from-step (fn [step] 
              (if-let [route (simple-route-to step target grid)]
                (cons step route)
                nil))
            best-step (->> first-steps
                        (map route-from-step)
                        (remove nil?)
                        (sort-by count)
                        (first) ; first step of first route
                        (first))
            ; _ (println "in-progr" step best-step target first-steps route)
            ]
        (when best-step
          (recur best-step (cons best-step route) (assoc visited best-step true)))
        ))))

(defn route-to [source target grid] {:pre [(seq source) (seq target)]}
  (let [
        first-steps (in-range-of [-1 -1] source grid)
        ;first-steps (remove #(contains? visited %) first-steps)
        route-from-step (fn [step] 
          (if-let [route (simple-route-to step target grid)]
            (cons step route)
            nil))
        ; _ (println "in-progr" step best-step target first-steps route)
        ]
    (->> first-steps
      (map route-from-step)
      (remove nil?)
      (sort-by count)
      (first))))

(defn route-to-nearest-enemy [unit-atom {:keys [:unit-atoms :grid] :as ugrid}]
  (let [unit @unit-atom
        pos (:pos unit)
        enemy (enemy-of (:unit unit))
        enemies (targets enemy unit-atoms)
        ; target-positions (map :pos enemies)
        destinations (in-range pos enemies grid)
        ; _ (println "rtne: " (:unit unit) (:id unit) (:pos unit) "targetting" destinations)
        ]
    (->> destinations
         (map #(route-to pos % grid))
         (remove nil?)
         (sort-by count)
        ;(inspect "sorted-dest")
         (first))))

(defn nearest [unit-atom ugrid]
  (last (route-to-nearest-enemy unit-atom ugrid)))

(defn move [unit pos] (update-in unit [:pos] (constantly pos)))

(defn move-to [unit-atom route grid]
  (let [unit @unit-atom
        old-pos (:pos unit)]
    (if-let [new-pos (first route)]
      (let [moved-unit (swap! unit-atom move new-pos)
            grid (update-in grid old-pos (constantly :space))
            _ (println "moving" (:unit unit) (:id unit) "from" old-pos "to" new-pos "goal" (last route) "via" route)]
        (update-in grid new-pos (constantly unit-atom)))
      grid)))

(defn deref-or-identity [cell]
  (if (instance? clojure.lang.Atom cell)
    (deref cell)
    cell))

(defn select-target [unit-atom grid]
  (let [unit @unit-atom
        [r c] (:pos unit)
        enemy (enemy-of (:unit unit))
        above [(- r 1) c]
        below [(+ r 1) c]
        right [r (+ c 1)]
        left [r (- c 1)]
        dir [above left right below]
        loc (map (juxt :unit :hp) (map deref-or-identity (map #(get-in grid %) dir)))]
    (->> (zipmap dir loc)
         (filter #(= enemy (first (second %))))
         (sort-by (juxt (comp second second) first))
         (map first)
         (first))))

(defn plan-move [unit-atom {:keys [:unit-atoms :grid] :as ugrid}]
  (if-let [target (select-target unit-atom grid)]
    {:target target :unit-atoms unit-atoms :grid grid}
    (let [route (route-to-nearest-enemy unit-atom ugrid)
          grid (move-to unit-atom route grid)
          ;_ (println (:unit old-unit) (:id old-unit) (:pos old-unit) "route" route)
          ]
      {:unit-atoms unit-atoms :grid grid})))

(defn bring-out-your-dead
  ([{:keys [:unit-atoms :grid] :as ugrid} unit-atom]
   (let [unit @unit-atom
         pos (:pos unit)
         grid (update-in grid pos (constantly :space))
         _ (println  (:unit unit) (:id unit) "died!")]
     {:unit-atoms unit-atoms :grid grid}))
  ([{:keys [:unit-atoms :grid] :as ugrid}]
   (let [dead (filter #(> 0 (:hp (deref %))) unit-atoms)]
     (reduce bring-out-your-dead ugrid dead))))

(defn dec3 [v] (- v 3))
(defn damage [unit] (update-in unit [:hp] dec3))

(defn perform-attack [unit-atom {:keys [:unit-atoms :grid :target] :as ugrid}]
  (if-let [target (or target (select-target unit-atom grid))]
    (let [enemy (get-in grid target)
          enemy (swap! enemy damage)
          unit @unit-atom
          _ (println ">>" (:unit unit) (:id unit) "attacks" (:unit enemy) (:id enemy) "now hp" (:hp enemy))]
      (bring-out-your-dead {:unit-atoms unit-atoms :grid grid}))
    ugrid))

(defn exit-criteria? [{:keys [:unit-atoms :grid :target] :as ugrid} unit-atom]
  (let [unit @unit-atom
        pos (:pos unit)
        enemy (enemy-of (:unit unit))
        enemies (targets enemy unit-atoms)
        ; target-positions (map :pos enemies)
        ]
    (empty? enemies)))

(defn take-turn [ugrid unit-atom]
  (if-let [hp (< 0 (:hp @unit-atom))]
    (if (exit-criteria? ugrid unit-atom)
      (update-in ugrid [:grid] reduced)
      (->> ugrid
           (plan-move unit-atom)
           (perform-attack unit-atom)))
    ugrid))

(defn turn [grid]
  (let [unit-atoms (get-unit-atoms grid)
        unit-atoms (sort-by (comp :pos deref) unit-atoms)
        _ (println "new turn with" (map deref unit-atoms))
        ugrid {:unit-atoms unit-atoms :grid grid}]
    (:grid (reduce take-turn ugrid unit-atoms))))


; :elf 25 died!
; after turn 74
; :goblin 10 [11 14] 104
; :goblin 1 [14 21] 200
; :goblin 5 [20 7] 200
; :goblin 2 [20 9] 200
; :goblin 13 [20 12] 71
; :goblin 7 [21 11] 155
; :goblin 16 [21 13] 185
; :goblin 6 [21 15] 200
; :goblin 17 [22 10] 188
; :goblin 8 [22 12] 200
; :goblin 3 [23 10] 200
; :goblin 22 [23 11] 50
; :goblin 9 [24 8] 167
; :goblin 11 [24 9] 200
; :goblin 19 [24 11] 200
; 
; (* 74 (+ 104 200 200 200 71 155 185 200 188 200 200 50 167 200 200))

; 186480 is too low
