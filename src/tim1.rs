#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tim1_cr1: Tim1Cr1,
    _reserved1: [u8; 0x02],
    tim1_cr2: Tim1Cr2,
    tim1_smcr: Tim1Smcr,
    tim1_dier: Tim1Dier,
    _reserved4: [u8; 0x02],
    tim1_sr: Tim1Sr,
    tim1_egr: Tim1Egr,
    _reserved6: [u8; 0x02],
    _reserved_6_tim1_ccmr1: [u8; 0x04],
    _reserved_7_tim1_ccmr2: [u8; 0x04],
    tim1_ccer: Tim1Ccer,
    tim1_cnt: Tim1Cnt,
    tim1_psc: Tim1Psc,
    _reserved11: [u8; 0x02],
    tim1_arr: Tim1Arr,
    _reserved12: [u8; 0x02],
    tim1_rcr: Tim1Rcr,
    _reserved13: [u8; 0x02],
    tim1_ccr1: Tim1Ccr1,
    _reserved14: [u8; 0x02],
    tim1_ccr2: Tim1Ccr2,
    _reserved15: [u8; 0x02],
    tim1_ccr3: Tim1Ccr3,
    _reserved16: [u8; 0x02],
    tim1_ccr4: Tim1Ccr4,
    _reserved17: [u8; 0x02],
    tim1_bdtr: Tim1Bdtr,
    tim1_dcr: Tim1Dcr,
    _reserved19: [u8; 0x02],
    tim1_dmar: Tim1Dmar,
    tim1_or1: Tim1Or1,
    tim1_ccmr3: Tim1Ccmr3,
    tim1_ccr5: Tim1Ccr5,
    tim1_ccr6: Tim1Ccr6,
    _reserved24: [u8; 0x02],
    tim1_af1: Tim1Af1,
    tim1_af2: Tim1Af2,
    tim1_tisel: Tim1Tisel,
}
impl RegisterBlock {
    #[doc = "0x00 - TIM1 control register 1"]
    #[inline(always)]
    pub const fn tim1_cr1(&self) -> &Tim1Cr1 {
        &self.tim1_cr1
    }
    #[doc = "0x04 - TIM1 control register 2"]
    #[inline(always)]
    pub const fn tim1_cr2(&self) -> &Tim1Cr2 {
        &self.tim1_cr2
    }
    #[doc = "0x08 - TIM1 slave mode control register"]
    #[inline(always)]
    pub const fn tim1_smcr(&self) -> &Tim1Smcr {
        &self.tim1_smcr
    }
    #[doc = "0x0c - TIM1 DMA/interrupt enable register"]
    #[inline(always)]
    pub const fn tim1_dier(&self) -> &Tim1Dier {
        &self.tim1_dier
    }
    #[doc = "0x10 - TIM1 status register"]
    #[inline(always)]
    pub const fn tim1_sr(&self) -> &Tim1Sr {
        &self.tim1_sr
    }
    #[doc = "0x14 - TIM1 event generation register"]
    #[inline(always)]
    pub const fn tim1_egr(&self) -> &Tim1Egr {
        &self.tim1_egr
    }
    #[doc = "0x18 - TIM1 capture/compare mode register 1"]
    #[inline(always)]
    pub const fn tim1_ccmr1_output(&self) -> &Tim1Ccmr1Output {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - TIM1 capture/compare mode register 1"]
    #[inline(always)]
    pub const fn tim1_ccmr1_input(&self) -> &Tim1Ccmr1Input {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - TIM1 capture/compare mode register 1"]
    #[inline(always)]
    pub const fn tim1_ccmr2_output(&self) -> &Tim1Ccmr2Output {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - TIM1 capture/compare mode register 2"]
    #[inline(always)]
    pub const fn tim1_ccmr2_input(&self) -> &Tim1Ccmr2Input {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - TIM1 capture/compare enable register"]
    #[inline(always)]
    pub const fn tim1_ccer(&self) -> &Tim1Ccer {
        &self.tim1_ccer
    }
    #[doc = "0x24 - TIM1 counter"]
    #[inline(always)]
    pub const fn tim1_cnt(&self) -> &Tim1Cnt {
        &self.tim1_cnt
    }
    #[doc = "0x28 - TIM1 prescaler"]
    #[inline(always)]
    pub const fn tim1_psc(&self) -> &Tim1Psc {
        &self.tim1_psc
    }
    #[doc = "0x2c - TIM1 auto-reload register"]
    #[inline(always)]
    pub const fn tim1_arr(&self) -> &Tim1Arr {
        &self.tim1_arr
    }
    #[doc = "0x30 - TIM1 repetition counter register"]
    #[inline(always)]
    pub const fn tim1_rcr(&self) -> &Tim1Rcr {
        &self.tim1_rcr
    }
    #[doc = "0x34 - TIM1 capture/compare register 1"]
    #[inline(always)]
    pub const fn tim1_ccr1(&self) -> &Tim1Ccr1 {
        &self.tim1_ccr1
    }
    #[doc = "0x38 - TIM1 capture/compare register 2"]
    #[inline(always)]
    pub const fn tim1_ccr2(&self) -> &Tim1Ccr2 {
        &self.tim1_ccr2
    }
    #[doc = "0x3c - TIM1 capture/compare register 3"]
    #[inline(always)]
    pub const fn tim1_ccr3(&self) -> &Tim1Ccr3 {
        &self.tim1_ccr3
    }
    #[doc = "0x40 - TIM1 capture/compare register 4"]
    #[inline(always)]
    pub const fn tim1_ccr4(&self) -> &Tim1Ccr4 {
        &self.tim1_ccr4
    }
    #[doc = "0x44 - TIM1 break and dead-time register"]
    #[inline(always)]
    pub const fn tim1_bdtr(&self) -> &Tim1Bdtr {
        &self.tim1_bdtr
    }
    #[doc = "0x48 - TIM1 DMA control register"]
    #[inline(always)]
    pub const fn tim1_dcr(&self) -> &Tim1Dcr {
        &self.tim1_dcr
    }
    #[doc = "0x4c - TIM1 DMA address for full transfer"]
    #[inline(always)]
    pub const fn tim1_dmar(&self) -> &Tim1Dmar {
        &self.tim1_dmar
    }
    #[doc = "0x50 - TIM1 option register 1"]
    #[inline(always)]
    pub const fn tim1_or1(&self) -> &Tim1Or1 {
        &self.tim1_or1
    }
    #[doc = "0x54 - TIM1 capture/compare mode register 3"]
    #[inline(always)]
    pub const fn tim1_ccmr3(&self) -> &Tim1Ccmr3 {
        &self.tim1_ccmr3
    }
    #[doc = "0x58 - TIM1 capture/compare register 5"]
    #[inline(always)]
    pub const fn tim1_ccr5(&self) -> &Tim1Ccr5 {
        &self.tim1_ccr5
    }
    #[doc = "0x5c - TIM1 capture/compare register 6"]
    #[inline(always)]
    pub const fn tim1_ccr6(&self) -> &Tim1Ccr6 {
        &self.tim1_ccr6
    }
    #[doc = "0x60 - TIM1 alternate function option register 1"]
    #[inline(always)]
    pub const fn tim1_af1(&self) -> &Tim1Af1 {
        &self.tim1_af1
    }
    #[doc = "0x64 - TIM1 Alternate function register 2"]
    #[inline(always)]
    pub const fn tim1_af2(&self) -> &Tim1Af2 {
        &self.tim1_af2
    }
    #[doc = "0x68 - TIM1 timer input selection register"]
    #[inline(always)]
    pub const fn tim1_tisel(&self) -> &Tim1Tisel {
        &self.tim1_tisel
    }
}
#[doc = "TIM1_CR1 (rw) register accessor: TIM1 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_cr1`]
module"]
#[doc(alias = "TIM1_CR1")]
pub type Tim1Cr1 = crate::Reg<tim1_cr1::Tim1Cr1Spec>;
#[doc = "TIM1 control register 1"]
pub mod tim1_cr1;
#[doc = "TIM1_CR2 (rw) register accessor: TIM1 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_cr2`]
module"]
#[doc(alias = "TIM1_CR2")]
pub type Tim1Cr2 = crate::Reg<tim1_cr2::Tim1Cr2Spec>;
#[doc = "TIM1 control register 2"]
pub mod tim1_cr2;
#[doc = "TIM1_SMCR (rw) register accessor: TIM1 slave mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_smcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_smcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_smcr`]
module"]
#[doc(alias = "TIM1_SMCR")]
pub type Tim1Smcr = crate::Reg<tim1_smcr::Tim1SmcrSpec>;
#[doc = "TIM1 slave mode control register"]
pub mod tim1_smcr;
#[doc = "TIM1_DIER (rw) register accessor: TIM1 DMA/interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_dier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_dier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_dier`]
module"]
#[doc(alias = "TIM1_DIER")]
pub type Tim1Dier = crate::Reg<tim1_dier::Tim1DierSpec>;
#[doc = "TIM1 DMA/interrupt enable register"]
pub mod tim1_dier;
#[doc = "TIM1_SR (rw) register accessor: TIM1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_sr`]
module"]
#[doc(alias = "TIM1_SR")]
pub type Tim1Sr = crate::Reg<tim1_sr::Tim1SrSpec>;
#[doc = "TIM1 status register"]
pub mod tim1_sr;
#[doc = "TIM1_EGR (w) register accessor: TIM1 event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_egr`]
module"]
#[doc(alias = "TIM1_EGR")]
pub type Tim1Egr = crate::Reg<tim1_egr::Tim1EgrSpec>;
#[doc = "TIM1 event generation register"]
pub mod tim1_egr;
#[doc = "TIM1_CCMR1_INPUT (rw) register accessor: TIM1 capture/compare mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccmr1_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccmr1_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccmr1_input`]
module"]
#[doc(alias = "TIM1_CCMR1_INPUT")]
pub type Tim1Ccmr1Input = crate::Reg<tim1_ccmr1_input::Tim1Ccmr1InputSpec>;
#[doc = "TIM1 capture/compare mode register 1"]
pub mod tim1_ccmr1_input;
#[doc = "TIM1_CCMR1_OUTPUT (rw) register accessor: TIM1 capture/compare mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccmr1_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccmr1_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccmr1_output`]
module"]
#[doc(alias = "TIM1_CCMR1_OUTPUT")]
pub type Tim1Ccmr1Output = crate::Reg<tim1_ccmr1_output::Tim1Ccmr1OutputSpec>;
#[doc = "TIM1 capture/compare mode register 1"]
pub mod tim1_ccmr1_output;
#[doc = "TIM1_CCMR2_INPUT (rw) register accessor: TIM1 capture/compare mode register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccmr2_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccmr2_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccmr2_input`]
module"]
#[doc(alias = "TIM1_CCMR2_INPUT")]
pub type Tim1Ccmr2Input = crate::Reg<tim1_ccmr2_input::Tim1Ccmr2InputSpec>;
#[doc = "TIM1 capture/compare mode register 2"]
pub mod tim1_ccmr2_input;
#[doc = "TIM1_CCMR2_OUTPUT (rw) register accessor: TIM1 capture/compare mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccmr2_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccmr2_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccmr2_output`]
module"]
#[doc(alias = "TIM1_CCMR2_OUTPUT")]
pub type Tim1Ccmr2Output = crate::Reg<tim1_ccmr2_output::Tim1Ccmr2OutputSpec>;
#[doc = "TIM1 capture/compare mode register 1"]
pub mod tim1_ccmr2_output;
#[doc = "TIM1_CCER (rw) register accessor: TIM1 capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccer`]
module"]
#[doc(alias = "TIM1_CCER")]
pub type Tim1Ccer = crate::Reg<tim1_ccer::Tim1CcerSpec>;
#[doc = "TIM1 capture/compare enable register"]
pub mod tim1_ccer;
#[doc = "TIM1_CNT (rw) register accessor: TIM1 counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_cnt`]
module"]
#[doc(alias = "TIM1_CNT")]
pub type Tim1Cnt = crate::Reg<tim1_cnt::Tim1CntSpec>;
#[doc = "TIM1 counter"]
pub mod tim1_cnt;
#[doc = "TIM1_PSC (rw) register accessor: TIM1 prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_psc`]
module"]
#[doc(alias = "TIM1_PSC")]
pub type Tim1Psc = crate::Reg<tim1_psc::Tim1PscSpec>;
#[doc = "TIM1 prescaler"]
pub mod tim1_psc;
#[doc = "TIM1_ARR (rw) register accessor: TIM1 auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_arr`]
module"]
#[doc(alias = "TIM1_ARR")]
pub type Tim1Arr = crate::Reg<tim1_arr::Tim1ArrSpec>;
#[doc = "TIM1 auto-reload register"]
pub mod tim1_arr;
#[doc = "TIM1_RCR (rw) register accessor: TIM1 repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_rcr`]
module"]
#[doc(alias = "TIM1_RCR")]
pub type Tim1Rcr = crate::Reg<tim1_rcr::Tim1RcrSpec>;
#[doc = "TIM1 repetition counter register"]
pub mod tim1_rcr;
#[doc = "TIM1_CCR1 (rw) register accessor: TIM1 capture/compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccr1`]
module"]
#[doc(alias = "TIM1_CCR1")]
pub type Tim1Ccr1 = crate::Reg<tim1_ccr1::Tim1Ccr1Spec>;
#[doc = "TIM1 capture/compare register 1"]
pub mod tim1_ccr1;
#[doc = "TIM1_CCR2 (rw) register accessor: TIM1 capture/compare register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccr2`]
module"]
#[doc(alias = "TIM1_CCR2")]
pub type Tim1Ccr2 = crate::Reg<tim1_ccr2::Tim1Ccr2Spec>;
#[doc = "TIM1 capture/compare register 2"]
pub mod tim1_ccr2;
#[doc = "TIM1_CCR3 (rw) register accessor: TIM1 capture/compare register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccr3`]
module"]
#[doc(alias = "TIM1_CCR3")]
pub type Tim1Ccr3 = crate::Reg<tim1_ccr3::Tim1Ccr3Spec>;
#[doc = "TIM1 capture/compare register 3"]
pub mod tim1_ccr3;
#[doc = "TIM1_CCR4 (rw) register accessor: TIM1 capture/compare register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccr4`]
module"]
#[doc(alias = "TIM1_CCR4")]
pub type Tim1Ccr4 = crate::Reg<tim1_ccr4::Tim1Ccr4Spec>;
#[doc = "TIM1 capture/compare register 4"]
pub mod tim1_ccr4;
#[doc = "TIM1_BDTR (rw) register accessor: TIM1 break and dead-time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_bdtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_bdtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_bdtr`]
module"]
#[doc(alias = "TIM1_BDTR")]
pub type Tim1Bdtr = crate::Reg<tim1_bdtr::Tim1BdtrSpec>;
#[doc = "TIM1 break and dead-time register"]
pub mod tim1_bdtr;
#[doc = "TIM1_DCR (rw) register accessor: TIM1 DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_dcr`]
module"]
#[doc(alias = "TIM1_DCR")]
pub type Tim1Dcr = crate::Reg<tim1_dcr::Tim1DcrSpec>;
#[doc = "TIM1 DMA control register"]
pub mod tim1_dcr;
#[doc = "TIM1_DMAR (rw) register accessor: TIM1 DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_dmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_dmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_dmar`]
module"]
#[doc(alias = "TIM1_DMAR")]
pub type Tim1Dmar = crate::Reg<tim1_dmar::Tim1DmarSpec>;
#[doc = "TIM1 DMA address for full transfer"]
pub mod tim1_dmar;
#[doc = "TIM1_OR1 (rw) register accessor: TIM1 option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_or1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_or1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_or1`]
module"]
#[doc(alias = "TIM1_OR1")]
pub type Tim1Or1 = crate::Reg<tim1_or1::Tim1Or1Spec>;
#[doc = "TIM1 option register 1"]
pub mod tim1_or1;
#[doc = "TIM1_CCMR3 (rw) register accessor: TIM1 capture/compare mode register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccmr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccmr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccmr3`]
module"]
#[doc(alias = "TIM1_CCMR3")]
pub type Tim1Ccmr3 = crate::Reg<tim1_ccmr3::Tim1Ccmr3Spec>;
#[doc = "TIM1 capture/compare mode register 3"]
pub mod tim1_ccmr3;
#[doc = "TIM1_CCR5 (rw) register accessor: TIM1 capture/compare register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccr5`]
module"]
#[doc(alias = "TIM1_CCR5")]
pub type Tim1Ccr5 = crate::Reg<tim1_ccr5::Tim1Ccr5Spec>;
#[doc = "TIM1 capture/compare register 5"]
pub mod tim1_ccr5;
#[doc = "TIM1_CCR6 (rw) register accessor: TIM1 capture/compare register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_ccr6`]
module"]
#[doc(alias = "TIM1_CCR6")]
pub type Tim1Ccr6 = crate::Reg<tim1_ccr6::Tim1Ccr6Spec>;
#[doc = "TIM1 capture/compare register 6"]
pub mod tim1_ccr6;
#[doc = "TIM1_AF1 (rw) register accessor: TIM1 alternate function option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_af1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_af1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_af1`]
module"]
#[doc(alias = "TIM1_AF1")]
pub type Tim1Af1 = crate::Reg<tim1_af1::Tim1Af1Spec>;
#[doc = "TIM1 alternate function option register 1"]
pub mod tim1_af1;
#[doc = "TIM1_AF2 (rw) register accessor: TIM1 Alternate function register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_af2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_af2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_af2`]
module"]
#[doc(alias = "TIM1_AF2")]
pub type Tim1Af2 = crate::Reg<tim1_af2::Tim1Af2Spec>;
#[doc = "TIM1 Alternate function register 2"]
pub mod tim1_af2;
#[doc = "TIM1_TISEL (rw) register accessor: TIM1 timer input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_tisel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_tisel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim1_tisel`]
module"]
#[doc(alias = "TIM1_TISEL")]
pub type Tim1Tisel = crate::Reg<tim1_tisel::Tim1TiselSpec>;
#[doc = "TIM1 timer input selection register"]
pub mod tim1_tisel;
