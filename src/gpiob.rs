#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpiob_moder: GpiobModer,
    gpiob_otyper: GpiobOtyper,
    gpiob_ospeedr: GpiobOspeedr,
    gpiob_pupdr: GpiobPupdr,
    gpiob_idr: GpiobIdr,
    gpiob_odr: GpiobOdr,
    gpiob_bsrr: GpiobBsrr,
    gpiob_lckr: GpiobLckr,
    gpiob_afrl: GpiobAfrl,
    gpiob_afrh: GpiobAfrh,
    gpiob_brr: GpiobBrr,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    #[inline(always)]
    pub const fn gpiob_moder(&self) -> &GpiobModer {
        &self.gpiob_moder
    }
    #[doc = "0x04 - GPIO port output type register"]
    #[inline(always)]
    pub const fn gpiob_otyper(&self) -> &GpiobOtyper {
        &self.gpiob_otyper
    }
    #[doc = "0x08 - GPIO port output speed register"]
    #[inline(always)]
    pub const fn gpiob_ospeedr(&self) -> &GpiobOspeedr {
        &self.gpiob_ospeedr
    }
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    #[inline(always)]
    pub const fn gpiob_pupdr(&self) -> &GpiobPupdr {
        &self.gpiob_pupdr
    }
    #[doc = "0x10 - GPIO port input data register"]
    #[inline(always)]
    pub const fn gpiob_idr(&self) -> &GpiobIdr {
        &self.gpiob_idr
    }
    #[doc = "0x14 - GPIO port output data register"]
    #[inline(always)]
    pub const fn gpiob_odr(&self) -> &GpiobOdr {
        &self.gpiob_odr
    }
    #[doc = "0x18 - GPIO port bit set/reset register"]
    #[inline(always)]
    pub const fn gpiob_bsrr(&self) -> &GpiobBsrr {
        &self.gpiob_bsrr
    }
    #[doc = "0x1c - GPIO port configuration lock register"]
    #[inline(always)]
    pub const fn gpiob_lckr(&self) -> &GpiobLckr {
        &self.gpiob_lckr
    }
    #[doc = "0x20 - GPIO alternate function low register"]
    #[inline(always)]
    pub const fn gpiob_afrl(&self) -> &GpiobAfrl {
        &self.gpiob_afrl
    }
    #[doc = "0x24 - GPIO alternate function high register"]
    #[inline(always)]
    pub const fn gpiob_afrh(&self) -> &GpiobAfrh {
        &self.gpiob_afrh
    }
    #[doc = "0x28 - GPIO port bit reset register"]
    #[inline(always)]
    pub const fn gpiob_brr(&self) -> &GpiobBrr {
        &self.gpiob_brr
    }
}
#[doc = "GPIOB_MODER (rw) register accessor: GPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiob_moder::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiob_moder::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_moder`]
module"]
#[doc(alias = "GPIOB_MODER")]
pub type GpiobModer = crate::Reg<gpiob_moder::GpiobModerSpec>;
#[doc = "GPIO port mode register"]
pub mod gpiob_moder;
#[doc = "GPIOB_OTYPER (rw) register accessor: GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiob_otyper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiob_otyper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_otyper`]
module"]
#[doc(alias = "GPIOB_OTYPER")]
pub type GpiobOtyper = crate::Reg<gpiob_otyper::GpiobOtyperSpec>;
#[doc = "GPIO port output type register"]
pub mod gpiob_otyper;
#[doc = "GPIOB_OSPEEDR (rw) register accessor: GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiob_ospeedr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiob_ospeedr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_ospeedr`]
module"]
#[doc(alias = "GPIOB_OSPEEDR")]
pub type GpiobOspeedr = crate::Reg<gpiob_ospeedr::GpiobOspeedrSpec>;
#[doc = "GPIO port output speed register"]
pub mod gpiob_ospeedr;
#[doc = "GPIOB_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiob_pupdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiob_pupdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_pupdr`]
module"]
#[doc(alias = "GPIOB_PUPDR")]
pub type GpiobPupdr = crate::Reg<gpiob_pupdr::GpiobPupdrSpec>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpiob_pupdr;
#[doc = "GPIOB_IDR (r) register accessor: GPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiob_idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_idr`]
module"]
#[doc(alias = "GPIOB_IDR")]
pub type GpiobIdr = crate::Reg<gpiob_idr::GpiobIdrSpec>;
#[doc = "GPIO port input data register"]
pub mod gpiob_idr;
#[doc = "GPIOB_ODR (rw) register accessor: GPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiob_odr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiob_odr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_odr`]
module"]
#[doc(alias = "GPIOB_ODR")]
pub type GpiobOdr = crate::Reg<gpiob_odr::GpiobOdrSpec>;
#[doc = "GPIO port output data register"]
pub mod gpiob_odr;
#[doc = "GPIOB_BSRR (w) register accessor: GPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiob_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_bsrr`]
module"]
#[doc(alias = "GPIOB_BSRR")]
pub type GpiobBsrr = crate::Reg<gpiob_bsrr::GpiobBsrrSpec>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpiob_bsrr;
#[doc = "GPIOB_LCKR (rw) register accessor: GPIO port configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiob_lckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiob_lckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_lckr`]
module"]
#[doc(alias = "GPIOB_LCKR")]
pub type GpiobLckr = crate::Reg<gpiob_lckr::GpiobLckrSpec>;
#[doc = "GPIO port configuration lock register"]
pub mod gpiob_lckr;
#[doc = "GPIOB_AFRL (rw) register accessor: GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiob_afrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiob_afrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_afrl`]
module"]
#[doc(alias = "GPIOB_AFRL")]
pub type GpiobAfrl = crate::Reg<gpiob_afrl::GpiobAfrlSpec>;
#[doc = "GPIO alternate function low register"]
pub mod gpiob_afrl;
#[doc = "GPIOB_AFRH (rw) register accessor: GPIO alternate function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiob_afrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiob_afrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_afrh`]
module"]
#[doc(alias = "GPIOB_AFRH")]
pub type GpiobAfrh = crate::Reg<gpiob_afrh::GpiobAfrhSpec>;
#[doc = "GPIO alternate function high register"]
pub mod gpiob_afrh;
#[doc = "GPIOB_BRR (w) register accessor: GPIO port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiob_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_brr`]
module"]
#[doc(alias = "GPIOB_BRR")]
pub type GpiobBrr = crate::Reg<gpiob_brr::GpiobBrrSpec>;
#[doc = "GPIO port bit reset register"]
pub mod gpiob_brr;
