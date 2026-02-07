(
    ; Import example: using functions and variables from another file
    ; Import the math-utils library
    (import "math-utils.lsp")
    
    ; Now we can use the imported functions and variables
    (io.print "=== Import Example ===")
    (io.print "")
    
    ; Use imported functions
    (io.print "Square of 5:" (square 5))
    (io.print "Cube of 3:" (cube 3))
    (io.print "2 to the power of 8:" (power 2 8))
    
    ; Use imported variable
    (io.print "Pi value:" pi)
    
    ; Use imported function with imported variable
    (io.print "Area of circle with radius 5:" (circle-area 5))
    
    (io.print "")
    (io.print "All functions and variables from math-utils.lsp are now available!")
)
