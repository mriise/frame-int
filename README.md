# WARNING: very much a work in progress

An interesting method of storing time inspired by https://github.com/abeusher/timehash

## format

`[ higher | lower | lower-higher = total bits ]`

index of higher steps = 8 bits
lower omitted = 8 bits 

max height is 256 (2^8)

## defining 

using plank time (5.391247*10^-44s) as the smallest possible unit of time, this format stores times as a perfect binary tree with 2^256*5.391247*10^-44 being the parent and highest possible timeframe value stored.
since plank time is incredibly small and it is not often we would want to store timeframes smaller than 1ms and larger than 50 years, there needs to be a bit of control for us to store the timeframes we care about.
This is why the first 16 bits stores the higher, and lower omitted steps. this gives us a way to store timeframes with a large degree of granulatrity and flexibility.

### examples

`[highest, lowest]` steps

omit `[180, 132]` gives you a range of ~266,107,207.5864 years to ~.0029 seconds taking up only 48 bits to store the direction of steps. added with the 16 bit header leaves us with a 64 bit value.

storing the extremities of human perception can be done with `[159, 134]` giving a range of ~126.8898 years to ~.0117 seconds with a size of 41 bits.

## note: the contents of this repository are not guaranteed to match the spec above.
