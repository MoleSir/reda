# SI2 Lef 

This library contains code adapted from [Si2 LEF parser (v5.8)](https://github.com/The-OpenROAD-Project-Attic/lef.git).

Licensed under the Apache License, Version 2.0. See the LICENSE file for details.


## Changs

Since clef does not seem to encapsulate spaceTable functionality, I implemented the following two functions:

```c
EXTERN int lefiLayer_numSpacingTable (const lefiLayer* obj);
EXTERN const void* lefiLayer_spacingTable(const lefiLayer* obj, int index);
```