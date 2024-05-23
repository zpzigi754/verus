#![no_std]
use vstd::prelude::*;

verus! {

pub fn add(left: usize, right: usize) -> usize 
    requires 
        0 <= left <= 100,
        0 <= right <= 100,
{
    left + right
}

}
