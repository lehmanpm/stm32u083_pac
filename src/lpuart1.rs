#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_lpuart_cr: [u8; 0x04],
    lpuart_cr2: LpuartCr2,
    lpuart_cr3: LpuartCr3,
    lpuart_brr: LpuartBrr,
    _reserved4: [u8; 0x08],
    lpuart_rqr: LpuartRqr,
    _reserved_5_lpuart_: [u8; 0x04],
    lpuart_icr: LpuartIcr,
    lpuart_rdr: LpuartRdr,
    lpuart_tdr: LpuartTdr,
    lpuart_presc: LpuartPresc,
}
impl RegisterBlock {
    #[doc = "0x00 - LPUART control register 1"]
    #[inline(always)]
    pub const fn lpuart_cr1_alternate(&self) -> &LpuartCr1Alternate {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - LPUART control register 1"]
    #[inline(always)]
    pub const fn lpuart_cr1(&self) -> &LpuartCr1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - LPUART control register 2"]
    #[inline(always)]
    pub const fn lpuart_cr2(&self) -> &LpuartCr2 {
        &self.lpuart_cr2
    }
    #[doc = "0x08 - LPUART control register 3"]
    #[inline(always)]
    pub const fn lpuart_cr3(&self) -> &LpuartCr3 {
        &self.lpuart_cr3
    }
    #[doc = "0x0c - LPUART baud rate register"]
    #[inline(always)]
    pub const fn lpuart_brr(&self) -> &LpuartBrr {
        &self.lpuart_brr
    }
    #[doc = "0x18 - LPUART request register"]
    #[inline(always)]
    pub const fn lpuart_rqr(&self) -> &LpuartRqr {
        &self.lpuart_rqr
    }
    #[doc = "0x1c - LPUART interrupt and status register"]
    #[inline(always)]
    pub const fn lpuart_isr_alternate(&self) -> &LpuartIsrAlternate {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - LPUART interrupt and status register"]
    #[inline(always)]
    pub const fn lpuart_isr(&self) -> &LpuartIsr {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - LPUART interrupt flag clear register"]
    #[inline(always)]
    pub const fn lpuart_icr(&self) -> &LpuartIcr {
        &self.lpuart_icr
    }
    #[doc = "0x24 - LPUART receive data register"]
    #[inline(always)]
    pub const fn lpuart_rdr(&self) -> &LpuartRdr {
        &self.lpuart_rdr
    }
    #[doc = "0x28 - LPUART transmit data register"]
    #[inline(always)]
    pub const fn lpuart_tdr(&self) -> &LpuartTdr {
        &self.lpuart_tdr
    }
    #[doc = "0x2c - LPUART prescaler register"]
    #[inline(always)]
    pub const fn lpuart_presc(&self) -> &LpuartPresc {
        &self.lpuart_presc
    }
}
#[doc = "LPUART_CR1 (rw) register accessor: LPUART control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpuart_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpuart_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpuart_cr1`]
module"]
#[doc(alias = "LPUART_CR1")]
pub type LpuartCr1 = crate::Reg<lpuart_cr1::LpuartCr1Spec>;
#[doc = "LPUART control register 1"]
pub mod lpuart_cr1;
#[doc = "LPUART_CR1_ALTERNATE (rw) register accessor: LPUART control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpuart_cr1_alternate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpuart_cr1_alternate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpuart_cr1_alternate`]
module"]
#[doc(alias = "LPUART_CR1_ALTERNATE")]
pub type LpuartCr1Alternate = crate::Reg<lpuart_cr1_alternate::LpuartCr1AlternateSpec>;
#[doc = "LPUART control register 1"]
pub mod lpuart_cr1_alternate;
#[doc = "LPUART_CR2 (rw) register accessor: LPUART control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpuart_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpuart_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpuart_cr2`]
module"]
#[doc(alias = "LPUART_CR2")]
pub type LpuartCr2 = crate::Reg<lpuart_cr2::LpuartCr2Spec>;
#[doc = "LPUART control register 2"]
pub mod lpuart_cr2;
#[doc = "LPUART_CR3 (rw) register accessor: LPUART control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpuart_cr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpuart_cr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpuart_cr3`]
module"]
#[doc(alias = "LPUART_CR3")]
pub type LpuartCr3 = crate::Reg<lpuart_cr3::LpuartCr3Spec>;
#[doc = "LPUART control register 3"]
pub mod lpuart_cr3;
#[doc = "LPUART_BRR (rw) register accessor: LPUART baud rate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpuart_brr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpuart_brr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpuart_brr`]
module"]
#[doc(alias = "LPUART_BRR")]
pub type LpuartBrr = crate::Reg<lpuart_brr::LpuartBrrSpec>;
#[doc = "LPUART baud rate register"]
pub mod lpuart_brr;
#[doc = "LPUART_RQR (w) register accessor: LPUART request register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpuart_rqr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpuart_rqr`]
module"]
#[doc(alias = "LPUART_RQR")]
pub type LpuartRqr = crate::Reg<lpuart_rqr::LpuartRqrSpec>;
#[doc = "LPUART request register"]
pub mod lpuart_rqr;
#[doc = "LPUART_ISR (r) register accessor: LPUART interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpuart_isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpuart_isr`]
module"]
#[doc(alias = "LPUART_ISR")]
pub type LpuartIsr = crate::Reg<lpuart_isr::LpuartIsrSpec>;
#[doc = "LPUART interrupt and status register"]
pub mod lpuart_isr;
#[doc = "LPUART_ISR_ALTERNATE (r) register accessor: LPUART interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpuart_isr_alternate::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpuart_isr_alternate`]
module"]
#[doc(alias = "LPUART_ISR_ALTERNATE")]
pub type LpuartIsrAlternate = crate::Reg<lpuart_isr_alternate::LpuartIsrAlternateSpec>;
#[doc = "LPUART interrupt and status register"]
pub mod lpuart_isr_alternate;
#[doc = "LPUART_ICR (w) register accessor: LPUART interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpuart_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpuart_icr`]
module"]
#[doc(alias = "LPUART_ICR")]
pub type LpuartIcr = crate::Reg<lpuart_icr::LpuartIcrSpec>;
#[doc = "LPUART interrupt flag clear register"]
pub mod lpuart_icr;
#[doc = "LPUART_RDR (r) register accessor: LPUART receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpuart_rdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpuart_rdr`]
module"]
#[doc(alias = "LPUART_RDR")]
pub type LpuartRdr = crate::Reg<lpuart_rdr::LpuartRdrSpec>;
#[doc = "LPUART receive data register"]
pub mod lpuart_rdr;
#[doc = "LPUART_TDR (rw) register accessor: LPUART transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpuart_tdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpuart_tdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpuart_tdr`]
module"]
#[doc(alias = "LPUART_TDR")]
pub type LpuartTdr = crate::Reg<lpuart_tdr::LpuartTdrSpec>;
#[doc = "LPUART transmit data register"]
pub mod lpuart_tdr;
#[doc = "LPUART_PRESC (rw) register accessor: LPUART prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpuart_presc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpuart_presc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpuart_presc`]
module"]
#[doc(alias = "LPUART_PRESC")]
pub type LpuartPresc = crate::Reg<lpuart_presc::LpuartPrescSpec>;
#[doc = "LPUART prescaler register"]
pub mod lpuart_presc;
