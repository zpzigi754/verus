#![allow(unused_imports)]
use vstd::{seq::*, set::*, map::*, prelude::*};

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

proof fn granule_seq() {
    let gst: Seq<Granule> = Seq::new(0xfc000, |i: int| Granule { state: GranuleState::Undelegated });

    assert(gst[20] == Granule { state: GranuleState::Undelegated });

    let gst2 = gst.update(20, Granule { state: GranuleState::Delegated });

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
    ensures
//       ret == gst@[index as int]
       ret == gst.entries[index as int]
{
    assert(gst@.dom().contains(index as int));
//    assert(gst@[index as int] == Granule { state: GranuleState::Delegated });
//    gst.entries[index] 
    let ret = gst.entries[index];
    assert(ret == gst.entries[index as int]);
//    assert(ret == gst@[index as int]);
    ret
}


fn main() {
}

} // verus!
