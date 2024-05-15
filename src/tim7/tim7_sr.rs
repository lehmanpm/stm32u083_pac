#[doc = "Register `TIM7_SR` reader"]
pub type R = crate::R<Tim7SrSpec>;
#[doc = "Register `TIM7_SR` writer"]
pub type W = crate::W<Tim7SrSpec>;
#[doc = "Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value and if UDIS = 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in the TIMx_EGR register, if URS1=10 and UDIS1=10 in the TIMx_CR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uif {
    #[doc = "0: No update occurred."]
    B0x0 = 0,
    #[doc = "1: Update interrupt pending. This bit is set by hardware when the registers are updated:"]
    B0x1 = 1,
}
impl From<Uif> for bool {
    #[inline(always)]
    fn from(variant: Uif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIF` reader - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value and if UDIS = 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in the TIMx_EGR register, if URS1=10 and UDIS1=10 in the TIMx_CR1 register."]
pub type UifR = crate::BitReader<Uif>;
impl UifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uif {
        match self.bits {
            false => Uif::B0x0,
            true => Uif::B0x1,
        }
    }
    #[doc = "No update occurred."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Uif::B0x0
    }
    #[doc = "Update interrupt pending. This bit is set by hardware when the registers are updated:"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Uif::B0x1
    }
}
#[doc = "Field `UIF` writer - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value and if UDIS = 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in the TIMx_EGR register, if URS1=10 and UDIS1=10 in the TIMx_CR1 register."]
pub type UifW<'a, REG> = crate::BitWriter<'a, REG, Uif>;
impl<'a, REG> UifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No update occurred."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Uif::B0x0)
    }
    #[doc = "Update interrupt pending. This bit is set by hardware when the registers are updated:"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Uif::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value and if UDIS = 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in the TIMx_EGR register, if URS1=10 and UDIS1=10 in the TIMx_CR1 register."]
    #[inline(always)]
    pub fn uif(&self) -> UifR {
        UifR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value and if UDIS = 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in the TIMx_EGR register, if URS1=10 and UDIS1=10 in the TIMx_CR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn uif(&mut self) -> UifW<Tim7SrSpec> {
        UifW::new(self, 0)
    }
}
#[doc = "TIM7 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim7_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim7_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim7SrSpec;
impl crate::RegisterSpec for Tim7SrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim7_sr::R`](R) reader structure"]
impl crate::Readable for Tim7SrSpec {}
#[doc = "`write(|w| ..)` method takes [`tim7_sr::W`](W) writer structure"]
impl crate::Writable for Tim7SrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM7_SR to value 0"]
impl crate::Resettable for Tim7SrSpec {
    const RESET_VALUE: u16 = 0;
}
