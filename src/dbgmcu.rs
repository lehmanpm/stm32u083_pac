#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dbgmcu_idcode: DbgmcuIdcode,
    dbgmcu_cr: DbgmcuCr,
    dbgmcu_apb1fzr: DbgmcuApb1fzr,
    dbgmcu_apb2fzr: DbgmcuApb2fzr,
    _reserved4: [u8; 0xec],
    dbgmcu_sr: DbgmcuSr,
    dbgmcu_dbg_auth_host: DbgmcuDbgAuthHost,
    dbgmcu_dbg_auth_device: DbgmcuDbgAuthDevice,
    _reserved7: [u8; 0x0ec8],
    dbgmcu_pidr4: DbgmcuPidr4,
    _reserved8: [u8; 0x0c],
    dbgmcu_pidr0: DbgmcuPidr0,
    dbgmcu_pidr1: DbgmcuPidr1,
    dbgmcu_pidr2: DbgmcuPidr2,
    dbgmcu_pidr3: DbgmcuPidr3,
    dbgmcu_cidr0: DbgmcuCidr0,
    dbgmcu_cidr1: DbgmcuCidr1,
    dbgmcu_cidr2: DbgmcuCidr2,
    dbgmcu_cidr3: DbgmcuCidr3,
}
impl RegisterBlock {
    #[doc = "0x00 - DBGMCU device ID code register"]
    #[inline(always)]
    pub const fn dbgmcu_idcode(&self) -> &DbgmcuIdcode {
        &self.dbgmcu_idcode
    }
    #[doc = "0x04 - DBGMCU configuration register"]
    #[inline(always)]
    pub const fn dbgmcu_cr(&self) -> &DbgmcuCr {
        &self.dbgmcu_cr
    }
    #[doc = "0x08 - DBGMCU APB1 freeze register"]
    #[inline(always)]
    pub const fn dbgmcu_apb1fzr(&self) -> &DbgmcuApb1fzr {
        &self.dbgmcu_apb1fzr
    }
    #[doc = "0x0c - DBG APB2 freeze register"]
    #[inline(always)]
    pub const fn dbgmcu_apb2fzr(&self) -> &DbgmcuApb2fzr {
        &self.dbgmcu_apb2fzr
    }
    #[doc = "0xfc - DBGMCU status register"]
    #[inline(always)]
    pub const fn dbgmcu_sr(&self) -> &DbgmcuSr {
        &self.dbgmcu_sr
    }
    #[doc = "0x100 - DBGMCU debug authentication mailbox host register"]
    #[inline(always)]
    pub const fn dbgmcu_dbg_auth_host(&self) -> &DbgmcuDbgAuthHost {
        &self.dbgmcu_dbg_auth_host
    }
    #[doc = "0x104 - DBGMCU debug authentication mailbox device register"]
    #[inline(always)]
    pub const fn dbgmcu_dbg_auth_device(&self) -> &DbgmcuDbgAuthDevice {
        &self.dbgmcu_dbg_auth_device
    }
    #[doc = "0xfd0 - DBGMCU CoreSight peripheral identity register 4"]
    #[inline(always)]
    pub const fn dbgmcu_pidr4(&self) -> &DbgmcuPidr4 {
        &self.dbgmcu_pidr4
    }
    #[doc = "0xfe0 - DBGMCU CoreSight peripheral identity register 0"]
    #[inline(always)]
    pub const fn dbgmcu_pidr0(&self) -> &DbgmcuPidr0 {
        &self.dbgmcu_pidr0
    }
    #[doc = "0xfe4 - DBGMCU CoreSight peripheral identity register 1"]
    #[inline(always)]
    pub const fn dbgmcu_pidr1(&self) -> &DbgmcuPidr1 {
        &self.dbgmcu_pidr1
    }
    #[doc = "0xfe8 - DBGMCU CoreSight peripheral identity register 2"]
    #[inline(always)]
    pub const fn dbgmcu_pidr2(&self) -> &DbgmcuPidr2 {
        &self.dbgmcu_pidr2
    }
    #[doc = "0xfec - DBGMCU CoreSight peripheral identity register 3"]
    #[inline(always)]
    pub const fn dbgmcu_pidr3(&self) -> &DbgmcuPidr3 {
        &self.dbgmcu_pidr3
    }
    #[doc = "0xff0 - DBGMCU CoreSight component identity register 0"]
    #[inline(always)]
    pub const fn dbgmcu_cidr0(&self) -> &DbgmcuCidr0 {
        &self.dbgmcu_cidr0
    }
    #[doc = "0xff4 - DBGMCU CoreSight component identity register 1"]
    #[inline(always)]
    pub const fn dbgmcu_cidr1(&self) -> &DbgmcuCidr1 {
        &self.dbgmcu_cidr1
    }
    #[doc = "0xff8 - DBGMCU CoreSight component identity register 2"]
    #[inline(always)]
    pub const fn dbgmcu_cidr2(&self) -> &DbgmcuCidr2 {
        &self.dbgmcu_cidr2
    }
    #[doc = "0xffc - DBGMCU CoreSight component identity register 3"]
    #[inline(always)]
    pub const fn dbgmcu_cidr3(&self) -> &DbgmcuCidr3 {
        &self.dbgmcu_cidr3
    }
}
#[doc = "DBGMCU_IDCODE (r) register accessor: DBGMCU device ID code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_idcode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_idcode`]
module"]
#[doc(alias = "DBGMCU_IDCODE")]
pub type DbgmcuIdcode = crate::Reg<dbgmcu_idcode::DbgmcuIdcodeSpec>;
#[doc = "DBGMCU device ID code register"]
pub mod dbgmcu_idcode;
#[doc = "DBGMCU_CR (rw) register accessor: DBGMCU configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgmcu_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_cr`]
module"]
#[doc(alias = "DBGMCU_CR")]
pub type DbgmcuCr = crate::Reg<dbgmcu_cr::DbgmcuCrSpec>;
#[doc = "DBGMCU configuration register"]
pub mod dbgmcu_cr;
#[doc = "DBGMCU_APB1FZR (rw) register accessor: DBGMCU APB1 freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_apb1fzr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgmcu_apb1fzr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_apb1fzr`]
module"]
#[doc(alias = "DBGMCU_APB1FZR")]
pub type DbgmcuApb1fzr = crate::Reg<dbgmcu_apb1fzr::DbgmcuApb1fzrSpec>;
#[doc = "DBGMCU APB1 freeze register"]
pub mod dbgmcu_apb1fzr;
#[doc = "DBGMCU_APB2FZR (rw) register accessor: DBG APB2 freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_apb2fzr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgmcu_apb2fzr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_apb2fzr`]
module"]
#[doc(alias = "DBGMCU_APB2FZR")]
pub type DbgmcuApb2fzr = crate::Reg<dbgmcu_apb2fzr::DbgmcuApb2fzrSpec>;
#[doc = "DBG APB2 freeze register"]
pub mod dbgmcu_apb2fzr;
#[doc = "DBGMCU_SR (r) register accessor: DBGMCU status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_sr`]
module"]
#[doc(alias = "DBGMCU_SR")]
pub type DbgmcuSr = crate::Reg<dbgmcu_sr::DbgmcuSrSpec>;
#[doc = "DBGMCU status register"]
pub mod dbgmcu_sr;
#[doc = "DBGMCU_DBG_AUTH_HOST (rw) register accessor: DBGMCU debug authentication mailbox host register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_dbg_auth_host::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgmcu_dbg_auth_host::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_dbg_auth_host`]
module"]
#[doc(alias = "DBGMCU_DBG_AUTH_HOST")]
pub type DbgmcuDbgAuthHost = crate::Reg<dbgmcu_dbg_auth_host::DbgmcuDbgAuthHostSpec>;
#[doc = "DBGMCU debug authentication mailbox host register"]
pub mod dbgmcu_dbg_auth_host;
#[doc = "DBGMCU_DBG_AUTH_DEVICE (r) register accessor: DBGMCU debug authentication mailbox device register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_dbg_auth_device::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_dbg_auth_device`]
module"]
#[doc(alias = "DBGMCU_DBG_AUTH_DEVICE")]
pub type DbgmcuDbgAuthDevice = crate::Reg<dbgmcu_dbg_auth_device::DbgmcuDbgAuthDeviceSpec>;
#[doc = "DBGMCU debug authentication mailbox device register"]
pub mod dbgmcu_dbg_auth_device;
#[doc = "DBGMCU_PIDR4 (r) register accessor: DBGMCU CoreSight peripheral identity register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_pidr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_pidr4`]
module"]
#[doc(alias = "DBGMCU_PIDR4")]
pub type DbgmcuPidr4 = crate::Reg<dbgmcu_pidr4::DbgmcuPidr4Spec>;
#[doc = "DBGMCU CoreSight peripheral identity register 4"]
pub mod dbgmcu_pidr4;
#[doc = "DBGMCU_PIDR0 (r) register accessor: DBGMCU CoreSight peripheral identity register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_pidr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_pidr0`]
module"]
#[doc(alias = "DBGMCU_PIDR0")]
pub type DbgmcuPidr0 = crate::Reg<dbgmcu_pidr0::DbgmcuPidr0Spec>;
#[doc = "DBGMCU CoreSight peripheral identity register 0"]
pub mod dbgmcu_pidr0;
#[doc = "DBGMCU_PIDR1 (r) register accessor: DBGMCU CoreSight peripheral identity register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_pidr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_pidr1`]
module"]
#[doc(alias = "DBGMCU_PIDR1")]
pub type DbgmcuPidr1 = crate::Reg<dbgmcu_pidr1::DbgmcuPidr1Spec>;
#[doc = "DBGMCU CoreSight peripheral identity register 1"]
pub mod dbgmcu_pidr1;
#[doc = "DBGMCU_PIDR2 (r) register accessor: DBGMCU CoreSight peripheral identity register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_pidr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_pidr2`]
module"]
#[doc(alias = "DBGMCU_PIDR2")]
pub type DbgmcuPidr2 = crate::Reg<dbgmcu_pidr2::DbgmcuPidr2Spec>;
#[doc = "DBGMCU CoreSight peripheral identity register 2"]
pub mod dbgmcu_pidr2;
#[doc = "DBGMCU_PIDR3 (r) register accessor: DBGMCU CoreSight peripheral identity register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_pidr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_pidr3`]
module"]
#[doc(alias = "DBGMCU_PIDR3")]
pub type DbgmcuPidr3 = crate::Reg<dbgmcu_pidr3::DbgmcuPidr3Spec>;
#[doc = "DBGMCU CoreSight peripheral identity register 3"]
pub mod dbgmcu_pidr3;
#[doc = "DBGMCU_CIDR0 (r) register accessor: DBGMCU CoreSight component identity register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_cidr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_cidr0`]
module"]
#[doc(alias = "DBGMCU_CIDR0")]
pub type DbgmcuCidr0 = crate::Reg<dbgmcu_cidr0::DbgmcuCidr0Spec>;
#[doc = "DBGMCU CoreSight component identity register 0"]
pub mod dbgmcu_cidr0;
#[doc = "DBGMCU_CIDR1 (r) register accessor: DBGMCU CoreSight component identity register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_cidr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_cidr1`]
module"]
#[doc(alias = "DBGMCU_CIDR1")]
pub type DbgmcuCidr1 = crate::Reg<dbgmcu_cidr1::DbgmcuCidr1Spec>;
#[doc = "DBGMCU CoreSight component identity register 1"]
pub mod dbgmcu_cidr1;
#[doc = "DBGMCU_CIDR2 (r) register accessor: DBGMCU CoreSight component identity register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_cidr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_cidr2`]
module"]
#[doc(alias = "DBGMCU_CIDR2")]
pub type DbgmcuCidr2 = crate::Reg<dbgmcu_cidr2::DbgmcuCidr2Spec>;
#[doc = "DBGMCU CoreSight component identity register 2"]
pub mod dbgmcu_cidr2;
#[doc = "DBGMCU_CIDR3 (r) register accessor: DBGMCU CoreSight component identity register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_cidr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_cidr3`]
module"]
#[doc(alias = "DBGMCU_CIDR3")]
pub type DbgmcuCidr3 = crate::Reg<dbgmcu_cidr3::DbgmcuCidr3Spec>;
#[doc = "DBGMCU CoreSight component identity register 3"]
pub mod dbgmcu_cidr3;
