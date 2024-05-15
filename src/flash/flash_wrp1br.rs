#[doc = "Register `FLASH_WRP1BR` reader"]
pub type R = crate::R<FlashWrp1brSpec>;
#[doc = "Register `FLASH_WRP1BR` writer"]
pub type W = crate::W<FlashWrp1brSpec>;
#[doc = "Field `WRP1B_STRT` reader - WRP area B start offset This bitfield contains the offset of the first page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device."]
pub type Wrp1bStrtR = crate::FieldReader;
#[doc = "Field `WRP1B_STRT` writer - WRP area B start offset This bitfield contains the offset of the first page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device."]
pub type Wrp1bStrtW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRP1B_END` reader - WRP area B end offset This bitfield contains the offset of the last page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device."]
pub type Wrp1bEndR = crate::FieldReader;
#[doc = "Field `WRP1B_END` writer - WRP area B end offset This bitfield contains the offset of the last page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device."]
pub type Wrp1bEndW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - WRP area B start offset This bitfield contains the offset of the first page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device."]
    #[inline(always)]
    pub fn wrp1b_strt(&self) -> Wrp1bStrtR {
        Wrp1bStrtR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - WRP area B end offset This bitfield contains the offset of the last page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device."]
    #[inline(always)]
    pub fn wrp1b_end(&self) -> Wrp1bEndR {
        Wrp1bEndR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - WRP area B start offset This bitfield contains the offset of the first page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device."]
    #[inline(always)]
    #[must_use]
    pub fn wrp1b_strt(&mut self) -> Wrp1bStrtW<FlashWrp1brSpec> {
        Wrp1bStrtW::new(self, 0)
    }
    #[doc = "Bits 16:22 - WRP area B end offset This bitfield contains the offset of the last page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device."]
    #[inline(always)]
    #[must_use]
    pub fn wrp1b_end(&mut self) -> Wrp1bEndW<FlashWrp1brSpec> {
        Wrp1bEndW::new(self, 16)
    }
}
#[doc = "FLASH WRP area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_wrp1br::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_wrp1br::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashWrp1brSpec;
impl crate::RegisterSpec for FlashWrp1brSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_wrp1br::R`](R) reader structure"]
impl crate::Readable for FlashWrp1brSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_wrp1br::W`](W) writer structure"]
impl crate::Writable for FlashWrp1brSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_WRP1BR to value 0"]
impl crate::Resettable for FlashWrp1brSpec {
    const RESET_VALUE: u32 = 0;
}
