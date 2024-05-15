#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tamp_cr1: TampCr1,
    tamp_cr2: TampCr2,
    tamp_cr3: TampCr3,
    tamp_fltcr: TampFltcr,
    _reserved4: [u8; 0x1c],
    tamp_ier: TampIer,
    tamp_sr: TampSr,
    tamp_misr: TampMisr,
    _reserved7: [u8; 0x04],
    tamp_scr: TampScr,
    _reserved8: [u8; 0xc0],
    tamp_bkp0r: TampBkp0r,
    tamp_bkp1r: TampBkp1r,
    tamp_bkp2r: TampBkp2r,
    tamp_bkp3r: TampBkp3r,
    tamp_bkp4r: TampBkp4r,
    tamp_bkp5r: TampBkp5r,
    tamp_bkp6r: TampBkp6r,
    tamp_bkp7r: TampBkp7r,
    tamp_bkp8r: TampBkp8r,
}
impl RegisterBlock {
    #[doc = "0x00 - TAMP control register 1"]
    #[inline(always)]
    pub const fn tamp_cr1(&self) -> &TampCr1 {
        &self.tamp_cr1
    }
    #[doc = "0x04 - TAMP control register 2"]
    #[inline(always)]
    pub const fn tamp_cr2(&self) -> &TampCr2 {
        &self.tamp_cr2
    }
    #[doc = "0x08 - TAMP control register 3"]
    #[inline(always)]
    pub const fn tamp_cr3(&self) -> &TampCr3 {
        &self.tamp_cr3
    }
    #[doc = "0x0c - TAMP filter control register"]
    #[inline(always)]
    pub const fn tamp_fltcr(&self) -> &TampFltcr {
        &self.tamp_fltcr
    }
    #[doc = "0x2c - TAMP interrupt enable register"]
    #[inline(always)]
    pub const fn tamp_ier(&self) -> &TampIer {
        &self.tamp_ier
    }
    #[doc = "0x30 - TAMP status register"]
    #[inline(always)]
    pub const fn tamp_sr(&self) -> &TampSr {
        &self.tamp_sr
    }
    #[doc = "0x34 - TAMP masked interrupt status register"]
    #[inline(always)]
    pub const fn tamp_misr(&self) -> &TampMisr {
        &self.tamp_misr
    }
    #[doc = "0x3c - TAMP status clear register"]
    #[inline(always)]
    pub const fn tamp_scr(&self) -> &TampScr {
        &self.tamp_scr
    }
    #[doc = "0x100 - TAMP backup 0 register"]
    #[inline(always)]
    pub const fn tamp_bkp0r(&self) -> &TampBkp0r {
        &self.tamp_bkp0r
    }
    #[doc = "0x104 - TAMP backup 1 register"]
    #[inline(always)]
    pub const fn tamp_bkp1r(&self) -> &TampBkp1r {
        &self.tamp_bkp1r
    }
    #[doc = "0x108 - TAMP backup 2 register"]
    #[inline(always)]
    pub const fn tamp_bkp2r(&self) -> &TampBkp2r {
        &self.tamp_bkp2r
    }
    #[doc = "0x10c - TAMP backup 3 register"]
    #[inline(always)]
    pub const fn tamp_bkp3r(&self) -> &TampBkp3r {
        &self.tamp_bkp3r
    }
    #[doc = "0x110 - TAMP backup 4 register"]
    #[inline(always)]
    pub const fn tamp_bkp4r(&self) -> &TampBkp4r {
        &self.tamp_bkp4r
    }
    #[doc = "0x114 - TAMP backup 5 register"]
    #[inline(always)]
    pub const fn tamp_bkp5r(&self) -> &TampBkp5r {
        &self.tamp_bkp5r
    }
    #[doc = "0x118 - TAMP backup 6 register"]
    #[inline(always)]
    pub const fn tamp_bkp6r(&self) -> &TampBkp6r {
        &self.tamp_bkp6r
    }
    #[doc = "0x11c - TAMP backup 7 register"]
    #[inline(always)]
    pub const fn tamp_bkp7r(&self) -> &TampBkp7r {
        &self.tamp_bkp7r
    }
    #[doc = "0x120 - TAMP backup 8 register"]
    #[inline(always)]
    pub const fn tamp_bkp8r(&self) -> &TampBkp8r {
        &self.tamp_bkp8r
    }
}
#[doc = "TAMP_CR1 (rw) register accessor: TAMP control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_cr1`]
module"]
#[doc(alias = "TAMP_CR1")]
pub type TampCr1 = crate::Reg<tamp_cr1::TampCr1Spec>;
#[doc = "TAMP control register 1"]
pub mod tamp_cr1;
#[doc = "TAMP_CR2 (rw) register accessor: TAMP control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_cr2`]
module"]
#[doc(alias = "TAMP_CR2")]
pub type TampCr2 = crate::Reg<tamp_cr2::TampCr2Spec>;
#[doc = "TAMP control register 2"]
pub mod tamp_cr2;
#[doc = "TAMP_CR3 (rw) register accessor: TAMP control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_cr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_cr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_cr3`]
module"]
#[doc(alias = "TAMP_CR3")]
pub type TampCr3 = crate::Reg<tamp_cr3::TampCr3Spec>;
#[doc = "TAMP control register 3"]
pub mod tamp_cr3;
#[doc = "TAMP_FLTCR (rw) register accessor: TAMP filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_fltcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_fltcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_fltcr`]
module"]
#[doc(alias = "TAMP_FLTCR")]
pub type TampFltcr = crate::Reg<tamp_fltcr::TampFltcrSpec>;
#[doc = "TAMP filter control register"]
pub mod tamp_fltcr;
#[doc = "TAMP_IER (rw) register accessor: TAMP interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_ier`]
module"]
#[doc(alias = "TAMP_IER")]
pub type TampIer = crate::Reg<tamp_ier::TampIerSpec>;
#[doc = "TAMP interrupt enable register"]
pub mod tamp_ier;
#[doc = "TAMP_SR (r) register accessor: TAMP status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_sr`]
module"]
#[doc(alias = "TAMP_SR")]
pub type TampSr = crate::Reg<tamp_sr::TampSrSpec>;
#[doc = "TAMP status register"]
pub mod tamp_sr;
#[doc = "TAMP_MISR (r) register accessor: TAMP masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_misr`]
module"]
#[doc(alias = "TAMP_MISR")]
pub type TampMisr = crate::Reg<tamp_misr::TampMisrSpec>;
#[doc = "TAMP masked interrupt status register"]
pub mod tamp_misr;
#[doc = "TAMP_SCR (w) register accessor: TAMP status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_scr`]
module"]
#[doc(alias = "TAMP_SCR")]
pub type TampScr = crate::Reg<tamp_scr::TampScrSpec>;
#[doc = "TAMP status clear register"]
pub mod tamp_scr;
#[doc = "TAMP_BKP0R (rw) register accessor: TAMP backup 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_bkp0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_bkp0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_bkp0r`]
module"]
#[doc(alias = "TAMP_BKP0R")]
pub type TampBkp0r = crate::Reg<tamp_bkp0r::TampBkp0rSpec>;
#[doc = "TAMP backup 0 register"]
pub mod tamp_bkp0r;
#[doc = "TAMP_BKP1R (rw) register accessor: TAMP backup 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_bkp1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_bkp1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_bkp1r`]
module"]
#[doc(alias = "TAMP_BKP1R")]
pub type TampBkp1r = crate::Reg<tamp_bkp1r::TampBkp1rSpec>;
#[doc = "TAMP backup 1 register"]
pub mod tamp_bkp1r;
#[doc = "TAMP_BKP2R (rw) register accessor: TAMP backup 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_bkp2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_bkp2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_bkp2r`]
module"]
#[doc(alias = "TAMP_BKP2R")]
pub type TampBkp2r = crate::Reg<tamp_bkp2r::TampBkp2rSpec>;
#[doc = "TAMP backup 2 register"]
pub mod tamp_bkp2r;
#[doc = "TAMP_BKP3R (rw) register accessor: TAMP backup 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_bkp3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_bkp3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_bkp3r`]
module"]
#[doc(alias = "TAMP_BKP3R")]
pub type TampBkp3r = crate::Reg<tamp_bkp3r::TampBkp3rSpec>;
#[doc = "TAMP backup 3 register"]
pub mod tamp_bkp3r;
#[doc = "TAMP_BKP4R (rw) register accessor: TAMP backup 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_bkp4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_bkp4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_bkp4r`]
module"]
#[doc(alias = "TAMP_BKP4R")]
pub type TampBkp4r = crate::Reg<tamp_bkp4r::TampBkp4rSpec>;
#[doc = "TAMP backup 4 register"]
pub mod tamp_bkp4r;
#[doc = "TAMP_BKP5R (rw) register accessor: TAMP backup 5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_bkp5r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_bkp5r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_bkp5r`]
module"]
#[doc(alias = "TAMP_BKP5R")]
pub type TampBkp5r = crate::Reg<tamp_bkp5r::TampBkp5rSpec>;
#[doc = "TAMP backup 5 register"]
pub mod tamp_bkp5r;
#[doc = "TAMP_BKP6R (rw) register accessor: TAMP backup 6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_bkp6r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_bkp6r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_bkp6r`]
module"]
#[doc(alias = "TAMP_BKP6R")]
pub type TampBkp6r = crate::Reg<tamp_bkp6r::TampBkp6rSpec>;
#[doc = "TAMP backup 6 register"]
pub mod tamp_bkp6r;
#[doc = "TAMP_BKP7R (rw) register accessor: TAMP backup 7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_bkp7r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_bkp7r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_bkp7r`]
module"]
#[doc(alias = "TAMP_BKP7R")]
pub type TampBkp7r = crate::Reg<tamp_bkp7r::TampBkp7rSpec>;
#[doc = "TAMP backup 7 register"]
pub mod tamp_bkp7r;
#[doc = "TAMP_BKP8R (rw) register accessor: TAMP backup 8 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_bkp8r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_bkp8r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_bkp8r`]
module"]
#[doc(alias = "TAMP_BKP8R")]
pub type TampBkp8r = crate::Reg<tamp_bkp8r::TampBkp8rSpec>;
#[doc = "TAMP backup 8 register"]
pub mod tamp_bkp8r;
