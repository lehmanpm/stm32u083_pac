#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpioc_moder: GpiocModer,
    gpioc_otyper: GpiocOtyper,
    gpioc_ospeedr: GpiocOspeedr,
    gpioc_pupdr: GpiocPupdr,
    gpioc_idr: GpiocIdr,
    gpioc_odr: GpiocOdr,
    gpioc_bsrr: GpiocBsrr,
    gpioc_lckr: GpiocLckr,
    gpioc_afrl: GpiocAfrl,
    gpioc_afrh: GpiocAfrh,
    gpioc_brr: GpiocBrr,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    #[inline(always)]
    pub const fn gpioc_moder(&self) -> &GpiocModer {
        &self.gpioc_moder
    }
    #[doc = "0x04 - GPIO port output type register"]
    #[inline(always)]
    pub const fn gpioc_otyper(&self) -> &GpiocOtyper {
        &self.gpioc_otyper
    }
    #[doc = "0x08 - GPIO port output speed register"]
    #[inline(always)]
    pub const fn gpioc_ospeedr(&self) -> &GpiocOspeedr {
        &self.gpioc_ospeedr
    }
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    #[inline(always)]
    pub const fn gpioc_pupdr(&self) -> &GpiocPupdr {
        &self.gpioc_pupdr
    }
    #[doc = "0x10 - GPIO port input data register"]
    #[inline(always)]
    pub const fn gpioc_idr(&self) -> &GpiocIdr {
        &self.gpioc_idr
    }
    #[doc = "0x14 - GPIO port output data register"]
    #[inline(always)]
    pub const fn gpioc_odr(&self) -> &GpiocOdr {
        &self.gpioc_odr
    }
    #[doc = "0x18 - GPIO port bit set/reset register"]
    #[inline(always)]
    pub const fn gpioc_bsrr(&self) -> &GpiocBsrr {
        &self.gpioc_bsrr
    }
    #[doc = "0x1c - GPIO port configuration lock register"]
    #[inline(always)]
    pub const fn gpioc_lckr(&self) -> &GpiocLckr {
        &self.gpioc_lckr
    }
    #[doc = "0x20 - GPIO alternate function low register"]
    #[inline(always)]
    pub const fn gpioc_afrl(&self) -> &GpiocAfrl {
        &self.gpioc_afrl
    }
    #[doc = "0x24 - GPIO alternate function high register"]
    #[inline(always)]
    pub const fn gpioc_afrh(&self) -> &GpiocAfrh {
        &self.gpioc_afrh
    }
    #[doc = "0x28 - GPIO port bit reset register"]
    #[inline(always)]
    pub const fn gpioc_brr(&self) -> &GpiocBrr {
        &self.gpioc_brr
    }
}
#[doc = "GPIOC_MODER (rw) register accessor: GPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioc_moder::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioc_moder::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_moder`]
module"]
#[doc(alias = "GPIOC_MODER")]
pub type GpiocModer = crate::Reg<gpioc_moder::GpiocModerSpec>;
#[doc = "GPIO port mode register"]
pub mod gpioc_moder;
#[doc = "GPIOC_OTYPER (rw) register accessor: GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioc_otyper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioc_otyper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_otyper`]
module"]
#[doc(alias = "GPIOC_OTYPER")]
pub type GpiocOtyper = crate::Reg<gpioc_otyper::GpiocOtyperSpec>;
#[doc = "GPIO port output type register"]
pub mod gpioc_otyper;
#[doc = "GPIOC_OSPEEDR (rw) register accessor: GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioc_ospeedr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioc_ospeedr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_ospeedr`]
module"]
#[doc(alias = "GPIOC_OSPEEDR")]
pub type GpiocOspeedr = crate::Reg<gpioc_ospeedr::GpiocOspeedrSpec>;
#[doc = "GPIO port output speed register"]
pub mod gpioc_ospeedr;
#[doc = "GPIOC_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioc_pupdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioc_pupdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_pupdr`]
module"]
#[doc(alias = "GPIOC_PUPDR")]
pub type GpiocPupdr = crate::Reg<gpioc_pupdr::GpiocPupdrSpec>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioc_pupdr;
#[doc = "GPIOC_IDR (r) register accessor: GPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioc_idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_idr`]
module"]
#[doc(alias = "GPIOC_IDR")]
pub type GpiocIdr = crate::Reg<gpioc_idr::GpiocIdrSpec>;
#[doc = "GPIO port input data register"]
pub mod gpioc_idr;
#[doc = "GPIOC_ODR (rw) register accessor: GPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioc_odr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioc_odr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_odr`]
module"]
#[doc(alias = "GPIOC_ODR")]
pub type GpiocOdr = crate::Reg<gpioc_odr::GpiocOdrSpec>;
#[doc = "GPIO port output data register"]
pub mod gpioc_odr;
#[doc = "GPIOC_BSRR (w) register accessor: GPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioc_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_bsrr`]
module"]
#[doc(alias = "GPIOC_BSRR")]
pub type GpiocBsrr = crate::Reg<gpioc_bsrr::GpiocBsrrSpec>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpioc_bsrr;
#[doc = "GPIOC_LCKR (rw) register accessor: GPIO port configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioc_lckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioc_lckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_lckr`]
module"]
#[doc(alias = "GPIOC_LCKR")]
pub type GpiocLckr = crate::Reg<gpioc_lckr::GpiocLckrSpec>;
#[doc = "GPIO port configuration lock register"]
pub mod gpioc_lckr;
#[doc = "GPIOC_AFRL (rw) register accessor: GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioc_afrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioc_afrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_afrl`]
module"]
#[doc(alias = "GPIOC_AFRL")]
pub type GpiocAfrl = crate::Reg<gpioc_afrl::GpiocAfrlSpec>;
#[doc = "GPIO alternate function low register"]
pub mod gpioc_afrl;
#[doc = "GPIOC_AFRH (rw) register accessor: GPIO alternate function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioc_afrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioc_afrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_afrh`]
module"]
#[doc(alias = "GPIOC_AFRH")]
pub type GpiocAfrh = crate::Reg<gpioc_afrh::GpiocAfrhSpec>;
#[doc = "GPIO alternate function high register"]
pub mod gpioc_afrh;
#[doc = "GPIOC_BRR (w) register accessor: GPIO port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioc_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_brr`]
module"]
#[doc(alias = "GPIOC_BRR")]
pub type GpiocBrr = crate::Reg<gpioc_brr::GpiocBrrSpec>;
#[doc = "GPIO port bit reset register"]
pub mod gpioc_brr;
