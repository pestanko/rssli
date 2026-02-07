(
    ; Simple while with a counter
    (def counter 0)
    (while (< counter 5) (
        (io.print counter)
        (def counter (+ counter 1))
    ))
)
