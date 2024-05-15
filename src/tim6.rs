#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tim6_cr1: Tim6Cr1,
    _reserved1: [u8; 0x02],
    tim6_cr2: Tim6Cr2,
    _reserved2: [u8; 0x06],
    tim6_dier: Tim6Dier,
    _reserved3: [u8; 0x02],
    tim6_sr: Tim6Sr,
    _reserved4: [u8; 0x02],
    tim6_egr: Tim6Egr,
    _reserved5: [u8; 0x0e],
    tim6_cnt: Tim6Cnt,
    tim6_psc: Tim6Psc,
    _reserved7: [u8; 0x02],
    tim6_arr: Tim6Arr,
}
impl RegisterBlock {
    #[doc = "0x00 - TIM6 control register 1"]
    #[inline(always)]
    pub const fn tim6_cr1(&self) -> &Tim6Cr1 {
        &self.tim6_cr1
    }
    #[doc = "0x04 - TIM6 control register 2"]
    #[inline(always)]
    pub const fn tim6_cr2(&self) -> &Tim6Cr2 {
        &self.tim6_cr2
    }
    #[doc = "0x0c - TIM6 DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn tim6_dier(&self) -> &Tim6Dier {
        &self.tim6_dier
    }
    #[doc = "0x10 - TIM6 status register"]
    #[inline(always)]
    pub const fn tim6_sr(&self) -> &Tim6Sr {
        &self.tim6_sr
    }
    #[doc = "0x14 - TIM6 event generation register"]
    #[inline(always)]
    pub const fn tim6_egr(&self) -> &Tim6Egr {
        &self.tim6_egr
    }
    #[doc = "0x24 - TIM6 counter"]
    #[inline(always)]
    pub const fn tim6_cnt(&self) -> &Tim6Cnt {
        &self.tim6_cnt
    }
    #[doc = "0x28 - TIM6 prescaler"]
    #[inline(always)]
    pub const fn tim6_psc(&self) -> &Tim6Psc {
        &self.tim6_psc
    }
    #[doc = "0x2c - TIM6 auto-reload register"]
    #[inline(always)]
    pub const fn tim6_arr(&self) -> &Tim6Arr {
        &self.tim6_arr
    }
}
#[doc = "TIM6_CR1 (rw) register accessor: TIM6 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim6_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim6_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim6_cr1`]
module"]
#[doc(alias = "TIM6_CR1")]
pub type Tim6Cr1 = crate::Reg<tim6_cr1::Tim6Cr1Spec>;
#[doc = "TIM6 control register 1"]
pub mod tim6_cr1;
#[doc = "TIM6_CR2 (rw) register accessor: TIM6 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim6_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim6_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim6_cr2`]
module"]
#[doc(alias = "TIM6_CR2")]
pub type Tim6Cr2 = crate::Reg<tim6_cr2::Tim6Cr2Spec>;
#[doc = "TIM6 control register 2"]
pub mod tim6_cr2;
#[doc = "TIM6_DIER (rw) register accessor: TIM6 DMA/Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim6_dier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim6_dier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim6_dier`]
module"]
#[doc(alias = "TIM6_DIER")]
pub type Tim6Dier = crate::Reg<tim6_dier::Tim6DierSpec>;
#[doc = "TIM6 DMA/Interrupt enable register"]
pub mod tim6_dier;
#[doc = "TIM6_SR (rw) register accessor: TIM6 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim6_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim6_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim6_sr`]
module"]
#[doc(alias = "TIM6_SR")]
pub type Tim6Sr = crate::Reg<tim6_sr::Tim6SrSpec>;
#[doc = "TIM6 status register"]
pub mod tim6_sr;
#[doc = "TIM6_EGR (w) register accessor: TIM6 event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim6_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim6_egr`]
module"]
#[doc(alias = "TIM6_EGR")]
pub type Tim6Egr = crate::Reg<tim6_egr::Tim6EgrSpec>;
#[doc = "TIM6 event generation register"]
pub mod tim6_egr;
#[doc = "TIM6_CNT (rw) register accessor: TIM6 counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim6_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim6_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim6_cnt`]
module"]
#[doc(alias = "TIM6_CNT")]
pub type Tim6Cnt = crate::Reg<tim6_cnt::Tim6CntSpec>;
#[doc = "TIM6 counter"]
pub mod tim6_cnt;
#[doc = "TIM6_PSC (rw) register accessor: TIM6 prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim6_psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim6_psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim6_psc`]
module"]
#[doc(alias = "TIM6_PSC")]
pub type Tim6Psc = crate::Reg<tim6_psc::Tim6PscSpec>;
#[doc = "TIM6 prescaler"]
pub mod tim6_psc;
#[doc = "TIM6_ARR (rw) register accessor: TIM6 auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim6_arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim6_arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim6_arr`]
module"]
#[doc(alias = "TIM6_ARR")]
pub type Tim6Arr = crate::Reg<tim6_arr::Tim6ArrSpec>;
#[doc = "TIM6 auto-reload register"]
pub mod tim6_arr;
