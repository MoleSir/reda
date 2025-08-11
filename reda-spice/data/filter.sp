vgen 1 0 dc 0 ac 1 portnum 1

l1 1 2 0.058u
c2 2 0 40.84p
l3 2 3 0.128u
c4 3 0 47.91p
l5 3 4 0.128u
c6 4 0 40.48p
l7 4 5 0.058u

la 5 6 0.044u
lb 6 a 0.078u
cb a 0 17.61p
lc 6 b 0.151u
cc b 0 34.12p
c7 6 7 26.035p

l8 7 0 0.0653u
c8 7 8 20.8p
l9 8 0 0.055u
c9 8 9 20.8p
l10 9 0 0.653u

c10 9 out 45.64p

rl out 0 50

* Behavioural circuit, for comparison.

R1 in port1 50
xsp port1 xout 0 filter
R2 xout 0 50