# kawk
Rustで記述されたVMを用いて実行するAWK言語実装

https://docs.google.com/presentation/d/1_CgdMLAVXLAmFmBwPPnGr5Saqmcv_piudENB0Iuc0CQ/edit#slide=id.g29ef2a599f3_0_65

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
