#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpiof_moder: GpiofModer,
    gpiof_otyper: GpiofOtyper,
    gpiof_ospeedr: GpiofOspeedr,
    gpiof_pupdr: GpiofPupdr,
    gpiof_idr: GpiofIdr,
    gpiof_odr: GpiofOdr,
    gpiof_bsrr: GpiofBsrr,
    gpiof_lckr: GpiofLckr,
    gpiof_afrl: GpiofAfrl,
    gpiof_afrh: GpiofAfrh,
    gpiof_brr: GpiofBrr,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    #[inline(always)]
    pub const fn gpiof_moder(&self) -> &GpiofModer {
        &self.gpiof_moder
    }
    #[doc = "0x04 - GPIO port output type register"]
    #[inline(always)]
    pub const fn gpiof_otyper(&self) -> &GpiofOtyper {
        &self.gpiof_otyper
    }
    #[doc = "0x08 - GPIO port output speed register"]
    #[inline(always)]
    pub const fn gpiof_ospeedr(&self) -> &GpiofOspeedr {
        &self.gpiof_ospeedr
    }
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    #[inline(always)]
    pub const fn gpiof_pupdr(&self) -> &GpiofPupdr {
        &self.gpiof_pupdr
    }
    #[doc = "0x10 - GPIO port input data register"]
    #[inline(always)]
    pub const fn gpiof_idr(&self) -> &GpiofIdr {
        &self.gpiof_idr
    }
    #[doc = "0x14 - GPIO port output data register"]
    #[inline(always)]
    pub const fn gpiof_odr(&self) -> &GpiofOdr {
        &self.gpiof_odr
    }
    #[doc = "0x18 - GPIO port bit set/reset register"]
    #[inline(always)]
    pub const fn gpiof_bsrr(&self) -> &GpiofBsrr {
        &self.gpiof_bsrr
    }
    #[doc = "0x1c - GPIO port configuration lock register"]
    #[inline(always)]
    pub const fn gpiof_lckr(&self) -> &GpiofLckr {
        &self.gpiof_lckr
    }
    #[doc = "0x20 - GPIO alternate function low register"]
    #[inline(always)]
    pub const fn gpiof_afrl(&self) -> &GpiofAfrl {
        &self.gpiof_afrl
    }
    #[doc = "0x24 - GPIO alternate function high register"]
    #[inline(always)]
    pub const fn gpiof_afrh(&self) -> &GpiofAfrh {
        &self.gpiof_afrh
    }
    #[doc = "0x28 - GPIO port bit reset register"]
    #[inline(always)]
    pub const fn gpiof_brr(&self) -> &GpiofBrr {
        &self.gpiof_brr
    }
}
#[doc = "GPIOF_MODER (rw) register accessor: GPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_moder::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_moder::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_moder`]
module"]
#[doc(alias = "GPIOF_MODER")]
pub type GpiofModer = crate::Reg<gpiof_moder::GpiofModerSpec>;
#[doc = "GPIO port mode register"]
pub mod gpiof_moder;
#[doc = "GPIOF_OTYPER (rw) register accessor: GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_otyper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_otyper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_otyper`]
module"]
#[doc(alias = "GPIOF_OTYPER")]
pub type GpiofOtyper = crate::Reg<gpiof_otyper::GpiofOtyperSpec>;
#[doc = "GPIO port output type register"]
pub mod gpiof_otyper;
#[doc = "GPIOF_OSPEEDR (rw) register accessor: GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_ospeedr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_ospeedr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_ospeedr`]
module"]
#[doc(alias = "GPIOF_OSPEEDR")]
pub type GpiofOspeedr = crate::Reg<gpiof_ospeedr::GpiofOspeedrSpec>;
#[doc = "GPIO port output speed register"]
pub mod gpiof_ospeedr;
#[doc = "GPIOF_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_pupdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_pupdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_pupdr`]
module"]
#[doc(alias = "GPIOF_PUPDR")]
pub type GpiofPupdr = crate::Reg<gpiof_pupdr::GpiofPupdrSpec>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpiof_pupdr;
#[doc = "GPIOF_IDR (r) register accessor: GPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_idr`]
module"]
#[doc(alias = "GPIOF_IDR")]
pub type GpiofIdr = crate::Reg<gpiof_idr::GpiofIdrSpec>;
#[doc = "GPIO port input data register"]
pub mod gpiof_idr;
#[doc = "GPIOF_ODR (rw) register accessor: GPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_odr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_odr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_odr`]
module"]
#[doc(alias = "GPIOF_ODR")]
pub type GpiofOdr = crate::Reg<gpiof_odr::GpiofOdrSpec>;
#[doc = "GPIO port output data register"]
pub mod gpiof_odr;
#[doc = "GPIOF_BSRR (w) register accessor: GPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_bsrr`]
module"]
#[doc(alias = "GPIOF_BSRR")]
pub type GpiofBsrr = crate::Reg<gpiof_bsrr::GpiofBsrrSpec>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpiof_bsrr;
#[doc = "GPIOF_LCKR (rw) register accessor: GPIO port configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_lckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_lckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_lckr`]
module"]
#[doc(alias = "GPIOF_LCKR")]
pub type GpiofLckr = crate::Reg<gpiof_lckr::GpiofLckrSpec>;
#[doc = "GPIO port configuration lock register"]
pub mod gpiof_lckr;
#[doc = "GPIOF_AFRL (rw) register accessor: GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_afrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_afrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_afrl`]
module"]
#[doc(alias = "GPIOF_AFRL")]
pub type GpiofAfrl = crate::Reg<gpiof_afrl::GpiofAfrlSpec>;
#[doc = "GPIO alternate function low register"]
pub mod gpiof_afrl;
#[doc = "GPIOF_AFRH (rw) register accessor: GPIO alternate function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_afrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_afrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_afrh`]
module"]
#[doc(alias = "GPIOF_AFRH")]
pub type GpiofAfrh = crate::Reg<gpiof_afrh::GpiofAfrhSpec>;
#[doc = "GPIO alternate function high register"]
pub mod gpiof_afrh;
#[doc = "GPIOF_BRR (w) register accessor: GPIO port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_brr`]
module"]
#[doc(alias = "GPIOF_BRR")]
pub type GpiofBrr = crate::Reg<gpiof_brr::GpiofBrrSpec>;
#[doc = "GPIO port bit reset register"]
pub mod gpiof_brr;
