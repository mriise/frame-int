#![feature(min_const_generics)]
#![forbid(unsafe_code)]
use std::{error, fmt, };

// TODO Proper Error handling
// (2^134*(5.391247*.0000000000000000000000000000000000000000001))/3104.787
// 5.391247x10^-44
// 2^256

#[derive(Debug, Clone)]
pub struct Dta<const N: usize> ( [u8; N] );

impl<const N: usize> Dta<N> {
    
    pub fn new(highest: u8, lowest: u8) -> Result<Self, DtaError<N>> {
        let total_bits = highest - lowest;
        // 256/8 = 32 + 2 = 34
        if N <= 2 || N > 34 || total_bits < 1 || total_bits > (N as u8 - 2)*8 {
            return Err(DtaError::<N>);
        }
        let mut dta: [u8; N] = [0u8; N];
        dta[0] = highest;
        dta[1] = lowest;
        Ok(Dta(dta))
    }

    pub fn as_bytes(&self) -> [u8; N] {
        self.0
    }
}

impl <const N: usize> fmt::Display for Dta<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.as_bytes();
        let mut string = "".to_owned();
        let total_bytes = (bytes[0] - bytes[1])/8;
        for byte_index in 2..total_bytes+2 {
            let byte_string = format!("{:008b}|", bytes[ byte_index as usize ]);
            string.push_str(byte_string.as_str());
        }
        let remaining_bits = bytes[0] - bytes[1] - (total_bytes * 8);
        if remaining_bits > 0 {
            for bit_index in 0..remaining_bits {
                // (byte & ( 1 << bit )) >> bit
                // https://stackoverflow.com/questions/2249731/how-do-i-get-bit-by-bit-data-from-an-integer-value-in-c
                let bit = ( bytes[(total_bytes+1) as usize] & ( 1 << bit_index )) >> bit_index;
                string.push_str(bit.to_string().as_str())
            }
        }
        write!(f, "[{}-{}: {}]", bytes[0], bytes[1], string)
    }
}

#[derive(Debug, Clone)]
pub struct DtaError<const N: usize>;

impl <const N: usize> fmt::Display for DtaError<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Total size {} out of range", N)
    }
}

impl<const N: usize> error::Error for DtaError<N> {}


// 274941996890625 seconds range for code 0
// 1_599_130_443_882
