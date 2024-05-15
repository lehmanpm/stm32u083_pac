#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2c_cr1: I2cCr1,
    i2c_cr2: I2cCr2,
    i2c_oar1: I2cOar1,
    i2c_oar2: I2cOar2,
    i2c_timingr: I2cTimingr,
    i2c_timeoutr: I2cTimeoutr,
    i2c_isr: I2cIsr,
    i2c_icr: I2cIcr,
    i2c_pecr: I2cPecr,
    i2c_rxdr: I2cRxdr,
    i2c_txdr: I2cTxdr,
}
impl RegisterBlock {
    #[doc = "0x00 - I2C control register 1"]
    #[inline(always)]
    pub const fn i2c_cr1(&self) -> &I2cCr1 {
        &self.i2c_cr1
    }
    #[doc = "0x04 - I2C control register 2"]
    #[inline(always)]
    pub const fn i2c_cr2(&self) -> &I2cCr2 {
        &self.i2c_cr2
    }
    #[doc = "0x08 - I2C own address 1 register"]
    #[inline(always)]
    pub const fn i2c_oar1(&self) -> &I2cOar1 {
        &self.i2c_oar1
    }
    #[doc = "0x0c - I2C own address 2 register"]
    #[inline(always)]
    pub const fn i2c_oar2(&self) -> &I2cOar2 {
        &self.i2c_oar2
    }
    #[doc = "0x10 - I2C timing register"]
    #[inline(always)]
    pub const fn i2c_timingr(&self) -> &I2cTimingr {
        &self.i2c_timingr
    }
    #[doc = "0x14 - I2C timeout register"]
    #[inline(always)]
    pub const fn i2c_timeoutr(&self) -> &I2cTimeoutr {
        &self.i2c_timeoutr
    }
    #[doc = "0x18 - I2C interrupt and status register"]
    #[inline(always)]
    pub const fn i2c_isr(&self) -> &I2cIsr {
        &self.i2c_isr
    }
    #[doc = "0x1c - I2C interrupt clear register"]
    #[inline(always)]
    pub const fn i2c_icr(&self) -> &I2cIcr {
        &self.i2c_icr
    }
    #[doc = "0x20 - I2C PEC register"]
    #[inline(always)]
    pub const fn i2c_pecr(&self) -> &I2cPecr {
        &self.i2c_pecr
    }
    #[doc = "0x24 - I2C receive data register"]
    #[inline(always)]
    pub const fn i2c_rxdr(&self) -> &I2cRxdr {
        &self.i2c_rxdr
    }
    #[doc = "0x28 - I2C transmit data register"]
    #[inline(always)]
    pub const fn i2c_txdr(&self) -> &I2cTxdr {
        &self.i2c_txdr
    }
}
#[doc = "I2C_CR1 (rw) register accessor: I2C control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_cr1`]
module"]
#[doc(alias = "I2C_CR1")]
pub type I2cCr1 = crate::Reg<i2c_cr1::I2cCr1Spec>;
#[doc = "I2C control register 1"]
pub mod i2c_cr1;
#[doc = "I2C_CR2 (rw) register accessor: I2C control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_cr2`]
module"]
#[doc(alias = "I2C_CR2")]
pub type I2cCr2 = crate::Reg<i2c_cr2::I2cCr2Spec>;
#[doc = "I2C control register 2"]
pub mod i2c_cr2;
#[doc = "I2C_OAR1 (rw) register accessor: I2C own address 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_oar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_oar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_oar1`]
module"]
#[doc(alias = "I2C_OAR1")]
pub type I2cOar1 = crate::Reg<i2c_oar1::I2cOar1Spec>;
#[doc = "I2C own address 1 register"]
pub mod i2c_oar1;
#[doc = "I2C_OAR2 (rw) register accessor: I2C own address 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_oar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_oar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_oar2`]
module"]
#[doc(alias = "I2C_OAR2")]
pub type I2cOar2 = crate::Reg<i2c_oar2::I2cOar2Spec>;
#[doc = "I2C own address 2 register"]
pub mod i2c_oar2;
#[doc = "I2C_TIMINGR (rw) register accessor: I2C timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_timingr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_timingr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_timingr`]
module"]
#[doc(alias = "I2C_TIMINGR")]
pub type I2cTimingr = crate::Reg<i2c_timingr::I2cTimingrSpec>;
#[doc = "I2C timing register"]
pub mod i2c_timingr;
#[doc = "I2C_TIMEOUTR (rw) register accessor: I2C timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_timeoutr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_timeoutr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_timeoutr`]
module"]
#[doc(alias = "I2C_TIMEOUTR")]
pub type I2cTimeoutr = crate::Reg<i2c_timeoutr::I2cTimeoutrSpec>;
#[doc = "I2C timeout register"]
pub mod i2c_timeoutr;
#[doc = "I2C_ISR (rw) register accessor: I2C interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_isr`]
module"]
#[doc(alias = "I2C_ISR")]
pub type I2cIsr = crate::Reg<i2c_isr::I2cIsrSpec>;
#[doc = "I2C interrupt and status register"]
pub mod i2c_isr;
#[doc = "I2C_ICR (w) register accessor: I2C interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_icr`]
module"]
#[doc(alias = "I2C_ICR")]
pub type I2cIcr = crate::Reg<i2c_icr::I2cIcrSpec>;
#[doc = "I2C interrupt clear register"]
pub mod i2c_icr;
#[doc = "I2C_PECR (r) register accessor: I2C PEC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_pecr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_pecr`]
module"]
#[doc(alias = "I2C_PECR")]
pub type I2cPecr = crate::Reg<i2c_pecr::I2cPecrSpec>;
#[doc = "I2C PEC register"]
pub mod i2c_pecr;
#[doc = "I2C_RXDR (r) register accessor: I2C receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_rxdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_rxdr`]
module"]
#[doc(alias = "I2C_RXDR")]
pub type I2cRxdr = crate::Reg<i2c_rxdr::I2cRxdrSpec>;
#[doc = "I2C receive data register"]
pub mod i2c_rxdr;
#[doc = "I2C_TXDR (rw) register accessor: I2C transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_txdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_txdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_txdr`]
module"]
#[doc(alias = "I2C_TXDR")]
pub type I2cTxdr = crate::Reg<i2c_txdr::I2cTxdrSpec>;
#[doc = "I2C transmit data register"]
pub mod i2c_txdr;
