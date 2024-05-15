#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_lptim1_isr: [u8; 0x04],
    _reserved_1_lptim1_icr: [u8; 0x04],
    _reserved_2_lptim1_dier: [u8; 0x04],
    lptim1_cfgr: Lptim1Cfgr,
    lptim1_cr: Lptim1Cr,
    lptim1_ccr1: Lptim1Ccr1,
    lptim1_arr: Lptim1Arr,
    lptim1_cnt: Lptim1Cnt,
    _reserved8: [u8; 0x04],
    lptim1_cfgr2: Lptim1Cfgr2,
    lptim1_rcr: Lptim1Rcr,
    lptim1_ccmr1: Lptim1Ccmr1,
    lptim1_ccmr2: Lptim1Ccmr2,
    lptim1_ccr2: Lptim1Ccr2,
    lptim1_ccr3: Lptim1Ccr3,
    lptim1_ccr4: Lptim1Ccr4,
}
impl RegisterBlock {
    #[doc = "0x00 - LPTIM1 interrupt and status register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim1_isr_input(&self) -> &Lptim1IsrInput {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - LPTIM1 interrupt and status register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim1_isr_output(&self) -> &Lptim1IsrOutput {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - LPTIM1 interrupt clear register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim1_icr_input(&self) -> &Lptim1IcrInput {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - LPTIM1 interrupt clear register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim1_icr_output(&self) -> &Lptim1IcrOutput {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - LPTIM1 interrupt enable register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim1_dier_input(&self) -> &Lptim1DierInput {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - LPTIM1 interrupt enable register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim1_dier_output(&self) -> &Lptim1DierOutput {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - LPTIM configuration register"]
    #[inline(always)]
    pub const fn lptim1_cfgr(&self) -> &Lptim1Cfgr {
        &self.lptim1_cfgr
    }
    #[doc = "0x10 - LPTIM control register"]
    #[inline(always)]
    pub const fn lptim1_cr(&self) -> &Lptim1Cr {
        &self.lptim1_cr
    }
    #[doc = "0x14 - LPTIM compare register 1"]
    #[inline(always)]
    pub const fn lptim1_ccr1(&self) -> &Lptim1Ccr1 {
        &self.lptim1_ccr1
    }
    #[doc = "0x18 - LPTIM autoreload register"]
    #[inline(always)]
    pub const fn lptim1_arr(&self) -> &Lptim1Arr {
        &self.lptim1_arr
    }
    #[doc = "0x1c - LPTIM counter register"]
    #[inline(always)]
    pub const fn lptim1_cnt(&self) -> &Lptim1Cnt {
        &self.lptim1_cnt
    }
    #[doc = "0x24 - LPTIM configuration register 2"]
    #[inline(always)]
    pub const fn lptim1_cfgr2(&self) -> &Lptim1Cfgr2 {
        &self.lptim1_cfgr2
    }
    #[doc = "0x28 - LPTIM repetition register"]
    #[inline(always)]
    pub const fn lptim1_rcr(&self) -> &Lptim1Rcr {
        &self.lptim1_rcr
    }
    #[doc = "0x2c - LPTIM capture/compare mode register 1"]
    #[inline(always)]
    pub const fn lptim1_ccmr1(&self) -> &Lptim1Ccmr1 {
        &self.lptim1_ccmr1
    }
    #[doc = "0x30 - LPTIM capture/compare mode register 2"]
    #[inline(always)]
    pub const fn lptim1_ccmr2(&self) -> &Lptim1Ccmr2 {
        &self.lptim1_ccmr2
    }
    #[doc = "0x34 - LPTIM compare register 2"]
    #[inline(always)]
    pub const fn lptim1_ccr2(&self) -> &Lptim1Ccr2 {
        &self.lptim1_ccr2
    }
    #[doc = "0x38 - LPTIM compare register 3"]
    #[inline(always)]
    pub const fn lptim1_ccr3(&self) -> &Lptim1Ccr3 {
        &self.lptim1_ccr3
    }
    #[doc = "0x3c - LPTIM compare register 4"]
    #[inline(always)]
    pub const fn lptim1_ccr4(&self) -> &Lptim1Ccr4 {
        &self.lptim1_ccr4
    }
}
#[doc = "LPTIM1_ISR_OUTPUT (r) register accessor: LPTIM1 interrupt and status register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_isr_output::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_isr_output`]
module"]
#[doc(alias = "LPTIM1_ISR_OUTPUT")]
pub type Lptim1IsrOutput = crate::Reg<lptim1_isr_output::Lptim1IsrOutputSpec>;
#[doc = "LPTIM1 interrupt and status register \\[alternate\\]"]
pub mod lptim1_isr_output;
#[doc = "LPTIM1_ISR_INPUT (r) register accessor: LPTIM1 interrupt and status register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_isr_input::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_isr_input`]
module"]
#[doc(alias = "LPTIM1_ISR_INPUT")]
pub type Lptim1IsrInput = crate::Reg<lptim1_isr_input::Lptim1IsrInputSpec>;
#[doc = "LPTIM1 interrupt and status register \\[alternate\\]"]
pub mod lptim1_isr_input;
#[doc = "LPTIM1_ICR_OUTPUT (w) register accessor: LPTIM1 interrupt clear register \\[alternate\\]\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim1_icr_output::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_icr_output`]
module"]
#[doc(alias = "LPTIM1_ICR_OUTPUT")]
pub type Lptim1IcrOutput = crate::Reg<lptim1_icr_output::Lptim1IcrOutputSpec>;
#[doc = "LPTIM1 interrupt clear register \\[alternate\\]"]
pub mod lptim1_icr_output;
#[doc = "LPTIM1_ICR_INPUT (w) register accessor: LPTIM1 interrupt clear register \\[alternate\\]\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim1_icr_input::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_icr_input`]
module"]
#[doc(alias = "LPTIM1_ICR_INPUT")]
pub type Lptim1IcrInput = crate::Reg<lptim1_icr_input::Lptim1IcrInputSpec>;
#[doc = "LPTIM1 interrupt clear register \\[alternate\\]"]
pub mod lptim1_icr_input;
#[doc = "LPTIM1_DIER_OUTPUT (rw) register accessor: LPTIM1 interrupt enable register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_dier_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim1_dier_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_dier_output`]
module"]
#[doc(alias = "LPTIM1_DIER_OUTPUT")]
pub type Lptim1DierOutput = crate::Reg<lptim1_dier_output::Lptim1DierOutputSpec>;
#[doc = "LPTIM1 interrupt enable register \\[alternate\\]"]
pub mod lptim1_dier_output;
#[doc = "LPTIM1_DIER_INPUT (rw) register accessor: LPTIM1 interrupt enable register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_dier_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim1_dier_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_dier_input`]
module"]
#[doc(alias = "LPTIM1_DIER_INPUT")]
pub type Lptim1DierInput = crate::Reg<lptim1_dier_input::Lptim1DierInputSpec>;
#[doc = "LPTIM1 interrupt enable register \\[alternate\\]"]
pub mod lptim1_dier_input;
#[doc = "LPTIM1_CFGR (rw) register accessor: LPTIM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim1_cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_cfgr`]
module"]
#[doc(alias = "LPTIM1_CFGR")]
pub type Lptim1Cfgr = crate::Reg<lptim1_cfgr::Lptim1CfgrSpec>;
#[doc = "LPTIM configuration register"]
pub mod lptim1_cfgr;
#[doc = "LPTIM1_CR (rw) register accessor: LPTIM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim1_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_cr`]
module"]
#[doc(alias = "LPTIM1_CR")]
pub type Lptim1Cr = crate::Reg<lptim1_cr::Lptim1CrSpec>;
#[doc = "LPTIM control register"]
pub mod lptim1_cr;
#[doc = "LPTIM1_CCR1 (rw) register accessor: LPTIM compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim1_ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_ccr1`]
module"]
#[doc(alias = "LPTIM1_CCR1")]
pub type Lptim1Ccr1 = crate::Reg<lptim1_ccr1::Lptim1Ccr1Spec>;
#[doc = "LPTIM compare register 1"]
pub mod lptim1_ccr1;
#[doc = "LPTIM1_ARR (rw) register accessor: LPTIM autoreload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim1_arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_arr`]
module"]
#[doc(alias = "LPTIM1_ARR")]
pub type Lptim1Arr = crate::Reg<lptim1_arr::Lptim1ArrSpec>;
#[doc = "LPTIM autoreload register"]
pub mod lptim1_arr;
#[doc = "LPTIM1_CNT (r) register accessor: LPTIM counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_cnt`]
module"]
#[doc(alias = "LPTIM1_CNT")]
pub type Lptim1Cnt = crate::Reg<lptim1_cnt::Lptim1CntSpec>;
#[doc = "LPTIM counter register"]
pub mod lptim1_cnt;
#[doc = "LPTIM1_CFGR2 (rw) register accessor: LPTIM configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim1_cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_cfgr2`]
module"]
#[doc(alias = "LPTIM1_CFGR2")]
pub type Lptim1Cfgr2 = crate::Reg<lptim1_cfgr2::Lptim1Cfgr2Spec>;
#[doc = "LPTIM configuration register 2"]
pub mod lptim1_cfgr2;
#[doc = "LPTIM1_RCR (rw) register accessor: LPTIM repetition register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim1_rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_rcr`]
module"]
#[doc(alias = "LPTIM1_RCR")]
pub type Lptim1Rcr = crate::Reg<lptim1_rcr::Lptim1RcrSpec>;
#[doc = "LPTIM repetition register"]
pub mod lptim1_rcr;
#[doc = "LPTIM1_CCMR1 (rw) register accessor: LPTIM capture/compare mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_ccmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim1_ccmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_ccmr1`]
module"]
#[doc(alias = "LPTIM1_CCMR1")]
pub type Lptim1Ccmr1 = crate::Reg<lptim1_ccmr1::Lptim1Ccmr1Spec>;
#[doc = "LPTIM capture/compare mode register 1"]
pub mod lptim1_ccmr1;
#[doc = "LPTIM1_CCMR2 (rw) register accessor: LPTIM capture/compare mode register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_ccmr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim1_ccmr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_ccmr2`]
module"]
#[doc(alias = "LPTIM1_CCMR2")]
pub type Lptim1Ccmr2 = crate::Reg<lptim1_ccmr2::Lptim1Ccmr2Spec>;
#[doc = "LPTIM capture/compare mode register 2"]
pub mod lptim1_ccmr2;
#[doc = "LPTIM1_CCR2 (rw) register accessor: LPTIM compare register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim1_ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_ccr2`]
module"]
#[doc(alias = "LPTIM1_CCR2")]
pub type Lptim1Ccr2 = crate::Reg<lptim1_ccr2::Lptim1Ccr2Spec>;
#[doc = "LPTIM compare register 2"]
pub mod lptim1_ccr2;
#[doc = "LPTIM1_CCR3 (rw) register accessor: LPTIM compare register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_ccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim1_ccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_ccr3`]
module"]
#[doc(alias = "LPTIM1_CCR3")]
pub type Lptim1Ccr3 = crate::Reg<lptim1_ccr3::Lptim1Ccr3Spec>;
#[doc = "LPTIM compare register 3"]
pub mod lptim1_ccr3;
#[doc = "LPTIM1_CCR4 (rw) register accessor: LPTIM compare register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_ccr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim1_ccr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_ccr4`]
module"]
#[doc(alias = "LPTIM1_CCR4")]
pub type Lptim1Ccr4 = crate::Reg<lptim1_ccr4::Lptim1Ccr4Spec>;
#[doc = "LPTIM compare register 4"]
pub mod lptim1_ccr4;
