#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpioe_moder: GpioeModer,
    gpioe_otyper: GpioeOtyper,
    gpioe_ospeedr: GpioeOspeedr,
    gpioe_pupdr: GpioePupdr,
    gpioe_idr: GpioeIdr,
    gpioe_odr: GpioeOdr,
    gpioe_bsrr: GpioeBsrr,
    gpioe_lckr: GpioeLckr,
    gpioe_afrl: GpioeAfrl,
    gpioe_afrh: GpioeAfrh,
    gpioe_brr: GpioeBrr,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    #[inline(always)]
    pub const fn gpioe_moder(&self) -> &GpioeModer {
        &self.gpioe_moder
    }
    #[doc = "0x04 - GPIO port output type register"]
    #[inline(always)]
    pub const fn gpioe_otyper(&self) -> &GpioeOtyper {
        &self.gpioe_otyper
    }
    #[doc = "0x08 - GPIO port output speed register"]
    #[inline(always)]
    pub const fn gpioe_ospeedr(&self) -> &GpioeOspeedr {
        &self.gpioe_ospeedr
    }
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    #[inline(always)]
    pub const fn gpioe_pupdr(&self) -> &GpioePupdr {
        &self.gpioe_pupdr
    }
    #[doc = "0x10 - GPIO port input data register"]
    #[inline(always)]
    pub const fn gpioe_idr(&self) -> &GpioeIdr {
        &self.gpioe_idr
    }
    #[doc = "0x14 - GPIO port output data register"]
    #[inline(always)]
    pub const fn gpioe_odr(&self) -> &GpioeOdr {
        &self.gpioe_odr
    }
    #[doc = "0x18 - GPIO port bit set/reset register"]
    #[inline(always)]
    pub const fn gpioe_bsrr(&self) -> &GpioeBsrr {
        &self.gpioe_bsrr
    }
    #[doc = "0x1c - GPIO port configuration lock register"]
    #[inline(always)]
    pub const fn gpioe_lckr(&self) -> &GpioeLckr {
        &self.gpioe_lckr
    }
    #[doc = "0x20 - GPIO alternate function low register"]
    #[inline(always)]
    pub const fn gpioe_afrl(&self) -> &GpioeAfrl {
        &self.gpioe_afrl
    }
    #[doc = "0x24 - GPIO alternate function high register"]
    #[inline(always)]
    pub const fn gpioe_afrh(&self) -> &GpioeAfrh {
        &self.gpioe_afrh
    }
    #[doc = "0x28 - GPIO port bit reset register"]
    #[inline(always)]
    pub const fn gpioe_brr(&self) -> &GpioeBrr {
        &self.gpioe_brr
    }
}
#[doc = "GPIOE_MODER (rw) register accessor: GPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_moder::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_moder::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_moder`]
module"]
#[doc(alias = "GPIOE_MODER")]
pub type GpioeModer = crate::Reg<gpioe_moder::GpioeModerSpec>;
#[doc = "GPIO port mode register"]
pub mod gpioe_moder;
#[doc = "GPIOE_OTYPER (rw) register accessor: GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_otyper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_otyper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_otyper`]
module"]
#[doc(alias = "GPIOE_OTYPER")]
pub type GpioeOtyper = crate::Reg<gpioe_otyper::GpioeOtyperSpec>;
#[doc = "GPIO port output type register"]
pub mod gpioe_otyper;
#[doc = "GPIOE_OSPEEDR (rw) register accessor: GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_ospeedr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_ospeedr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_ospeedr`]
module"]
#[doc(alias = "GPIOE_OSPEEDR")]
pub type GpioeOspeedr = crate::Reg<gpioe_ospeedr::GpioeOspeedrSpec>;
#[doc = "GPIO port output speed register"]
pub mod gpioe_ospeedr;
#[doc = "GPIOE_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_pupdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_pupdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_pupdr`]
module"]
#[doc(alias = "GPIOE_PUPDR")]
pub type GpioePupdr = crate::Reg<gpioe_pupdr::GpioePupdrSpec>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioe_pupdr;
#[doc = "GPIOE_IDR (r) register accessor: GPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_idr`]
module"]
#[doc(alias = "GPIOE_IDR")]
pub type GpioeIdr = crate::Reg<gpioe_idr::GpioeIdrSpec>;
#[doc = "GPIO port input data register"]
pub mod gpioe_idr;
#[doc = "GPIOE_ODR (rw) register accessor: GPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_odr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_odr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_odr`]
module"]
#[doc(alias = "GPIOE_ODR")]
pub type GpioeOdr = crate::Reg<gpioe_odr::GpioeOdrSpec>;
#[doc = "GPIO port output data register"]
pub mod gpioe_odr;
#[doc = "GPIOE_BSRR (w) register accessor: GPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_bsrr`]
module"]
#[doc(alias = "GPIOE_BSRR")]
pub type GpioeBsrr = crate::Reg<gpioe_bsrr::GpioeBsrrSpec>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpioe_bsrr;
#[doc = "GPIOE_LCKR (rw) register accessor: GPIO port configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_lckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_lckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_lckr`]
module"]
#[doc(alias = "GPIOE_LCKR")]
pub type GpioeLckr = crate::Reg<gpioe_lckr::GpioeLckrSpec>;
#[doc = "GPIO port configuration lock register"]
pub mod gpioe_lckr;
#[doc = "GPIOE_AFRL (rw) register accessor: GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_afrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_afrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_afrl`]
module"]
#[doc(alias = "GPIOE_AFRL")]
pub type GpioeAfrl = crate::Reg<gpioe_afrl::GpioeAfrlSpec>;
#[doc = "GPIO alternate function low register"]
pub mod gpioe_afrl;
#[doc = "GPIOE_AFRH (rw) register accessor: GPIO alternate function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_afrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_afrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_afrh`]
module"]
#[doc(alias = "GPIOE_AFRH")]
pub type GpioeAfrh = crate::Reg<gpioe_afrh::GpioeAfrhSpec>;
#[doc = "GPIO alternate function high register"]
pub mod gpioe_afrh;
#[doc = "GPIOE_BRR (w) register accessor: GPIO port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_brr`]
module"]
#[doc(alias = "GPIOE_BRR")]
pub type GpioeBrr = crate::Reg<gpioe_brr::GpioeBrrSpec>;
#[doc = "GPIO port bit reset register"]
pub mod gpioe_brr;
