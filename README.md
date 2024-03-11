# kawk
Rustで記述されたVMを用いて実行するAWK言語実装

https://docs.google.com/presentation/d/1-aDd-hfE19yOSD82JdqC18JRvB5aFJdDP6OY_KRKbBM/edit?usp=sharing

下記のプログラムが

```
function tarai(x,y,z) {
  count += 1
  if (x <= y) {
    return y
  } else {
    return tarai(tarai(x-1,y,z),tarai(y-1,z,x),tarai(z-1,x,y))
  }
}

BEGIN {
  tarai(12,6,0)
  print count
}
```

次のようにコンパイルされ，VMで実行される
```
0	initenv	1
1	push	12
2	push	6
3	push	0
4	push	3
5	calluserfunc	10
6	pop	
7	loadval	0
8	print	1
9	end	
10	loadval	0
11	push	1
12	add	
13	setval	0
14	push	None
15	pop	
16	loadsfvar	0
17	loadsfvar	1
18	lessequalthan	
19	nif	23
20	loadsfvar	1
21	return	
22	jump	47
23	loadsfvar	0
24	push	1
25	sub	
26	loadsfvar	1
27	loadsfvar	2
28	push	3
29	calluserfunc	10
30	loadsfvar	1
31	push	1
32	sub	
33	loadsfvar	2
34	loadsfvar	0
35	push	3
36	calluserfunc	10
37	loadsfvar	2
38	push	1
39	sub	
40	loadsfvar	0
41	loadsfvar	1
42	push	3
43	calluserfunc	10
44	push	3
45	calluserfunc	10
46	return	
47	push	None
48	return	
```

ほかにも，[このようなコード](./sample/11_mandelbrot.awk)が

```
# https://azisava.sakura.ne.jp/mandelbrot/algorithm.html
BEGIN {
  print "P1"

  size = 4
  pixel = 100
  print pixel, pixel
  for (i = 0; pixel > i; i++) {
      x = i * size / pixel - size / 2
      for (j = 0; pixel > j; j++) {
          y = j * size / pixel - size / 2
          a = 0
          b = 0
          d = 0
          for (k = 0; 50 > k; k++) {
              _a = a * a - b * b + x
              _b = 2 * a * b + y
              a = _a
              b = _b
              if (a * a + b * b > 4) {
                  d = 1
                  break
              }
          }
          if (d) {
              printf "1 "
          } else {
              printf "0 "
          }
      }
      print
  } 

}
```

こうなる
```
DEBUGLEVEL: 2
0	initenv	12
1	push	"P1"
2	print	1
3	push	4
4	setval	0
5	push	None
6	pop	
7	push	100
8	setval	1
9	push	None
10	pop	
11	loadval	1
12	loadval	1
13	print	2
14	push	0
15	setval	2
16	push	None
17	pop	
18	loadval	1
19	loadval	2
20	greaterthan	
21	nif	154
22	loadval	2
23	loadval	0
24	mul	
25	loadval	1
26	div	
27	loadval	0
28	push	2
29	div	
30	sub	
31	setval	3
32	push	None
33	pop	
34	push	0
35	setval	4
36	push	None
37	pop	
38	loadval	1
39	loadval	4
40	greaterthan	
41	nif	144
42	loadval	4
43	loadval	0
44	mul	
45	loadval	1
46	div	
47	loadval	0
48	push	2
49	div	
50	sub	
51	setval	5
52	push	None
53	pop	
54	push	0
55	setval	6
56	push	None
57	pop	
58	push	0
59	setval	7
60	push	None
61	pop	
62	push	0
63	setval	8
64	push	None
65	pop	
66	push	0
67	setval	9
68	push	None
69	pop	
70	push	50
71	loadval	9
72	greaterthan	
73	nif	128
74	loadval	6
75	loadval	6
76	mul	
77	loadval	7
78	loadval	7
79	mul	
80	sub	
81	loadval	3
82	add	
83	setval	10
84	push	None
85	pop	
86	push	2
87	loadval	6
88	mul	
89	loadval	7
90	mul	
91	loadval	5
92	add	
93	setval	11
94	push	None
95	pop	
96	loadval	10
97	setval	6
98	push	None
99	pop	
100	loadval	11
101	setval	7
102	push	None
103	pop	
104	loadval	6
105	loadval	6
106	mul	
107	loadval	7
108	loadval	7
109	mul	
110	add	
111	push	4
112	greaterthan	
113	nif	119
114	push	1
115	setval	8
116	push	None
117	pop	
118	jump	128
119	loadval	9
120	push	1
121	add	
122	setval	9
123	loadval	9
124	push	1
125	sub	
126	pop	
127	jump	70
128	loadval	8
129	nif	133
130	push	"1 "
131	printf	0
132	jump	135
133	push	"0 "
134	printf	0
135	loadval	4
136	push	1
137	add	
138	setval	4
139	loadval	4
140	push	1
141	sub	
142	pop	
143	jump	38
144	print	0
145	loadval	2
146	push	1
147	add	
148	setval	2
149	loadval	2
150	push	1
151	sub	
152	pop	
153	jump	18
154	end
```

でこの画像が出力される

![mandelbrot](./sample/test.png)
