# WARNING: very much a work in progress

## format
[ higher | lower | lower-higher = total bits ]
higher & lower omitted = 8 bits ea

max height is 256 (2^8)


indexed chunks = 1 bits (8/16)

## defining 

using plank time (5.391247*10^-44s) as the smallest possible unit of time, this format stores times a perfect binary tree.
since plank time is incredibly small and it is not often we would want to store timeframes smaller than 1ms and larger than 50 years, there needs to be a bit of control for us to store the timeframes we care about.
This is why the first 16 bits stores the higher, and lower omitted steps. this gives us a way to store timeframes with a large degree of granulatrity and flexibility.

### examples

[highest, lowest] steps

omit [72, 132] gives you a range of ~266,107,207.5864 years to ~.0029 seconds taking up only 48 bits to store the direction of steps. added with the 16 bit header leaves us with a 64 bit value.

storing the extremities of human perception can be done with [97, 134] giving a range of ~126.8898 years to ~.0117 seconds with a size of 53 bits.

## concating definitions

since the prefix defines the total number of steps, it is possible to concatinate ms and ps together

where the first prefix 