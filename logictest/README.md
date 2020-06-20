Logic Test
----------

Sample Output

```
Expr: (z | ((y <-> a) & ((x + a) -> y)))
CNF:  ((((z | ~y) | (z | a)) & ((z | y) | (z | ~a))) & (((z | y) | ((z | x) | (z | ~a))) & ((z | y) | ((z | a) | (z | ~x)))))
Vars: z, x, a, y

truth_table(Expr) == truth_table(CNF) => true

CNF Table:
 a ~y  z
~a  y  z
~a  x  y  z
 a ~x  y  z

Truth Table:
a x y z Result
F F F F T
F F F T T
F F T F F
F F T T T
F T F F F
F T F T T
F T T F F
F T T T T
T F F F F
T F F T T
T F T F T
T F T T T
T T F F F
T T F T T
T T T F T
T T T T T
```
