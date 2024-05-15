#[doc = "Register `TIM7_ARR` reader"]
pub type R = crate::R<Tim7ArrSpec>;
#[doc = "Register `TIM7_ARR` writer"]
pub type W = crate::W<Tim7ArrSpec>;
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
    pub fn arr(&mut self) -> ArrW<Tim7ArrSpec> {
        ArrW::new(self, 0)
    }
}
#[doc = "TIM7 auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim7_arr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim7_arr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim7ArrSpec;
impl crate::RegisterSpec for Tim7ArrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim7_arr::R`](R) reader structure"]
impl crate::Readable for Tim7ArrSpec {}
#[doc = "`write(|w| ..)` method takes [`tim7_arr::W`](W) writer structure"]
impl crate::Writable for Tim7ArrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM7_ARR to value 0xffff"]
impl crate::Resettable for Tim7ArrSpec {
    const RESET_VALUE: u16 = 0xffff;
}
