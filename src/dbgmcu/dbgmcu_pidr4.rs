#[doc = "Register `DBGMCU_PIDR4` reader"]
pub type R = crate::R<DbgmcuPidr4Spec>;
#[doc = "JEP106 continuation code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Jep106con {
    #[doc = "0: STMicroelectronics JEDEC code"]
    B0x0 = 0,
}
impl From<Jep106con> for u8 {
    #[inline(always)]
    fn from(variant: Jep106con) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Jep106con {
    type Ux = u8;
}
impl crate::IsEnum for Jep106con {}
#[doc = "Field `JEP106CON` reader - JEP106 continuation code"]
pub type Jep106conR = crate::FieldReader<Jep106con>;
impl Jep106conR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Jep106con> {
        match self.bits {
            0 => Some(Jep106con::B0x0),
            _ => None,
        }
    }
    #[doc = "STMicroelectronics JEDEC code"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Jep106con::B0x0
    }
}
#[doc = "register file size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Size {
    #[doc = "0: The register file occupies a single 4-Kbyte region."]
    B0x0 = 0,
}
impl From<Size> for u8 {
    #[inline(always)]
    fn from(variant: Size) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Size {
    type Ux = u8;
}
impl crate::IsEnum for Size {}
#[doc = "Field `SIZE` reader - register file size"]
pub type SizeR = crate::FieldReader<Size>;
impl SizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Size> {
        match self.bits {
            0 => Some(Size::B0x0),
            _ => None,
        }
    }
    #[doc = "The register file occupies a single 4-Kbyte region."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Size::B0x0
    }
}
impl R {
    #[doc = "Bits 0:3 - JEP106 continuation code"]
    #[inline(always)]
    pub fn jep106con(&self) -> Jep106conR {
        Jep106conR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - register file size"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DBGMCU CoreSight peripheral identity register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_pidr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgmcuPidr4Spec;
impl crate::RegisterSpec for DbgmcuPidr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_pidr4::R`](R) reader structure"]
impl crate::Readable for DbgmcuPidr4Spec {}
#[doc = "`reset()` method sets DBGMCU_PIDR4 to value 0"]
impl crate::Resettable for DbgmcuPidr4Spec {
    const RESET_VALUE: u32 = 0;
}
