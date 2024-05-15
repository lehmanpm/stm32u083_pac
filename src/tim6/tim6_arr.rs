#[doc = "Register `TIM6_ARR` reader"]
pub type R = crate::R<Tim6ArrSpec>;
#[doc = "Register `TIM6_ARR` writer"]
pub type W = crate::W<Tim6ArrSpec>;
#[doc = "Field `ARR` reader - Prescaler value ARR is the value to be loaded into the actual auto-reload register. Refer to Section123.3.1: Time-base unit on page1596 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null."]
pub type ArrR = crate::FieldReader<u16>;
#[doc = "Field `ARR` writer - Prescaler value ARR is the value to be loaded into the actual auto-reload register. Refer to Section123.3.1: Time-base unit on page1596 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null."]
pub type ArrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Prescaler value ARR is the value to be loaded into the actual auto-reload register. Refer to Section123.3.1: Time-base unit on page1596 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null."]
    #[inline(always)]
    pub fn arr(&self) -> ArrR {
        ArrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Prescaler value ARR is the value to be loaded into the actual auto-reload register. Refer to Section123.3.1: Time-base unit on page1596 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null."]
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ArrW<Tim6ArrSpec> {
        ArrW::new(self, 0)
    }
}
#[doc = "TIM6 auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim6_arr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim6_arr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim6ArrSpec;
impl crate::RegisterSpec for Tim6ArrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim6_arr::R`](R) reader structure"]
impl crate::Readable for Tim6ArrSpec {}
#[doc = "`write(|w| ..)` method takes [`tim6_arr::W`](W) writer structure"]
impl crate::Writable for Tim6ArrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM6_ARR to value 0xffff"]
impl crate::Resettable for Tim6ArrSpec {
    const RESET_VALUE: u16 = 0xffff;
}
