(
    ; Lets define a function that returns a closure
    (fn make-adder (n) (
        (fn (x) (+ n x))
    ))
    (def add5 (make-adder 5))
    (io.print (add5 10))
    (io.print (add5 20))
)
