#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    ctrlb: Ctrlb,
    _reserved2: [u8; 0x02],
    unk4c04: Unk4c04,
    ctrlc: Ctrlc,
    _reserved4: [u8; 0x02],
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    _reserved7: [u8; 0x01],
    freqctrl: Freqctrl,
    convctrl: Convctrl,
    _reserved9: [u8; 0x02],
    yselect: Yselect,
    xselect: Xselect,
    yselecten: Yselecten,
    xselecten: Xselecten,
    compcap: Compcap,
    intcap: Intcap,
    serres: Serres,
    result: Result,
    _reserved17: [u8; 0x02],
    burstmode: Burstmode,
    wcomode: Wcomode,
    _reserved19: [u8; 0x02],
    wcothresholdal: Wcothresholdal,
    wcothresholdah: Wcothresholdah,
    wcothresholdbl: Wcothresholdbl,
    wcothresholdbh: Wcothresholdbh,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x01 - Control B"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x04 - Unknown Register 0x42004C04"]
    #[inline(always)]
    pub const fn unk4c04(&self) -> &Unk4c04 {
        &self.unk4c04
    }
    #[doc = "0x05 - Control C"]
    #[inline(always)]
    pub const fn ctrlc(&self) -> &Ctrlc {
        &self.ctrlc
    }
    #[doc = "0x08 - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x09 - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x0a - Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x0c - Frequency Control"]
    #[inline(always)]
    pub const fn freqctrl(&self) -> &Freqctrl {
        &self.freqctrl
    }
    #[doc = "0x0d - Conversion control"]
    #[inline(always)]
    pub const fn convctrl(&self) -> &Convctrl {
        &self.convctrl
    }
    #[doc = "0x10 - Select Y line"]
    #[inline(always)]
    pub const fn yselect(&self) -> &Yselect {
        &self.yselect
    }
    #[doc = "0x12 - Select X line"]
    #[inline(always)]
    pub const fn xselect(&self) -> &Xselect {
        &self.xselect
    }
    #[doc = "0x14 - Enable Y lines"]
    #[inline(always)]
    pub const fn yselecten(&self) -> &Yselecten {
        &self.yselecten
    }
    #[doc = "0x16 - Enable X lines"]
    #[inline(always)]
    pub const fn xselecten(&self) -> &Xselecten {
        &self.xselecten
    }
    #[doc = "0x18 - Compensation capacitor value"]
    #[inline(always)]
    pub const fn compcap(&self) -> &Compcap {
        &self.compcap
    }
    #[doc = "0x1a - Internal capacitor value"]
    #[inline(always)]
    pub const fn intcap(&self) -> &Intcap {
        &self.intcap
    }
    #[doc = "0x1b - Series resistor for PTC measurements"]
    #[inline(always)]
    pub const fn serres(&self) -> &Serres {
        &self.serres
    }
    #[doc = "0x1c - Conversion result"]
    #[inline(always)]
    pub const fn result(&self) -> &Result {
        &self.result
    }
    #[doc = "0x20 - Enable burst or clear to send low power mode"]
    #[inline(always)]
    pub const fn burstmode(&self) -> &Burstmode {
        &self.burstmode
    }
    #[doc = "0x21 - Set WCO mode"]
    #[inline(always)]
    pub const fn wcomode(&self) -> &Wcomode {
        &self.wcomode
    }
    #[doc = "0x24 - Set lower WCO threshold for port A"]
    #[inline(always)]
    pub const fn wcothresholdal(&self) -> &Wcothresholdal {
        &self.wcothresholdal
    }
    #[doc = "0x25 - Set upper WCO threshold for port A"]
    #[inline(always)]
    pub const fn wcothresholdah(&self) -> &Wcothresholdah {
        &self.wcothresholdah
    }
    #[doc = "0x26 - Set lower WCO threshold for port B"]
    #[inline(always)]
    pub const fn wcothresholdbl(&self) -> &Wcothresholdbl {
        &self.wcothresholdbl
    }
    #[doc = "0x27 - Set upper WCO threshold for port B"]
    #[inline(always)]
    pub const fn wcothresholdbh(&self) -> &Wcothresholdbh {
        &self.wcothresholdbh
    }
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`]
module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "UNK4C04 (rw) register accessor: Unknown Register 0x42004C04\n\nYou can [`read`](crate::Reg::read) this register and get [`unk4c04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unk4c04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unk4c04`]
module"]
#[doc(alias = "UNK4C04")]
pub type Unk4c04 = crate::Reg<unk4c04::Unk4c04Spec>;
#[doc = "Unknown Register 0x42004C04"]
pub mod unk4c04;
#[doc = "CTRLC (rw) register accessor: Control C\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlc`]
module"]
#[doc(alias = "CTRLC")]
pub type Ctrlc = crate::Reg<ctrlc::CtrlcSpec>;
#[doc = "Control C"]
pub mod ctrlc;
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`]
module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "FREQCTRL (rw) register accessor: Frequency Control\n\nYou can [`read`](crate::Reg::read) this register and get [`freqctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freqctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqctrl`]
module"]
#[doc(alias = "FREQCTRL")]
pub type Freqctrl = crate::Reg<freqctrl::FreqctrlSpec>;
#[doc = "Frequency Control"]
pub mod freqctrl;
#[doc = "CONVCTRL (rw) register accessor: Conversion control\n\nYou can [`read`](crate::Reg::read) this register and get [`convctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`convctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@convctrl`]
module"]
#[doc(alias = "CONVCTRL")]
pub type Convctrl = crate::Reg<convctrl::ConvctrlSpec>;
#[doc = "Conversion control"]
pub mod convctrl;
#[doc = "YSELECT (rw) register accessor: Select Y line\n\nYou can [`read`](crate::Reg::read) this register and get [`yselect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`yselect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@yselect`]
module"]
#[doc(alias = "YSELECT")]
pub type Yselect = crate::Reg<yselect::YselectSpec>;
#[doc = "Select Y line"]
pub mod yselect;
#[doc = "XSELECT (rw) register accessor: Select X line\n\nYou can [`read`](crate::Reg::read) this register and get [`xselect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xselect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xselect`]
module"]
#[doc(alias = "XSELECT")]
pub type Xselect = crate::Reg<xselect::XselectSpec>;
#[doc = "Select X line"]
pub mod xselect;
#[doc = "YSELECTEN (rw) register accessor: Enable Y lines\n\nYou can [`read`](crate::Reg::read) this register and get [`yselecten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`yselecten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@yselecten`]
module"]
#[doc(alias = "YSELECTEN")]
pub type Yselecten = crate::Reg<yselecten::YselectenSpec>;
#[doc = "Enable Y lines"]
pub mod yselecten;
#[doc = "XSELECTEN (rw) register accessor: Enable X lines\n\nYou can [`read`](crate::Reg::read) this register and get [`xselecten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xselecten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xselecten`]
module"]
#[doc(alias = "XSELECTEN")]
pub type Xselecten = crate::Reg<xselecten::XselectenSpec>;
#[doc = "Enable X lines"]
pub mod xselecten;
#[doc = "COMPCAP (rw) register accessor: Compensation capacitor value\n\nYou can [`read`](crate::Reg::read) this register and get [`compcap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compcap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compcap`]
module"]
#[doc(alias = "COMPCAP")]
pub type Compcap = crate::Reg<compcap::CompcapSpec>;
#[doc = "Compensation capacitor value"]
pub mod compcap;
#[doc = "INTCAP (rw) register accessor: Internal capacitor value\n\nYou can [`read`](crate::Reg::read) this register and get [`intcap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intcap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intcap`]
module"]
#[doc(alias = "INTCAP")]
pub type Intcap = crate::Reg<intcap::IntcapSpec>;
#[doc = "Internal capacitor value"]
pub mod intcap;
#[doc = "SERRES (rw) register accessor: Series resistor for PTC measurements\n\nYou can [`read`](crate::Reg::read) this register and get [`serres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`serres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@serres`]
module"]
#[doc(alias = "SERRES")]
pub type Serres = crate::Reg<serres::SerresSpec>;
#[doc = "Series resistor for PTC measurements"]
pub mod serres;
#[doc = "RESULT (r) register accessor: Conversion result\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result`]
module"]
#[doc(alias = "RESULT")]
pub type Result = crate::Reg<result::ResultSpec>;
#[doc = "Conversion result"]
pub mod result;
#[doc = "BURSTMODE (rw) register accessor: Enable burst or clear to send low power mode\n\nYou can [`read`](crate::Reg::read) this register and get [`burstmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`burstmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@burstmode`]
module"]
#[doc(alias = "BURSTMODE")]
pub type Burstmode = crate::Reg<burstmode::BurstmodeSpec>;
#[doc = "Enable burst or clear to send low power mode"]
pub mod burstmode;
#[doc = "WCOMODE (rw) register accessor: Set WCO mode\n\nYou can [`read`](crate::Reg::read) this register and get [`wcomode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcomode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wcomode`]
module"]
#[doc(alias = "WCOMODE")]
pub type Wcomode = crate::Reg<wcomode::WcomodeSpec>;
#[doc = "Set WCO mode"]
pub mod wcomode;
#[doc = "WCOTHRESHOLDAL (rw) register accessor: Set lower WCO threshold for port A\n\nYou can [`read`](crate::Reg::read) this register and get [`wcothresholdal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcothresholdal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wcothresholdal`]
module"]
#[doc(alias = "WCOTHRESHOLDAL")]
pub type Wcothresholdal = crate::Reg<wcothresholdal::WcothresholdalSpec>;
#[doc = "Set lower WCO threshold for port A"]
pub mod wcothresholdal;
#[doc = "WCOTHRESHOLDAH (rw) register accessor: Set upper WCO threshold for port A\n\nYou can [`read`](crate::Reg::read) this register and get [`wcothresholdah::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcothresholdah::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wcothresholdah`]
module"]
#[doc(alias = "WCOTHRESHOLDAH")]
pub type Wcothresholdah = crate::Reg<wcothresholdah::WcothresholdahSpec>;
#[doc = "Set upper WCO threshold for port A"]
pub mod wcothresholdah;
#[doc = "WCOTHRESHOLDBL (rw) register accessor: Set lower WCO threshold for port B\n\nYou can [`read`](crate::Reg::read) this register and get [`wcothresholdbl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcothresholdbl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wcothresholdbl`]
module"]
#[doc(alias = "WCOTHRESHOLDBL")]
pub type Wcothresholdbl = crate::Reg<wcothresholdbl::WcothresholdblSpec>;
#[doc = "Set lower WCO threshold for port B"]
pub mod wcothresholdbl;
#[doc = "WCOTHRESHOLDBH (rw) register accessor: Set upper WCO threshold for port B\n\nYou can [`read`](crate::Reg::read) this register and get [`wcothresholdbh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcothresholdbh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wcothresholdbh`]
module"]
#[doc(alias = "WCOTHRESHOLDBH")]
pub type Wcothresholdbh = crate::Reg<wcothresholdbh::WcothresholdbhSpec>;
#[doc = "Set upper WCO threshold for port B"]
pub mod wcothresholdbh;
