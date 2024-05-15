#[doc = "Register `LPTIM3_ICR_INPUT` writer"]
pub type W = crate::W<Lptim3IcrInputSpec>;
#[doc = "Field `CC1CF` writer - Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register."]
pub type Cc1cfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRMCF` writer - Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register"]
pub type ArrmcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRIGCF` writer - External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register"]
pub type ExttrigcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARROKCF` writer - Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register"]
pub type ArrokcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPCF` writer - Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
pub type UpcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWNCF` writer - Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
pub type DowncfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UECF` writer - Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register."]
pub type UecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPOKCF` writer - Repetition register update OK clear flag Writing 1 to this bit clears the REPOK flag in the LPTIM_ISR register."]
pub type RepokcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2CF` writer - Capture/compare 2 clear flag Writing 1 to this bit clears the CC2IF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
pub type Cc2cfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3CF` writer - Capture/compare 3 clear flag Writing 1 to this bit clears the CC3IF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
pub type Cc3cfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4CF` writer - Capture/compare 4 clear flag Writing 1 to this bit clears the CC4IF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
pub type Cc4cfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1OCF` writer - Capture/compare 1 over-capture clear flag Writing 1 to this bit clears the CC1OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3."]
pub type Cc1ocfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2OCF` writer - Capture/compare 2 over-capture clear flag Writing 1 to this bit clears the CC2OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
pub type Cc2ocfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3OCF` writer - Capture/compare 3 over-capture clear flag Writing 1 to this bit clears the CC3OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
pub type Cc3ocfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4OCF` writer - Capture/compare 4 over-capture clear flag Writing 1 to this bit clears the CC4OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
pub type Cc4ocfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIEROKCF` writer - Interrupt enable register update OK clear flag Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register."]
pub type DierokcfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn cc1cf(&mut self) -> Cc1cfW<Lptim3IcrInputSpec> {
        Cc1cfW::new(self, 0)
    }
    #[doc = "Bit 1 - Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn arrmcf(&mut self) -> ArrmcfW<Lptim3IcrInputSpec> {
        ArrmcfW::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn exttrigcf(&mut self) -> ExttrigcfW<Lptim3IcrInputSpec> {
        ExttrigcfW::new(self, 2)
    }
    #[doc = "Bit 4 - Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn arrokcf(&mut self) -> ArrokcfW<Lptim3IcrInputSpec> {
        ArrokcfW::new(self, 4)
    }
    #[doc = "Bit 5 - Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn upcf(&mut self) -> UpcfW<Lptim3IcrInputSpec> {
        UpcfW::new(self, 5)
    }
    #[doc = "Bit 6 - Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn downcf(&mut self) -> DowncfW<Lptim3IcrInputSpec> {
        DowncfW::new(self, 6)
    }
    #[doc = "Bit 7 - Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn uecf(&mut self) -> UecfW<Lptim3IcrInputSpec> {
        UecfW::new(self, 7)
    }
    #[doc = "Bit 8 - Repetition register update OK clear flag Writing 1 to this bit clears the REPOK flag in the LPTIM_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn repokcf(&mut self) -> RepokcfW<Lptim3IcrInputSpec> {
        RepokcfW::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/compare 2 clear flag Writing 1 to this bit clears the CC2IF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cc2cf(&mut self) -> Cc2cfW<Lptim3IcrInputSpec> {
        Cc2cfW::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/compare 3 clear flag Writing 1 to this bit clears the CC3IF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cc3cf(&mut self) -> Cc3cfW<Lptim3IcrInputSpec> {
        Cc3cfW::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/compare 4 clear flag Writing 1 to this bit clears the CC4IF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cc4cf(&mut self) -> Cc4cfW<Lptim3IcrInputSpec> {
        Cc4cfW::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/compare 1 over-capture clear flag Writing 1 to this bit clears the CC1OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cc1ocf(&mut self) -> Cc1ocfW<Lptim3IcrInputSpec> {
        Cc1ocfW::new(self, 12)
    }
    #[doc = "Bit 13 - Capture/compare 2 over-capture clear flag Writing 1 to this bit clears the CC2OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cc2ocf(&mut self) -> Cc2ocfW<Lptim3IcrInputSpec> {
        Cc2ocfW::new(self, 13)
    }
    #[doc = "Bit 14 - Capture/compare 3 over-capture clear flag Writing 1 to this bit clears the CC3OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cc3ocf(&mut self) -> Cc3ocfW<Lptim3IcrInputSpec> {
        Cc3ocfW::new(self, 14)
    }
    #[doc = "Bit 15 - Capture/compare 4 over-capture clear flag Writing 1 to this bit clears the CC4OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cc4ocf(&mut self) -> Cc4ocfW<Lptim3IcrInputSpec> {
        Cc4ocfW::new(self, 15)
    }
    #[doc = "Bit 24 - Interrupt enable register update OK clear flag Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn dierokcf(&mut self) -> DierokcfW<Lptim3IcrInputSpec> {
        DierokcfW::new(self, 24)
    }
}
#[doc = "LPTIM3 interrupt clear register \\[alternate\\]\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_icr_input::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lptim3IcrInputSpec;
impl crate::RegisterSpec for Lptim3IcrInputSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lptim3_icr_input::W`](W) writer structure"]
impl crate::Writable for Lptim3IcrInputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTIM3_ICR_INPUT to value 0"]
impl crate::Resettable for Lptim3IcrInputSpec {
    const RESET_VALUE: u32 = 0;
}
