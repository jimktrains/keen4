McCluskey
=========

I'm attempting to build out a McKluskey simplification of a binary
expression. So far I simply have it building out the DNF.


Sample Output

    Original:    (d!(e + !b + !d))(e + c) + a
    Distributed: d!ebde + d!ebdc + a
    SAST: (d!ebde) + (d!ebdc) + (a)
    Ord:  (a) + (bcdd!e) + (bdd!ee)
    Simp: (a) + (bcd!e) + (bd!e)
    Terms: ["a", "b", "c", "d", "e"]
    MN: [
        (
            1,
            [
                [
                    One,
                    DontCare,
                    DontCare,
                    DontCare,
                    DontCare,
                ],
            ],
        ),
        (
            2,
            [
                [
                    DontCare,
                    One,
                    DontCare,
                    One,
                    Zero,
                ],
            ],
        ),
        (
            3,
            [
                [
                    DontCare,
                    One,
                    One,
                    One,
                    Zero,
                ],
            ],
        ),
    ]

