#[doc = "Register `FLASH_WRP1AR` reader"]
pub type R = crate::R<FlashWrp1arSpec>;
#[doc = "Register `FLASH_WRP1AR` writer"]
pub type W = crate::W<FlashWrp1arSpec>;
#[doc = "Field `WRP1A_STRT` reader - WRP area A start offset This bitfield contains the offset of the first page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device."]
pub type Wrp1aStrtR = crate::FieldReader;
#[doc = "Field `WRP1A_STRT` writer - WRP area A start offset This bitfield contains the offset of the first page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device."]
pub type Wrp1aStrtW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRP1A_END` reader - WRP area A end offset This bitfield contains the offset of the last page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device."]
pub type Wrp1aEndR = crate::FieldReader;
#[doc = "Field `WRP1A_END` writer - WRP area A end offset This bitfield contains the offset of the last page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device."]
pub type Wrp1aEndW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - WRP area A start offset This bitfield contains the offset of the first page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device."]
    #[inline(always)]
    pub fn wrp1a_strt(&self) -> Wrp1aStrtR {
        Wrp1aStrtR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - WRP area A end offset This bitfield contains the offset of the last page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device."]
    #[inline(always)]
    pub fn wrp1a_end(&self) -> Wrp1aEndR {
        Wrp1aEndR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - WRP area A start offset This bitfield contains the offset of the first page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device."]
    #[inline(always)]
    #[must_use]
    pub fn wrp1a_strt(&mut self) -> Wrp1aStrtW<FlashWrp1arSpec> {
        Wrp1aStrtW::new(self, 0)
    }
    #[doc = "Bits 16:22 - WRP area A end offset This bitfield contains the offset of the last page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device."]
    #[inline(always)]
    #[must_use]
    pub fn wrp1a_end(&mut self) -> Wrp1aEndW<FlashWrp1arSpec> {
        Wrp1aEndW::new(self, 16)
    }
}
#[doc = "FLASH WRP area A address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_wrp1ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_wrp1ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashWrp1arSpec;
impl crate::RegisterSpec for FlashWrp1arSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_wrp1ar::R`](R) reader structure"]
impl crate::Readable for FlashWrp1arSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_wrp1ar::W`](W) writer structure"]
impl crate::Writable for FlashWrp1arSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_WRP1AR to value 0"]
impl crate::Resettable for FlashWrp1arSpec {
    const RESET_VALUE: u32 = 0;
}
