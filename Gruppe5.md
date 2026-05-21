# Gruppe 5

## Aufgabe 1

### Teilaufgabe A
```
x3 = x1 + 0;
Loop x2 Do
  x3 = x3 - 1
End;
x4 = x1 + 0;
Loop x3 Do
  x4 = x2 + 0
End;
x0 = x4 + 0
```

### Teilaufgabe B
```
x4 = x1 + 0;
Loop x2 Do
  x4 = x4 - 1
End;
x6 = x1 + 0;
Loop x4 Do
  x6 = x2 + 0
End;

x4 = x1 + 0;
Loop x2 Do
  x4 = x4 - 1
End;
x7 = x2 + 0;
Loop x4 Do
  x7 = x1 + 0
End;

x4 = x7 + 0;
Loop x6 Do
  x4 = x4 - 1
End;

Loop x3 Do
  Loop x4 Do
    x5 = x5 + 1
  End
End;
Loop x2 Do
  x5 = x5 + 1
End;
Loop x2 Do
  x5 = x5 + 1
End;
x0 = x5 + 0
```

### Teilaufgabe C
```
Loop x1 Do
  Loop x1 Do
    x7 = x7 + 1
  End
End;
x4 = x7 + 0;

x5 = x5 + 5;
Loop x5 Do
  Loop x2 Do
    x6 = x6 + 1
  End
End;
Loop x6 Do
  x4 = x4 + 1
End;
x0 = x4 + 0
```

## Aufgabe 2

### Teilaufgabe A
```
x1 = x1 + 1;
x2 = x2 + 1;
x3 = x3 + 1;
x4 = x1 + 0;
While x2 > 0 Do
  x4 = x4 + 1;
  x2 = x2 - 1
End;
While x3 > 0 Do
  x4 = x4 + 1;
  x3 = x3 - 1
End;
x0 = x4 + 0;
```

### Teilaufgabe B
```
While x1 > 0 Do
  x1 = x1 + 1
End;
x0 = x1 + 0
```

### Teilaufgabe C
```
While x1 > 0 Do
  x3 = x1 + 0;
  While x3 > 0 Do
  	x2 = x2 + 1;
    x3 = x3 - 1
  End;
  x1 = x1 - 1
End;
x0 = x2 + 1
```

## Aufgabe 4

### Teilaufgabe A

Die Idee war eine Art Cache zu haben der die bereits berechneten Werte speichert.
Die meisten Werte sind unterhalb der gewünschten Zahl weshalb wir die oberen gar nicht speichern.

**Zeitentabelle**
| Anzahl     | Unoptimiert | Optimiert   |
| ---------- | ----------- | ----------- |
| 1_000      | 0.00004835  | 0.000060423 |
| 10_000     | 0.00061042  | 0.000283118 |
| 100_000    | 0.008135426 | 0.003396705 |
| 1_000_000  | 0.09356656  | 0.034925047 |
| 5_000_000  | 0.53190696  | 0.25177148  |
| 10_000_000 | 1.1778127   | 0.49160925  |
| 90_000_000 | 11.0582     | 4.1951714   |
