#[doc = "Register `FLASH_SECR` reader"]
pub type R = crate::R<FlashSecrSpec>;
#[doc = "Register `FLASH_SECR` writer"]
pub type W = crate::W<FlashSecrSpec>;
#[doc = "Field `HDP1_PEND` reader - Last page of the first hide protection area"]
pub type Hdp1PendR = crate::FieldReader;
#[doc = "Field `HDP1_PEND` writer - Last page of the first hide protection area"]
pub type Hdp1PendW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootLock {
    #[doc = "0: Boot based on the pad/option bit configuration"]
    B0x0 = 0,
    #[doc = "1: Boot forced from main flash memory"]
    B0x1 = 1,
}
impl From<BootLock> for bool {
    #[inline(always)]
    fn from(variant: BootLock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOT_LOCK` reader - used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch)."]
pub type BootLockR = crate::BitReader<BootLock>;
impl BootLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BootLock {
        match self.bits {
            false => BootLock::B0x0,
            true => BootLock::B0x1,
        }
    }
    #[doc = "Boot based on the pad/option bit configuration"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BootLock::B0x0
    }
    #[doc = "Boot forced from main flash memory"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BootLock::B0x1
    }
}
#[doc = "Field `BOOT_LOCK` writer - used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch)."]
pub type BootLockW<'a, REG> = crate::BitWriter<'a, REG, BootLock>;
impl<'a, REG> BootLockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Boot based on the pad/option bit configuration"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BootLock::B0x0)
    }
    #[doc = "Boot forced from main flash memory"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BootLock::B0x1)
    }
}
#[doc = "Field `HDP1EN` reader - Hide protection area enable"]
pub type Hdp1enR = crate::FieldReader;
#[doc = "Field `HDP1EN` writer - Hide protection area enable"]
pub type Hdp1enW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6 - Last page of the first hide protection area"]
    #[inline(always)]
    pub fn hdp1_pend(&self) -> Hdp1PendR {
        Hdp1PendR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch)."]
    #[inline(always)]
    pub fn boot_lock(&self) -> BootLockR {
        BootLockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Hide protection area enable"]
    #[inline(always)]
    pub fn hdp1en(&self) -> Hdp1enR {
        Hdp1enR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Last page of the first hide protection area"]
    #[inline(always)]
    #[must_use]
    pub fn hdp1_pend(&mut self) -> Hdp1PendW<FlashSecrSpec> {
        Hdp1PendW::new(self, 0)
    }
    #[doc = "Bit 16 - used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch)."]
    #[inline(always)]
    #[must_use]
    pub fn boot_lock(&mut self) -> BootLockW<FlashSecrSpec> {
        BootLockW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Hide protection area enable"]
    #[inline(always)]
    #[must_use]
    pub fn hdp1en(&mut self) -> Hdp1enW<FlashSecrSpec> {
        Hdp1enW::new(self, 24)
    }
}
#[doc = "FLASH security register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_secr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_secr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashSecrSpec;
impl crate::RegisterSpec for FlashSecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_secr::R`](R) reader structure"]
impl crate::Readable for FlashSecrSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_secr::W`](W) writer structure"]
impl crate::Writable for FlashSecrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_SECR to value 0"]
impl crate::Resettable for FlashSecrSpec {
    const RESET_VALUE: u32 = 0;
}
