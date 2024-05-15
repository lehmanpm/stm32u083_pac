#[doc = "Register `DBGMCU_CIDR0` reader"]
pub type R = crate::R<DbgmcuCidr0Spec>;
#[doc = "component identification bits \\[7:0\\]\n\nValue on reset: 13"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Preamble {
    #[doc = "13: common identification value"]
    B0x0d = 13,
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
#[doc = "Field `PREAMBLE` reader - component identification bits \\[7:0\\]"]
pub type PreambleR = crate::FieldReader<Preamble>;
impl PreambleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Preamble> {
        match self.bits {
            13 => Some(Preamble::B0x0d),
            _ => None,
        }
    }
    #[doc = "common identification value"]
    #[inline(always)]
    pub fn is_b_0x0d(&self) -> bool {
        *self == Preamble::B0x0d
    }
}
impl R {
    #[doc = "Bits 0:7 - component identification bits \\[7:0\\]"]
    #[inline(always)]
    pub fn preamble(&self) -> PreambleR {
        PreambleR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DBGMCU CoreSight component identity register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_cidr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgmcuCidr0Spec;
impl crate::RegisterSpec for DbgmcuCidr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_cidr0::R`](R) reader structure"]
impl crate::Readable for DbgmcuCidr0Spec {}
#[doc = "`reset()` method sets DBGMCU_CIDR0 to value 0x0d"]
impl crate::Resettable for DbgmcuCidr0Spec {
    const RESET_VALUE: u32 = 0x0d;
}
