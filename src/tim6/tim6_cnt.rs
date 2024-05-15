#[doc = "Register `TIM6_CNT` reader"]
pub type R = crate::R<Tim6CntSpec>;
#[doc = "Register `TIM6_CNT` writer"]
pub type W = crate::W<Tim6CntSpec>;
#[doc = "Field `CNT` reader - Counter value"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Counter value"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `UIFCPY` reader - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register. If the UIFREMAP bit in TIMx_CR1 is reset, bit 31 is reserved and read as 0."]
pub type UifcpyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register. If the UIFREMAP bit in TIMx_CR1 is reset, bit 31 is reserved and read as 0."]
    #[inline(always)]
    pub fn uifcpy(&self) -> UifcpyR {
        UifcpyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<Tim6CntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "TIM6 counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim6_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim6_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim6CntSpec;
impl crate::RegisterSpec for Tim6CntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim6_cnt::R`](R) reader structure"]
impl crate::Readable for Tim6CntSpec {}
#[doc = "`write(|w| ..)` method takes [`tim6_cnt::W`](W) writer structure"]
impl crate::Writable for Tim6CntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM6_CNT to value 0"]
impl crate::Resettable for Tim6CntSpec {
    const RESET_VALUE: u32 = 0;
}
