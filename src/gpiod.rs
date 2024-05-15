#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpiod_moder: GpiodModer,
    gpiod_otyper: GpiodOtyper,
    gpiod_ospeedr: GpiodOspeedr,
    gpiod_pupdr: GpiodPupdr,
    gpiod_idr: GpiodIdr,
    gpiod_odr: GpiodOdr,
    gpiod_bsrr: GpiodBsrr,
    gpiod_lckr: GpiodLckr,
    gpiod_afrl: GpiodAfrl,
    gpiod_afrh: GpiodAfrh,
    gpiod_brr: GpiodBrr,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    #[inline(always)]
    pub const fn gpiod_moder(&self) -> &GpiodModer {
        &self.gpiod_moder
    }
    #[doc = "0x04 - GPIO port output type register"]
    #[inline(always)]
    pub const fn gpiod_otyper(&self) -> &GpiodOtyper {
        &self.gpiod_otyper
    }
    #[doc = "0x08 - GPIO port output speed register"]
    #[inline(always)]
    pub const fn gpiod_ospeedr(&self) -> &GpiodOspeedr {
        &self.gpiod_ospeedr
    }
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    #[inline(always)]
    pub const fn gpiod_pupdr(&self) -> &GpiodPupdr {
        &self.gpiod_pupdr
    }
    #[doc = "0x10 - GPIO port input data register"]
    #[inline(always)]
    pub const fn gpiod_idr(&self) -> &GpiodIdr {
        &self.gpiod_idr
    }
    #[doc = "0x14 - GPIO port output data register"]
    #[inline(always)]
    pub const fn gpiod_odr(&self) -> &GpiodOdr {
        &self.gpiod_odr
    }
    #[doc = "0x18 - GPIO port bit set/reset register"]
    #[inline(always)]
    pub const fn gpiod_bsrr(&self) -> &GpiodBsrr {
        &self.gpiod_bsrr
    }
    #[doc = "0x1c - GPIO port configuration lock register"]
    #[inline(always)]
    pub const fn gpiod_lckr(&self) -> &GpiodLckr {
        &self.gpiod_lckr
    }
    #[doc = "0x20 - GPIO alternate function low register"]
    #[inline(always)]
    pub const fn gpiod_afrl(&self) -> &GpiodAfrl {
        &self.gpiod_afrl
    }
    #[doc = "0x24 - GPIO alternate function high register"]
    #[inline(always)]
    pub const fn gpiod_afrh(&self) -> &GpiodAfrh {
        &self.gpiod_afrh
    }
    #[doc = "0x28 - GPIO port bit reset register"]
    #[inline(always)]
    pub const fn gpiod_brr(&self) -> &GpiodBrr {
        &self.gpiod_brr
    }
}
#[doc = "GPIOD_MODER (rw) register accessor: GPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_moder::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_moder::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_moder`]
module"]
#[doc(alias = "GPIOD_MODER")]
pub type GpiodModer = crate::Reg<gpiod_moder::GpiodModerSpec>;
#[doc = "GPIO port mode register"]
pub mod gpiod_moder;
#[doc = "GPIOD_OTYPER (rw) register accessor: GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_otyper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_otyper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_otyper`]
module"]
#[doc(alias = "GPIOD_OTYPER")]
pub type GpiodOtyper = crate::Reg<gpiod_otyper::GpiodOtyperSpec>;
#[doc = "GPIO port output type register"]
pub mod gpiod_otyper;
#[doc = "GPIOD_OSPEEDR (rw) register accessor: GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_ospeedr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_ospeedr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_ospeedr`]
module"]
#[doc(alias = "GPIOD_OSPEEDR")]
pub type GpiodOspeedr = crate::Reg<gpiod_ospeedr::GpiodOspeedrSpec>;
#[doc = "GPIO port output speed register"]
pub mod gpiod_ospeedr;
#[doc = "GPIOD_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_pupdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_pupdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_pupdr`]
module"]
#[doc(alias = "GPIOD_PUPDR")]
pub type GpiodPupdr = crate::Reg<gpiod_pupdr::GpiodPupdrSpec>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpiod_pupdr;
#[doc = "GPIOD_IDR (r) register accessor: GPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_idr`]
module"]
#[doc(alias = "GPIOD_IDR")]
pub type GpiodIdr = crate::Reg<gpiod_idr::GpiodIdrSpec>;
#[doc = "GPIO port input data register"]
pub mod gpiod_idr;
#[doc = "GPIOD_ODR (rw) register accessor: GPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_odr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_odr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_odr`]
module"]
#[doc(alias = "GPIOD_ODR")]
pub type GpiodOdr = crate::Reg<gpiod_odr::GpiodOdrSpec>;
#[doc = "GPIO port output data register"]
pub mod gpiod_odr;
#[doc = "GPIOD_BSRR (w) register accessor: GPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_bsrr`]
module"]
#[doc(alias = "GPIOD_BSRR")]
pub type GpiodBsrr = crate::Reg<gpiod_bsrr::GpiodBsrrSpec>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpiod_bsrr;
#[doc = "GPIOD_LCKR (rw) register accessor: GPIO port configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_lckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_lckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_lckr`]
module"]
#[doc(alias = "GPIOD_LCKR")]
pub type GpiodLckr = crate::Reg<gpiod_lckr::GpiodLckrSpec>;
#[doc = "GPIO port configuration lock register"]
pub mod gpiod_lckr;
#[doc = "GPIOD_AFRL (rw) register accessor: GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_afrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_afrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_afrl`]
module"]
#[doc(alias = "GPIOD_AFRL")]
pub type GpiodAfrl = crate::Reg<gpiod_afrl::GpiodAfrlSpec>;
#[doc = "GPIO alternate function low register"]
pub mod gpiod_afrl;
#[doc = "GPIOD_AFRH (rw) register accessor: GPIO alternate function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_afrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_afrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_afrh`]
module"]
#[doc(alias = "GPIOD_AFRH")]
pub type GpiodAfrh = crate::Reg<gpiod_afrh::GpiodAfrhSpec>;
#[doc = "GPIO alternate function high register"]
pub mod gpiod_afrh;
#[doc = "GPIOD_BRR (w) register accessor: GPIO port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_brr`]
module"]
#[doc(alias = "GPIOD_BRR")]
pub type GpiodBrr = crate::Reg<gpiod_brr::GpiodBrrSpec>;
#[doc = "GPIO port bit reset register"]
pub mod gpiod_brr;
