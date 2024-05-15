#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    comp1_csr: Comp1Csr,
    comp2_csr: Comp2Csr,
}
impl RegisterBlock {
    #[doc = "0x00 - Comparator 1 control and status register"]
    #[inline(always)]
    pub const fn comp1_csr(&self) -> &Comp1Csr {
        &self.comp1_csr
    }
    #[doc = "0x04 - Comparator 2 control and status register"]
    #[inline(always)]
    pub const fn comp2_csr(&self) -> &Comp2Csr {
        &self.comp2_csr
    }
}
#[doc = "COMP1_CSR (rw) register accessor: Comparator 1 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_csr`]
module"]
#[doc(alias = "COMP1_CSR")]
pub type Comp1Csr = crate::Reg<comp1_csr::Comp1CsrSpec>;
#[doc = "Comparator 1 control and status register"]
pub mod comp1_csr;
#[doc = "COMP2_CSR (rw) register accessor: Comparator 2 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp2_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp2_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp2_csr`]
module"]
#[doc(alias = "COMP2_CSR")]
pub type Comp2Csr = crate::Reg<comp2_csr::Comp2CsrSpec>;
#[doc = "Comparator 2 control and status register"]
pub mod comp2_csr;
