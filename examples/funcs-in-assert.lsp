(
    (fn gen (x) (
        fn (y) (
            x + y
        )
    ))

    (def add3 gen(3))
    (print "Result is" (add3 5))
)
