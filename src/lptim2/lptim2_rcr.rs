#[doc = "Register `LPTIM2_RCR` reader"]
pub type R = crate::R<Lptim2RcrSpec>;
#[doc = "Register `LPTIM2_RCR` writer"]
pub type W = crate::W<Lptim2RcrSpec>;
#[doc = "Field `REP` reader - Repetition register value REP is the repetition value for the LPTIM."]
pub type RepR = crate::FieldReader;
#[doc = "Field `REP` writer - Repetition register value REP is the repetition value for the LPTIM."]
pub type RepW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Repetition register value REP is the repetition value for the LPTIM."]
    #[inline(always)]
    pub fn rep(&self) -> RepR {
        RepR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repetition register value REP is the repetition value for the LPTIM."]
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> RepW<Lptim2RcrSpec> {
        RepW::new(self, 0)
    }
}
#[doc = "LPTIM repetition register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_rcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_rcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lptim2RcrSpec;
impl crate::RegisterSpec for Lptim2RcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim2_rcr::R`](R) reader structure"]
impl crate::Readable for Lptim2RcrSpec {}
#[doc = "`write(|w| ..)` method takes [`lptim2_rcr::W`](W) writer structure"]
impl crate::Writable for Lptim2RcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTIM2_RCR to value 0"]
impl crate::Resettable for Lptim2RcrSpec {
    const RESET_VALUE: u32 = 0;
}
