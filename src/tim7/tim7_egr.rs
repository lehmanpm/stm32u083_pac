#[doc = "Register `TIM7_EGR` writer"]
pub type W = crate::W<Tim7EgrSpec>;
#[doc = "Update generation This bit can be set by software, it is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ug {
    #[doc = "0: No action."]
    B0x0 = 0,
    #[doc = "1: Re-initializes the timer counter and generates an update of the registers. Note that the prescaler counter is cleared too (but the prescaler ratio is not affected)."]
    B0x1 = 1,
}
impl From<Ug> for bool {
    #[inline(always)]
    fn from(variant: Ug) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UG` writer - Update generation This bit can be set by software, it is automatically cleared by hardware."]
pub type UgW<'a, REG> = crate::BitWriter<'a, REG, Ug>;
impl<'a, REG> UgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ug::B0x0)
    }
    #[doc = "Re-initializes the timer counter and generates an update of the registers. Note that the prescaler counter is cleared too (but the prescaler ratio is not affected)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ug::B0x1)
    }
}
impl W {
    #[doc = "Bit 0 - Update generation This bit can be set by software, it is automatically cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UgW<Tim7EgrSpec> {
        UgW::new(self, 0)
    }
}
#[doc = "TIM7 event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim7_egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim7EgrSpec;
impl crate::RegisterSpec for Tim7EgrSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`tim7_egr::W`](W) writer structure"]
impl crate::Writable for Tim7EgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM7_EGR to value 0"]
impl crate::Resettable for Tim7EgrSpec {
    const RESET_VALUE: u16 = 0;
}
