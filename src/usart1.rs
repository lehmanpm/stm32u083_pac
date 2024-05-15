#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_usart_cr: [u8; 0x04],
    usart_cr2: UsartCr2,
    usart_cr3: UsartCr3,
    usart_brr: UsartBrr,
    usart_gtpr: UsartGtpr,
    usart_rtor: UsartRtor,
    usart_rqr: UsartRqr,
    _reserved_7_usart_: [u8; 0x04],
    usart_icr: UsartIcr,
    usart_rdr: UsartRdr,
    usart_tdr: UsartTdr,
    usart_presc: UsartPresc,
}
impl RegisterBlock {
    #[doc = "0x00 - USART control register 1"]
    #[inline(always)]
    pub const fn usart_cr1_alternate(&self) -> &UsartCr1Alternate {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - USART control register 1"]
    #[inline(always)]
    pub const fn usart_cr1(&self) -> &UsartCr1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - USART control register 2"]
    #[inline(always)]
    pub const fn usart_cr2(&self) -> &UsartCr2 {
        &self.usart_cr2
    }
    #[doc = "0x08 - USART control register 3"]
    #[inline(always)]
    pub const fn usart_cr3(&self) -> &UsartCr3 {
        &self.usart_cr3
    }
    #[doc = "0x0c - USART baud rate register"]
    #[inline(always)]
    pub const fn usart_brr(&self) -> &UsartBrr {
        &self.usart_brr
    }
    #[doc = "0x10 - USART guard time and prescaler register"]
    #[inline(always)]
    pub const fn usart_gtpr(&self) -> &UsartGtpr {
        &self.usart_gtpr
    }
    #[doc = "0x14 - USART receiver timeout register"]
    #[inline(always)]
    pub const fn usart_rtor(&self) -> &UsartRtor {
        &self.usart_rtor
    }
    #[doc = "0x18 - USART request register"]
    #[inline(always)]
    pub const fn usart_rqr(&self) -> &UsartRqr {
        &self.usart_rqr
    }
    #[doc = "0x1c - USART interrupt and status register"]
    #[inline(always)]
    pub const fn usart_isr_alternate(&self) -> &UsartIsrAlternate {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - USART interrupt and status register"]
    #[inline(always)]
    pub const fn usart_isr(&self) -> &UsartIsr {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - USART interrupt flag clear register"]
    #[inline(always)]
    pub const fn usart_icr(&self) -> &UsartIcr {
        &self.usart_icr
    }
    #[doc = "0x24 - USART receive data register"]
    #[inline(always)]
    pub const fn usart_rdr(&self) -> &UsartRdr {
        &self.usart_rdr
    }
    #[doc = "0x28 - USART transmit data register"]
    #[inline(always)]
    pub const fn usart_tdr(&self) -> &UsartTdr {
        &self.usart_tdr
    }
    #[doc = "0x2c - USART prescaler register"]
    #[inline(always)]
    pub const fn usart_presc(&self) -> &UsartPresc {
        &self.usart_presc
    }
}
#[doc = "USART_CR1 (rw) register accessor: USART control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_cr1`]
module"]
#[doc(alias = "USART_CR1")]
pub type UsartCr1 = crate::Reg<usart_cr1::UsartCr1Spec>;
#[doc = "USART control register 1"]
pub mod usart_cr1;
#[doc = "USART_CR1_ALTERNATE (rw) register accessor: USART control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_cr1_alternate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_cr1_alternate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_cr1_alternate`]
module"]
#[doc(alias = "USART_CR1_ALTERNATE")]
pub type UsartCr1Alternate = crate::Reg<usart_cr1_alternate::UsartCr1AlternateSpec>;
#[doc = "USART control register 1"]
pub mod usart_cr1_alternate;
#[doc = "USART_CR2 (rw) register accessor: USART control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_cr2`]
module"]
#[doc(alias = "USART_CR2")]
pub type UsartCr2 = crate::Reg<usart_cr2::UsartCr2Spec>;
#[doc = "USART control register 2"]
pub mod usart_cr2;
#[doc = "USART_CR3 (rw) register accessor: USART control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_cr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_cr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_cr3`]
module"]
#[doc(alias = "USART_CR3")]
pub type UsartCr3 = crate::Reg<usart_cr3::UsartCr3Spec>;
#[doc = "USART control register 3"]
pub mod usart_cr3;
#[doc = "USART_BRR (rw) register accessor: USART baud rate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_brr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_brr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_brr`]
module"]
#[doc(alias = "USART_BRR")]
pub type UsartBrr = crate::Reg<usart_brr::UsartBrrSpec>;
#[doc = "USART baud rate register"]
pub mod usart_brr;
#[doc = "USART_GTPR (rw) register accessor: USART guard time and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_gtpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_gtpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_gtpr`]
module"]
#[doc(alias = "USART_GTPR")]
pub type UsartGtpr = crate::Reg<usart_gtpr::UsartGtprSpec>;
#[doc = "USART guard time and prescaler register"]
pub mod usart_gtpr;
#[doc = "USART_RTOR (rw) register accessor: USART receiver timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_rtor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_rtor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_rtor`]
module"]
#[doc(alias = "USART_RTOR")]
pub type UsartRtor = crate::Reg<usart_rtor::UsartRtorSpec>;
#[doc = "USART receiver timeout register"]
pub mod usart_rtor;
#[doc = "USART_RQR (w) register accessor: USART request register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_rqr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_rqr`]
module"]
#[doc(alias = "USART_RQR")]
pub type UsartRqr = crate::Reg<usart_rqr::UsartRqrSpec>;
#[doc = "USART request register"]
pub mod usart_rqr;
#[doc = "USART_ISR (r) register accessor: USART interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_isr`]
module"]
#[doc(alias = "USART_ISR")]
pub type UsartIsr = crate::Reg<usart_isr::UsartIsrSpec>;
#[doc = "USART interrupt and status register"]
pub mod usart_isr;
#[doc = "USART_ISR_ALTERNATE (r) register accessor: USART interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_isr_alternate::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_isr_alternate`]
module"]
#[doc(alias = "USART_ISR_ALTERNATE")]
pub type UsartIsrAlternate = crate::Reg<usart_isr_alternate::UsartIsrAlternateSpec>;
#[doc = "USART interrupt and status register"]
pub mod usart_isr_alternate;
#[doc = "USART_ICR (w) register accessor: USART interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_icr`]
module"]
#[doc(alias = "USART_ICR")]
pub type UsartIcr = crate::Reg<usart_icr::UsartIcrSpec>;
#[doc = "USART interrupt flag clear register"]
pub mod usart_icr;
#[doc = "USART_RDR (r) register accessor: USART receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_rdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_rdr`]
module"]
#[doc(alias = "USART_RDR")]
pub type UsartRdr = crate::Reg<usart_rdr::UsartRdrSpec>;
#[doc = "USART receive data register"]
pub mod usart_rdr;
#[doc = "USART_TDR (rw) register accessor: USART transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_tdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_tdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_tdr`]
module"]
#[doc(alias = "USART_TDR")]
pub type UsartTdr = crate::Reg<usart_tdr::UsartTdrSpec>;
#[doc = "USART transmit data register"]
pub mod usart_tdr;
#[doc = "USART_PRESC (rw) register accessor: USART prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_presc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_presc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_presc`]
module"]
#[doc(alias = "USART_PRESC")]
pub type UsartPresc = crate::Reg<usart_presc::UsartPrescSpec>;
#[doc = "USART prescaler register"]
pub mod usart_presc;
