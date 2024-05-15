#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rng_cr: RngCr,
    rng_sr: RngSr,
    rng_dr: RngDr,
    _reserved3: [u8; 0x04],
    rng_htcr: RngHtcr,
}
impl RegisterBlock {
    #[doc = "0x00 - RNG control register"]
    #[inline(always)]
    pub const fn rng_cr(&self) -> &RngCr {
        &self.rng_cr
    }
    #[doc = "0x04 - RNG status register"]
    #[inline(always)]
    pub const fn rng_sr(&self) -> &RngSr {
        &self.rng_sr
    }
    #[doc = "0x08 - RNG data register"]
    #[inline(always)]
    pub const fn rng_dr(&self) -> &RngDr {
        &self.rng_dr
    }
    #[doc = "0x10 - RNG health test control register"]
    #[inline(always)]
    pub const fn rng_htcr(&self) -> &RngHtcr {
        &self.rng_htcr
    }
}
#[doc = "RNG_CR (rw) register accessor: RNG control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rng_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_cr`]
module"]
#[doc(alias = "RNG_CR")]
pub type RngCr = crate::Reg<rng_cr::RngCrSpec>;
#[doc = "RNG control register"]
pub mod rng_cr;
#[doc = "RNG_SR (rw) register accessor: RNG status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rng_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_sr`]
module"]
#[doc(alias = "RNG_SR")]
pub type RngSr = crate::Reg<rng_sr::RngSrSpec>;
#[doc = "RNG status register"]
pub mod rng_sr;
#[doc = "RNG_DR (r) register accessor: RNG data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_dr`]
module"]
#[doc(alias = "RNG_DR")]
pub type RngDr = crate::Reg<rng_dr::RngDrSpec>;
#[doc = "RNG data register"]
pub mod rng_dr;
#[doc = "RNG_HTCR (rw) register accessor: RNG health test control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_htcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rng_htcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_htcr`]
module"]
#[doc(alias = "RNG_HTCR")]
pub type RngHtcr = crate::Reg<rng_htcr::RngHtcrSpec>;
#[doc = "RNG health test control register"]
pub mod rng_htcr;
