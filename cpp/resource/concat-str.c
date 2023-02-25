#define join(x, y)  x ## y
#define str(s)      # s

/* Prescan is disabled when arguments are stringized or concatenated */
join(A, join(B, C))
str(join(A, B))
