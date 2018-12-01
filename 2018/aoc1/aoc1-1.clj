(with-open [rdr (clojure.java.io/reader "input-1.txt")]
         (count (line-seq rdr)))
