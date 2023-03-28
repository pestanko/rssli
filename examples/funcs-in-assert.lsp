(
    (def add (fn (a b) (
        (+ a b)
    )))

    (fn gen (x) (
        fn (y) (
            x + y
        )
    ))

    (def add2 +)
    (assert.eq 3 (add 1 2))
    (assert.eq 3 (add2 1 2))
    (assert.eq 3 (add2 1 2))
    (def add3 gen(3))
    (print (add3 5))
    (assert.eq 8 (gen(5) 3))
    (assert.eq 4 add3(1))
    (assert.eq 5 add3(2))
)