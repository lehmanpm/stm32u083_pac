#[doc = "Register `LPUART_BRR` reader"]
pub type R = crate::R<LpuartBrrSpec>;
#[doc = "Register `LPUART_BRR` writer"]
pub type W = crate::W<LpuartBrrSpec>;
#[doc = "Field `BRR` reader - LPUART baud rate division (LPUARTDIV)"]
pub type BrrR = crate::FieldReader<u32>;
#[doc = "Field `BRR` writer - LPUART baud rate division (LPUARTDIV)"]
pub type BrrW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - LPUART baud rate division (LPUARTDIV)"]
    #[inline(always)]
    pub fn brr(&self) -> BrrR {
        BrrR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - LPUART baud rate division (LPUARTDIV)"]
    #[inline(always)]
    #[must_use]
    pub fn brr(&mut self) -> BrrW<LpuartBrrSpec> {
        BrrW::new(self, 0)
    }
}
#[doc = "LPUART baud rate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpuart_brr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpuart_brr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpuartBrrSpec;
impl crate::RegisterSpec for LpuartBrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpuart_brr::R`](R) reader structure"]
impl crate::Readable for LpuartBrrSpec {}
#[doc = "`write(|w| ..)` method takes [`lpuart_brr::W`](W) writer structure"]
impl crate::Writable for LpuartBrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPUART_BRR to value 0"]
impl crate::Resettable for LpuartBrrSpec {
    const RESET_VALUE: u32 = 0;
}
