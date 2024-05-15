#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crs_cr: CrsCr,
    crs_cfgr: CrsCfgr,
    crs_isr: CrsIsr,
    crs_icr: CrsIcr,
}
impl RegisterBlock {
    #[doc = "0x00 - CRS control register"]
    #[inline(always)]
    pub const fn crs_cr(&self) -> &CrsCr {
        &self.crs_cr
    }
    #[doc = "0x04 - CRS configuration register"]
    #[inline(always)]
    pub const fn crs_cfgr(&self) -> &CrsCfgr {
        &self.crs_cfgr
    }
    #[doc = "0x08 - CRS interrupt and status register"]
    #[inline(always)]
    pub const fn crs_isr(&self) -> &CrsIsr {
        &self.crs_isr
    }
    #[doc = "0x0c - CRS interrupt flag clear register"]
    #[inline(always)]
    pub const fn crs_icr(&self) -> &CrsIcr {
        &self.crs_icr
    }
}
#[doc = "CRS_CR (rw) register accessor: CRS control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crs_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crs_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crs_cr`]
module"]
#[doc(alias = "CRS_CR")]
pub type CrsCr = crate::Reg<crs_cr::CrsCrSpec>;
#[doc = "CRS control register"]
pub mod crs_cr;
#[doc = "CRS_CFGR (rw) register accessor: CRS configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crs_cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crs_cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crs_cfgr`]
module"]
#[doc(alias = "CRS_CFGR")]
pub type CrsCfgr = crate::Reg<crs_cfgr::CrsCfgrSpec>;
#[doc = "CRS configuration register"]
pub mod crs_cfgr;
#[doc = "CRS_ISR (r) register accessor: CRS interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crs_isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crs_isr`]
module"]
#[doc(alias = "CRS_ISR")]
pub type CrsIsr = crate::Reg<crs_isr::CrsIsrSpec>;
#[doc = "CRS interrupt and status register"]
pub mod crs_isr;
#[doc = "CRS_ICR (rw) register accessor: CRS interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crs_icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crs_icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crs_icr`]
module"]
#[doc(alias = "CRS_ICR")]
pub type CrsIcr = crate::Reg<crs_icr::CrsIcrSpec>;
#[doc = "CRS interrupt flag clear register"]
pub mod crs_icr;
