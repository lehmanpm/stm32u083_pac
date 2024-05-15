#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    syscfg_cfgr1: SyscfgCfgr1,
    _reserved1: [u8; 0x14],
    syscfg_cfgr2: SyscfgCfgr2,
    syscfg_scsr: SyscfgScsr,
    syscfg_skr: SyscfgSkr,
    syscfg_tsccr: SyscfgTsccr,
    _reserved5: [u8; 0x58],
    syscfg_itline0: SyscfgItline0,
    syscfg_itline1: SyscfgItline1,
    syscfg_itline2: SyscfgItline2,
    syscfg_itline3: SyscfgItline3,
    syscfg_itline4: SyscfgItline4,
    syscfg_itline5: SyscfgItline5,
    syscfg_itline6: SyscfgItline6,
    syscfg_itline7: SyscfgItline7,
    syscfg_itline8: SyscfgItline8,
    syscfg_itline9: SyscfgItline9,
    syscfg_itline10: SyscfgItline10,
    syscfg_itline11: SyscfgItline11,
    syscfg_itline12: SyscfgItline12,
    syscfg_itline13: SyscfgItline13,
    syscfg_itline14: SyscfgItline14,
    syscfg_itline15: SyscfgItline15,
    syscfg_itline16: SyscfgItline16,
    syscfg_itline17: SyscfgItline17,
    syscfg_itline18: SyscfgItline18,
    syscfg_itline19: SyscfgItline19,
    syscfg_itline20: SyscfgItline20,
    syscfg_itline21: SyscfgItline21,
    syscfg_itline22: SyscfgItline22,
    syscfg_itline23: SyscfgItline23,
    syscfg_itline24: SyscfgItline24,
    syscfg_itline25: SyscfgItline25,
    syscfg_itline26: SyscfgItline26,
    syscfg_itline27: SyscfgItline27,
    syscfg_itline28: SyscfgItline28,
    syscfg_itline29: SyscfgItline29,
    syscfg_itline30: SyscfgItline30,
    syscfg_itline31: SyscfgItline31,
}
impl RegisterBlock {
    #[doc = "0x00 - SYSCFG configuration register 1"]
    #[inline(always)]
    pub const fn syscfg_cfgr1(&self) -> &SyscfgCfgr1 {
        &self.syscfg_cfgr1
    }
    #[doc = "0x18 - SYSCFG configuration register 2"]
    #[inline(always)]
    pub const fn syscfg_cfgr2(&self) -> &SyscfgCfgr2 {
        &self.syscfg_cfgr2
    }
    #[doc = "0x1c - SYSCFG SRAM2 control and status register"]
    #[inline(always)]
    pub const fn syscfg_scsr(&self) -> &SyscfgScsr {
        &self.syscfg_scsr
    }
    #[doc = "0x20 - SYSCFG SRAM2 key register"]
    #[inline(always)]
    pub const fn syscfg_skr(&self) -> &SyscfgSkr {
        &self.syscfg_skr
    }
    #[doc = "0x24 - SYSCFG TSC comparator register"]
    #[inline(always)]
    pub const fn syscfg_tsccr(&self) -> &SyscfgTsccr {
        &self.syscfg_tsccr
    }
    #[doc = "0x80 - SYSCFG interrupt line 0 status register"]
    #[inline(always)]
    pub const fn syscfg_itline0(&self) -> &SyscfgItline0 {
        &self.syscfg_itline0
    }
    #[doc = "0x84 - SYSCFG interrupt line 1 status register"]
    #[inline(always)]
    pub const fn syscfg_itline1(&self) -> &SyscfgItline1 {
        &self.syscfg_itline1
    }
    #[doc = "0x88 - SYSCFG interrupt line 2 status register"]
    #[inline(always)]
    pub const fn syscfg_itline2(&self) -> &SyscfgItline2 {
        &self.syscfg_itline2
    }
    #[doc = "0x8c - SYSCFG interrupt line 3 status register"]
    #[inline(always)]
    pub const fn syscfg_itline3(&self) -> &SyscfgItline3 {
        &self.syscfg_itline3
    }
    #[doc = "0x90 - SYSCFG interrupt line 4 status register"]
    #[inline(always)]
    pub const fn syscfg_itline4(&self) -> &SyscfgItline4 {
        &self.syscfg_itline4
    }
    #[doc = "0x94 - SYSCFG interrupt line 5 status register"]
    #[inline(always)]
    pub const fn syscfg_itline5(&self) -> &SyscfgItline5 {
        &self.syscfg_itline5
    }
    #[doc = "0x98 - SYSCFG interrupt line 6 status register"]
    #[inline(always)]
    pub const fn syscfg_itline6(&self) -> &SyscfgItline6 {
        &self.syscfg_itline6
    }
    #[doc = "0x9c - SYSCFG interrupt line 7 status register"]
    #[inline(always)]
    pub const fn syscfg_itline7(&self) -> &SyscfgItline7 {
        &self.syscfg_itline7
    }
    #[doc = "0xa0 - SYSCFG interrupt line 8 status register"]
    #[inline(always)]
    pub const fn syscfg_itline8(&self) -> &SyscfgItline8 {
        &self.syscfg_itline8
    }
    #[doc = "0xa4 - SYSCFG interrupt line 9 status register"]
    #[inline(always)]
    pub const fn syscfg_itline9(&self) -> &SyscfgItline9 {
        &self.syscfg_itline9
    }
    #[doc = "0xa8 - SYSCFG interrupt line 10 status register"]
    #[inline(always)]
    pub const fn syscfg_itline10(&self) -> &SyscfgItline10 {
        &self.syscfg_itline10
    }
    #[doc = "0xac - SYSCFG interrupt line 11 status register"]
    #[inline(always)]
    pub const fn syscfg_itline11(&self) -> &SyscfgItline11 {
        &self.syscfg_itline11
    }
    #[doc = "0xb0 - SYSCFG interrupt line 12 status register"]
    #[inline(always)]
    pub const fn syscfg_itline12(&self) -> &SyscfgItline12 {
        &self.syscfg_itline12
    }
    #[doc = "0xb4 - SYSCFG interrupt line 13 status register"]
    #[inline(always)]
    pub const fn syscfg_itline13(&self) -> &SyscfgItline13 {
        &self.syscfg_itline13
    }
    #[doc = "0xb8 - SYSCFG interrupt line 14 status register"]
    #[inline(always)]
    pub const fn syscfg_itline14(&self) -> &SyscfgItline14 {
        &self.syscfg_itline14
    }
    #[doc = "0xbc - SYSCFG interrupt line 15 status register"]
    #[inline(always)]
    pub const fn syscfg_itline15(&self) -> &SyscfgItline15 {
        &self.syscfg_itline15
    }
    #[doc = "0xc0 - SYSCFG interrupt line 16 status register"]
    #[inline(always)]
    pub const fn syscfg_itline16(&self) -> &SyscfgItline16 {
        &self.syscfg_itline16
    }
    #[doc = "0xc4 - SYSCFG interrupt line 17 status register"]
    #[inline(always)]
    pub const fn syscfg_itline17(&self) -> &SyscfgItline17 {
        &self.syscfg_itline17
    }
    #[doc = "0xc8 - SYSCFG interrupt line 18 status register"]
    #[inline(always)]
    pub const fn syscfg_itline18(&self) -> &SyscfgItline18 {
        &self.syscfg_itline18
    }
    #[doc = "0xcc - SYSCFG interrupt line 19 status register"]
    #[inline(always)]
    pub const fn syscfg_itline19(&self) -> &SyscfgItline19 {
        &self.syscfg_itline19
    }
    #[doc = "0xd0 - SYSCFG interrupt line 20 status register"]
    #[inline(always)]
    pub const fn syscfg_itline20(&self) -> &SyscfgItline20 {
        &self.syscfg_itline20
    }
    #[doc = "0xd4 - SYSCFG interrupt line 21 status register"]
    #[inline(always)]
    pub const fn syscfg_itline21(&self) -> &SyscfgItline21 {
        &self.syscfg_itline21
    }
    #[doc = "0xd8 - SYSCFG interrupt line 22 status register"]
    #[inline(always)]
    pub const fn syscfg_itline22(&self) -> &SyscfgItline22 {
        &self.syscfg_itline22
    }
    #[doc = "0xdc - SYSCFG interrupt line 23 status register"]
    #[inline(always)]
    pub const fn syscfg_itline23(&self) -> &SyscfgItline23 {
        &self.syscfg_itline23
    }
    #[doc = "0xe0 - SYSCFG interrupt line 24 status register"]
    #[inline(always)]
    pub const fn syscfg_itline24(&self) -> &SyscfgItline24 {
        &self.syscfg_itline24
    }
    #[doc = "0xe4 - SYSCFG interrupt line 25 status register"]
    #[inline(always)]
    pub const fn syscfg_itline25(&self) -> &SyscfgItline25 {
        &self.syscfg_itline25
    }
    #[doc = "0xe8 - SYSCFG interrupt line 26 status register"]
    #[inline(always)]
    pub const fn syscfg_itline26(&self) -> &SyscfgItline26 {
        &self.syscfg_itline26
    }
    #[doc = "0xec - SYSCFG interrupt line 27 status register"]
    #[inline(always)]
    pub const fn syscfg_itline27(&self) -> &SyscfgItline27 {
        &self.syscfg_itline27
    }
    #[doc = "0xf0 - SYSCFG interrupt line 28 status register"]
    #[inline(always)]
    pub const fn syscfg_itline28(&self) -> &SyscfgItline28 {
        &self.syscfg_itline28
    }
    #[doc = "0xf4 - SYSCFG interrupt line 29 status register"]
    #[inline(always)]
    pub const fn syscfg_itline29(&self) -> &SyscfgItline29 {
        &self.syscfg_itline29
    }
    #[doc = "0xf8 - SYSCFG interrupt line 30 status register"]
    #[inline(always)]
    pub const fn syscfg_itline30(&self) -> &SyscfgItline30 {
        &self.syscfg_itline30
    }
    #[doc = "0xfc - SYSCFG interrupt line 31 status register"]
    #[inline(always)]
    pub const fn syscfg_itline31(&self) -> &SyscfgItline31 {
        &self.syscfg_itline31
    }
}
#[doc = "SYSCFG_CFGR1 (rw) register accessor: SYSCFG configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_cfgr1`]
module"]
#[doc(alias = "SYSCFG_CFGR1")]
pub type SyscfgCfgr1 = crate::Reg<syscfg_cfgr1::SyscfgCfgr1Spec>;
#[doc = "SYSCFG configuration register 1"]
pub mod syscfg_cfgr1;
#[doc = "SYSCFG_CFGR2 (rw) register accessor: SYSCFG configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_cfgr2`]
module"]
#[doc(alias = "SYSCFG_CFGR2")]
pub type SyscfgCfgr2 = crate::Reg<syscfg_cfgr2::SyscfgCfgr2Spec>;
#[doc = "SYSCFG configuration register 2"]
pub mod syscfg_cfgr2;
#[doc = "SYSCFG_SCSR (rw) register accessor: SYSCFG SRAM2 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_scsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_scsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_scsr`]
module"]
#[doc(alias = "SYSCFG_SCSR")]
pub type SyscfgScsr = crate::Reg<syscfg_scsr::SyscfgScsrSpec>;
#[doc = "SYSCFG SRAM2 control and status register"]
pub mod syscfg_scsr;
#[doc = "SYSCFG_SKR (w) register accessor: SYSCFG SRAM2 key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_skr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_skr`]
module"]
#[doc(alias = "SYSCFG_SKR")]
pub type SyscfgSkr = crate::Reg<syscfg_skr::SyscfgSkrSpec>;
#[doc = "SYSCFG SRAM2 key register"]
pub mod syscfg_skr;
#[doc = "SYSCFG_TSCCR (rw) register accessor: SYSCFG TSC comparator register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_tsccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_tsccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_tsccr`]
module"]
#[doc(alias = "SYSCFG_TSCCR")]
pub type SyscfgTsccr = crate::Reg<syscfg_tsccr::SyscfgTsccrSpec>;
#[doc = "SYSCFG TSC comparator register"]
pub mod syscfg_tsccr;
#[doc = "SYSCFG_ITLINE0 (r) register accessor: SYSCFG interrupt line 0 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline0`]
module"]
#[doc(alias = "SYSCFG_ITLINE0")]
pub type SyscfgItline0 = crate::Reg<syscfg_itline0::SyscfgItline0Spec>;
#[doc = "SYSCFG interrupt line 0 status register"]
pub mod syscfg_itline0;
#[doc = "SYSCFG_ITLINE1 (r) register accessor: SYSCFG interrupt line 1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline1`]
module"]
#[doc(alias = "SYSCFG_ITLINE1")]
pub type SyscfgItline1 = crate::Reg<syscfg_itline1::SyscfgItline1Spec>;
#[doc = "SYSCFG interrupt line 1 status register"]
pub mod syscfg_itline1;
#[doc = "SYSCFG_ITLINE2 (r) register accessor: SYSCFG interrupt line 2 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline2`]
module"]
#[doc(alias = "SYSCFG_ITLINE2")]
pub type SyscfgItline2 = crate::Reg<syscfg_itline2::SyscfgItline2Spec>;
#[doc = "SYSCFG interrupt line 2 status register"]
pub mod syscfg_itline2;
#[doc = "SYSCFG_ITLINE3 (r) register accessor: SYSCFG interrupt line 3 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline3`]
module"]
#[doc(alias = "SYSCFG_ITLINE3")]
pub type SyscfgItline3 = crate::Reg<syscfg_itline3::SyscfgItline3Spec>;
#[doc = "SYSCFG interrupt line 3 status register"]
pub mod syscfg_itline3;
#[doc = "SYSCFG_ITLINE4 (r) register accessor: SYSCFG interrupt line 4 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline4`]
module"]
#[doc(alias = "SYSCFG_ITLINE4")]
pub type SyscfgItline4 = crate::Reg<syscfg_itline4::SyscfgItline4Spec>;
#[doc = "SYSCFG interrupt line 4 status register"]
pub mod syscfg_itline4;
#[doc = "SYSCFG_ITLINE5 (r) register accessor: SYSCFG interrupt line 5 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline5`]
module"]
#[doc(alias = "SYSCFG_ITLINE5")]
pub type SyscfgItline5 = crate::Reg<syscfg_itline5::SyscfgItline5Spec>;
#[doc = "SYSCFG interrupt line 5 status register"]
pub mod syscfg_itline5;
#[doc = "SYSCFG_ITLINE6 (r) register accessor: SYSCFG interrupt line 6 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline6`]
module"]
#[doc(alias = "SYSCFG_ITLINE6")]
pub type SyscfgItline6 = crate::Reg<syscfg_itline6::SyscfgItline6Spec>;
#[doc = "SYSCFG interrupt line 6 status register"]
pub mod syscfg_itline6;
#[doc = "SYSCFG_ITLINE7 (r) register accessor: SYSCFG interrupt line 7 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline7`]
module"]
#[doc(alias = "SYSCFG_ITLINE7")]
pub type SyscfgItline7 = crate::Reg<syscfg_itline7::SyscfgItline7Spec>;
#[doc = "SYSCFG interrupt line 7 status register"]
pub mod syscfg_itline7;
#[doc = "SYSCFG_ITLINE8 (r) register accessor: SYSCFG interrupt line 8 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline8`]
module"]
#[doc(alias = "SYSCFG_ITLINE8")]
pub type SyscfgItline8 = crate::Reg<syscfg_itline8::SyscfgItline8Spec>;
#[doc = "SYSCFG interrupt line 8 status register"]
pub mod syscfg_itline8;
#[doc = "SYSCFG_ITLINE9 (r) register accessor: SYSCFG interrupt line 9 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline9`]
module"]
#[doc(alias = "SYSCFG_ITLINE9")]
pub type SyscfgItline9 = crate::Reg<syscfg_itline9::SyscfgItline9Spec>;
#[doc = "SYSCFG interrupt line 9 status register"]
pub mod syscfg_itline9;
#[doc = "SYSCFG_ITLINE10 (r) register accessor: SYSCFG interrupt line 10 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline10`]
module"]
#[doc(alias = "SYSCFG_ITLINE10")]
pub type SyscfgItline10 = crate::Reg<syscfg_itline10::SyscfgItline10Spec>;
#[doc = "SYSCFG interrupt line 10 status register"]
pub mod syscfg_itline10;
#[doc = "SYSCFG_ITLINE11 (r) register accessor: SYSCFG interrupt line 11 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline11`]
module"]
#[doc(alias = "SYSCFG_ITLINE11")]
pub type SyscfgItline11 = crate::Reg<syscfg_itline11::SyscfgItline11Spec>;
#[doc = "SYSCFG interrupt line 11 status register"]
pub mod syscfg_itline11;
#[doc = "SYSCFG_ITLINE12 (r) register accessor: SYSCFG interrupt line 12 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline12`]
module"]
#[doc(alias = "SYSCFG_ITLINE12")]
pub type SyscfgItline12 = crate::Reg<syscfg_itline12::SyscfgItline12Spec>;
#[doc = "SYSCFG interrupt line 12 status register"]
pub mod syscfg_itline12;
#[doc = "SYSCFG_ITLINE13 (r) register accessor: SYSCFG interrupt line 13 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline13`]
module"]
#[doc(alias = "SYSCFG_ITLINE13")]
pub type SyscfgItline13 = crate::Reg<syscfg_itline13::SyscfgItline13Spec>;
#[doc = "SYSCFG interrupt line 13 status register"]
pub mod syscfg_itline13;
#[doc = "SYSCFG_ITLINE14 (r) register accessor: SYSCFG interrupt line 14 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline14::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline14`]
module"]
#[doc(alias = "SYSCFG_ITLINE14")]
pub type SyscfgItline14 = crate::Reg<syscfg_itline14::SyscfgItline14Spec>;
#[doc = "SYSCFG interrupt line 14 status register"]
pub mod syscfg_itline14;
#[doc = "SYSCFG_ITLINE15 (r) register accessor: SYSCFG interrupt line 15 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline15::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline15`]
module"]
#[doc(alias = "SYSCFG_ITLINE15")]
pub type SyscfgItline15 = crate::Reg<syscfg_itline15::SyscfgItline15Spec>;
#[doc = "SYSCFG interrupt line 15 status register"]
pub mod syscfg_itline15;
#[doc = "SYSCFG_ITLINE16 (r) register accessor: SYSCFG interrupt line 16 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline16::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline16`]
module"]
#[doc(alias = "SYSCFG_ITLINE16")]
pub type SyscfgItline16 = crate::Reg<syscfg_itline16::SyscfgItline16Spec>;
#[doc = "SYSCFG interrupt line 16 status register"]
pub mod syscfg_itline16;
#[doc = "SYSCFG_ITLINE17 (r) register accessor: SYSCFG interrupt line 17 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline17::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline17`]
module"]
#[doc(alias = "SYSCFG_ITLINE17")]
pub type SyscfgItline17 = crate::Reg<syscfg_itline17::SyscfgItline17Spec>;
#[doc = "SYSCFG interrupt line 17 status register"]
pub mod syscfg_itline17;
#[doc = "SYSCFG_ITLINE18 (r) register accessor: SYSCFG interrupt line 18 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline18::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline18`]
module"]
#[doc(alias = "SYSCFG_ITLINE18")]
pub type SyscfgItline18 = crate::Reg<syscfg_itline18::SyscfgItline18Spec>;
#[doc = "SYSCFG interrupt line 18 status register"]
pub mod syscfg_itline18;
#[doc = "SYSCFG_ITLINE19 (r) register accessor: SYSCFG interrupt line 19 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline19::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline19`]
module"]
#[doc(alias = "SYSCFG_ITLINE19")]
pub type SyscfgItline19 = crate::Reg<syscfg_itline19::SyscfgItline19Spec>;
#[doc = "SYSCFG interrupt line 19 status register"]
pub mod syscfg_itline19;
#[doc = "SYSCFG_ITLINE20 (r) register accessor: SYSCFG interrupt line 20 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline20::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline20`]
module"]
#[doc(alias = "SYSCFG_ITLINE20")]
pub type SyscfgItline20 = crate::Reg<syscfg_itline20::SyscfgItline20Spec>;
#[doc = "SYSCFG interrupt line 20 status register"]
pub mod syscfg_itline20;
#[doc = "SYSCFG_ITLINE21 (r) register accessor: SYSCFG interrupt line 21 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline21::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline21`]
module"]
#[doc(alias = "SYSCFG_ITLINE21")]
pub type SyscfgItline21 = crate::Reg<syscfg_itline21::SyscfgItline21Spec>;
#[doc = "SYSCFG interrupt line 21 status register"]
pub mod syscfg_itline21;
#[doc = "SYSCFG_ITLINE22 (r) register accessor: SYSCFG interrupt line 22 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline22::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline22`]
module"]
#[doc(alias = "SYSCFG_ITLINE22")]
pub type SyscfgItline22 = crate::Reg<syscfg_itline22::SyscfgItline22Spec>;
#[doc = "SYSCFG interrupt line 22 status register"]
pub mod syscfg_itline22;
#[doc = "SYSCFG_ITLINE23 (r) register accessor: SYSCFG interrupt line 23 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline23::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline23`]
module"]
#[doc(alias = "SYSCFG_ITLINE23")]
pub type SyscfgItline23 = crate::Reg<syscfg_itline23::SyscfgItline23Spec>;
#[doc = "SYSCFG interrupt line 23 status register"]
pub mod syscfg_itline23;
#[doc = "SYSCFG_ITLINE24 (r) register accessor: SYSCFG interrupt line 24 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline24::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline24`]
module"]
#[doc(alias = "SYSCFG_ITLINE24")]
pub type SyscfgItline24 = crate::Reg<syscfg_itline24::SyscfgItline24Spec>;
#[doc = "SYSCFG interrupt line 24 status register"]
pub mod syscfg_itline24;
#[doc = "SYSCFG_ITLINE25 (r) register accessor: SYSCFG interrupt line 25 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline25::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline25`]
module"]
#[doc(alias = "SYSCFG_ITLINE25")]
pub type SyscfgItline25 = crate::Reg<syscfg_itline25::SyscfgItline25Spec>;
#[doc = "SYSCFG interrupt line 25 status register"]
pub mod syscfg_itline25;
#[doc = "SYSCFG_ITLINE26 (r) register accessor: SYSCFG interrupt line 26 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline26::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline26`]
module"]
#[doc(alias = "SYSCFG_ITLINE26")]
pub type SyscfgItline26 = crate::Reg<syscfg_itline26::SyscfgItline26Spec>;
#[doc = "SYSCFG interrupt line 26 status register"]
pub mod syscfg_itline26;
#[doc = "SYSCFG_ITLINE27 (r) register accessor: SYSCFG interrupt line 27 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline27::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline27`]
module"]
#[doc(alias = "SYSCFG_ITLINE27")]
pub type SyscfgItline27 = crate::Reg<syscfg_itline27::SyscfgItline27Spec>;
#[doc = "SYSCFG interrupt line 27 status register"]
pub mod syscfg_itline27;
#[doc = "SYSCFG_ITLINE28 (r) register accessor: SYSCFG interrupt line 28 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline28::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline28`]
module"]
#[doc(alias = "SYSCFG_ITLINE28")]
pub type SyscfgItline28 = crate::Reg<syscfg_itline28::SyscfgItline28Spec>;
#[doc = "SYSCFG interrupt line 28 status register"]
pub mod syscfg_itline28;
#[doc = "SYSCFG_ITLINE29 (r) register accessor: SYSCFG interrupt line 29 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline29::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline29`]
module"]
#[doc(alias = "SYSCFG_ITLINE29")]
pub type SyscfgItline29 = crate::Reg<syscfg_itline29::SyscfgItline29Spec>;
#[doc = "SYSCFG interrupt line 29 status register"]
pub mod syscfg_itline29;
#[doc = "SYSCFG_ITLINE30 (r) register accessor: SYSCFG interrupt line 30 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline30::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline30`]
module"]
#[doc(alias = "SYSCFG_ITLINE30")]
pub type SyscfgItline30 = crate::Reg<syscfg_itline30::SyscfgItline30Spec>;
#[doc = "SYSCFG interrupt line 30 status register"]
pub mod syscfg_itline30;
#[doc = "SYSCFG_ITLINE31 (r) register accessor: SYSCFG interrupt line 31 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline31::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_itline31`]
module"]
#[doc(alias = "SYSCFG_ITLINE31")]
pub type SyscfgItline31 = crate::Reg<syscfg_itline31::SyscfgItline31Spec>;
#[doc = "SYSCFG interrupt line 31 status register"]
pub mod syscfg_itline31;
