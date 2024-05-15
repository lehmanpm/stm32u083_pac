#[doc = "Register `DBGMCU_IDCODE` reader"]
pub type R = crate::R<DbgmcuIdcodeSpec>;
#[doc = "Device identifier This field indicates the device ID.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum DevId {
    #[doc = "1113: STM32U031xx"]
    B0x459 = 1113,
    #[doc = "1161: STM32U073/083xx"]
    B0x489 = 1161,
}
impl From<DevId> for u16 {
    #[inline(always)]
    fn from(variant: DevId) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DevId {
    type Ux = u16;
}
impl crate::IsEnum for DevId {}
#[doc = "Field `DEV_ID` reader - Device identifier This field indicates the device ID."]
pub type DevIdR = crate::FieldReader<DevId>;
impl DevIdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DevId> {
        match self.bits {
            1113 => Some(DevId::B0x459),
            1161 => Some(DevId::B0x489),
            _ => None,
        }
    }
    #[doc = "STM32U031xx"]
    #[inline(always)]
    pub fn is_b_0x459(&self) -> bool {
        *self == DevId::B0x459
    }
    #[doc = "STM32U073/083xx"]
    #[inline(always)]
    pub fn is_b_0x489(&self) -> bool {
        *self == DevId::B0x489
    }
}
#[doc = "Revision identifier This field indicates the revision of the device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum RevId {
    #[doc = "4096: Revision A for STM32U031/73/83xx"]
    B0x1000 = 4096,
}
impl From<RevId> for u16 {
    #[inline(always)]
    fn from(variant: RevId) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RevId {
    type Ux = u16;
}
impl crate::IsEnum for RevId {}
#[doc = "Field `REV_ID` reader - Revision identifier This field indicates the revision of the device."]
pub type RevIdR = crate::FieldReader<RevId>;
impl RevIdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RevId> {
        match self.bits {
            4096 => Some(RevId::B0x1000),
            _ => None,
        }
    }
    #[doc = "Revision A for STM32U031/73/83xx"]
    #[inline(always)]
    pub fn is_b_0x1000(&self) -> bool {
        *self == RevId::B0x1000
    }
}
impl R {
    #[doc = "Bits 0:11 - Device identifier This field indicates the device ID."]
    #[inline(always)]
    pub fn dev_id(&self) -> DevIdR {
        DevIdR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - Revision identifier This field indicates the revision of the device."]
    #[inline(always)]
    pub fn rev_id(&self) -> RevIdR {
        RevIdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "DBGMCU device ID code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_idcode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgmcuIdcodeSpec;
impl crate::RegisterSpec for DbgmcuIdcodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_idcode::R`](R) reader structure"]
impl crate::Readable for DbgmcuIdcodeSpec {}
#[doc = "`reset()` method sets DBGMCU_IDCODE to value 0x6000"]
impl crate::Resettable for DbgmcuIdcodeSpec {
    const RESET_VALUE: u32 = 0x6000;
}
