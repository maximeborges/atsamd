//! Peripheral Touch Controller (PTC)
//! Please refer to the relevant documentation for your chip for information on
//! how to use this device peripheral.
//! - SAMD11: [Section 34 (page 855)](http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-42363-SAM-D11_Datasheet.pdf)
//! - SAMD21: [Section 36 (page 1040)](https://ww1.microchip.com/downloads/en/DeviceDoc/SAM_D21_DA1_Family_DataSheet_DS40001882F.pdf)
//!
//! Note that Y channels map to IDs `[0, 16)` and X channels to `[16, 32)`.

use crate::clock::GenericClockController;
use crate::ehal_02::adc::{Channel, OneShot};
use crate::gpio::*;
use crate::pac::{
    self, ptc, Pm
};
use crate::clock::{ClockGenId, ClockSource};

pub struct Ptc<PTC> {
    ptc: PTC,
}

impl Ptc<pac::Ptc> {
    /// Create a new `Ptc` instance.
    #[allow(clippy::self_named_constructors)]
    pub fn ptc(ptc: pac::Ptc, pm: &mut Pm, clocks: &mut GenericClockController) -> Self {
        // Enable the PTC clock
        let gclk3 = clocks
            .configure_gclk_divider_and_source(ClockGenId::Gclk3, 1, ClockSource::Osc8m, false)
            .expect("gclk3 already in use");
        clocks.ptc(&gclk3).expect("ptc clock setup failed");

        // Enable PTC in the APBC mask
        pm.apbcmask().modify(|_, w| w.ptc_().set_bit());

        // Reset the PTC module
        while ptc.ctrlb().read().syncflag().bit_is_set() {}
        ptc.ctrla().modify(|_, w| w.enable().clear_bit());
        while ptc.ctrlb().read().syncflag().bit_is_set() {}

        // Magic writes? Honestly dunno what these are for.
        // f7 => 11110111
        // fb => 11111011
        // fc => 11111100
        ptc.unk4c04()
            .modify(|r, w| unsafe { w.bits(r.bits() & 0xF7) });
        ptc.unk4c04()
            .modify(|r, w| unsafe { w.bits(r.bits() & 0xFB) });
        ptc.unk4c04()
            .modify(|r, w| unsafe { w.bits(r.bits() & 0xFC) });
        while ptc.ctrlb().read().syncflag().bit_is_set() {}

        // Next in the init sequence in the FreeTouch repo, writes of the following two
        // values are made to FREQCTRL:
        // 9f => 10011111
        //       ---baaaa
        // ef => 11101111
        //       ---baaaa
        // The upper three bits are unused, so I'm unsure what the point of this is
        // beyond setting all of the SAMPLEDELAY field to 1 and toggling
        // FREQSPREADEN. Furthermore, the next thing done is setting SAMPLEDELAY
        // to 0, thus ending up with (in theory):   11101111
        // & 11110000
        // ----------
        //   11100000
        // So honestly, I'm just going to set them to 0 in one step.
        ptc.freqctrl()
            .write(|w| w.freqspreaden().clear_bit().sampledelay().freqhop1());

        // Software init
        ptc.ctrlc().write(|w| w.init().set_bit());
        // Set to run in standby
        ptc.ctrla().write(|w| w.runstdby().set_bit());
        while ptc.ctrlb().read().syncflag().bit_is_set() {}

        // Set interrupt enables
        ptc.intenclr().modify(|_, w| w.wco().set_bit());
        while ptc.ctrlb().read().syncflag().bit() {}
        ptc.intenclr().modify(|_, w| w.eoc().set_bit());
        while ptc.ctrlb().read().syncflag().bit() {}

        ptc.yselecten().write(|w| unsafe { w.bits(0xFFFF) });
        while ptc.ctrlb().read().syncflag().bit_is_set() {}

        let mut this = Self { ptc };

        this.power_up();

        this
    }

    pub fn compcap(&mut self, compcap: u16) {
        self.ptc
            .compcap()
            .write(|w| unsafe { w.value().bits(compcap) });
        while self.ptc.ctrlb().read().syncflag().bit_is_set() {}
    }

    pub fn intcap(&mut self, intcap: u8) {
        self.ptc.intcap().write(|w| unsafe { w.value().bits(intcap) });
        while self.ptc.ctrlb().read().syncflag().bit_is_set() {}
    }

    pub fn oversample(&mut self, oversample: ptc::convctrl::Oversamplecount) {
        self.ptc
            .convctrl()
            .modify(|_, w| w.adcaccum().variant(oversample));
        while self.ptc.ctrlb().read().syncflag().bit_is_set() {}
    }

    pub fn sample_delay(&mut self, sampledelay: ptc::freqctrl::Freqhop) {
        let freqspreaden = matches!(sampledelay, ptc::freqctrl::Freqhop::Freqhop1);
        self.ptc.freqctrl().write(|w| {
            w.freqspreaden()
                .bit(freqspreaden)
                .sampledelay()
                .variant(sampledelay)
        });
        while self.ptc.ctrlb().read().syncflag().bit_is_set() {}
    }

    pub fn series_resistance(&mut self, serres: ptc::serres::Resvalue) {
        self.ptc.serres().write(|w| w.resistor().variant(serres));
        while self.ptc.ctrlb().read().syncflag().bit_is_set() {}
    }

    pub fn xselect(&mut self, xselect: ptc::xselect::Xmuxselect) {
        self.ptc.xselect().write(|w| w.xmux().variant(xselect));
        while self.ptc.ctrlb().read().syncflag().bit_is_set() {}
    }

    pub fn yselect(&mut self, yselect: ptc::yselect::Ymuxselect) {
        self.ptc.yselect().write(|w| w.ymux().variant(yselect));
        while self.ptc.ctrlb().read().syncflag().bit_is_set() {}
    }

    fn power_up(&mut self) {
        while self.ptc.ctrlb().read().syncflag().bit_is_set() {}
        self.ptc.ctrla().modify(|_, w| w.enable().set_bit());
        while self.ptc.ctrlb().read().syncflag().bit_is_set() {}
    }

    fn convert(&mut self) -> u16 {
        self.ptc
            .burstmode()
            .write(|w| unsafe { w.burstmode().bits(0xa4) });
        while self.ptc.ctrlb().read().syncflag().bit_is_set() {}

        self.ptc.convctrl().write(|w| w.convert().set_bit());
        while self.ptc.ctrlb().read().syncflag().bit_is_set() {}

        while self.ptc.convctrl().read().convert().bit_is_set() {}
        while self.ptc.ctrlb().read().syncflag().bit_is_set() {}

        self.ptc.result().read().result().bits()
    }
}

impl<WORD, PIN> OneShot<pac::Ptc, WORD, PIN> for Ptc<pac::Ptc>
where
    WORD: From<u16>,
    PIN: Channel<pac::Ptc, ID = u8>,
{
    type Error = ();

    fn read(&mut self, _pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
        let channel = PIN::channel();
        while self.ptc.ctrlb().read().syncflag().bit_is_set() {}

        // Select and enable specified channel
        if channel > 15 {
            let channel = channel - 16;
            self.ptc
                .xselect()
                .write(|w| unsafe { w.xmux().bits(1 << channel) });
        } else {
            self.ptc
                .yselect()
                .write(|w| unsafe { w.ymux().bits(1 << channel) });
        };
        while self.ptc.ctrlb().read().syncflag().bit_is_set() {}

        let result = self.convert();

        Ok(result.into())
    }
}

pub struct XAndY {
    pub x: u8,
    pub y: u8,
}

macro_rules! ptc_pins {
    ($($PinId:ident: $Chan:literal$( / $Chan2:literal)?),+) => {$(
        ptc_pins!{@$PinId: $Chan$( / $Chan2)?}
    )+};
    (@$PinId:ident: $Chan:literal) => {
        impl Channel<pac::Ptc> for Pin<$PinId, AlternateB> {
            type ID = u8;
            fn channel() -> u8 { $Chan }
        }
    };
    (@$PinId:ident: $XChan:literal / $YChan:literal) => {
        impl Channel<PTC> for Pin<$PinId, AlternateB> {
            type ID = XAndY;
            fn channel() -> XAndY { XAndY { x: $XChan, y: $YChan } }
        }
    };
}


#[cfg(feature = "samd11")]
ptc_pins! {
    PA02: 0,
    PA04: 2,
    PA05: 3,
    PA14: 16 / 6,
    PA15: 17 / 7,
    PA24: 24 / 14,
    PA25: 25 / 15
}

#[cfg(feature = "samd11d")]
ptc_pins! {
    PA03: 1,
    PA06: 4,
    PA06: 5,
    PA10: 18 / 8,
    PA11: 19 / 9,
    PA16: 20 / 10,
    PA17: 21 / 11,
    PA22: 22 / 12,
    PA23: 23 / 13,
    PA27: 26
}

#[cfg(feature = "samd21")]
ptc_pins! {
    PA02: 0,
    PA03: 1,
    PA04: 2,
    PA05: 3,
    PA06: 4,
    PA07: 5,
    PA08: 16,
    PA09: 17,
    PA10: 18,
    PA11: 19,
    PA16: 20,
    PA17: 21,
    PA18: 22,
    PA19: 23,
    PA22: 26,
    PA23: 27
}

#[cfg(any(feature = "samd21g", feature = "samd21j"))]
ptc_pins! {
    PB02: 8,
    PB03: 9,
    PB08: 14,
    PB09: 15,
    PA20: 24,
    PA21: 25
}

#[cfg(feature = "samd21j")]
ptc_pins! {
    PB00: 6,
    PB01: 7,
    PB04: 10,
    PB05: 11,
    PB06: 12,
    PB07: 13,
    PB12: 28,
    PB13: 29,
    PB14: 30,
    PB15: 31
}
