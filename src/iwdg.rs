#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    iwdg_kr: IwdgKr,
    iwdg_pr: IwdgPr,
    iwdg_rlr: IwdgRlr,
    iwdg_sr: IwdgSr,
    iwdg_winr: IwdgWinr,
    iwdg_ewcr: IwdgEwcr,
}
impl RegisterBlock {
    #[doc = "0x00 - IWDG key register"]
    #[inline(always)]
    pub const fn iwdg_kr(&self) -> &IwdgKr {
        &self.iwdg_kr
    }
    #[doc = "0x04 - IWDG prescaler register"]
    #[inline(always)]
    pub const fn iwdg_pr(&self) -> &IwdgPr {
        &self.iwdg_pr
    }
    #[doc = "0x08 - IWDG reload register"]
    #[inline(always)]
    pub const fn iwdg_rlr(&self) -> &IwdgRlr {
        &self.iwdg_rlr
    }
    #[doc = "0x0c - IWDG status register"]
    #[inline(always)]
    pub const fn iwdg_sr(&self) -> &IwdgSr {
        &self.iwdg_sr
    }
    #[doc = "0x10 - IWDG window register"]
    #[inline(always)]
    pub const fn iwdg_winr(&self) -> &IwdgWinr {
        &self.iwdg_winr
    }
    #[doc = "0x14 - IWDG early wake-up interrupt register"]
    #[inline(always)]
    pub const fn iwdg_ewcr(&self) -> &IwdgEwcr {
        &self.iwdg_ewcr
    }
}
#[doc = "IWDG_KR (w) register accessor: IWDG key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_kr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_kr`]
module"]
#[doc(alias = "IWDG_KR")]
pub type IwdgKr = crate::Reg<iwdg_kr::IwdgKrSpec>;
#[doc = "IWDG key register"]
pub mod iwdg_kr;
#[doc = "IWDG_PR (rw) register accessor: IWDG prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_pr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_pr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_pr`]
module"]
#[doc(alias = "IWDG_PR")]
pub type IwdgPr = crate::Reg<iwdg_pr::IwdgPrSpec>;
#[doc = "IWDG prescaler register"]
pub mod iwdg_pr;
#[doc = "IWDG_RLR (rw) register accessor: IWDG reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_rlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_rlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_rlr`]
module"]
#[doc(alias = "IWDG_RLR")]
pub type IwdgRlr = crate::Reg<iwdg_rlr::IwdgRlrSpec>;
#[doc = "IWDG reload register"]
pub mod iwdg_rlr;
#[doc = "IWDG_SR (r) register accessor: IWDG status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_sr`]
module"]
#[doc(alias = "IWDG_SR")]
pub type IwdgSr = crate::Reg<iwdg_sr::IwdgSrSpec>;
#[doc = "IWDG status register"]
pub mod iwdg_sr;
#[doc = "IWDG_WINR (rw) register accessor: IWDG window register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_winr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_winr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_winr`]
module"]
#[doc(alias = "IWDG_WINR")]
pub type IwdgWinr = crate::Reg<iwdg_winr::IwdgWinrSpec>;
#[doc = "IWDG window register"]
pub mod iwdg_winr;
#[doc = "IWDG_EWCR (rw) register accessor: IWDG early wake-up interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_ewcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_ewcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_ewcr`]
module"]
#[doc(alias = "IWDG_EWCR")]
pub type IwdgEwcr = crate::Reg<iwdg_ewcr::IwdgEwcrSpec>;
#[doc = "IWDG early wake-up interrupt register"]
pub mod iwdg_ewcr;
