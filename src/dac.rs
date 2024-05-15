#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dac_cr: DacCr,
    dac_swtrgr: DacSwtrgr,
    dac_dhr12r1: DacDhr12r1,
    dac_dhr12l1: DacDhr12l1,
    dac_dhr8r1: DacDhr8r1,
    _reserved5: [u8; 0x18],
    dac_dor1: DacDor1,
    _reserved6: [u8; 0x04],
    dac_sr: DacSr,
    dac_ccr: DacCcr,
    dac_mcr: DacMcr,
    dac_shsr1: DacShsr1,
    _reserved10: [u8; 0x04],
    dac_shhr: DacShhr,
    dac_shrr: DacShrr,
}
impl RegisterBlock {
    #[doc = "0x00 - DAC control register"]
    #[inline(always)]
    pub const fn dac_cr(&self) -> &DacCr {
        &self.dac_cr
    }
    #[doc = "0x04 - DAC software trigger register"]
    #[inline(always)]
    pub const fn dac_swtrgr(&self) -> &DacSwtrgr {
        &self.dac_swtrgr
    }
    #[doc = "0x08 - DAC channel1 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr12r1(&self) -> &DacDhr12r1 {
        &self.dac_dhr12r1
    }
    #[doc = "0x0c - DAC channel1 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr12l1(&self) -> &DacDhr12l1 {
        &self.dac_dhr12l1
    }
    #[doc = "0x10 - DAC channel1 8-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr8r1(&self) -> &DacDhr8r1 {
        &self.dac_dhr8r1
    }
    #[doc = "0x2c - DAC channel1 data output register"]
    #[inline(always)]
    pub const fn dac_dor1(&self) -> &DacDor1 {
        &self.dac_dor1
    }
    #[doc = "0x34 - DAC status register"]
    #[inline(always)]
    pub const fn dac_sr(&self) -> &DacSr {
        &self.dac_sr
    }
    #[doc = "0x38 - DAC calibration control register"]
    #[inline(always)]
    pub const fn dac_ccr(&self) -> &DacCcr {
        &self.dac_ccr
    }
    #[doc = "0x3c - DAC mode control register"]
    #[inline(always)]
    pub const fn dac_mcr(&self) -> &DacMcr {
        &self.dac_mcr
    }
    #[doc = "0x40 - DAC channel1 sample and hold sample time register"]
    #[inline(always)]
    pub const fn dac_shsr1(&self) -> &DacShsr1 {
        &self.dac_shsr1
    }
    #[doc = "0x48 - DAC sample and hold time register"]
    #[inline(always)]
    pub const fn dac_shhr(&self) -> &DacShhr {
        &self.dac_shhr
    }
    #[doc = "0x4c - DAC sample and hold refresh time register"]
    #[inline(always)]
    pub const fn dac_shrr(&self) -> &DacShrr {
        &self.dac_shrr
    }
}
#[doc = "DAC_CR (rw) register accessor: DAC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_cr`]
module"]
#[doc(alias = "DAC_CR")]
pub type DacCr = crate::Reg<dac_cr::DacCrSpec>;
#[doc = "DAC control register"]
pub mod dac_cr;
#[doc = "DAC_SWTRGR (w) register accessor: DAC software trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_swtrgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_swtrgr`]
module"]
#[doc(alias = "DAC_SWTRGR")]
pub type DacSwtrgr = crate::Reg<dac_swtrgr::DacSwtrgrSpec>;
#[doc = "DAC software trigger register"]
pub mod dac_swtrgr;
#[doc = "DAC_DHR12R1 (rw) register accessor: DAC channel1 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr12r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr12r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr12r1`]
module"]
#[doc(alias = "DAC_DHR12R1")]
pub type DacDhr12r1 = crate::Reg<dac_dhr12r1::DacDhr12r1Spec>;
#[doc = "DAC channel1 12-bit right-aligned data holding register"]
pub mod dac_dhr12r1;
#[doc = "DAC_DHR12L1 (rw) register accessor: DAC channel1 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr12l1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr12l1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr12l1`]
module"]
#[doc(alias = "DAC_DHR12L1")]
pub type DacDhr12l1 = crate::Reg<dac_dhr12l1::DacDhr12l1Spec>;
#[doc = "DAC channel1 12-bit left aligned data holding register"]
pub mod dac_dhr12l1;
#[doc = "DAC_DHR8R1 (rw) register accessor: DAC channel1 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr8r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr8r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr8r1`]
module"]
#[doc(alias = "DAC_DHR8R1")]
pub type DacDhr8r1 = crate::Reg<dac_dhr8r1::DacDhr8r1Spec>;
#[doc = "DAC channel1 8-bit right aligned data holding register"]
pub mod dac_dhr8r1;
#[doc = "DAC_DOR1 (r) register accessor: DAC channel1 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dor1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dor1`]
module"]
#[doc(alias = "DAC_DOR1")]
pub type DacDor1 = crate::Reg<dac_dor1::DacDor1Spec>;
#[doc = "DAC channel1 data output register"]
pub mod dac_dor1;
#[doc = "DAC_SR (rw) register accessor: DAC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_sr`]
module"]
#[doc(alias = "DAC_SR")]
pub type DacSr = crate::Reg<dac_sr::DacSrSpec>;
#[doc = "DAC status register"]
pub mod dac_sr;
#[doc = "DAC_CCR (rw) register accessor: DAC calibration control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_ccr`]
module"]
#[doc(alias = "DAC_CCR")]
pub type DacCcr = crate::Reg<dac_ccr::DacCcrSpec>;
#[doc = "DAC calibration control register"]
pub mod dac_ccr;
#[doc = "DAC_MCR (rw) register accessor: DAC mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_mcr`]
module"]
#[doc(alias = "DAC_MCR")]
pub type DacMcr = crate::Reg<dac_mcr::DacMcrSpec>;
#[doc = "DAC mode control register"]
pub mod dac_mcr;
#[doc = "DAC_SHSR1 (rw) register accessor: DAC channel1 sample and hold sample time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_shsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_shsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_shsr1`]
module"]
#[doc(alias = "DAC_SHSR1")]
pub type DacShsr1 = crate::Reg<dac_shsr1::DacShsr1Spec>;
#[doc = "DAC channel1 sample and hold sample time register"]
pub mod dac_shsr1;
#[doc = "DAC_SHHR (rw) register accessor: DAC sample and hold time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_shhr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_shhr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_shhr`]
module"]
#[doc(alias = "DAC_SHHR")]
pub type DacShhr = crate::Reg<dac_shhr::DacShhrSpec>;
#[doc = "DAC sample and hold time register"]
pub mod dac_shhr;
#[doc = "DAC_SHRR (rw) register accessor: DAC sample and hold refresh time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_shrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_shrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_shrr`]
module"]
#[doc(alias = "DAC_SHRR")]
pub type DacShrr = crate::Reg<dac_shrr::DacShrrSpec>;
#[doc = "DAC sample and hold refresh time register"]
pub mod dac_shrr;
