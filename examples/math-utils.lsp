(
    ; Math utility functions library
    ; This file can be imported to use these functions in other scripts
    
    ; Calculate the square of a number
    (fn square (x) (* x x))
    
    ; Calculate the cube of a number
    (fn cube (x) (* x x x))
    
    ; Calculate power: x raised to the power of n
    (fn power (x n) (
        (if (== n 0)
            1
            (if (== n 1)
                x
                (* x (power x (- n 1)))
            )
        )
    ))
    
    ; Mathematical constant
    (def pi 3.14159)
    
    ; Helper function to calculate area of a circle
    (fn circle-area (radius) (* pi (square radius)))
)
