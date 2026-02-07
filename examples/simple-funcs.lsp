(
    (fn add (a b) (
        (+ a b)
    ))
    (fn fact (x) (
        (if (== x 0)
            (1)
            (* x (fact (- x 1)))
        )
    ))
    (io.print "Result is" (add 1 2))
    (io.print "Result for fact 5:" (fact 5))
)
