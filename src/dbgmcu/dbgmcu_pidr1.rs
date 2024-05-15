#[doc = "Register `DBGMCU_PIDR1` reader"]
pub type R = crate::R<DbgmcuPidr1Spec>;
#[doc = "part number bits \\[11:8\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Partnum {
    #[doc = "0: DBGMCU part number"]
    B0x0 = 0,
}
impl From<Partnum> for u8 {
    #[inline(always)]
    fn from(variant: Partnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Partnum {
    type Ux = u8;
}
impl crate::IsEnum for Partnum {}
#[doc = "Field `PARTNUM` reader - part number bits \\[11:8\\]"]
pub type PartnumR = crate::FieldReader<Partnum>;
impl PartnumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Partnum> {
        match self.bits {
            0 => Some(Partnum::B0x0),
            _ => None,
        }
    }
    #[doc = "DBGMCU part number"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Partnum::B0x0
    }
}
#[doc = "JEP106 identity code bits \\[3:0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Jep106id {
    #[doc = "0: STMicroelectronics JEDEC code"]
    B0x0 = 0,
}
impl From<Jep106id> for u8 {
    #[inline(always)]
    fn from(variant: Jep106id) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Jep106id {
    type Ux = u8;
}
impl crate::IsEnum for Jep106id {}
#[doc = "Field `JEP106ID` reader - JEP106 identity code bits \\[3:0\\]"]
pub type Jep106idR = crate::FieldReader<Jep106id>;
impl Jep106idR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Jep106id> {
        match self.bits {
            0 => Some(Jep106id::B0x0),
            _ => None,
        }
    }
    #[doc = "STMicroelectronics JEDEC code"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Jep106id::B0x0
    }
}
impl R {
    #[doc = "Bits 0:3 - part number bits \\[11:8\\]"]
    #[inline(always)]
    pub fn partnum(&self) -> PartnumR {
        PartnumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - JEP106 identity code bits \\[3:0\\]"]
    #[inline(always)]
    pub fn jep106id(&self) -> Jep106idR {
        Jep106idR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DBGMCU CoreSight peripheral identity register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_pidr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgmcuPidr1Spec;
impl crate::RegisterSpec for DbgmcuPidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_pidr1::R`](R) reader structure"]
impl crate::Readable for DbgmcuPidr1Spec {}
#[doc = "`reset()` method sets DBGMCU_PIDR1 to value 0"]
impl crate::Resettable for DbgmcuPidr1Spec {
    const RESET_VALUE: u32 = 0;
}
