#![allow(unused_imports)]
use vstd::{seq::*, set::*, map::*, prelude::*};

verus! {

#[derive(Copy, Clone)]
pub struct Granule {
    /// granule state
    state: u8,
}

impl Granule {
    pub fn state(&self) -> u8 {
        self.state
    }

    pub fn set_state(&mut self, state: u8) {
        self.state = state;
    }
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

proof fn granule_seq() {
    let gst: Seq<Granule> = Seq::new(GRANULE_STATUS_TABLE_SIZE as nat, |i: int| Granule { state: GranuleState::Undelegated });

    assert(gst[20] == Granule { state: GranuleState::Undelegated });

    let gst2 = gst.update(20, Granule { state: GranuleState::Delegated });

    assert(gst2[20] == Granule { state: GranuleState::Delegated });
}

const GRANULE_STATUS_TABLE_SIZE: usize = 0xfc000;

pub struct GranuleStatusTable {
    pub entries: [Entry; GRANULE_STATUS_TABLE_SIZE],
}

impl GranuleStatusTable {
    pub fn new() -> Self {
        Self {
            entries: [Entry::new(); GRANULE_STATUS_TABLE_SIZE],
//            entries: core::array::from_fn(|_| Entry::new()),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Entry(Granule);
impl Entry {
    pub fn new() -> Self {
        Self(Granule {
            state: GranuleState::Undelegated,
        })
    }
}

fn granule_index(gst: &GranuleStatusTable, index: usize) -> (ret: &Granule)
    requires 
       0 <= index < GRANULE_STATUS_TABLE_SIZE,
    ensures
       ret == gst.entries@[index as int].0
{
    &gst.entries[index].0
}

fn granule_update(gst: &mut GranuleStatusTable, index: usize, val: u8) 
    requires 
       0 <= index < GRANULE_STATUS_TABLE_SIZE,
    ensures
       old(gst).entries@.update(index as int, Entry(Granule { state: val })) == gst.entries@
{
    // XXX: `gst.entries[index] = Granule { state: val };` does not work now
    gst.entries.set(index, Entry(Granule { state: val }));
    assert(old(gst).entries@.update(index as int, Entry(Granule { state: val })) == gst.entries@);
}

pub fn set_granule(granule: &mut Granule, state: u8) {
    granule.set_state(state)
}

pub const GRANULE_SIZE: usize = 4096;
pub const FVP_DRAM0_REGION_START: usize = 0x8000_0000;
pub const FVP_DRAM0_REGION_END: usize = 0x8000_0000 + 0x7C00_0000;

pub fn granule_addr_to_index(addr: usize) -> (ret: usize)
    ensures 
        ret == usize::MAX || 0 <= ret < 0x7C000
{
    if FVP_DRAM0_REGION_START <= addr &&
       addr < FVP_DRAM0_REGION_END {
        return (addr - FVP_DRAM0_REGION_START) / GRANULE_SIZE;
    }
    usize::MAX
}

pub fn get_granule(gst: &GranuleStatusTable, addr: usize) -> Result<&Granule, ()>
{
    let idx = granule_addr_to_index(addr);
    if idx == usize::MAX {
        return Err(());
    }
    Ok(granule_index(gst, idx))
}

pub fn get_granule_if(gst: &GranuleStatusTable, addr: usize, state: u8) -> Result<&Granule, ()>
{
    let g = get_granule(gst, addr);
    if let Ok(x) = g {
        if x.state() != state {
            return Err(());
        } else {
            return Ok(x);
        }
    } else {
        return Err(());
    }
}

fn main() {
}

} // verus!
