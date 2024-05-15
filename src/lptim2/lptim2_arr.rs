#[doc = "Register `LPTIM2_ARR` reader"]
pub type R = crate::R<Lptim2ArrSpec>;
#[doc = "Register `LPTIM2_ARR` writer"]
pub type W = crate::W<Lptim2ArrSpec>;
#[doc = "Field `ARR` reader - Auto reload value ARR is the autoreload value for the LPTIM. This value must be strictly greater than the CCRx\\[15:0\\]
value."]
pub type ArrR = crate::FieldReader<u16>;
#[doc = "Field `ARR` writer - Auto reload value ARR is the autoreload value for the LPTIM. This value must be strictly greater than the CCRx\\[15:0\\]
value."]
pub type ArrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Auto reload value ARR is the autoreload value for the LPTIM. This value must be strictly greater than the CCRx\\[15:0\\]
value."]
    #[inline(always)]
    pub fn arr(&self) -> ArrR {
        ArrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Auto reload value ARR is the autoreload value for the LPTIM. This value must be strictly greater than the CCRx\\[15:0\\]
value."]
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ArrW<Lptim2ArrSpec> {
        ArrW::new(self, 0)
    }
}
#[doc = "LPTIM autoreload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_arr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_arr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lptim2ArrSpec;
impl crate::RegisterSpec for Lptim2ArrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim2_arr::R`](R) reader structure"]
impl crate::Readable for Lptim2ArrSpec {}
#[doc = "`write(|w| ..)` method takes [`lptim2_arr::W`](W) writer structure"]
impl crate::Writable for Lptim2ArrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTIM2_ARR to value 0x01"]
impl crate::Resettable for Lptim2ArrSpec {
    const RESET_VALUE: u32 = 0x01;
}