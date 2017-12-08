(ns advent
  (:use clojure.string))


(def code "pbga (66)\nxhth (57)\nebii (61)\nhavc (66)\nktlj (57)\nfwft (72) -> ktlj, cntj, xhth\nqoyq (66)\npadx (45) -> pbga, havc, qoyq\ntknk (41) -> ugml, padx, fwft\njptl (61)\nugml (68) -> gyxo, ebii, jptl\ngyxo (61)\ncntj (57)\n")

(defn weight [tree node] (let [imm (get-in tree [node :weight])
                               dep (get-in tree [node :deps])
                               dsu (apply + (map #(weight tree %) dep))]

                           (+ imm dsu)))
(defn partition-2
  "Partitions the collection into exactly two [[all-truthy] [all-falsy]]
   collection."
  [pred coll]
  (mapv persistent!
        (reduce
          (fn [[t f] x]
            (if (pred x)
              [(conj! t x) f]
              [t (conj! f x)]))
          [(transient []) (transient [])]
          coll)))

(defn aoc72 [code]
  (let [build-node (fn [name] (let [[_ name weight] (re-matches #"(\w+) \((\d+)\)" name)]
                                {:name name :weight (Integer/parseInt weight)}))
        parse-node (fn [[name deps]]
                     (let [node (build-node name)]
                       (if (nil? deps)
                         node
                         (assoc node
                           :deps (split deps #", ")))))
        clean-up (fn [{:keys [name weight deps]}] {:name name :weight weight :deps (map keyword deps)})
        construct-tree (fn [tree dict] (assoc tree (keyword (:name dict)) (clean-up dict)))
        add-weight (fn [tree node] (assoc-in tree [node :accum] (weight tree node)))
        add-accum (fn [tree node] (assoc-in tree [node :dep-acc] (map #(get-in tree [% :accum]) (get-in tree [node :deps]))))
        tree (->> (split-lines code)
                  (map #(split % #" -> "))
                  (map parse-node)
                  (reduce construct-tree {}))
        keys (keys tree)
        wtree (reduce add-weight tree keys)
        atree (reduce add-accum wtree keys)
        unbalanced (map #(nth % 1)
                        (filter (fn [node] (let [das (:dep-acc (nth node 1))]
                                            (if (empty? das) false (not (apply = das))))) atree))
        lowest (apply min-key :accum unbalanced)
        suspects (group-by (fn [[key weight]] weight)
                           (zipmap (:deps lowest) (:dep-acc lowest)))
        [[[badweight [[badkey]]]] [[goodweight]]] (partition-2 #(= 1 (count (nth % 1))) suspects)
        badkeyweight (:weight (badkey atree))]

    [badkey (- badkeyweight (- badweight goodweight))]))


