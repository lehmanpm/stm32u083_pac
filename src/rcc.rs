#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rcc_cr: RccCr,
    rcc_icscr: RccIcscr,
    rcc_cfgr: RccCfgr,
    rcc_pllcfgr: RccPllcfgr,
    _reserved4: [u8; 0x08],
    rcc_cier: RccCier,
    rcc_cifr: RccCifr,
    rcc_cicr: RccCicr,
    _reserved7: [u8; 0x04],
    rcc_ahbrstr: RccAhbrstr,
    rcc_ioprstr: RccIoprstr,
    _reserved9: [u8; 0x08],
    rcc_apbrstr1: RccApbrstr1,
    _reserved10: [u8; 0x04],
    rcc_apbrstr2: RccApbrstr2,
    _reserved11: [u8; 0x04],
    rcc_ahbenr: RccAhbenr,
    rcc_iopenr: RccIopenr,
    rcc_dbgcfgr: RccDbgcfgr,
    _reserved14: [u8; 0x04],
    rcc_apbenr1: RccApbenr1,
    _reserved15: [u8; 0x04],
    rcc_apbenr2: RccApbenr2,
    _reserved16: [u8; 0x04],
    rcc_ahbsmenr: RccAhbsmenr,
    rcc_iopsmenr: RccIopsmenr,
    _reserved18: [u8; 0x08],
    rcc_apbsmenr1: RccApbsmenr1,
    _reserved19: [u8; 0x04],
    rcc_apbsmenr2: RccApbsmenr2,
    _reserved20: [u8; 0x04],
    rcc_ccipr: RccCcipr,
    _reserved21: [u8; 0x04],
    rcc_bdcr: RccBdcr,
    rcc_csr: RccCsr,
    rcc_crrcr: RccCrrcr,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    #[inline(always)]
    pub const fn rcc_cr(&self) -> &RccCr {
        &self.rcc_cr
    }
    #[doc = "0x04 - Internal clock sources calibration register"]
    #[inline(always)]
    pub const fn rcc_icscr(&self) -> &RccIcscr {
        &self.rcc_icscr
    }
    #[doc = "0x08 - Clock configuration register"]
    #[inline(always)]
    pub const fn rcc_cfgr(&self) -> &RccCfgr {
        &self.rcc_cfgr
    }
    #[doc = "0x0c - PLL configuration register"]
    #[inline(always)]
    pub const fn rcc_pllcfgr(&self) -> &RccPllcfgr {
        &self.rcc_pllcfgr
    }
    #[doc = "0x18 - Clock interrupt enable register"]
    #[inline(always)]
    pub const fn rcc_cier(&self) -> &RccCier {
        &self.rcc_cier
    }
    #[doc = "0x1c - Clock interrupt flag register"]
    #[inline(always)]
    pub const fn rcc_cifr(&self) -> &RccCifr {
        &self.rcc_cifr
    }
    #[doc = "0x20 - Clock interrupt clear register"]
    #[inline(always)]
    pub const fn rcc_cicr(&self) -> &RccCicr {
        &self.rcc_cicr
    }
    #[doc = "0x28 - AHB peripheral reset register"]
    #[inline(always)]
    pub const fn rcc_ahbrstr(&self) -> &RccAhbrstr {
        &self.rcc_ahbrstr
    }
    #[doc = "0x2c - I/O port reset register"]
    #[inline(always)]
    pub const fn rcc_ioprstr(&self) -> &RccIoprstr {
        &self.rcc_ioprstr
    }
    #[doc = "0x38 - APB peripheral reset register 1"]
    #[inline(always)]
    pub const fn rcc_apbrstr1(&self) -> &RccApbrstr1 {
        &self.rcc_apbrstr1
    }
    #[doc = "0x40 - APB peripheral reset register 2"]
    #[inline(always)]
    pub const fn rcc_apbrstr2(&self) -> &RccApbrstr2 {
        &self.rcc_apbrstr2
    }
    #[doc = "0x48 - AHB peripheral clock enable register"]
    #[inline(always)]
    pub const fn rcc_ahbenr(&self) -> &RccAhbenr {
        &self.rcc_ahbenr
    }
    #[doc = "0x4c - I/O port clock enable register"]
    #[inline(always)]
    pub const fn rcc_iopenr(&self) -> &RccIopenr {
        &self.rcc_iopenr
    }
    #[doc = "0x50 - Debug configuration register"]
    #[inline(always)]
    pub const fn rcc_dbgcfgr(&self) -> &RccDbgcfgr {
        &self.rcc_dbgcfgr
    }
    #[doc = "0x58 - APB peripheral clock enable register 1"]
    #[inline(always)]
    pub const fn rcc_apbenr1(&self) -> &RccApbenr1 {
        &self.rcc_apbenr1
    }
    #[doc = "0x60 - APB peripheral clock enable register 2"]
    #[inline(always)]
    pub const fn rcc_apbenr2(&self) -> &RccApbenr2 {
        &self.rcc_apbenr2
    }
    #[doc = "0x68 - AHB peripheral clock enable in Sleep/Stop mode register"]
    #[inline(always)]
    pub const fn rcc_ahbsmenr(&self) -> &RccAhbsmenr {
        &self.rcc_ahbsmenr
    }
    #[doc = "0x6c - I/O port in Sleep mode clock enable register"]
    #[inline(always)]
    pub const fn rcc_iopsmenr(&self) -> &RccIopsmenr {
        &self.rcc_iopsmenr
    }
    #[doc = "0x78 - APB peripheral clock enable in Sleep/Stop mode register 1"]
    #[inline(always)]
    pub const fn rcc_apbsmenr1(&self) -> &RccApbsmenr1 {
        &self.rcc_apbsmenr1
    }
    #[doc = "0x80 - APB peripheral clock enable in Sleep/Stop mode register 2"]
    #[inline(always)]
    pub const fn rcc_apbsmenr2(&self) -> &RccApbsmenr2 {
        &self.rcc_apbsmenr2
    }
    #[doc = "0x88 - Peripherals independent clock configuration register"]
    #[inline(always)]
    pub const fn rcc_ccipr(&self) -> &RccCcipr {
        &self.rcc_ccipr
    }
    #[doc = "0x90 - RTC domain control register"]
    #[inline(always)]
    pub const fn rcc_bdcr(&self) -> &RccBdcr {
        &self.rcc_bdcr
    }
    #[doc = "0x94 - Control/status register"]
    #[inline(always)]
    pub const fn rcc_csr(&self) -> &RccCsr {
        &self.rcc_csr
    }
    #[doc = "0x98 - RCC clock recovery RC register"]
    #[inline(always)]
    pub const fn rcc_crrcr(&self) -> &RccCrrcr {
        &self.rcc_crrcr
    }
}
#[doc = "RCC_CR (rw) register accessor: Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cr`]
module"]
#[doc(alias = "RCC_CR")]
pub type RccCr = crate::Reg<rcc_cr::RccCrSpec>;
#[doc = "Clock control register"]
pub mod rcc_cr;
#[doc = "RCC_ICSCR (rw) register accessor: Internal clock sources calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_icscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_icscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_icscr`]
module"]
#[doc(alias = "RCC_ICSCR")]
pub type RccIcscr = crate::Reg<rcc_icscr::RccIcscrSpec>;
#[doc = "Internal clock sources calibration register"]
pub mod rcc_icscr;
#[doc = "RCC_CFGR (rw) register accessor: Clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cfgr`]
module"]
#[doc(alias = "RCC_CFGR")]
pub type RccCfgr = crate::Reg<rcc_cfgr::RccCfgrSpec>;
#[doc = "Clock configuration register"]
pub mod rcc_cfgr;
#[doc = "RCC_PLLCFGR (rw) register accessor: PLL configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pllcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pllcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pllcfgr`]
module"]
#[doc(alias = "RCC_PLLCFGR")]
pub type RccPllcfgr = crate::Reg<rcc_pllcfgr::RccPllcfgrSpec>;
#[doc = "PLL configuration register"]
pub mod rcc_pllcfgr;
#[doc = "RCC_CIER (rw) register accessor: Clock interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_cier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_cier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cier`]
module"]
#[doc(alias = "RCC_CIER")]
pub type RccCier = crate::Reg<rcc_cier::RccCierSpec>;
#[doc = "Clock interrupt enable register"]
pub mod rcc_cier;
#[doc = "RCC_CIFR (r) register accessor: Clock interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_cifr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cifr`]
module"]
#[doc(alias = "RCC_CIFR")]
pub type RccCifr = crate::Reg<rcc_cifr::RccCifrSpec>;
#[doc = "Clock interrupt flag register"]
pub mod rcc_cifr;
#[doc = "RCC_CICR (w) register accessor: Clock interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cicr`]
module"]
#[doc(alias = "RCC_CICR")]
pub type RccCicr = crate::Reg<rcc_cicr::RccCicrSpec>;
#[doc = "Clock interrupt clear register"]
pub mod rcc_cicr;
#[doc = "RCC_AHBRSTR (rw) register accessor: AHB peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahbrstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahbrstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahbrstr`]
module"]
#[doc(alias = "RCC_AHBRSTR")]
pub type RccAhbrstr = crate::Reg<rcc_ahbrstr::RccAhbrstrSpec>;
#[doc = "AHB peripheral reset register"]
pub mod rcc_ahbrstr;
#[doc = "RCC_IOPRSTR (rw) register accessor: I/O port reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ioprstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ioprstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ioprstr`]
module"]
#[doc(alias = "RCC_IOPRSTR")]
pub type RccIoprstr = crate::Reg<rcc_ioprstr::RccIoprstrSpec>;
#[doc = "I/O port reset register"]
pub mod rcc_ioprstr;
#[doc = "RCC_APBRSTR1 (rw) register accessor: APB peripheral reset register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apbrstr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apbrstr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apbrstr1`]
module"]
#[doc(alias = "RCC_APBRSTR1")]
pub type RccApbrstr1 = crate::Reg<rcc_apbrstr1::RccApbrstr1Spec>;
#[doc = "APB peripheral reset register 1"]
pub mod rcc_apbrstr1;
#[doc = "RCC_APBRSTR2 (rw) register accessor: APB peripheral reset register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apbrstr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apbrstr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apbrstr2`]
module"]
#[doc(alias = "RCC_APBRSTR2")]
pub type RccApbrstr2 = crate::Reg<rcc_apbrstr2::RccApbrstr2Spec>;
#[doc = "APB peripheral reset register 2"]
pub mod rcc_apbrstr2;
#[doc = "RCC_AHBENR (rw) register accessor: AHB peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahbenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahbenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahbenr`]
module"]
#[doc(alias = "RCC_AHBENR")]
pub type RccAhbenr = crate::Reg<rcc_ahbenr::RccAhbenrSpec>;
#[doc = "AHB peripheral clock enable register"]
pub mod rcc_ahbenr;
#[doc = "RCC_IOPENR (rw) register accessor: I/O port clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_iopenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_iopenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_iopenr`]
module"]
#[doc(alias = "RCC_IOPENR")]
pub type RccIopenr = crate::Reg<rcc_iopenr::RccIopenrSpec>;
#[doc = "I/O port clock enable register"]
pub mod rcc_iopenr;
#[doc = "RCC_DBGCFGR (rw) register accessor: Debug configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_dbgcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_dbgcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_dbgcfgr`]
module"]
#[doc(alias = "RCC_DBGCFGR")]
pub type RccDbgcfgr = crate::Reg<rcc_dbgcfgr::RccDbgcfgrSpec>;
#[doc = "Debug configuration register"]
pub mod rcc_dbgcfgr;
#[doc = "RCC_APBENR1 (rw) register accessor: APB peripheral clock enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apbenr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apbenr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apbenr1`]
module"]
#[doc(alias = "RCC_APBENR1")]
pub type RccApbenr1 = crate::Reg<rcc_apbenr1::RccApbenr1Spec>;
#[doc = "APB peripheral clock enable register 1"]
pub mod rcc_apbenr1;
#[doc = "RCC_APBENR2 (rw) register accessor: APB peripheral clock enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apbenr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apbenr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apbenr2`]
module"]
#[doc(alias = "RCC_APBENR2")]
pub type RccApbenr2 = crate::Reg<rcc_apbenr2::RccApbenr2Spec>;
#[doc = "APB peripheral clock enable register 2"]
pub mod rcc_apbenr2;
#[doc = "RCC_AHBSMENR (rw) register accessor: AHB peripheral clock enable in Sleep/Stop mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahbsmenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahbsmenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahbsmenr`]
module"]
#[doc(alias = "RCC_AHBSMENR")]
pub type RccAhbsmenr = crate::Reg<rcc_ahbsmenr::RccAhbsmenrSpec>;
#[doc = "AHB peripheral clock enable in Sleep/Stop mode register"]
pub mod rcc_ahbsmenr;
#[doc = "RCC_IOPSMENR (rw) register accessor: I/O port in Sleep mode clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_iopsmenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_iopsmenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_iopsmenr`]
module"]
#[doc(alias = "RCC_IOPSMENR")]
pub type RccIopsmenr = crate::Reg<rcc_iopsmenr::RccIopsmenrSpec>;
#[doc = "I/O port in Sleep mode clock enable register"]
pub mod rcc_iopsmenr;
#[doc = "RCC_APBSMENR1 (rw) register accessor: APB peripheral clock enable in Sleep/Stop mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apbsmenr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apbsmenr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apbsmenr1`]
module"]
#[doc(alias = "RCC_APBSMENR1")]
pub type RccApbsmenr1 = crate::Reg<rcc_apbsmenr1::RccApbsmenr1Spec>;
#[doc = "APB peripheral clock enable in Sleep/Stop mode register 1"]
pub mod rcc_apbsmenr1;
#[doc = "RCC_APBSMENR2 (rw) register accessor: APB peripheral clock enable in Sleep/Stop mode register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apbsmenr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apbsmenr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apbsmenr2`]
module"]
#[doc(alias = "RCC_APBSMENR2")]
pub type RccApbsmenr2 = crate::Reg<rcc_apbsmenr2::RccApbsmenr2Spec>;
#[doc = "APB peripheral clock enable in Sleep/Stop mode register 2"]
pub mod rcc_apbsmenr2;
#[doc = "RCC_CCIPR (rw) register accessor: Peripherals independent clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ccipr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ccipr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ccipr`]
module"]
#[doc(alias = "RCC_CCIPR")]
pub type RccCcipr = crate::Reg<rcc_ccipr::RccCciprSpec>;
#[doc = "Peripherals independent clock configuration register"]
pub mod rcc_ccipr;
#[doc = "RCC_BDCR (rw) register accessor: RTC domain control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_bdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_bdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_bdcr`]
module"]
#[doc(alias = "RCC_BDCR")]
pub type RccBdcr = crate::Reg<rcc_bdcr::RccBdcrSpec>;
#[doc = "RTC domain control register"]
pub mod rcc_bdcr;
#[doc = "RCC_CSR (rw) register accessor: Control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_csr`]
module"]
#[doc(alias = "RCC_CSR")]
pub type RccCsr = crate::Reg<rcc_csr::RccCsrSpec>;
#[doc = "Control/status register"]
pub mod rcc_csr;
#[doc = "RCC_CRRCR (rw) register accessor: RCC clock recovery RC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_crrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_crrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_crrcr`]
module"]
#[doc(alias = "RCC_CRRCR")]
pub type RccCrrcr = crate::Reg<rcc_crrcr::RccCrrcrSpec>;
#[doc = "RCC clock recovery RC register"]
pub mod rcc_crrcr;
