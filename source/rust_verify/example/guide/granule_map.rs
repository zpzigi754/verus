#![allow(unused_imports)]
use vstd::{seq::*, set::*, map::*, prelude::*};
use std::ops::IndexMut;

verus! {

#[derive(Copy, Clone)]
pub struct Granule {
    /// granule state
    state: u8,
}

mod GranuleState {
    pub const Undelegated: u8 = 0;
    pub const Delegated: u8 = 1;
    pub const RD: u8 = 2;
    pub const Rec: u8 = 3;
    pub const RecAux: u8 = 4;
    pub const Data: u8 = 5;
    pub const RTT: u8 = 6;
}

proof fn granule_map() {
    let gst: Map<int, Granule> = Map::new(
        |i: int| 0 <= i <= 0xfc000,
        |i: int| Granule { state: GranuleState::Undelegated },
    );
    
    assert(gst[20] == Granule { state: GranuleState::Undelegated });

    let gst2 = gst.insert(20, Granule { state: GranuleState::Delegated });
    
    assert(gst2[20] == Granule { state: GranuleState::Delegated });
}

const GRANULE_STATUS_TABLE_SIZE: usize = 0xfc000;

pub struct GranuleStatusTable {
    pub entries: [Granule; GRANULE_STATUS_TABLE_SIZE],
}

impl View for GranuleStatusTable {
    type V = Map<int, Granule>;

    spec fn view(&self) -> Map<int, Granule>;
}

fn granule_index(gst: &GranuleStatusTable, index: usize) -> (ret: Granule)
    requires 
       0 <= index < GRANULE_STATUS_TABLE_SIZE,
       gst@.dom().contains(index as int),
       gst@[index as int] == gst.entries@[index as int]
    ensures
       ret == gst@[index as int]
//       ret == gst.entries[index as int]
{
    assert(gst@.dom().contains(index as int));
    let ret = gst.entries[index];
    assert(ret == gst.entries[index as int]);
    assert(ret == gst@[index as int]);
    ret
}

fn granule_update(gst: &mut GranuleStatusTable, index: usize, val: u8) 
    requires 
       0 <= index < GRANULE_STATUS_TABLE_SIZE,
       old(gst)@.dom().contains(index as int),
       forall |i:int| old(gst)@[i] == old(gst).entries@[i],
//    ensures
//       old(gst).entries@.update(index as int, Granule { state: val }) == gst.entries@
{
    // XXX: `gst.entries[index] = Granule { state: set };` does not work currently
    gst.entries.set(index, Granule { state: val });
    //assert(old(gst)@.insert(index as int, Granule { state: val }) == gst@);
    // XXX: `assert(old(gst)@.insert(index as int, Granule { state: val }) == gst@);` assertion
    //      fail for some reasons
}


fn update_test(test: &mut usize, index: usize, val: u8) 
{
    *test = 4;
}

fn update_array(test: &mut [usize;4], index: usize, val: u8) 
    requires 
        0 <= index < 4,
{
    test.set(index, 4);
}

fn main() {
}

} // verus!
