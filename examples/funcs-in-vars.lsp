(
    (def add (fn (a b) (
        (+ a b)
    )))

    (def add2 +)
    (io.print "Result is" (add 1 2))
    (io.print "Result is" (add2 1 2))
    (assert.eq 3 (add 1 2))
    (assert.eq 3 (add2 1 2))
)
