#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dmamux_c0cr: DmamuxC0cr,
    dmamux_c1cr: DmamuxC1cr,
    dmamux_c2cr: DmamuxC2cr,
    dmamux_c3cr: DmamuxC3cr,
    dmamux_c4cr: DmamuxC4cr,
    dmamux_c5cr: DmamuxC5cr,
    dmamux_c6cr: DmamuxC6cr,
    dmamux_c7cr: DmamuxC7cr,
    dmamux_c8cr: DmamuxC8cr,
    dmamux_c9cr: DmamuxC9cr,
    dmamux_c10cr: DmamuxC10cr,
    dmamux_c11cr: DmamuxC11cr,
    _reserved12: [u8; 0x50],
    dmamux_csr: DmamuxCsr,
    dmamux_cfr: DmamuxCfr,
    _reserved14: [u8; 0x78],
    dmamux_rg0cr: DmamuxRg0cr,
    dmamux_rg1cr: DmamuxRg1cr,
    dmamux_rg2cr: DmamuxRg2cr,
    dmamux_rg3cr: DmamuxRg3cr,
    _reserved18: [u8; 0x30],
    dmamux_rgsr: DmamuxRgsr,
    dmamux_rgcfr: DmamuxRgcfr,
}
impl RegisterBlock {
    #[doc = "0x00 - DMAMUX request line multiplexer channel 0 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c0cr(&self) -> &DmamuxC0cr {
        &self.dmamux_c0cr
    }
    #[doc = "0x04 - DMAMUX request line multiplexer channel 1 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c1cr(&self) -> &DmamuxC1cr {
        &self.dmamux_c1cr
    }
    #[doc = "0x08 - DMAMUX request line multiplexer channel 2 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c2cr(&self) -> &DmamuxC2cr {
        &self.dmamux_c2cr
    }
    #[doc = "0x0c - DMAMUX request line multiplexer channel 3 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c3cr(&self) -> &DmamuxC3cr {
        &self.dmamux_c3cr
    }
    #[doc = "0x10 - DMAMUX request line multiplexer channel 4 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c4cr(&self) -> &DmamuxC4cr {
        &self.dmamux_c4cr
    }
    #[doc = "0x14 - DMAMUX request line multiplexer channel 5 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c5cr(&self) -> &DmamuxC5cr {
        &self.dmamux_c5cr
    }
    #[doc = "0x18 - DMAMUX request line multiplexer channel 6 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c6cr(&self) -> &DmamuxC6cr {
        &self.dmamux_c6cr
    }
    #[doc = "0x1c - DMAMUX request line multiplexer channel 7 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c7cr(&self) -> &DmamuxC7cr {
        &self.dmamux_c7cr
    }
    #[doc = "0x20 - DMAMUX request line multiplexer channel 8 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c8cr(&self) -> &DmamuxC8cr {
        &self.dmamux_c8cr
    }
    #[doc = "0x24 - DMAMUX request line multiplexer channel 9 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c9cr(&self) -> &DmamuxC9cr {
        &self.dmamux_c9cr
    }
    #[doc = "0x28 - DMAMUX request line multiplexer channel 10 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c10cr(&self) -> &DmamuxC10cr {
        &self.dmamux_c10cr
    }
    #[doc = "0x2c - DMAMUX request line multiplexer channel 11 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c11cr(&self) -> &DmamuxC11cr {
        &self.dmamux_c11cr
    }
    #[doc = "0x80 - DMAMUX request line multiplexer interrupt channel status register"]
    #[inline(always)]
    pub const fn dmamux_csr(&self) -> &DmamuxCsr {
        &self.dmamux_csr
    }
    #[doc = "0x84 - DMAMUX request line multiplexer interrupt clear flag register"]
    #[inline(always)]
    pub const fn dmamux_cfr(&self) -> &DmamuxCfr {
        &self.dmamux_cfr
    }
    #[doc = "0x100 - DMAMUX request generator channel 0 configuration register"]
    #[inline(always)]
    pub const fn dmamux_rg0cr(&self) -> &DmamuxRg0cr {
        &self.dmamux_rg0cr
    }
    #[doc = "0x104 - DMAMUX request generator channel 1 configuration register"]
    #[inline(always)]
    pub const fn dmamux_rg1cr(&self) -> &DmamuxRg1cr {
        &self.dmamux_rg1cr
    }
    #[doc = "0x108 - DMAMUX request generator channel 2 configuration register"]
    #[inline(always)]
    pub const fn dmamux_rg2cr(&self) -> &DmamuxRg2cr {
        &self.dmamux_rg2cr
    }
    #[doc = "0x10c - DMAMUX request generator channel 3 configuration register"]
    #[inline(always)]
    pub const fn dmamux_rg3cr(&self) -> &DmamuxRg3cr {
        &self.dmamux_rg3cr
    }
    #[doc = "0x140 - DMAMUX request generator interrupt status register"]
    #[inline(always)]
    pub const fn dmamux_rgsr(&self) -> &DmamuxRgsr {
        &self.dmamux_rgsr
    }
    #[doc = "0x144 - DMAMUX request generator interrupt clear flag register"]
    #[inline(always)]
    pub const fn dmamux_rgcfr(&self) -> &DmamuxRgcfr {
        &self.dmamux_rgcfr
    }
}
#[doc = "DMAMUX_C0CR (rw) register accessor: DMAMUX request line multiplexer channel 0 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c0cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c0cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c0cr`]
module"]
#[doc(alias = "DMAMUX_C0CR")]
pub type DmamuxC0cr = crate::Reg<dmamux_c0cr::DmamuxC0crSpec>;
#[doc = "DMAMUX request line multiplexer channel 0 configuration register"]
pub mod dmamux_c0cr;
#[doc = "DMAMUX_C1CR (rw) register accessor: DMAMUX request line multiplexer channel 1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c1cr`]
module"]
#[doc(alias = "DMAMUX_C1CR")]
pub type DmamuxC1cr = crate::Reg<dmamux_c1cr::DmamuxC1crSpec>;
#[doc = "DMAMUX request line multiplexer channel 1 configuration register"]
pub mod dmamux_c1cr;
#[doc = "DMAMUX_C2CR (rw) register accessor: DMAMUX request line multiplexer channel 2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c2cr`]
module"]
#[doc(alias = "DMAMUX_C2CR")]
pub type DmamuxC2cr = crate::Reg<dmamux_c2cr::DmamuxC2crSpec>;
#[doc = "DMAMUX request line multiplexer channel 2 configuration register"]
pub mod dmamux_c2cr;
#[doc = "DMAMUX_C3CR (rw) register accessor: DMAMUX request line multiplexer channel 3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c3cr`]
module"]
#[doc(alias = "DMAMUX_C3CR")]
pub type DmamuxC3cr = crate::Reg<dmamux_c3cr::DmamuxC3crSpec>;
#[doc = "DMAMUX request line multiplexer channel 3 configuration register"]
pub mod dmamux_c3cr;
#[doc = "DMAMUX_C4CR (rw) register accessor: DMAMUX request line multiplexer channel 4 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c4cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c4cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c4cr`]
module"]
#[doc(alias = "DMAMUX_C4CR")]
pub type DmamuxC4cr = crate::Reg<dmamux_c4cr::DmamuxC4crSpec>;
#[doc = "DMAMUX request line multiplexer channel 4 configuration register"]
pub mod dmamux_c4cr;
#[doc = "DMAMUX_C5CR (rw) register accessor: DMAMUX request line multiplexer channel 5 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c5cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c5cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c5cr`]
module"]
#[doc(alias = "DMAMUX_C5CR")]
pub type DmamuxC5cr = crate::Reg<dmamux_c5cr::DmamuxC5crSpec>;
#[doc = "DMAMUX request line multiplexer channel 5 configuration register"]
pub mod dmamux_c5cr;
#[doc = "DMAMUX_C6CR (rw) register accessor: DMAMUX request line multiplexer channel 6 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c6cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c6cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c6cr`]
module"]
#[doc(alias = "DMAMUX_C6CR")]
pub type DmamuxC6cr = crate::Reg<dmamux_c6cr::DmamuxC6crSpec>;
#[doc = "DMAMUX request line multiplexer channel 6 configuration register"]
pub mod dmamux_c6cr;
#[doc = "DMAMUX_C7CR (rw) register accessor: DMAMUX request line multiplexer channel 7 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c7cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c7cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c7cr`]
module"]
#[doc(alias = "DMAMUX_C7CR")]
pub type DmamuxC7cr = crate::Reg<dmamux_c7cr::DmamuxC7crSpec>;
#[doc = "DMAMUX request line multiplexer channel 7 configuration register"]
pub mod dmamux_c7cr;
#[doc = "DMAMUX_C8CR (rw) register accessor: DMAMUX request line multiplexer channel 8 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c8cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c8cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c8cr`]
module"]
#[doc(alias = "DMAMUX_C8CR")]
pub type DmamuxC8cr = crate::Reg<dmamux_c8cr::DmamuxC8crSpec>;
#[doc = "DMAMUX request line multiplexer channel 8 configuration register"]
pub mod dmamux_c8cr;
#[doc = "DMAMUX_C9CR (rw) register accessor: DMAMUX request line multiplexer channel 9 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c9cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c9cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c9cr`]
module"]
#[doc(alias = "DMAMUX_C9CR")]
pub type DmamuxC9cr = crate::Reg<dmamux_c9cr::DmamuxC9crSpec>;
#[doc = "DMAMUX request line multiplexer channel 9 configuration register"]
pub mod dmamux_c9cr;
#[doc = "DMAMUX_C10CR (rw) register accessor: DMAMUX request line multiplexer channel 10 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c10cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c10cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c10cr`]
module"]
#[doc(alias = "DMAMUX_C10CR")]
pub type DmamuxC10cr = crate::Reg<dmamux_c10cr::DmamuxC10crSpec>;
#[doc = "DMAMUX request line multiplexer channel 10 configuration register"]
pub mod dmamux_c10cr;
#[doc = "DMAMUX_C11CR (rw) register accessor: DMAMUX request line multiplexer channel 11 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c11cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c11cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c11cr`]
module"]
#[doc(alias = "DMAMUX_C11CR")]
pub type DmamuxC11cr = crate::Reg<dmamux_c11cr::DmamuxC11crSpec>;
#[doc = "DMAMUX request line multiplexer channel 11 configuration register"]
pub mod dmamux_c11cr;
#[doc = "DMAMUX_CSR (r) register accessor: DMAMUX request line multiplexer interrupt channel status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_csr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_csr`]
module"]
#[doc(alias = "DMAMUX_CSR")]
pub type DmamuxCsr = crate::Reg<dmamux_csr::DmamuxCsrSpec>;
#[doc = "DMAMUX request line multiplexer interrupt channel status register"]
pub mod dmamux_csr;
#[doc = "DMAMUX_CFR (w) register accessor: DMAMUX request line multiplexer interrupt clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_cfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_cfr`]
module"]
#[doc(alias = "DMAMUX_CFR")]
pub type DmamuxCfr = crate::Reg<dmamux_cfr::DmamuxCfrSpec>;
#[doc = "DMAMUX request line multiplexer interrupt clear flag register"]
pub mod dmamux_cfr;
#[doc = "DMAMUX_RG0CR (rw) register accessor: DMAMUX request generator channel 0 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg0cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg0cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rg0cr`]
module"]
#[doc(alias = "DMAMUX_RG0CR")]
pub type DmamuxRg0cr = crate::Reg<dmamux_rg0cr::DmamuxRg0crSpec>;
#[doc = "DMAMUX request generator channel 0 configuration register"]
pub mod dmamux_rg0cr;
#[doc = "DMAMUX_RG1CR (rw) register accessor: DMAMUX request generator channel 1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rg1cr`]
module"]
#[doc(alias = "DMAMUX_RG1CR")]
pub type DmamuxRg1cr = crate::Reg<dmamux_rg1cr::DmamuxRg1crSpec>;
#[doc = "DMAMUX request generator channel 1 configuration register"]
pub mod dmamux_rg1cr;
#[doc = "DMAMUX_RG2CR (rw) register accessor: DMAMUX request generator channel 2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rg2cr`]
module"]
#[doc(alias = "DMAMUX_RG2CR")]
pub type DmamuxRg2cr = crate::Reg<dmamux_rg2cr::DmamuxRg2crSpec>;
#[doc = "DMAMUX request generator channel 2 configuration register"]
pub mod dmamux_rg2cr;
#[doc = "DMAMUX_RG3CR (rw) register accessor: DMAMUX request generator channel 3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rg3cr`]
module"]
#[doc(alias = "DMAMUX_RG3CR")]
pub type DmamuxRg3cr = crate::Reg<dmamux_rg3cr::DmamuxRg3crSpec>;
#[doc = "DMAMUX request generator channel 3 configuration register"]
pub mod dmamux_rg3cr;
#[doc = "DMAMUX_RGSR (r) register accessor: DMAMUX request generator interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rgsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rgsr`]
module"]
#[doc(alias = "DMAMUX_RGSR")]
pub type DmamuxRgsr = crate::Reg<dmamux_rgsr::DmamuxRgsrSpec>;
#[doc = "DMAMUX request generator interrupt status register"]
pub mod dmamux_rgsr;
#[doc = "DMAMUX_RGCFR (w) register accessor: DMAMUX request generator interrupt clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rgcfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rgcfr`]
module"]
#[doc(alias = "DMAMUX_RGCFR")]
pub type DmamuxRgcfr = crate::Reg<dmamux_rgcfr::DmamuxRgcfrSpec>;
#[doc = "DMAMUX request generator interrupt clear flag register"]
pub mod dmamux_rgcfr;
