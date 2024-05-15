#[doc = "Register `DBGMCU_PIDR3` reader"]
pub type R = crate::R<DbgmcuPidr3Spec>;
#[doc = "customer modified\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmod {
    #[doc = "0: no customer modifications"]
    B0x0 = 0,
}
impl From<Cmod> for u8 {
    #[inline(always)]
    fn from(variant: Cmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmod {
    type Ux = u8;
}
impl crate::IsEnum for Cmod {}
#[doc = "Field `CMOD` reader - customer modified"]
pub type CmodR = crate::FieldReader<Cmod>;
impl CmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmod> {
        match self.bits {
            0 => Some(Cmod::B0x0),
            _ => None,
        }
    }
    #[doc = "no customer modifications"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cmod::B0x0
    }
}
#[doc = "metal fix version\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Revand {
    #[doc = "0: no metal fix"]
    B0x0 = 0,
}
impl From<Revand> for u8 {
    #[inline(always)]
    fn from(variant: Revand) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Revand {
    type Ux = u8;
}
impl crate::IsEnum for Revand {}
#[doc = "Field `REVAND` reader - metal fix version"]
pub type RevandR = crate::FieldReader<Revand>;
impl RevandR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Revand> {
        match self.bits {
            0 => Some(Revand::B0x0),
            _ => None,
        }
    }
    #[doc = "no metal fix"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Revand::B0x0
    }
}
impl R {
    #[doc = "Bits 0:3 - customer modified"]
    #[inline(always)]
    pub fn cmod(&self) -> CmodR {
        CmodR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - metal fix version"]
    #[inline(always)]
    pub fn revand(&self) -> RevandR {
        RevandR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DBGMCU CoreSight peripheral identity register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_pidr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgmcuPidr3Spec;
impl crate::RegisterSpec for DbgmcuPidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_pidr3::R`](R) reader structure"]
impl crate::Readable for DbgmcuPidr3Spec {}
#[doc = "`reset()` method sets DBGMCU_PIDR3 to value 0"]
impl crate::Resettable for DbgmcuPidr3Spec {
    const RESET_VALUE: u32 = 0;
}
