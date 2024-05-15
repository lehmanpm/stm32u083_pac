#[doc = "Register `DBGMCU_CIDR3` reader"]
pub type R = crate::R<DbgmcuCidr3Spec>;
#[doc = "component identification bits \\[31:24\\]\n\nValue on reset: 177"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Preamble {
    #[doc = "177: common identification value"]
    B0xB1 = 177,
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
#[doc = "Field `PREAMBLE` reader - component identification bits \\[31:24\\]"]
pub type PreambleR = crate::FieldReader<Preamble>;
impl PreambleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Preamble> {
        match self.bits {
            177 => Some(Preamble::B0xB1),
            _ => None,
        }
    }
    #[doc = "common identification value"]
    #[inline(always)]
    pub fn is_b_0x_b1(&self) -> bool {
        *self == Preamble::B0xB1
    }
}
impl R {
    #[doc = "Bits 0:7 - component identification bits \\[31:24\\]"]
    #[inline(always)]
    pub fn preamble(&self) -> PreambleR {
        PreambleR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DBGMCU CoreSight component identity register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_cidr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgmcuCidr3Spec;
impl crate::RegisterSpec for DbgmcuCidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_cidr3::R`](R) reader structure"]
impl crate::Readable for DbgmcuCidr3Spec {}
#[doc = "`reset()` method sets DBGMCU_CIDR3 to value 0xb1"]
impl crate::Resettable for DbgmcuCidr3Spec {
    const RESET_VALUE: u32 = 0xb1;
}
