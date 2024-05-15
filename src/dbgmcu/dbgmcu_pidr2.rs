#[doc = "Register `DBGMCU_PIDR2` reader"]
pub type R = crate::R<DbgmcuPidr2Spec>;
#[doc = "JEP106 identity code bits \\[6:4\\]\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Jep106id {
    #[doc = "2: STMicroelectronics JEDEC code"]
    B0x2 = 2,
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
#[doc = "Field `JEP106ID` reader - JEP106 identity code bits \\[6:4\\]"]
pub type Jep106idR = crate::FieldReader<Jep106id>;
impl Jep106idR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Jep106id> {
        match self.bits {
            2 => Some(Jep106id::B0x2),
            _ => None,
        }
    }
    #[doc = "STMicroelectronics JEDEC code"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Jep106id::B0x2
    }
}
#[doc = "JEDEC assigned value\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jedec {
    #[doc = "1: designer identification specified by JEDEC"]
    B0x1 = 1,
}
impl From<Jedec> for bool {
    #[inline(always)]
    fn from(variant: Jedec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEDEC` reader - JEDEC assigned value"]
pub type JedecR = crate::BitReader<Jedec>;
impl JedecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Jedec> {
        match self.bits {
            true => Some(Jedec::B0x1),
            _ => None,
        }
    }
    #[doc = "designer identification specified by JEDEC"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Jedec::B0x1
    }
}
#[doc = "component revision number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Revision {
    #[doc = "0: r0p0"]
    B0x0 = 0,
}
impl From<Revision> for u8 {
    #[inline(always)]
    fn from(variant: Revision) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Revision {
    type Ux = u8;
}
impl crate::IsEnum for Revision {}
#[doc = "Field `REVISION` reader - component revision number"]
pub type RevisionR = crate::FieldReader<Revision>;
impl RevisionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Revision> {
        match self.bits {
            0 => Some(Revision::B0x0),
            _ => None,
        }
    }
    #[doc = "r0p0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Revision::B0x0
    }
}
impl R {
    #[doc = "Bits 0:2 - JEP106 identity code bits \\[6:4\\]"]
    #[inline(always)]
    pub fn jep106id(&self) -> Jep106idR {
        Jep106idR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - JEDEC assigned value"]
    #[inline(always)]
    pub fn jedec(&self) -> JedecR {
        JedecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - component revision number"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DBGMCU CoreSight peripheral identity register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_pidr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgmcuPidr2Spec;
impl crate::RegisterSpec for DbgmcuPidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_pidr2::R`](R) reader structure"]
impl crate::Readable for DbgmcuPidr2Spec {}
#[doc = "`reset()` method sets DBGMCU_PIDR2 to value 0x0a"]
impl crate::Resettable for DbgmcuPidr2Spec {
    const RESET_VALUE: u32 = 0x0a;
}
