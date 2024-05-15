#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    flash_acr: FlashAcr,
    _reserved1: [u8; 0x04],
    flash_keyr: FlashKeyr,
    flash_optkeyr: FlashOptkeyr,
    flash_sr: FlashSr,
    flash_cr: FlashCr,
    flash_eccr: FlashEccr,
    _reserved6: [u8; 0x04],
    flash_optr: FlashOptr,
    _reserved7: [u8; 0x08],
    flash_wrp1ar: FlashWrp1ar,
    flash_wrp1br: FlashWrp1br,
    _reserved9: [u8; 0x4c],
    flash_secr: FlashSecr,
}
impl RegisterBlock {
    #[doc = "0x00 - FLASH access control register"]
    #[inline(always)]
    pub const fn flash_acr(&self) -> &FlashAcr {
        &self.flash_acr
    }
    #[doc = "0x08 - FLASH key register"]
    #[inline(always)]
    pub const fn flash_keyr(&self) -> &FlashKeyr {
        &self.flash_keyr
    }
    #[doc = "0x0c - FLASH option key register"]
    #[inline(always)]
    pub const fn flash_optkeyr(&self) -> &FlashOptkeyr {
        &self.flash_optkeyr
    }
    #[doc = "0x10 - FLASH status register"]
    #[inline(always)]
    pub const fn flash_sr(&self) -> &FlashSr {
        &self.flash_sr
    }
    #[doc = "0x14 - FLASH control register"]
    #[inline(always)]
    pub const fn flash_cr(&self) -> &FlashCr {
        &self.flash_cr
    }
    #[doc = "0x18 - FLASH ECC register"]
    #[inline(always)]
    pub const fn flash_eccr(&self) -> &FlashEccr {
        &self.flash_eccr
    }
    #[doc = "0x20 - FLASH option register"]
    #[inline(always)]
    pub const fn flash_optr(&self) -> &FlashOptr {
        &self.flash_optr
    }
    #[doc = "0x2c - FLASH WRP area A address register"]
    #[inline(always)]
    pub const fn flash_wrp1ar(&self) -> &FlashWrp1ar {
        &self.flash_wrp1ar
    }
    #[doc = "0x30 - FLASH WRP area B address register"]
    #[inline(always)]
    pub const fn flash_wrp1br(&self) -> &FlashWrp1br {
        &self.flash_wrp1br
    }
    #[doc = "0x80 - FLASH security register"]
    #[inline(always)]
    pub const fn flash_secr(&self) -> &FlashSecr {
        &self.flash_secr
    }
}
#[doc = "FLASH_ACR (rw) register accessor: FLASH access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_acr`]
module"]
#[doc(alias = "FLASH_ACR")]
pub type FlashAcr = crate::Reg<flash_acr::FlashAcrSpec>;
#[doc = "FLASH access control register"]
pub mod flash_acr;
#[doc = "FLASH_KEYR (w) register accessor: FLASH key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_keyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_keyr`]
module"]
#[doc(alias = "FLASH_KEYR")]
pub type FlashKeyr = crate::Reg<flash_keyr::FlashKeyrSpec>;
#[doc = "FLASH key register"]
pub mod flash_keyr;
#[doc = "FLASH_OPTKEYR (w) register accessor: FLASH option key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_optkeyr`]
module"]
#[doc(alias = "FLASH_OPTKEYR")]
pub type FlashOptkeyr = crate::Reg<flash_optkeyr::FlashOptkeyrSpec>;
#[doc = "FLASH option key register"]
pub mod flash_optkeyr;
#[doc = "FLASH_SR (rw) register accessor: FLASH status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sr`]
module"]
#[doc(alias = "FLASH_SR")]
pub type FlashSr = crate::Reg<flash_sr::FlashSrSpec>;
#[doc = "FLASH status register"]
pub mod flash_sr;
#[doc = "FLASH_CR (rw) register accessor: FLASH control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_cr`]
module"]
#[doc(alias = "FLASH_CR")]
pub type FlashCr = crate::Reg<flash_cr::FlashCrSpec>;
#[doc = "FLASH control register"]
pub mod flash_cr;
#[doc = "FLASH_ECCR (rw) register accessor: FLASH ECC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_eccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_eccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_eccr`]
module"]
#[doc(alias = "FLASH_ECCR")]
pub type FlashEccr = crate::Reg<flash_eccr::FlashEccrSpec>;
#[doc = "FLASH ECC register"]
pub mod flash_eccr;
#[doc = "FLASH_OPTR (rw) register accessor: FLASH option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_optr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_optr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_optr`]
module"]
#[doc(alias = "FLASH_OPTR")]
pub type FlashOptr = crate::Reg<flash_optr::FlashOptrSpec>;
#[doc = "FLASH option register"]
pub mod flash_optr;
#[doc = "FLASH_WRP1AR (rw) register accessor: FLASH WRP area A address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_wrp1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_wrp1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_wrp1ar`]
module"]
#[doc(alias = "FLASH_WRP1AR")]
pub type FlashWrp1ar = crate::Reg<flash_wrp1ar::FlashWrp1arSpec>;
#[doc = "FLASH WRP area A address register"]
pub mod flash_wrp1ar;
#[doc = "FLASH_WRP1BR (rw) register accessor: FLASH WRP area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_wrp1br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_wrp1br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_wrp1br`]
module"]
#[doc(alias = "FLASH_WRP1BR")]
pub type FlashWrp1br = crate::Reg<flash_wrp1br::FlashWrp1brSpec>;
#[doc = "FLASH WRP area B address register"]
pub mod flash_wrp1br;
#[doc = "FLASH_SECR (rw) register accessor: FLASH security register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_secr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_secr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_secr`]
module"]
#[doc(alias = "FLASH_SECR")]
pub type FlashSecr = crate::Reg<flash_secr::FlashSecrSpec>;
#[doc = "FLASH security register"]
pub mod flash_secr;
