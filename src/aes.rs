#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    aes_cr: AesCr,
    aes_sr: AesSr,
    aes_dinr: AesDinr,
    aes_doutr: AesDoutr,
    aes_keyr0: AesKeyr0,
    aes_keyr1: AesKeyr1,
    aes_keyr2: AesKeyr2,
    aes_keyr3: AesKeyr3,
    aes_ivr0: AesIvr0,
    aes_ivr1: AesIvr1,
    aes_ivr2: AesIvr2,
    aes_ivr3: AesIvr3,
    aes_keyr4: AesKeyr4,
    aes_keyr5: AesKeyr5,
    aes_keyr6: AesKeyr6,
    aes_keyr7: AesKeyr7,
    aes_suspr0: AesSuspr0,
    aes_suspr1: AesSuspr1,
    aes_suspr2: AesSuspr2,
    aes_suspr3: AesSuspr3,
    aes_suspr4: AesSuspr4,
    aes_suspr5: AesSuspr5,
    aes_suspr6: AesSuspr6,
    aes_suspr7: AesSuspr7,
    _reserved24: [u8; 0x02a0],
    aes_ier: AesIer,
    aes_isr: AesIsr,
    aes_icr: AesIcr,
}
impl RegisterBlock {
    #[doc = "0x00 - AES control register"]
    #[inline(always)]
    pub const fn aes_cr(&self) -> &AesCr {
        &self.aes_cr
    }
    #[doc = "0x04 - AES status register"]
    #[inline(always)]
    pub const fn aes_sr(&self) -> &AesSr {
        &self.aes_sr
    }
    #[doc = "0x08 - AES data input register"]
    #[inline(always)]
    pub const fn aes_dinr(&self) -> &AesDinr {
        &self.aes_dinr
    }
    #[doc = "0x0c - AES data output register"]
    #[inline(always)]
    pub const fn aes_doutr(&self) -> &AesDoutr {
        &self.aes_doutr
    }
    #[doc = "0x10 - AES key register 0"]
    #[inline(always)]
    pub const fn aes_keyr0(&self) -> &AesKeyr0 {
        &self.aes_keyr0
    }
    #[doc = "0x14 - AES key register 1"]
    #[inline(always)]
    pub const fn aes_keyr1(&self) -> &AesKeyr1 {
        &self.aes_keyr1
    }
    #[doc = "0x18 - AES key register 2"]
    #[inline(always)]
    pub const fn aes_keyr2(&self) -> &AesKeyr2 {
        &self.aes_keyr2
    }
    #[doc = "0x1c - AES key register 3"]
    #[inline(always)]
    pub const fn aes_keyr3(&self) -> &AesKeyr3 {
        &self.aes_keyr3
    }
    #[doc = "0x20 - AES initialization vector register 0"]
    #[inline(always)]
    pub const fn aes_ivr0(&self) -> &AesIvr0 {
        &self.aes_ivr0
    }
    #[doc = "0x24 - AES initialization vector register 1"]
    #[inline(always)]
    pub const fn aes_ivr1(&self) -> &AesIvr1 {
        &self.aes_ivr1
    }
    #[doc = "0x28 - AES initialization vector register 2"]
    #[inline(always)]
    pub const fn aes_ivr2(&self) -> &AesIvr2 {
        &self.aes_ivr2
    }
    #[doc = "0x2c - AES initialization vector register 3"]
    #[inline(always)]
    pub const fn aes_ivr3(&self) -> &AesIvr3 {
        &self.aes_ivr3
    }
    #[doc = "0x30 - AES key register 4"]
    #[inline(always)]
    pub const fn aes_keyr4(&self) -> &AesKeyr4 {
        &self.aes_keyr4
    }
    #[doc = "0x34 - AES key register 5"]
    #[inline(always)]
    pub const fn aes_keyr5(&self) -> &AesKeyr5 {
        &self.aes_keyr5
    }
    #[doc = "0x38 - AES key register 6"]
    #[inline(always)]
    pub const fn aes_keyr6(&self) -> &AesKeyr6 {
        &self.aes_keyr6
    }
    #[doc = "0x3c - AES key register 7"]
    #[inline(always)]
    pub const fn aes_keyr7(&self) -> &AesKeyr7 {
        &self.aes_keyr7
    }
    #[doc = "0x40 - AES suspend registers"]
    #[inline(always)]
    pub const fn aes_suspr0(&self) -> &AesSuspr0 {
        &self.aes_suspr0
    }
    #[doc = "0x44 - AES suspend registers"]
    #[inline(always)]
    pub const fn aes_suspr1(&self) -> &AesSuspr1 {
        &self.aes_suspr1
    }
    #[doc = "0x48 - AES suspend registers"]
    #[inline(always)]
    pub const fn aes_suspr2(&self) -> &AesSuspr2 {
        &self.aes_suspr2
    }
    #[doc = "0x4c - AES suspend registers"]
    #[inline(always)]
    pub const fn aes_suspr3(&self) -> &AesSuspr3 {
        &self.aes_suspr3
    }
    #[doc = "0x50 - AES suspend registers"]
    #[inline(always)]
    pub const fn aes_suspr4(&self) -> &AesSuspr4 {
        &self.aes_suspr4
    }
    #[doc = "0x54 - AES suspend registers"]
    #[inline(always)]
    pub const fn aes_suspr5(&self) -> &AesSuspr5 {
        &self.aes_suspr5
    }
    #[doc = "0x58 - AES suspend registers"]
    #[inline(always)]
    pub const fn aes_suspr6(&self) -> &AesSuspr6 {
        &self.aes_suspr6
    }
    #[doc = "0x5c - AES suspend registers"]
    #[inline(always)]
    pub const fn aes_suspr7(&self) -> &AesSuspr7 {
        &self.aes_suspr7
    }
    #[doc = "0x300 - AES interrupt enable register"]
    #[inline(always)]
    pub const fn aes_ier(&self) -> &AesIer {
        &self.aes_ier
    }
    #[doc = "0x304 - AES interrupt status register"]
    #[inline(always)]
    pub const fn aes_isr(&self) -> &AesIsr {
        &self.aes_isr
    }
    #[doc = "0x308 - AES interrupt clear register"]
    #[inline(always)]
    pub const fn aes_icr(&self) -> &AesIcr {
        &self.aes_icr
    }
}
#[doc = "AES_CR (rw) register accessor: AES control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_cr`]
module"]
#[doc(alias = "AES_CR")]
pub type AesCr = crate::Reg<aes_cr::AesCrSpec>;
#[doc = "AES control register"]
pub mod aes_cr;
#[doc = "AES_SR (r) register accessor: AES status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_sr`]
module"]
#[doc(alias = "AES_SR")]
pub type AesSr = crate::Reg<aes_sr::AesSrSpec>;
#[doc = "AES status register"]
pub mod aes_sr;
#[doc = "AES_DINR (w) register accessor: AES data input register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_dinr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_dinr`]
module"]
#[doc(alias = "AES_DINR")]
pub type AesDinr = crate::Reg<aes_dinr::AesDinrSpec>;
#[doc = "AES data input register"]
pub mod aes_dinr;
#[doc = "AES_DOUTR (r) register accessor: AES data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_doutr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_doutr`]
module"]
#[doc(alias = "AES_DOUTR")]
pub type AesDoutr = crate::Reg<aes_doutr::AesDoutrSpec>;
#[doc = "AES data output register"]
pub mod aes_doutr;
#[doc = "AES_KEYR0 (w) register accessor: AES key register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_keyr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_keyr0`]
module"]
#[doc(alias = "AES_KEYR0")]
pub type AesKeyr0 = crate::Reg<aes_keyr0::AesKeyr0Spec>;
#[doc = "AES key register 0"]
pub mod aes_keyr0;
#[doc = "AES_KEYR1 (w) register accessor: AES key register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_keyr1`]
module"]
#[doc(alias = "AES_KEYR1")]
pub type AesKeyr1 = crate::Reg<aes_keyr1::AesKeyr1Spec>;
#[doc = "AES key register 1"]
pub mod aes_keyr1;
#[doc = "AES_KEYR2 (w) register accessor: AES key register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_keyr2`]
module"]
#[doc(alias = "AES_KEYR2")]
pub type AesKeyr2 = crate::Reg<aes_keyr2::AesKeyr2Spec>;
#[doc = "AES key register 2"]
pub mod aes_keyr2;
#[doc = "AES_KEYR3 (w) register accessor: AES key register 3\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_keyr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_keyr3`]
module"]
#[doc(alias = "AES_KEYR3")]
pub type AesKeyr3 = crate::Reg<aes_keyr3::AesKeyr3Spec>;
#[doc = "AES key register 3"]
pub mod aes_keyr3;
#[doc = "AES_IVR0 (rw) register accessor: AES initialization vector register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_ivr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_ivr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_ivr0`]
module"]
#[doc(alias = "AES_IVR0")]
pub type AesIvr0 = crate::Reg<aes_ivr0::AesIvr0Spec>;
#[doc = "AES initialization vector register 0"]
pub mod aes_ivr0;
#[doc = "AES_IVR1 (rw) register accessor: AES initialization vector register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_ivr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_ivr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_ivr1`]
module"]
#[doc(alias = "AES_IVR1")]
pub type AesIvr1 = crate::Reg<aes_ivr1::AesIvr1Spec>;
#[doc = "AES initialization vector register 1"]
pub mod aes_ivr1;
#[doc = "AES_IVR2 (rw) register accessor: AES initialization vector register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_ivr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_ivr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_ivr2`]
module"]
#[doc(alias = "AES_IVR2")]
pub type AesIvr2 = crate::Reg<aes_ivr2::AesIvr2Spec>;
#[doc = "AES initialization vector register 2"]
pub mod aes_ivr2;
#[doc = "AES_IVR3 (rw) register accessor: AES initialization vector register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_ivr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_ivr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_ivr3`]
module"]
#[doc(alias = "AES_IVR3")]
pub type AesIvr3 = crate::Reg<aes_ivr3::AesIvr3Spec>;
#[doc = "AES initialization vector register 3"]
pub mod aes_ivr3;
#[doc = "AES_KEYR4 (w) register accessor: AES key register 4\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_keyr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_keyr4`]
module"]
#[doc(alias = "AES_KEYR4")]
pub type AesKeyr4 = crate::Reg<aes_keyr4::AesKeyr4Spec>;
#[doc = "AES key register 4"]
pub mod aes_keyr4;
#[doc = "AES_KEYR5 (w) register accessor: AES key register 5\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_keyr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_keyr5`]
module"]
#[doc(alias = "AES_KEYR5")]
pub type AesKeyr5 = crate::Reg<aes_keyr5::AesKeyr5Spec>;
#[doc = "AES key register 5"]
pub mod aes_keyr5;
#[doc = "AES_KEYR6 (w) register accessor: AES key register 6\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_keyr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_keyr6`]
module"]
#[doc(alias = "AES_KEYR6")]
pub type AesKeyr6 = crate::Reg<aes_keyr6::AesKeyr6Spec>;
#[doc = "AES key register 6"]
pub mod aes_keyr6;
#[doc = "AES_KEYR7 (w) register accessor: AES key register 7\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_keyr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_keyr7`]
module"]
#[doc(alias = "AES_KEYR7")]
pub type AesKeyr7 = crate::Reg<aes_keyr7::AesKeyr7Spec>;
#[doc = "AES key register 7"]
pub mod aes_keyr7;
#[doc = "AES_SUSPR0 (rw) register accessor: AES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_suspr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_suspr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_suspr0`]
module"]
#[doc(alias = "AES_SUSPR0")]
pub type AesSuspr0 = crate::Reg<aes_suspr0::AesSuspr0Spec>;
#[doc = "AES suspend registers"]
pub mod aes_suspr0;
#[doc = "AES_SUSPR1 (rw) register accessor: AES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_suspr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_suspr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_suspr1`]
module"]
#[doc(alias = "AES_SUSPR1")]
pub type AesSuspr1 = crate::Reg<aes_suspr1::AesSuspr1Spec>;
#[doc = "AES suspend registers"]
pub mod aes_suspr1;
#[doc = "AES_SUSPR2 (rw) register accessor: AES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_suspr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_suspr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_suspr2`]
module"]
#[doc(alias = "AES_SUSPR2")]
pub type AesSuspr2 = crate::Reg<aes_suspr2::AesSuspr2Spec>;
#[doc = "AES suspend registers"]
pub mod aes_suspr2;
#[doc = "AES_SUSPR3 (rw) register accessor: AES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_suspr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_suspr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_suspr3`]
module"]
#[doc(alias = "AES_SUSPR3")]
pub type AesSuspr3 = crate::Reg<aes_suspr3::AesSuspr3Spec>;
#[doc = "AES suspend registers"]
pub mod aes_suspr3;
#[doc = "AES_SUSPR4 (rw) register accessor: AES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_suspr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_suspr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_suspr4`]
module"]
#[doc(alias = "AES_SUSPR4")]
pub type AesSuspr4 = crate::Reg<aes_suspr4::AesSuspr4Spec>;
#[doc = "AES suspend registers"]
pub mod aes_suspr4;
#[doc = "AES_SUSPR5 (rw) register accessor: AES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_suspr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_suspr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_suspr5`]
module"]
#[doc(alias = "AES_SUSPR5")]
pub type AesSuspr5 = crate::Reg<aes_suspr5::AesSuspr5Spec>;
#[doc = "AES suspend registers"]
pub mod aes_suspr5;
#[doc = "AES_SUSPR6 (rw) register accessor: AES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_suspr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_suspr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_suspr6`]
module"]
#[doc(alias = "AES_SUSPR6")]
pub type AesSuspr6 = crate::Reg<aes_suspr6::AesSuspr6Spec>;
#[doc = "AES suspend registers"]
pub mod aes_suspr6;
#[doc = "AES_SUSPR7 (rw) register accessor: AES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_suspr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_suspr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_suspr7`]
module"]
#[doc(alias = "AES_SUSPR7")]
pub type AesSuspr7 = crate::Reg<aes_suspr7::AesSuspr7Spec>;
#[doc = "AES suspend registers"]
pub mod aes_suspr7;
#[doc = "AES_IER (rw) register accessor: AES interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_ier`]
module"]
#[doc(alias = "AES_IER")]
pub type AesIer = crate::Reg<aes_ier::AesIerSpec>;
#[doc = "AES interrupt enable register"]
pub mod aes_ier;
#[doc = "AES_ISR (r) register accessor: AES interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_isr`]
module"]
#[doc(alias = "AES_ISR")]
pub type AesIsr = crate::Reg<aes_isr::AesIsrSpec>;
#[doc = "AES interrupt status register"]
pub mod aes_isr;
#[doc = "AES_ICR (w) register accessor: AES interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_icr`]
module"]
#[doc(alias = "AES_ICR")]
pub type AesIcr = crate::Reg<aes_icr::AesIcrSpec>;
#[doc = "AES interrupt clear register"]
pub mod aes_icr;
