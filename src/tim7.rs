#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tim7_cr1: Tim7Cr1,
    _reserved1: [u8; 0x02],
    tim7_cr2: Tim7Cr2,
    _reserved2: [u8; 0x06],
    tim7_dier: Tim7Dier,
    _reserved3: [u8; 0x02],
    tim7_sr: Tim7Sr,
    _reserved4: [u8; 0x02],
    tim7_egr: Tim7Egr,
    _reserved5: [u8; 0x0e],
    tim7_cnt: Tim7Cnt,
    tim7_psc: Tim7Psc,
    _reserved7: [u8; 0x02],
    tim7_arr: Tim7Arr,
}
impl RegisterBlock {
    #[doc = "0x00 - TIM7 control register 1"]
    #[inline(always)]
    pub const fn tim7_cr1(&self) -> &Tim7Cr1 {
        &self.tim7_cr1
    }
    #[doc = "0x04 - TIM7 control register 2"]
    #[inline(always)]
    pub const fn tim7_cr2(&self) -> &Tim7Cr2 {
        &self.tim7_cr2
    }
    #[doc = "0x0c - TIM7 DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn tim7_dier(&self) -> &Tim7Dier {
        &self.tim7_dier
    }
    #[doc = "0x10 - TIM7 status register"]
    #[inline(always)]
    pub const fn tim7_sr(&self) -> &Tim7Sr {
        &self.tim7_sr
    }
    #[doc = "0x14 - TIM7 event generation register"]
    #[inline(always)]
    pub const fn tim7_egr(&self) -> &Tim7Egr {
        &self.tim7_egr
    }
    #[doc = "0x24 - TIM7 counter"]
    #[inline(always)]
    pub const fn tim7_cnt(&self) -> &Tim7Cnt {
        &self.tim7_cnt
    }
    #[doc = "0x28 - TIM7 prescaler"]
    #[inline(always)]
    pub const fn tim7_psc(&self) -> &Tim7Psc {
        &self.tim7_psc
    }
    #[doc = "0x2c - TIM7 auto-reload register"]
    #[inline(always)]
    pub const fn tim7_arr(&self) -> &Tim7Arr {
        &self.tim7_arr
    }
}
#[doc = "TIM7_CR1 (rw) register accessor: TIM7 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim7_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim7_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim7_cr1`]
module"]
#[doc(alias = "TIM7_CR1")]
pub type Tim7Cr1 = crate::Reg<tim7_cr1::Tim7Cr1Spec>;
#[doc = "TIM7 control register 1"]
pub mod tim7_cr1;
#[doc = "TIM7_CR2 (rw) register accessor: TIM7 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim7_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim7_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim7_cr2`]
module"]
#[doc(alias = "TIM7_CR2")]
pub type Tim7Cr2 = crate::Reg<tim7_cr2::Tim7Cr2Spec>;
#[doc = "TIM7 control register 2"]
pub mod tim7_cr2;
#[doc = "TIM7_DIER (rw) register accessor: TIM7 DMA/Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim7_dier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim7_dier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim7_dier`]
module"]
#[doc(alias = "TIM7_DIER")]
pub type Tim7Dier = crate::Reg<tim7_dier::Tim7DierSpec>;
#[doc = "TIM7 DMA/Interrupt enable register"]
pub mod tim7_dier;
#[doc = "TIM7_SR (rw) register accessor: TIM7 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim7_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim7_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim7_sr`]
module"]
#[doc(alias = "TIM7_SR")]
pub type Tim7Sr = crate::Reg<tim7_sr::Tim7SrSpec>;
#[doc = "TIM7 status register"]
pub mod tim7_sr;
#[doc = "TIM7_EGR (w) register accessor: TIM7 event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim7_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim7_egr`]
module"]
#[doc(alias = "TIM7_EGR")]
pub type Tim7Egr = crate::Reg<tim7_egr::Tim7EgrSpec>;
#[doc = "TIM7 event generation register"]
pub mod tim7_egr;
#[doc = "TIM7_CNT (rw) register accessor: TIM7 counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim7_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim7_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim7_cnt`]
module"]
#[doc(alias = "TIM7_CNT")]
pub type Tim7Cnt = crate::Reg<tim7_cnt::Tim7CntSpec>;
#[doc = "TIM7 counter"]
pub mod tim7_cnt;
#[doc = "TIM7_PSC (rw) register accessor: TIM7 prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim7_psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim7_psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim7_psc`]
module"]
#[doc(alias = "TIM7_PSC")]
pub type Tim7Psc = crate::Reg<tim7_psc::Tim7PscSpec>;
#[doc = "TIM7 prescaler"]
pub mod tim7_psc;
#[doc = "TIM7_ARR (rw) register accessor: TIM7 auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim7_arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim7_arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim7_arr`]
module"]
#[doc(alias = "TIM7_ARR")]
pub type Tim7Arr = crate::Reg<tim7_arr::Tim7ArrSpec>;
#[doc = "TIM7 auto-reload register"]
pub mod tim7_arr;
