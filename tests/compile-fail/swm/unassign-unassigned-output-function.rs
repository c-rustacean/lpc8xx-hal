extern crate lpc82x_hal;


use lpc82x_hal::Peripherals;
use lpc82x_hal::swm::{
    self,
    pin_state,
    Pin,
};


fn main() {
    let mut p = Peripherals::take().unwrap();

    let     swm    = p.swm.split();
    let mut syscon = p.syscon.split();

    let pio0_0: Pin<_, pin_state::Unused> = swm.pins.pio0_0;

    let u0_txd: swm::Function<_, swm::state::Unassigned> =
        swm.movable_functions.u0_txd;

    let (u0_txd, pio0_0) =
        u0_txd.assign(pio0_0.into_swm_pin(), &mut swm.handle);
    let (u0_txd, pio0_0) =
        u0_txd.unassign(pio0_0, &mut swm.handle);
    let (u0_txd, pio0_0) =
        u0_txd.unassign(pio0_0, &mut swm.handle);
    //~^ ERROR no method named `unassign` found for type
}