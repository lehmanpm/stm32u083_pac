#[doc = "Register `DBGMCU_CIDR2` reader"]
pub type R = crate::R<DbgmcuCidr2Spec>;
#[doc = "component identification bits \\[23:16\\]\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Preamble {
    #[doc = "5: common identification value"]
    B0x05 = 5,
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
#[doc = "Field `PREAMBLE` reader - component identification bits \\[23:16\\]"]
pub type PreambleR = crate::FieldReader<Preamble>;
impl PreambleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Preamble> {
        match self.bits {
            5 => Some(Preamble::B0x05),
            _ => None,
        }
    }
    #[doc = "common identification value"]
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == Preamble::B0x05
    }
}
impl R {
    #[doc = "Bits 0:7 - component identification bits \\[23:16\\]"]
    #[inline(always)]
    pub fn preamble(&self) -> PreambleR {
        PreambleR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DBGMCU CoreSight component identity register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_cidr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgmcuCidr2Spec;
impl crate::RegisterSpec for DbgmcuCidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_cidr2::R`](R) reader structure"]
impl crate::Readable for DbgmcuCidr2Spec {}
#[doc = "`reset()` method sets DBGMCU_CIDR2 to value 0x05"]
impl crate::Resettable for DbgmcuCidr2Spec {
    const RESET_VALUE: u32 = 0x05;
}
