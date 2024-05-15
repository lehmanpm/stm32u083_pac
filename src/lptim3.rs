#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_lptim3_isr: [u8; 0x04],
    _reserved_1_lptim3_icr: [u8; 0x04],
    _reserved_2_lptim3_dier: [u8; 0x04],
    lptim3_cfgr: Lptim3Cfgr,
    lptim3_cr: Lptim3Cr,
    lptim3_ccr1: Lptim3Ccr1,
    lptim3_arr: Lptim3Arr,
    lptim3_cnt: Lptim3Cnt,
    _reserved8: [u8; 0x04],
    lptim3_cfgr2: Lptim3Cfgr2,
    lptim3_rcr: Lptim3Rcr,
    lptim3_ccmr1: Lptim3Ccmr1,
    lptim3_ccmr2: Lptim3Ccmr2,
    lptim3_ccr2: Lptim3Ccr2,
    lptim3_ccr3: Lptim3Ccr3,
    lptim3_ccr4: Lptim3Ccr4,
}
impl RegisterBlock {
    #[doc = "0x00 - LPTIM3 interrupt and status register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim3_isr_input(&self) -> &Lptim3IsrInput {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - LPTIM3 interrupt and status register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim3_isr_output(&self) -> &Lptim3IsrOutput {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - LPTIM3 interrupt clear register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim3_icr_input(&self) -> &Lptim3IcrInput {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - LPTIM3 interrupt clear register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim3_icr_output(&self) -> &Lptim3IcrOutput {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - LPTIM3 interrupt enable register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim3_dier_input(&self) -> &Lptim3DierInput {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - LPTIM3 interrupt enable register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim3_dier_output(&self) -> &Lptim3DierOutput {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - LPTIM configuration register"]
    #[inline(always)]
    pub const fn lptim3_cfgr(&self) -> &Lptim3Cfgr {
        &self.lptim3_cfgr
    }
    #[doc = "0x10 - LPTIM control register"]
    #[inline(always)]
    pub const fn lptim3_cr(&self) -> &Lptim3Cr {
        &self.lptim3_cr
    }
    #[doc = "0x14 - LPTIM compare register 1"]
    #[inline(always)]
    pub const fn lptim3_ccr1(&self) -> &Lptim3Ccr1 {
        &self.lptim3_ccr1
    }
    #[doc = "0x18 - LPTIM autoreload register"]
    #[inline(always)]
    pub const fn lptim3_arr(&self) -> &Lptim3Arr {
        &self.lptim3_arr
    }
    #[doc = "0x1c - LPTIM counter register"]
    #[inline(always)]
    pub const fn lptim3_cnt(&self) -> &Lptim3Cnt {
        &self.lptim3_cnt
    }
    #[doc = "0x24 - LPTIM configuration register 2"]
    #[inline(always)]
    pub const fn lptim3_cfgr2(&self) -> &Lptim3Cfgr2 {
        &self.lptim3_cfgr2
    }
    #[doc = "0x28 - LPTIM repetition register"]
    #[inline(always)]
    pub const fn lptim3_rcr(&self) -> &Lptim3Rcr {
        &self.lptim3_rcr
    }
    #[doc = "0x2c - LPTIM capture/compare mode register 1"]
    #[inline(always)]
    pub const fn lptim3_ccmr1(&self) -> &Lptim3Ccmr1 {
        &self.lptim3_ccmr1
    }
    #[doc = "0x30 - LPTIM capture/compare mode register 2"]
    #[inline(always)]
    pub const fn lptim3_ccmr2(&self) -> &Lptim3Ccmr2 {
        &self.lptim3_ccmr2
    }
    #[doc = "0x34 - LPTIM compare register 2"]
    #[inline(always)]
    pub const fn lptim3_ccr2(&self) -> &Lptim3Ccr2 {
        &self.lptim3_ccr2
    }
    #[doc = "0x38 - LPTIM compare register 3"]
    #[inline(always)]
    pub const fn lptim3_ccr3(&self) -> &Lptim3Ccr3 {
        &self.lptim3_ccr3
    }
    #[doc = "0x3c - LPTIM compare register 4"]
    #[inline(always)]
    pub const fn lptim3_ccr4(&self) -> &Lptim3Ccr4 {
        &self.lptim3_ccr4
    }
}
#[doc = "LPTIM3_ISR_OUTPUT (r) register accessor: LPTIM3 interrupt and status register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_isr_output::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim3_isr_output`]
module"]
#[doc(alias = "LPTIM3_ISR_OUTPUT")]
pub type Lptim3IsrOutput = crate::Reg<lptim3_isr_output::Lptim3IsrOutputSpec>;
#[doc = "LPTIM3 interrupt and status register \\[alternate\\]"]
pub mod lptim3_isr_output;
#[doc = "LPTIM3_ISR_INPUT (r) register accessor: LPTIM3 interrupt and status register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_isr_input::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim3_isr_input`]
module"]
#[doc(alias = "LPTIM3_ISR_INPUT")]
pub type Lptim3IsrInput = crate::Reg<lptim3_isr_input::Lptim3IsrInputSpec>;
#[doc = "LPTIM3 interrupt and status register \\[alternate\\]"]
pub mod lptim3_isr_input;
#[doc = "LPTIM3_ICR_OUTPUT (w) register accessor: LPTIM3 interrupt clear register \\[alternate\\]\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_icr_output::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim3_icr_output`]
module"]
#[doc(alias = "LPTIM3_ICR_OUTPUT")]
pub type Lptim3IcrOutput = crate::Reg<lptim3_icr_output::Lptim3IcrOutputSpec>;
#[doc = "LPTIM3 interrupt clear register \\[alternate\\]"]
pub mod lptim3_icr_output;
#[doc = "LPTIM3_ICR_INPUT (w) register accessor: LPTIM3 interrupt clear register \\[alternate\\]\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_icr_input::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim3_icr_input`]
module"]
#[doc(alias = "LPTIM3_ICR_INPUT")]
pub type Lptim3IcrInput = crate::Reg<lptim3_icr_input::Lptim3IcrInputSpec>;
#[doc = "LPTIM3 interrupt clear register \\[alternate\\]"]
pub mod lptim3_icr_input;
#[doc = "LPTIM3_DIER_OUTPUT (rw) register accessor: LPTIM3 interrupt enable register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_dier_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_dier_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim3_dier_output`]
module"]
#[doc(alias = "LPTIM3_DIER_OUTPUT")]
pub type Lptim3DierOutput = crate::Reg<lptim3_dier_output::Lptim3DierOutputSpec>;
#[doc = "LPTIM3 interrupt enable register \\[alternate\\]"]
pub mod lptim3_dier_output;
#[doc = "LPTIM3_DIER_INPUT (rw) register accessor: LPTIM3 interrupt enable register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_dier_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_dier_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim3_dier_input`]
module"]
#[doc(alias = "LPTIM3_DIER_INPUT")]
pub type Lptim3DierInput = crate::Reg<lptim3_dier_input::Lptim3DierInputSpec>;
#[doc = "LPTIM3 interrupt enable register \\[alternate\\]"]
pub mod lptim3_dier_input;
#[doc = "LPTIM3_CFGR (rw) register accessor: LPTIM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim3_cfgr`]
module"]
#[doc(alias = "LPTIM3_CFGR")]
pub type Lptim3Cfgr = crate::Reg<lptim3_cfgr::Lptim3CfgrSpec>;
#[doc = "LPTIM configuration register"]
pub mod lptim3_cfgr;
#[doc = "LPTIM3_CR (rw) register accessor: LPTIM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim3_cr`]
module"]
#[doc(alias = "LPTIM3_CR")]
pub type Lptim3Cr = crate::Reg<lptim3_cr::Lptim3CrSpec>;
#[doc = "LPTIM control register"]
pub mod lptim3_cr;
#[doc = "LPTIM3_CCR1 (rw) register accessor: LPTIM compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim3_ccr1`]
module"]
#[doc(alias = "LPTIM3_CCR1")]
pub type Lptim3Ccr1 = crate::Reg<lptim3_ccr1::Lptim3Ccr1Spec>;
#[doc = "LPTIM compare register 1"]
pub mod lptim3_ccr1;
#[doc = "LPTIM3_ARR (rw) register accessor: LPTIM autoreload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim3_arr`]
module"]
#[doc(alias = "LPTIM3_ARR")]
pub type Lptim3Arr = crate::Reg<lptim3_arr::Lptim3ArrSpec>;
#[doc = "LPTIM autoreload register"]
pub mod lptim3_arr;
#[doc = "LPTIM3_CNT (r) register accessor: LPTIM counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim3_cnt`]
module"]
#[doc(alias = "LPTIM3_CNT")]
pub type Lptim3Cnt = crate::Reg<lptim3_cnt::Lptim3CntSpec>;
#[doc = "LPTIM counter register"]
pub mod lptim3_cnt;
#[doc = "LPTIM3_CFGR2 (rw) register accessor: LPTIM configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim3_cfgr2`]
module"]
#[doc(alias = "LPTIM3_CFGR2")]
pub type Lptim3Cfgr2 = crate::Reg<lptim3_cfgr2::Lptim3Cfgr2Spec>;
#[doc = "LPTIM configuration register 2"]
pub mod lptim3_cfgr2;
#[doc = "LPTIM3_RCR (rw) register accessor: LPTIM repetition register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim3_rcr`]
module"]
#[doc(alias = "LPTIM3_RCR")]
pub type Lptim3Rcr = crate::Reg<lptim3_rcr::Lptim3RcrSpec>;
#[doc = "LPTIM repetition register"]
pub mod lptim3_rcr;
#[doc = "LPTIM3_CCMR1 (rw) register accessor: LPTIM capture/compare mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_ccmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_ccmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim3_ccmr1`]
module"]
#[doc(alias = "LPTIM3_CCMR1")]
pub type Lptim3Ccmr1 = crate::Reg<lptim3_ccmr1::Lptim3Ccmr1Spec>;
#[doc = "LPTIM capture/compare mode register 1"]
pub mod lptim3_ccmr1;
#[doc = "LPTIM3_CCMR2 (rw) register accessor: LPTIM capture/compare mode register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_ccmr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_ccmr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim3_ccmr2`]
module"]
#[doc(alias = "LPTIM3_CCMR2")]
pub type Lptim3Ccmr2 = crate::Reg<lptim3_ccmr2::Lptim3Ccmr2Spec>;
#[doc = "LPTIM capture/compare mode register 2"]
pub mod lptim3_ccmr2;
#[doc = "LPTIM3_CCR2 (rw) register accessor: LPTIM compare register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim3_ccr2`]
module"]
#[doc(alias = "LPTIM3_CCR2")]
pub type Lptim3Ccr2 = crate::Reg<lptim3_ccr2::Lptim3Ccr2Spec>;
#[doc = "LPTIM compare register 2"]
pub mod lptim3_ccr2;
#[doc = "LPTIM3_CCR3 (rw) register accessor: LPTIM compare register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_ccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_ccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim3_ccr3`]
module"]
#[doc(alias = "LPTIM3_CCR3")]
pub type Lptim3Ccr3 = crate::Reg<lptim3_ccr3::Lptim3Ccr3Spec>;
#[doc = "LPTIM compare register 3"]
pub mod lptim3_ccr3;
#[doc = "LPTIM3_CCR4 (rw) register accessor: LPTIM compare register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_ccr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_ccr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim3_ccr4`]
module"]
#[doc(alias = "LPTIM3_CCR4")]
pub type Lptim3Ccr4 = crate::Reg<lptim3_ccr4::Lptim3Ccr4Spec>;
#[doc = "LPTIM compare register 4"]
pub mod lptim3_ccr4;
