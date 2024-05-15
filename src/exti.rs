#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    exti_rtsr1: ExtiRtsr1,
    exti_ftsr1: ExtiFtsr1,
    exti_swier1: ExtiSwier1,
    exti_rpr1: ExtiRpr1,
    exti_fpr1: ExtiFpr1,
    _reserved5: [u8; 0x4c],
    exti_exticr1: ExtiExticr1,
    exti_exticr2: ExtiExticr2,
    exti_exticr3: ExtiExticr3,
    exti_exticr4: ExtiExticr4,
    _reserved9: [u8; 0x10],
    exti_imr1: ExtiImr1,
    exti_emr1: ExtiEmr1,
    _reserved11: [u8; 0x08],
    exti_imr2: ExtiImr2,
    exti_emr2: ExtiEmr2,
}
impl RegisterBlock {
    #[doc = "0x00 - EXTI rising trigger selection register"]
    #[inline(always)]
    pub const fn exti_rtsr1(&self) -> &ExtiRtsr1 {
        &self.exti_rtsr1
    }
    #[doc = "0x04 - EXTI falling trigger selection register 1"]
    #[inline(always)]
    pub const fn exti_ftsr1(&self) -> &ExtiFtsr1 {
        &self.exti_ftsr1
    }
    #[doc = "0x08 - EXTI software interrupt event register 1"]
    #[inline(always)]
    pub const fn exti_swier1(&self) -> &ExtiSwier1 {
        &self.exti_swier1
    }
    #[doc = "0x0c - EXTI rising edge pending register 1"]
    #[inline(always)]
    pub const fn exti_rpr1(&self) -> &ExtiRpr1 {
        &self.exti_rpr1
    }
    #[doc = "0x10 - EXTI falling edge pending register 1"]
    #[inline(always)]
    pub const fn exti_fpr1(&self) -> &ExtiFpr1 {
        &self.exti_fpr1
    }
    #[doc = "0x60 - EXTI external interrupt selection register 1"]
    #[inline(always)]
    pub const fn exti_exticr1(&self) -> &ExtiExticr1 {
        &self.exti_exticr1
    }
    #[doc = "0x64 - EXTI external interrupt selection register 2"]
    #[inline(always)]
    pub const fn exti_exticr2(&self) -> &ExtiExticr2 {
        &self.exti_exticr2
    }
    #[doc = "0x68 - EXTI external interrupt selection register 3"]
    #[inline(always)]
    pub const fn exti_exticr3(&self) -> &ExtiExticr3 {
        &self.exti_exticr3
    }
    #[doc = "0x6c - EXTI external interrupt selection register 4"]
    #[inline(always)]
    pub const fn exti_exticr4(&self) -> &ExtiExticr4 {
        &self.exti_exticr4
    }
    #[doc = "0x80 - EXTI CPU wake-up with interrupt mask register"]
    #[inline(always)]
    pub const fn exti_imr1(&self) -> &ExtiImr1 {
        &self.exti_imr1
    }
    #[doc = "0x84 - EXTI CPU wake-up with event mask register"]
    #[inline(always)]
    pub const fn exti_emr1(&self) -> &ExtiEmr1 {
        &self.exti_emr1
    }
    #[doc = "0x90 - EXTI CPU wake-up with interrupt mask register"]
    #[inline(always)]
    pub const fn exti_imr2(&self) -> &ExtiImr2 {
        &self.exti_imr2
    }
    #[doc = "0x94 - EXTI CPU wake-up with event mask register"]
    #[inline(always)]
    pub const fn exti_emr2(&self) -> &ExtiEmr2 {
        &self.exti_emr2
    }
}
#[doc = "EXTI_RTSR1 (rw) register accessor: EXTI rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_rtsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_rtsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_rtsr1`]
module"]
#[doc(alias = "EXTI_RTSR1")]
pub type ExtiRtsr1 = crate::Reg<exti_rtsr1::ExtiRtsr1Spec>;
#[doc = "EXTI rising trigger selection register"]
pub mod exti_rtsr1;
#[doc = "EXTI_FTSR1 (rw) register accessor: EXTI falling trigger selection register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_ftsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_ftsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_ftsr1`]
module"]
#[doc(alias = "EXTI_FTSR1")]
pub type ExtiFtsr1 = crate::Reg<exti_ftsr1::ExtiFtsr1Spec>;
#[doc = "EXTI falling trigger selection register 1"]
pub mod exti_ftsr1;
#[doc = "EXTI_SWIER1 (rw) register accessor: EXTI software interrupt event register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_swier1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_swier1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_swier1`]
module"]
#[doc(alias = "EXTI_SWIER1")]
pub type ExtiSwier1 = crate::Reg<exti_swier1::ExtiSwier1Spec>;
#[doc = "EXTI software interrupt event register 1"]
pub mod exti_swier1;
#[doc = "EXTI_RPR1 (rw) register accessor: EXTI rising edge pending register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_rpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_rpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_rpr1`]
module"]
#[doc(alias = "EXTI_RPR1")]
pub type ExtiRpr1 = crate::Reg<exti_rpr1::ExtiRpr1Spec>;
#[doc = "EXTI rising edge pending register 1"]
pub mod exti_rpr1;
#[doc = "EXTI_FPR1 (rw) register accessor: EXTI falling edge pending register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_fpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_fpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_fpr1`]
module"]
#[doc(alias = "EXTI_FPR1")]
pub type ExtiFpr1 = crate::Reg<exti_fpr1::ExtiFpr1Spec>;
#[doc = "EXTI falling edge pending register 1"]
pub mod exti_fpr1;
#[doc = "EXTI_EXTICR1 (rw) register accessor: EXTI external interrupt selection register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_exticr1`]
module"]
#[doc(alias = "EXTI_EXTICR1")]
pub type ExtiExticr1 = crate::Reg<exti_exticr1::ExtiExticr1Spec>;
#[doc = "EXTI external interrupt selection register 1"]
pub mod exti_exticr1;
#[doc = "EXTI_EXTICR2 (rw) register accessor: EXTI external interrupt selection register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_exticr2`]
module"]
#[doc(alias = "EXTI_EXTICR2")]
pub type ExtiExticr2 = crate::Reg<exti_exticr2::ExtiExticr2Spec>;
#[doc = "EXTI external interrupt selection register 2"]
pub mod exti_exticr2;
#[doc = "EXTI_EXTICR3 (rw) register accessor: EXTI external interrupt selection register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_exticr3`]
module"]
#[doc(alias = "EXTI_EXTICR3")]
pub type ExtiExticr3 = crate::Reg<exti_exticr3::ExtiExticr3Spec>;
#[doc = "EXTI external interrupt selection register 3"]
pub mod exti_exticr3;
#[doc = "EXTI_EXTICR4 (rw) register accessor: EXTI external interrupt selection register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_exticr4`]
module"]
#[doc(alias = "EXTI_EXTICR4")]
pub type ExtiExticr4 = crate::Reg<exti_exticr4::ExtiExticr4Spec>;
#[doc = "EXTI external interrupt selection register 4"]
pub mod exti_exticr4;
#[doc = "EXTI_IMR1 (rw) register accessor: EXTI CPU wake-up with interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_imr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_imr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_imr1`]
module"]
#[doc(alias = "EXTI_IMR1")]
pub type ExtiImr1 = crate::Reg<exti_imr1::ExtiImr1Spec>;
#[doc = "EXTI CPU wake-up with interrupt mask register"]
pub mod exti_imr1;
#[doc = "EXTI_EMR1 (rw) register accessor: EXTI CPU wake-up with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_emr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_emr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_emr1`]
module"]
#[doc(alias = "EXTI_EMR1")]
pub type ExtiEmr1 = crate::Reg<exti_emr1::ExtiEmr1Spec>;
#[doc = "EXTI CPU wake-up with event mask register"]
pub mod exti_emr1;
#[doc = "EXTI_IMR2 (rw) register accessor: EXTI CPU wake-up with interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_imr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_imr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_imr2`]
module"]
#[doc(alias = "EXTI_IMR2")]
pub type ExtiImr2 = crate::Reg<exti_imr2::ExtiImr2Spec>;
#[doc = "EXTI CPU wake-up with interrupt mask register"]
pub mod exti_imr2;
#[doc = "EXTI_EMR2 (rw) register accessor: EXTI CPU wake-up with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_emr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_emr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_emr2`]
module"]
#[doc(alias = "EXTI_EMR2")]
pub type ExtiEmr2 = crate::Reg<exti_emr2::ExtiEmr2Spec>;
#[doc = "EXTI CPU wake-up with event mask register"]
pub mod exti_emr2;
