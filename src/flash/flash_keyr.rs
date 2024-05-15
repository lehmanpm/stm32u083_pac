#[doc = "Register `FLASH_KEYR` writer"]
pub type W = crate::W<FlashKeyrSpec>;
#[doc = "Field `KEY` writer - FLASH key The following values must be written consecutively to unlock the FLASH control register (FLASH_CR), thus enabling programming/erasing operations: KEY1: 0x4567 0123 KEY2: 0xCDEF 89AB"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - FLASH key The following values must be written consecutively to unlock the FLASH control register (FLASH_CR), thus enabling programming/erasing operations: KEY1: 0x4567 0123 KEY2: 0xCDEF 89AB"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<FlashKeyrSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "FLASH key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_keyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashKeyrSpec;
impl crate::RegisterSpec for FlashKeyrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flash_keyr::W`](W) writer structure"]
impl crate::Writable for FlashKeyrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_KEYR to value 0"]
impl crate::Resettable for FlashKeyrSpec {
    const RESET_VALUE: u32 = 0;
}
