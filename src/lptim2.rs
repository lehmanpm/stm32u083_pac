#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_lptim2_isr: [u8; 0x04],
    _reserved_1_lptim2_icr: [u8; 0x04],
    _reserved_2_lptim2_dier: [u8; 0x04],
    lptim2_cfgr: Lptim2Cfgr,
    lptim2_cr: Lptim2Cr,
    lptim2_ccr1: Lptim2Ccr1,
    lptim2_arr: Lptim2Arr,
    lptim2_cnt: Lptim2Cnt,
    _reserved8: [u8; 0x04],
    lptim2_cfgr2: Lptim2Cfgr2,
    lptim2_rcr: Lptim2Rcr,
    lptim2_ccmr1: Lptim2Ccmr1,
    lptim2_ccmr2: Lptim2Ccmr2,
    lptim2_ccr2: Lptim2Ccr2,
    lptim2_ccr3: Lptim2Ccr3,
    lptim2_ccr4: Lptim2Ccr4,
}
impl RegisterBlock {
    #[doc = "0x00 - LPTIM2 interrupt and status register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim2_isr_input(&self) -> &Lptim2IsrInput {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - LPTIM2 interrupt and status register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim2_isr_output(&self) -> &Lptim2IsrOutput {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - LPTIM2 interrupt clear register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim2_icr_input(&self) -> &Lptim2IcrInput {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - LPTIM2 interrupt clear register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim2_icr_output(&self) -> &Lptim2IcrOutput {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - LPTIM2 interrupt enable register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim2_dier_input(&self) -> &Lptim2DierInput {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - LPTIM2 interrupt enable register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim2_dier_output(&self) -> &Lptim2DierOutput {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - LPTIM configuration register"]
    #[inline(always)]
    pub const fn lptim2_cfgr(&self) -> &Lptim2Cfgr {
        &self.lptim2_cfgr
    }
    #[doc = "0x10 - LPTIM control register"]
    #[inline(always)]
    pub const fn lptim2_cr(&self) -> &Lptim2Cr {
        &self.lptim2_cr
    }
    #[doc = "0x14 - LPTIM compare register 1"]
    #[inline(always)]
    pub const fn lptim2_ccr1(&self) -> &Lptim2Ccr1 {
        &self.lptim2_ccr1
    }
    #[doc = "0x18 - LPTIM autoreload register"]
    #[inline(always)]
    pub const fn lptim2_arr(&self) -> &Lptim2Arr {
        &self.lptim2_arr
    }
    #[doc = "0x1c - LPTIM counter register"]
    #[inline(always)]
    pub const fn lptim2_cnt(&self) -> &Lptim2Cnt {
        &self.lptim2_cnt
    }
    #[doc = "0x24 - LPTIM configuration register 2"]
    #[inline(always)]
    pub const fn lptim2_cfgr2(&self) -> &Lptim2Cfgr2 {
        &self.lptim2_cfgr2
    }
    #[doc = "0x28 - LPTIM repetition register"]
    #[inline(always)]
    pub const fn lptim2_rcr(&self) -> &Lptim2Rcr {
        &self.lptim2_rcr
    }
    #[doc = "0x2c - LPTIM capture/compare mode register 1"]
    #[inline(always)]
    pub const fn lptim2_ccmr1(&self) -> &Lptim2Ccmr1 {
        &self.lptim2_ccmr1
    }
    #[doc = "0x30 - LPTIM capture/compare mode register 2"]
    #[inline(always)]
    pub const fn lptim2_ccmr2(&self) -> &Lptim2Ccmr2 {
        &self.lptim2_ccmr2
    }
    #[doc = "0x34 - LPTIM compare register 2"]
    #[inline(always)]
    pub const fn lptim2_ccr2(&self) -> &Lptim2Ccr2 {
        &self.lptim2_ccr2
    }
    #[doc = "0x38 - LPTIM compare register 3"]
    #[inline(always)]
    pub const fn lptim2_ccr3(&self) -> &Lptim2Ccr3 {
        &self.lptim2_ccr3
    }
    #[doc = "0x3c - LPTIM compare register 4"]
    #[inline(always)]
    pub const fn lptim2_ccr4(&self) -> &Lptim2Ccr4 {
        &self.lptim2_ccr4
    }
}
#[doc = "LPTIM2_ISR_OUTPUT (r) register accessor: LPTIM2 interrupt and status register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_isr_output::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim2_isr_output`]
module"]
#[doc(alias = "LPTIM2_ISR_OUTPUT")]
pub type Lptim2IsrOutput = crate::Reg<lptim2_isr_output::Lptim2IsrOutputSpec>;
#[doc = "LPTIM2 interrupt and status register \\[alternate\\]"]
pub mod lptim2_isr_output;
#[doc = "LPTIM2_ISR_INPUT (r) register accessor: LPTIM2 interrupt and status register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_isr_input::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim2_isr_input`]
module"]
#[doc(alias = "LPTIM2_ISR_INPUT")]
pub type Lptim2IsrInput = crate::Reg<lptim2_isr_input::Lptim2IsrInputSpec>;
#[doc = "LPTIM2 interrupt and status register \\[alternate\\]"]
pub mod lptim2_isr_input;
#[doc = "LPTIM2_ICR_OUTPUT (w) register accessor: LPTIM2 interrupt clear register \\[alternate\\]\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_icr_output::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim2_icr_output`]
module"]
#[doc(alias = "LPTIM2_ICR_OUTPUT")]
pub type Lptim2IcrOutput = crate::Reg<lptim2_icr_output::Lptim2IcrOutputSpec>;
#[doc = "LPTIM2 interrupt clear register \\[alternate\\]"]
pub mod lptim2_icr_output;
#[doc = "LPTIM2_ICR_INPUT (w) register accessor: LPTIM2 interrupt clear register \\[alternate\\]\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_icr_input::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim2_icr_input`]
module"]
#[doc(alias = "LPTIM2_ICR_INPUT")]
pub type Lptim2IcrInput = crate::Reg<lptim2_icr_input::Lptim2IcrInputSpec>;
#[doc = "LPTIM2 interrupt clear register \\[alternate\\]"]
pub mod lptim2_icr_input;
#[doc = "LPTIM2_DIER_OUTPUT (rw) register accessor: LPTIM2 interrupt enable register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_dier_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_dier_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim2_dier_output`]
module"]
#[doc(alias = "LPTIM2_DIER_OUTPUT")]
pub type Lptim2DierOutput = crate::Reg<lptim2_dier_output::Lptim2DierOutputSpec>;
#[doc = "LPTIM2 interrupt enable register \\[alternate\\]"]
pub mod lptim2_dier_output;
#[doc = "LPTIM2_DIER_INPUT (rw) register accessor: LPTIM2 interrupt enable register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_dier_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_dier_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim2_dier_input`]
module"]
#[doc(alias = "LPTIM2_DIER_INPUT")]
pub type Lptim2DierInput = crate::Reg<lptim2_dier_input::Lptim2DierInputSpec>;
#[doc = "LPTIM2 interrupt enable register \\[alternate\\]"]
pub mod lptim2_dier_input;
#[doc = "LPTIM2_CFGR (rw) register accessor: LPTIM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim2_cfgr`]
module"]
#[doc(alias = "LPTIM2_CFGR")]
pub type Lptim2Cfgr = crate::Reg<lptim2_cfgr::Lptim2CfgrSpec>;
#[doc = "LPTIM configuration register"]
pub mod lptim2_cfgr;
#[doc = "LPTIM2_CR (rw) register accessor: LPTIM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim2_cr`]
module"]
#[doc(alias = "LPTIM2_CR")]
pub type Lptim2Cr = crate::Reg<lptim2_cr::Lptim2CrSpec>;
#[doc = "LPTIM control register"]
pub mod lptim2_cr;
#[doc = "LPTIM2_CCR1 (rw) register accessor: LPTIM compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim2_ccr1`]
module"]
#[doc(alias = "LPTIM2_CCR1")]
pub type Lptim2Ccr1 = crate::Reg<lptim2_ccr1::Lptim2Ccr1Spec>;
#[doc = "LPTIM compare register 1"]
pub mod lptim2_ccr1;
#[doc = "LPTIM2_ARR (rw) register accessor: LPTIM autoreload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim2_arr`]
module"]
#[doc(alias = "LPTIM2_ARR")]
pub type Lptim2Arr = crate::Reg<lptim2_arr::Lptim2ArrSpec>;
#[doc = "LPTIM autoreload register"]
pub mod lptim2_arr;
#[doc = "LPTIM2_CNT (r) register accessor: LPTIM counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim2_cnt`]
module"]
#[doc(alias = "LPTIM2_CNT")]
pub type Lptim2Cnt = crate::Reg<lptim2_cnt::Lptim2CntSpec>;
#[doc = "LPTIM counter register"]
pub mod lptim2_cnt;
#[doc = "LPTIM2_CFGR2 (rw) register accessor: LPTIM configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim2_cfgr2`]
module"]
#[doc(alias = "LPTIM2_CFGR2")]
pub type Lptim2Cfgr2 = crate::Reg<lptim2_cfgr2::Lptim2Cfgr2Spec>;
#[doc = "LPTIM configuration register 2"]
pub mod lptim2_cfgr2;
#[doc = "LPTIM2_RCR (rw) register accessor: LPTIM repetition register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim2_rcr`]
module"]
#[doc(alias = "LPTIM2_RCR")]
pub type Lptim2Rcr = crate::Reg<lptim2_rcr::Lptim2RcrSpec>;
#[doc = "LPTIM repetition register"]
pub mod lptim2_rcr;
#[doc = "LPTIM2_CCMR1 (rw) register accessor: LPTIM capture/compare mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_ccmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_ccmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim2_ccmr1`]
module"]
#[doc(alias = "LPTIM2_CCMR1")]
pub type Lptim2Ccmr1 = crate::Reg<lptim2_ccmr1::Lptim2Ccmr1Spec>;
#[doc = "LPTIM capture/compare mode register 1"]
pub mod lptim2_ccmr1;
#[doc = "LPTIM2_CCMR2 (rw) register accessor: LPTIM capture/compare mode register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_ccmr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_ccmr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim2_ccmr2`]
module"]
#[doc(alias = "LPTIM2_CCMR2")]
pub type Lptim2Ccmr2 = crate::Reg<lptim2_ccmr2::Lptim2Ccmr2Spec>;
#[doc = "LPTIM capture/compare mode register 2"]
pub mod lptim2_ccmr2;
#[doc = "LPTIM2_CCR2 (rw) register accessor: LPTIM compare register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim2_ccr2`]
module"]
#[doc(alias = "LPTIM2_CCR2")]
pub type Lptim2Ccr2 = crate::Reg<lptim2_ccr2::Lptim2Ccr2Spec>;
#[doc = "LPTIM compare register 2"]
pub mod lptim2_ccr2;
#[doc = "LPTIM2_CCR3 (rw) register accessor: LPTIM compare register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_ccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_ccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim2_ccr3`]
module"]
#[doc(alias = "LPTIM2_CCR3")]
pub type Lptim2Ccr3 = crate::Reg<lptim2_ccr3::Lptim2Ccr3Spec>;
#[doc = "LPTIM compare register 3"]
pub mod lptim2_ccr3;
#[doc = "LPTIM2_CCR4 (rw) register accessor: LPTIM compare register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_ccr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_ccr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim2_ccr4`]
module"]
#[doc(alias = "LPTIM2_CCR4")]
pub type Lptim2Ccr4 = crate::Reg<lptim2_ccr4::Lptim2Ccr4Spec>;
#[doc = "LPTIM compare register 4"]
pub mod lptim2_ccr4;
