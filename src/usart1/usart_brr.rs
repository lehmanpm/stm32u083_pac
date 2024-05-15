#[doc = "Register `USART_BRR` reader"]
pub type R = crate::R<UsartBrrSpec>;
#[doc = "Register `USART_BRR` writer"]
pub type W = crate::W<UsartBrrSpec>;
#[doc = "Field `BRR` reader - USART baud rate BRR\\[15:4\\]
BRR\\[15:4\\]
correspond to USARTDIV\\[15:4\\]
BRR\\[3:0\\]
When OVER8 = 0, BRR\\[3:0\\]
= USARTDIV\\[3:0\\]. When OVER8 = 1: BRR\\[2:0\\]
= USARTDIV\\[3:0\\]
shifted 1 bit to the right. BRR\\[3\\]
must be kept cleared."]
pub type BrrR = crate::FieldReader<u16>;
#[doc = "Field `BRR` writer - USART baud rate BRR\\[15:4\\]
BRR\\[15:4\\]
correspond to USARTDIV\\[15:4\\]
BRR\\[3:0\\]
When OVER8 = 0, BRR\\[3:0\\]
= USARTDIV\\[3:0\\]. When OVER8 = 1: BRR\\[2:0\\]
= USARTDIV\\[3:0\\]
shifted 1 bit to the right. BRR\\[3\\]
must be kept cleared."]
pub type BrrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - USART baud rate BRR\\[15:4\\]
BRR\\[15:4\\]
correspond to USARTDIV\\[15:4\\]
BRR\\[3:0\\]
When OVER8 = 0, BRR\\[3:0\\]
= USARTDIV\\[3:0\\]. When OVER8 = 1: BRR\\[2:0\\]
= USARTDIV\\[3:0\\]
shifted 1 bit to the right. BRR\\[3\\]
must be kept cleared."]
    #[inline(always)]
    pub fn brr(&self) -> BrrR {
        BrrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - USART baud rate BRR\\[15:4\\]
BRR\\[15:4\\]
correspond to USARTDIV\\[15:4\\]
BRR\\[3:0\\]
When OVER8 = 0, BRR\\[3:0\\]
= USARTDIV\\[3:0\\]. When OVER8 = 1: BRR\\[2:0\\]
= USARTDIV\\[3:0\\]
shifted 1 bit to the right. BRR\\[3\\]
must be kept cleared."]
    #[inline(always)]
    #[must_use]
    pub fn brr(&mut self) -> BrrW<UsartBrrSpec> {
        BrrW::new(self, 0)
    }
}
#[doc = "USART baud rate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_brr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_brr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsartBrrSpec;
impl crate::RegisterSpec for UsartBrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart_brr::R`](R) reader structure"]
impl crate::Readable for UsartBrrSpec {}
#[doc = "`write(|w| ..)` method takes [`usart_brr::W`](W) writer structure"]
impl crate::Writable for UsartBrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USART_BRR to value 0"]
impl crate::Resettable for UsartBrrSpec {
    const RESET_VALUE: u32 = 0;
}
