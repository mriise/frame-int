# WARNING: very much a work in progress

An interesting method of storing framed integers inspired by https://github.com/abeusher/timehash

## format

`[ header | bytes ]`

#### header

`[ 5 bit position | 2 bit length | 1 bit align ]`

#### full header

`[ 5 bit start index | 3 bit align ] --- [ 5 bit length | 3 bit reserved ]`

header stores its index byte, along with the amount of extra bytes stored, with the extra 2 bits used to shift the result 2 bits per from the origin.

full header simply stores the start and end index, loosing the capability of extra granularity in storage in exchange for being able to adress larger frames.

## defining 

using plank time `5.391247x10^-44s` as the smallest possible unit of time, this format stores times as a perfect binary tree with `2^256 x 5.391247x10^-44` being the parent and highest possible timeframe value stored.
since plank time is incredibly small and it is not often we would want to store timeframes smaller than 1ms and larger than 50 years, there needs to be a bit of control for us to store the timeframes we care about.
This is why the first 16 bits stores the higher, and lower omitted steps. this gives us a way to store timeframes with a large degree of granulatrity and flexibility.

____
for the header we are essentially encoding a u32 out of a u128 ignoring some percision above and below, with a bit of granularity outside of byte borders


## use

able to create more space efficient timestamps where the time outside of the frame stored can either be assumed or ignored. 

### examples

`[highest, lowest]` steps

omit `[180, 132]` gives you a range of ~266,107,207.5864 years to ~.0029 seconds taking up only 48 bits to store the direction of steps. added with the 16 bit header leaves us with a 64 bit value.

`[178, 130]` ~66,526,801.8966 years to ~.7338 ms 

storing the extremities of human perception can be done with `[159, 134]` giving a range of ~126.8898 years to ~.0117 seconds with a size of 41 bits.

## note: the contents of this repository are not guaranteed to match the spec above.
