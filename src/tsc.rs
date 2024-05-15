#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tsc_cr: TscCr,
    tsc_ier: TscIer,
    tsc_icr: TscIcr,
    tsc_isr: TscIsr,
    tsc_iohcr: TscIohcr,
    _reserved5: [u8; 0x04],
    tsc_ioascr: TscIoascr,
    _reserved6: [u8; 0x04],
    tsc_ioscr: TscIoscr,
    _reserved7: [u8; 0x04],
    tsc_ioccr: TscIoccr,
    _reserved8: [u8; 0x04],
    tsc_iogcsr: TscIogcsr,
    tsc_iog1cr: TscIog1cr,
    tsc_iog2cr: TscIog2cr,
    tsc_iog3cr: TscIog3cr,
    tsc_iog4cr: TscIog4cr,
    tsc_iog5cr: TscIog5cr,
    tsc_iog6cr: TscIog6cr,
    tsc_iog7cr: TscIog7cr,
}
impl RegisterBlock {
    #[doc = "0x00 - TSC control register"]
    #[inline(always)]
    pub const fn tsc_cr(&self) -> &TscCr {
        &self.tsc_cr
    }
    #[doc = "0x04 - TSC interrupt enable register"]
    #[inline(always)]
    pub const fn tsc_ier(&self) -> &TscIer {
        &self.tsc_ier
    }
    #[doc = "0x08 - TSC interrupt clear register"]
    #[inline(always)]
    pub const fn tsc_icr(&self) -> &TscIcr {
        &self.tsc_icr
    }
    #[doc = "0x0c - TSC interrupt status register"]
    #[inline(always)]
    pub const fn tsc_isr(&self) -> &TscIsr {
        &self.tsc_isr
    }
    #[doc = "0x10 - TSC I/O hysteresis control register"]
    #[inline(always)]
    pub const fn tsc_iohcr(&self) -> &TscIohcr {
        &self.tsc_iohcr
    }
    #[doc = "0x18 - TSC I/O analog switch control register"]
    #[inline(always)]
    pub const fn tsc_ioascr(&self) -> &TscIoascr {
        &self.tsc_ioascr
    }
    #[doc = "0x20 - TSC I/O sampling control register"]
    #[inline(always)]
    pub const fn tsc_ioscr(&self) -> &TscIoscr {
        &self.tsc_ioscr
    }
    #[doc = "0x28 - TSC I/O channel control register"]
    #[inline(always)]
    pub const fn tsc_ioccr(&self) -> &TscIoccr {
        &self.tsc_ioccr
    }
    #[doc = "0x30 - TSC I/O group control status register"]
    #[inline(always)]
    pub const fn tsc_iogcsr(&self) -> &TscIogcsr {
        &self.tsc_iogcsr
    }
    #[doc = "0x34 - TSC I/O group 1 counter register"]
    #[inline(always)]
    pub const fn tsc_iog1cr(&self) -> &TscIog1cr {
        &self.tsc_iog1cr
    }
    #[doc = "0x38 - TSC I/O group 2 counter register"]
    #[inline(always)]
    pub const fn tsc_iog2cr(&self) -> &TscIog2cr {
        &self.tsc_iog2cr
    }
    #[doc = "0x3c - TSC I/O group 3 counter register"]
    #[inline(always)]
    pub const fn tsc_iog3cr(&self) -> &TscIog3cr {
        &self.tsc_iog3cr
    }
    #[doc = "0x40 - TSC I/O group 4 counter register"]
    #[inline(always)]
    pub const fn tsc_iog4cr(&self) -> &TscIog4cr {
        &self.tsc_iog4cr
    }
    #[doc = "0x44 - TSC I/O group 5 counter register"]
    #[inline(always)]
    pub const fn tsc_iog5cr(&self) -> &TscIog5cr {
        &self.tsc_iog5cr
    }
    #[doc = "0x48 - TSC I/O group 6 counter register"]
    #[inline(always)]
    pub const fn tsc_iog6cr(&self) -> &TscIog6cr {
        &self.tsc_iog6cr
    }
    #[doc = "0x4c - TSC I/O group 7 counter register"]
    #[inline(always)]
    pub const fn tsc_iog7cr(&self) -> &TscIog7cr {
        &self.tsc_iog7cr
    }
}
#[doc = "TSC_CR (rw) register accessor: TSC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsc_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsc_cr`]
module"]
#[doc(alias = "TSC_CR")]
pub type TscCr = crate::Reg<tsc_cr::TscCrSpec>;
#[doc = "TSC control register"]
pub mod tsc_cr;
#[doc = "TSC_IER (rw) register accessor: TSC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsc_ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsc_ier`]
module"]
#[doc(alias = "TSC_IER")]
pub type TscIer = crate::Reg<tsc_ier::TscIerSpec>;
#[doc = "TSC interrupt enable register"]
pub mod tsc_ier;
#[doc = "TSC_ICR (rw) register accessor: TSC interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsc_icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsc_icr`]
module"]
#[doc(alias = "TSC_ICR")]
pub type TscIcr = crate::Reg<tsc_icr::TscIcrSpec>;
#[doc = "TSC interrupt clear register"]
pub mod tsc_icr;
#[doc = "TSC_ISR (r) register accessor: TSC interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsc_isr`]
module"]
#[doc(alias = "TSC_ISR")]
pub type TscIsr = crate::Reg<tsc_isr::TscIsrSpec>;
#[doc = "TSC interrupt status register"]
pub mod tsc_isr;
#[doc = "TSC_IOHCR (rw) register accessor: TSC I/O hysteresis control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_iohcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsc_iohcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsc_iohcr`]
module"]
#[doc(alias = "TSC_IOHCR")]
pub type TscIohcr = crate::Reg<tsc_iohcr::TscIohcrSpec>;
#[doc = "TSC I/O hysteresis control register"]
pub mod tsc_iohcr;
#[doc = "TSC_IOASCR (rw) register accessor: TSC I/O analog switch control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_ioascr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsc_ioascr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsc_ioascr`]
module"]
#[doc(alias = "TSC_IOASCR")]
pub type TscIoascr = crate::Reg<tsc_ioascr::TscIoascrSpec>;
#[doc = "TSC I/O analog switch control register"]
pub mod tsc_ioascr;
#[doc = "TSC_IOSCR (rw) register accessor: TSC I/O sampling control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_ioscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsc_ioscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsc_ioscr`]
module"]
#[doc(alias = "TSC_IOSCR")]
pub type TscIoscr = crate::Reg<tsc_ioscr::TscIoscrSpec>;
#[doc = "TSC I/O sampling control register"]
pub mod tsc_ioscr;
#[doc = "TSC_IOCCR (rw) register accessor: TSC I/O channel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_ioccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsc_ioccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsc_ioccr`]
module"]
#[doc(alias = "TSC_IOCCR")]
pub type TscIoccr = crate::Reg<tsc_ioccr::TscIoccrSpec>;
#[doc = "TSC I/O channel control register"]
pub mod tsc_ioccr;
#[doc = "TSC_IOGCSR (rw) register accessor: TSC I/O group control status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_iogcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsc_iogcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsc_iogcsr`]
module"]
#[doc(alias = "TSC_IOGCSR")]
pub type TscIogcsr = crate::Reg<tsc_iogcsr::TscIogcsrSpec>;
#[doc = "TSC I/O group control status register"]
pub mod tsc_iogcsr;
#[doc = "TSC_IOG1CR (r) register accessor: TSC I/O group 1 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_iog1cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsc_iog1cr`]
module"]
#[doc(alias = "TSC_IOG1CR")]
pub type TscIog1cr = crate::Reg<tsc_iog1cr::TscIog1crSpec>;
#[doc = "TSC I/O group 1 counter register"]
pub mod tsc_iog1cr;
#[doc = "TSC_IOG2CR (r) register accessor: TSC I/O group 2 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_iog2cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsc_iog2cr`]
module"]
#[doc(alias = "TSC_IOG2CR")]
pub type TscIog2cr = crate::Reg<tsc_iog2cr::TscIog2crSpec>;
#[doc = "TSC I/O group 2 counter register"]
pub mod tsc_iog2cr;
#[doc = "TSC_IOG3CR (r) register accessor: TSC I/O group 3 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_iog3cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsc_iog3cr`]
module"]
#[doc(alias = "TSC_IOG3CR")]
pub type TscIog3cr = crate::Reg<tsc_iog3cr::TscIog3crSpec>;
#[doc = "TSC I/O group 3 counter register"]
pub mod tsc_iog3cr;
#[doc = "TSC_IOG4CR (r) register accessor: TSC I/O group 4 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_iog4cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsc_iog4cr`]
module"]
#[doc(alias = "TSC_IOG4CR")]
pub type TscIog4cr = crate::Reg<tsc_iog4cr::TscIog4crSpec>;
#[doc = "TSC I/O group 4 counter register"]
pub mod tsc_iog4cr;
#[doc = "TSC_IOG5CR (r) register accessor: TSC I/O group 5 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_iog5cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsc_iog5cr`]
module"]
#[doc(alias = "TSC_IOG5CR")]
pub type TscIog5cr = crate::Reg<tsc_iog5cr::TscIog5crSpec>;
#[doc = "TSC I/O group 5 counter register"]
pub mod tsc_iog5cr;
#[doc = "TSC_IOG6CR (r) register accessor: TSC I/O group 6 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_iog6cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsc_iog6cr`]
module"]
#[doc(alias = "TSC_IOG6CR")]
pub type TscIog6cr = crate::Reg<tsc_iog6cr::TscIog6crSpec>;
#[doc = "TSC I/O group 6 counter register"]
pub mod tsc_iog6cr;
#[doc = "TSC_IOG7CR (r) register accessor: TSC I/O group 7 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_iog7cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsc_iog7cr`]
module"]
#[doc(alias = "TSC_IOG7CR")]
pub type TscIog7cr = crate::Reg<tsc_iog7cr::TscIog7crSpec>;
#[doc = "TSC I/O group 7 counter register"]
pub mod tsc_iog7cr;
