#[doc = "Register `DBGMCU_CIDR1` reader"]
pub type R = crate::R<DbgmcuCidr1Spec>;
#[doc = "component identification bits \\[11:8\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Preamble {
    #[doc = "0: common identification value"]
    B0x0 = 0,
}
impl From<Preamble> for u8 {
    #[inline(always)]
    fn from(variant: Preamble) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Preamble {
    type Ux = u8;
}
impl crate::IsEnum for Preamble {}
#[doc = "Field `PREAMBLE` reader - component identification bits \\[11:8\\]"]
pub type PreambleR = crate::FieldReader<Preamble>;
impl PreambleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Preamble> {
        match self.bits {
            0 => Some(Preamble::B0x0),
            _ => None,
        }
    }
    #[doc = "common identification value"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Preamble::B0x0
    }
}
#[doc = "component identification bits \\[15:12\\]
- component class\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Class {
    #[doc = "15: Non-CoreSight component"]
    B0xF = 15,
}
impl From<Class> for u8 {
    #[inline(always)]
    fn from(variant: Class) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Class {
    type Ux = u8;
}
impl crate::IsEnum for Class {}
#[doc = "Field `CLASS` reader - component identification bits \\[15:12\\]
- component class"]
pub type ClassR = crate::FieldReader<Class>;
impl ClassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Class> {
        match self.bits {
            15 => Some(Class::B0xF),
            _ => None,
        }
    }
    #[doc = "Non-CoreSight component"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Class::B0xF
    }
}
impl R {
    #[doc = "Bits 0:3 - component identification bits \\[11:8\\]"]
    #[inline(always)]
    pub fn preamble(&self) -> PreambleR {
        PreambleR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - component identification bits \\[15:12\\]
- component class"]
    #[inline(always)]
    pub fn class(&self) -> ClassR {
        ClassR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DBGMCU CoreSight component identity register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_cidr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgmcuCidr1Spec;
impl crate::RegisterSpec for DbgmcuCidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_cidr1::R`](R) reader structure"]
impl crate::Readable for DbgmcuCidr1Spec {}
#[doc = "`reset()` method sets DBGMCU_CIDR1 to value 0xf0"]
impl crate::Resettable for DbgmcuCidr1Spec {
    const RESET_VALUE: u32 = 0xf0;
}
