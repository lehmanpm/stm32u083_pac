#[doc = "Register `DBGMCU_PIDR0` reader"]
pub type R = crate::R<DbgmcuPidr0Spec>;
#[doc = "part number bits \\[7:0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Partnum {
    #[doc = "0: DBGMCU part number"]
    B0x00 = 0,
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
#[doc = "Field `PARTNUM` reader - part number bits \\[7:0\\]"]
pub type PartnumR = crate::FieldReader<Partnum>;
impl PartnumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Partnum> {
        match self.bits {
            0 => Some(Partnum::B0x00),
            _ => None,
        }
    }
    #[doc = "DBGMCU part number"]
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == Partnum::B0x00
    }
}
impl R {
    #[doc = "Bits 0:7 - part number bits \\[7:0\\]"]
    #[inline(always)]
    pub fn partnum(&self) -> PartnumR {
        PartnumR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DBGMCU CoreSight peripheral identity register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_pidr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgmcuPidr0Spec;
impl crate::RegisterSpec for DbgmcuPidr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_pidr0::R`](R) reader structure"]
impl crate::Readable for DbgmcuPidr0Spec {}
#[doc = "`reset()` method sets DBGMCU_PIDR0 to value 0"]
impl crate::Resettable for DbgmcuPidr0Spec {
    const RESET_VALUE: u32 = 0;
}
