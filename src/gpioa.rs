#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpioa_moder: GpioaModer,
    gpioa_otyper: GpioaOtyper,
    gpioa_ospeedr: GpioaOspeedr,
    gpioa_pupdr: GpioaPupdr,
    gpioa_idr: GpioaIdr,
    gpioa_odr: GpioaOdr,
    gpioa_bsrr: GpioaBsrr,
    gpioa_lckr: GpioaLckr,
    gpioa_afrl: GpioaAfrl,
    gpioa_afrh: GpioaAfrh,
    gpioa_brr: GpioaBrr,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    #[inline(always)]
    pub const fn gpioa_moder(&self) -> &GpioaModer {
        &self.gpioa_moder
    }
    #[doc = "0x04 - GPIO port output type register"]
    #[inline(always)]
    pub const fn gpioa_otyper(&self) -> &GpioaOtyper {
        &self.gpioa_otyper
    }
    #[doc = "0x08 - GPIO port output speed register"]
    #[inline(always)]
    pub const fn gpioa_ospeedr(&self) -> &GpioaOspeedr {
        &self.gpioa_ospeedr
    }
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    #[inline(always)]
    pub const fn gpioa_pupdr(&self) -> &GpioaPupdr {
        &self.gpioa_pupdr
    }
    #[doc = "0x10 - GPIO port input data register"]
    #[inline(always)]
    pub const fn gpioa_idr(&self) -> &GpioaIdr {
        &self.gpioa_idr
    }
    #[doc = "0x14 - GPIO port output data register"]
    #[inline(always)]
    pub const fn gpioa_odr(&self) -> &GpioaOdr {
        &self.gpioa_odr
    }
    #[doc = "0x18 - GPIO port bit set/reset register"]
    #[inline(always)]
    pub const fn gpioa_bsrr(&self) -> &GpioaBsrr {
        &self.gpioa_bsrr
    }
    #[doc = "0x1c - GPIO port configuration lock register"]
    #[inline(always)]
    pub const fn gpioa_lckr(&self) -> &GpioaLckr {
        &self.gpioa_lckr
    }
    #[doc = "0x20 - GPIO alternate function low register"]
    #[inline(always)]
    pub const fn gpioa_afrl(&self) -> &GpioaAfrl {
        &self.gpioa_afrl
    }
    #[doc = "0x24 - GPIO alternate function high register"]
    #[inline(always)]
    pub const fn gpioa_afrh(&self) -> &GpioaAfrh {
        &self.gpioa_afrh
    }
    #[doc = "0x28 - GPIO port bit reset register"]
    #[inline(always)]
    pub const fn gpioa_brr(&self) -> &GpioaBrr {
        &self.gpioa_brr
    }
}
#[doc = "GPIOA_MODER (rw) register accessor: GPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioa_moder::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioa_moder::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_moder`]
module"]
#[doc(alias = "GPIOA_MODER")]
pub type GpioaModer = crate::Reg<gpioa_moder::GpioaModerSpec>;
#[doc = "GPIO port mode register"]
pub mod gpioa_moder;
#[doc = "GPIOA_OTYPER (rw) register accessor: GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioa_otyper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioa_otyper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_otyper`]
module"]
#[doc(alias = "GPIOA_OTYPER")]
pub type GpioaOtyper = crate::Reg<gpioa_otyper::GpioaOtyperSpec>;
#[doc = "GPIO port output type register"]
pub mod gpioa_otyper;
#[doc = "GPIOA_OSPEEDR (rw) register accessor: GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioa_ospeedr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioa_ospeedr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_ospeedr`]
module"]
#[doc(alias = "GPIOA_OSPEEDR")]
pub type GpioaOspeedr = crate::Reg<gpioa_ospeedr::GpioaOspeedrSpec>;
#[doc = "GPIO port output speed register"]
pub mod gpioa_ospeedr;
#[doc = "GPIOA_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioa_pupdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioa_pupdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_pupdr`]
module"]
#[doc(alias = "GPIOA_PUPDR")]
pub type GpioaPupdr = crate::Reg<gpioa_pupdr::GpioaPupdrSpec>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioa_pupdr;
#[doc = "GPIOA_IDR (r) register accessor: GPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioa_idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_idr`]
module"]
#[doc(alias = "GPIOA_IDR")]
pub type GpioaIdr = crate::Reg<gpioa_idr::GpioaIdrSpec>;
#[doc = "GPIO port input data register"]
pub mod gpioa_idr;
#[doc = "GPIOA_ODR (rw) register accessor: GPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioa_odr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioa_odr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_odr`]
module"]
#[doc(alias = "GPIOA_ODR")]
pub type GpioaOdr = crate::Reg<gpioa_odr::GpioaOdrSpec>;
#[doc = "GPIO port output data register"]
pub mod gpioa_odr;
#[doc = "GPIOA_BSRR (w) register accessor: GPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioa_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_bsrr`]
module"]
#[doc(alias = "GPIOA_BSRR")]
pub type GpioaBsrr = crate::Reg<gpioa_bsrr::GpioaBsrrSpec>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpioa_bsrr;
#[doc = "GPIOA_LCKR (rw) register accessor: GPIO port configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioa_lckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioa_lckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_lckr`]
module"]
#[doc(alias = "GPIOA_LCKR")]
pub type GpioaLckr = crate::Reg<gpioa_lckr::GpioaLckrSpec>;
#[doc = "GPIO port configuration lock register"]
pub mod gpioa_lckr;
#[doc = "GPIOA_AFRL (rw) register accessor: GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioa_afrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioa_afrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_afrl`]
module"]
#[doc(alias = "GPIOA_AFRL")]
pub type GpioaAfrl = crate::Reg<gpioa_afrl::GpioaAfrlSpec>;
#[doc = "GPIO alternate function low register"]
pub mod gpioa_afrl;
#[doc = "GPIOA_AFRH (rw) register accessor: GPIO alternate function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioa_afrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioa_afrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_afrh`]
module"]
#[doc(alias = "GPIOA_AFRH")]
pub type GpioaAfrh = crate::Reg<gpioa_afrh::GpioaAfrhSpec>;
#[doc = "GPIO alternate function high register"]
pub mod gpioa_afrh;
#[doc = "GPIOA_BRR (w) register accessor: GPIO port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioa_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_brr`]
module"]
#[doc(alias = "GPIOA_BRR")]
pub type GpioaBrr = crate::Reg<gpioa_brr::GpioaBrrSpec>;
#[doc = "GPIO port bit reset register"]
pub mod gpioa_brr;
