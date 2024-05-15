#[doc = "Register `FLASH_ECCR` reader"]
pub type R = crate::R<FlashEccrSpec>;
#[doc = "Register `FLASH_ECCR` writer"]
pub type W = crate::W<FlashEccrSpec>;
#[doc = "Field `ADDR_ECC` reader - ECC fail double-word address offset In case of ECC error or ECC correction detected, this bitfield contains double-word offset (multiple of 64 bits) to main Flash memory."]
pub type AddrEccR = crate::FieldReader<u16>;
#[doc = "Field `SYSF_ECC` reader - System Flash memory ECC fail This bit indicates that the ECC error correction or double ECC error detection is located in the system Flash memory."]
pub type SysfEccR = crate::BitReader;
#[doc = "ECC correction interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecccie {
    #[doc = "0: ECCC interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: ECCC interrupt enabled"]
    B0x1 = 1,
}
impl From<Ecccie> for bool {
    #[inline(always)]
    fn from(variant: Ecccie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCCIE` reader - ECC correction interrupt enable"]
pub type EcccieR = crate::BitReader<Ecccie>;
impl EcccieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecccie {
        match self.bits {
            false => Ecccie::B0x0,
            true => Ecccie::B0x1,
        }
    }
    #[doc = "ECCC interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ecccie::B0x0
    }
    #[doc = "ECCC interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ecccie::B0x1
    }
}
#[doc = "Field `ECCCIE` writer - ECC correction interrupt enable"]
pub type EcccieW<'a, REG> = crate::BitWriter<'a, REG, Ecccie>;
impl<'a, REG> EcccieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECCC interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ecccie::B0x0)
    }
    #[doc = "ECCC interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ecccie::B0x1)
    }
}
#[doc = "Field `ECCC` reader - ECC correction Set by hardware when one ECC error has been detected and corrected. An interrupt is generated if ECCIE is set. Cleared by writing 1."]
pub type EcccR = crate::BitReader;
#[doc = "Field `ECCC` writer - ECC correction Set by hardware when one ECC error has been detected and corrected. An interrupt is generated if ECCIE is set. Cleared by writing 1."]
pub type EcccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCD` reader - ECC detection Set by hardware when two ECC errors have been detected. When this bit is set, a NMI is generated. Cleared by writing 1."]
pub type EccdR = crate::BitReader;
#[doc = "Field `ECCD` writer - ECC detection Set by hardware when two ECC errors have been detected. When this bit is set, a NMI is generated. Cleared by writing 1."]
pub type EccdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - ECC fail double-word address offset In case of ECC error or ECC correction detected, this bitfield contains double-word offset (multiple of 64 bits) to main Flash memory."]
    #[inline(always)]
    pub fn addr_ecc(&self) -> AddrEccR {
        AddrEccR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 20 - System Flash memory ECC fail This bit indicates that the ECC error correction or double ECC error detection is located in the system Flash memory."]
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SysfEccR {
        SysfEccR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - ECC correction interrupt enable"]
    #[inline(always)]
    pub fn ecccie(&self) -> EcccieR {
        EcccieR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - ECC correction Set by hardware when one ECC error has been detected and corrected. An interrupt is generated if ECCIE is set. Cleared by writing 1."]
    #[inline(always)]
    pub fn eccc(&self) -> EcccR {
        EcccR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ECC detection Set by hardware when two ECC errors have been detected. When this bit is set, a NMI is generated. Cleared by writing 1."]
    #[inline(always)]
    pub fn eccd(&self) -> EccdR {
        EccdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - ECC correction interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ecccie(&mut self) -> EcccieW<FlashEccrSpec> {
        EcccieW::new(self, 24)
    }
    #[doc = "Bit 30 - ECC correction Set by hardware when one ECC error has been detected and corrected. An interrupt is generated if ECCIE is set. Cleared by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn eccc(&mut self) -> EcccW<FlashEccrSpec> {
        EcccW::new(self, 30)
    }
    #[doc = "Bit 31 - ECC detection Set by hardware when two ECC errors have been detected. When this bit is set, a NMI is generated. Cleared by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn eccd(&mut self) -> EccdW<FlashEccrSpec> {
        EccdW::new(self, 31)
    }
}
#[doc = "FLASH ECC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_eccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_eccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashEccrSpec;
impl crate::RegisterSpec for FlashEccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_eccr::R`](R) reader structure"]
impl crate::Readable for FlashEccrSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_eccr::W`](W) writer structure"]
impl crate::Writable for FlashEccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_ECCR to value 0"]
impl crate::Resettable for FlashEccrSpec {
    const RESET_VALUE: u32 = 0;
}
