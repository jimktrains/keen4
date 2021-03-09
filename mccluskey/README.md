McKluskey
=========

I'm attempting to build out a McKluskey simplification of a binary
expression. So far I simply have it building out the DNF.


Sample Output

    Original:    (d!(e + !b + !d))(e + c) + a
    Distributed: d!ebde + d!ebdc + a
    SAST: (d!ebde) + (d!ebdc) + (a)
    Ord:  (a) + (bcdd!e) + (bdd!ee)
    Simp: (a) + (bcd!e)

