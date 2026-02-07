(
    ; Lets implement a simple guess number game
    ; At the start, we will generate a random number between 1 and 100
    (def secret_number (rnd.int 1 100))
    (io.print "I have chosen a number between 1 and 100. Try to guess it.")
    ; We will use a while loop to keep the game running until the user guesses the number
    (while true (
        (def guess (cast.int (io.readline "Enter your guess: ")))
        (if (< guess secret_number) (
            (io.print "Too low!")
        )
        (if (> guess secret_number) (
            (io.print "Too high!")
        )
        (if (== guess secret_number) (
            (io.print "You guessed it!")
            (exit 0)
        )
    ))
    (io.print "You lost! The number was " secret_number)
    (exit 1)
)
