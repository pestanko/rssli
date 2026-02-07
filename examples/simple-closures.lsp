(
    ; Lets define a function that returns a closure
    (fn make-adder (n) (
        (fn (x) (+ n x))
    ))
    (def add5 (make-adder 5))
    (io.print (add5 10))
    (io.print (add5 20))
    ; inc
    (fn inc (x) (+ x 1))
    (io.print (inc 10))
    ; dec
    (fn dec (x) (- x 1))
    (io.print (dec 10))
    ; add
    (fn add (a b) (+ a b))
    (io.print (add 10 20))
    ; sub
    (fn sub (a b) (- a b))
    (io.print (sub 10 20))
)
