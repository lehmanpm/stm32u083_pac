#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    opamp_csr: OpampCsr,
    opamp_otr: OpampOtr,
    opamp_lpotr: OpampLpotr,
}
impl RegisterBlock {
    #[doc = "0x00 - OPAMP control/status register"]
    #[inline(always)]
    pub const fn opamp_csr(&self) -> &OpampCsr {
        &self.opamp_csr
    }
    #[doc = "0x04 - OPAMP offset trimming register in normal mode"]
    #[inline(always)]
    pub const fn opamp_otr(&self) -> &OpampOtr {
        &self.opamp_otr
    }
    #[doc = "0x08 - OPAMP offset trimming register in low-power mode"]
    #[inline(always)]
    pub const fn opamp_lpotr(&self) -> &OpampLpotr {
        &self.opamp_lpotr
    }
}
#[doc = "OPAMP_CSR (rw) register accessor: OPAMP control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp_csr`]
module"]
#[doc(alias = "OPAMP_CSR")]
pub type OpampCsr = crate::Reg<opamp_csr::OpampCsrSpec>;
#[doc = "OPAMP control/status register"]
pub mod opamp_csr;
#[doc = "OPAMP_OTR (rw) register accessor: OPAMP offset trimming register in normal mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp_otr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp_otr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp_otr`]
module"]
#[doc(alias = "OPAMP_OTR")]
pub type OpampOtr = crate::Reg<opamp_otr::OpampOtrSpec>;
#[doc = "OPAMP offset trimming register in normal mode"]
pub mod opamp_otr;
#[doc = "OPAMP_LPOTR (rw) register accessor: OPAMP offset trimming register in low-power mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp_lpotr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp_lpotr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp_lpotr`]
module"]
#[doc(alias = "OPAMP_LPOTR")]
pub type OpampLpotr = crate::Reg<opamp_lpotr::OpampLpotrSpec>;
#[doc = "OPAMP offset trimming register in low-power mode"]
pub mod opamp_lpotr;
