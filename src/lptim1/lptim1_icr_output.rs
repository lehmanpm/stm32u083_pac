#[doc = "Register `LPTIM1_ICR_OUTPUT` writer"]
pub type W = crate::W<Lptim1IcrOutputSpec>;
#[doc = "Field `CC1CF` writer - Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register."]
pub type Cc1cfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRMCF` writer - Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register"]
pub type ArrmcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRIGCF` writer - External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register"]
pub type ExttrigcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1OKCF` writer - Compare register 1 update OK clear flag Writing 1 to this bit clears the CMP1OK flag in the LPTIM_ISR register."]
pub type Cmp1okcfW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `CMP2OKCF` writer - Compare register 2 update OK clear flag Writing 1 to this bit clears the CMP2OK flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
pub type Cmp2okcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP3OKCF` writer - Compare register 3 update OK clear flag Writing 1 to this bit clears the CMP3OK flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
pub type Cmp3okcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP4OKCF` writer - Compare register 4 update OK clear flag Writing 1 to this bit clears the CMP4OK flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
pub type Cmp4okcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIEROKCF` writer - Interrupt enable register update OK clear flag Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register."]
pub type DierokcfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn cc1cf(&mut self) -> Cc1cfW<Lptim1IcrOutputSpec> {
        Cc1cfW::new(self, 0)
    }
    #[doc = "Bit 1 - Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn arrmcf(&mut self) -> ArrmcfW<Lptim1IcrOutputSpec> {
        ArrmcfW::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn exttrigcf(&mut self) -> ExttrigcfW<Lptim1IcrOutputSpec> {
        ExttrigcfW::new(self, 2)
    }
    #[doc = "Bit 3 - Compare register 1 update OK clear flag Writing 1 to this bit clears the CMP1OK flag in the LPTIM_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn cmp1okcf(&mut self) -> Cmp1okcfW<Lptim1IcrOutputSpec> {
        Cmp1okcfW::new(self, 3)
    }
    #[doc = "Bit 4 - Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn arrokcf(&mut self) -> ArrokcfW<Lptim1IcrOutputSpec> {
        ArrokcfW::new(self, 4)
    }
    #[doc = "Bit 5 - Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn upcf(&mut self) -> UpcfW<Lptim1IcrOutputSpec> {
        UpcfW::new(self, 5)
    }
    #[doc = "Bit 6 - Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn downcf(&mut self) -> DowncfW<Lptim1IcrOutputSpec> {
        DowncfW::new(self, 6)
    }
    #[doc = "Bit 7 - Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn uecf(&mut self) -> UecfW<Lptim1IcrOutputSpec> {
        UecfW::new(self, 7)
    }
    #[doc = "Bit 8 - Repetition register update OK clear flag Writing 1 to this bit clears the REPOK flag in the LPTIM_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn repokcf(&mut self) -> RepokcfW<Lptim1IcrOutputSpec> {
        RepokcfW::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/compare 2 clear flag Writing 1 to this bit clears the CC2IF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cc2cf(&mut self) -> Cc2cfW<Lptim1IcrOutputSpec> {
        Cc2cfW::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/compare 3 clear flag Writing 1 to this bit clears the CC3IF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cc3cf(&mut self) -> Cc3cfW<Lptim1IcrOutputSpec> {
        Cc3cfW::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/compare 4 clear flag Writing 1 to this bit clears the CC4IF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cc4cf(&mut self) -> Cc4cfW<Lptim1IcrOutputSpec> {
        Cc4cfW::new(self, 11)
    }
    #[doc = "Bit 19 - Compare register 2 update OK clear flag Writing 1 to this bit clears the CMP2OK flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cmp2okcf(&mut self) -> Cmp2okcfW<Lptim1IcrOutputSpec> {
        Cmp2okcfW::new(self, 19)
    }
    #[doc = "Bit 20 - Compare register 3 update OK clear flag Writing 1 to this bit clears the CMP3OK flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cmp3okcf(&mut self) -> Cmp3okcfW<Lptim1IcrOutputSpec> {
        Cmp3okcfW::new(self, 20)
    }
    #[doc = "Bit 21 - Compare register 4 update OK clear flag Writing 1 to this bit clears the CMP4OK flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cmp4okcf(&mut self) -> Cmp4okcfW<Lptim1IcrOutputSpec> {
        Cmp4okcfW::new(self, 21)
    }
    #[doc = "Bit 24 - Interrupt enable register update OK clear flag Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn dierokcf(&mut self) -> DierokcfW<Lptim1IcrOutputSpec> {
        DierokcfW::new(self, 24)
    }
}
#[doc = "LPTIM1 interrupt clear register \\[alternate\\]\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim1_icr_output::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lptim1IcrOutputSpec;
impl crate::RegisterSpec for Lptim1IcrOutputSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lptim1_icr_output::W`](W) writer structure"]
impl crate::Writable for Lptim1IcrOutputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTIM1_ICR_OUTPUT to value 0"]
impl crate::Resettable for Lptim1IcrOutputSpec {
    const RESET_VALUE: u32 = 0;
}
