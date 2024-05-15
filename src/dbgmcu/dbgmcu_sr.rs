#[doc = "Register `DBGMCU_SR` reader"]
pub type R = crate::R<DbgmcuSrSpec>;
#[doc = "Identifies whether access port AP1 is present in device\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ap1Present {
    #[doc = "1: AP1 present"]
    B0x1 = 1,
}
impl From<Ap1Present> for bool {
    #[inline(always)]
    fn from(variant: Ap1Present) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AP1_PRESENT` reader - Identifies whether access port AP1 is present in device"]
pub type Ap1PresentR = crate::BitReader<Ap1Present>;
impl Ap1PresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ap1Present> {
        match self.bits {
            true => Some(Ap1Present::B0x1),
            _ => None,
        }
    }
    #[doc = "AP1 present"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ap1Present::B0x1
    }
}
#[doc = "Identifies whether access port AP0 is present in device\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ap0Present {
    #[doc = "1: AP0 present"]
    B0x1 = 1,
}
impl From<Ap0Present> for bool {
    #[inline(always)]
    fn from(variant: Ap0Present) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AP0_PRESENT` reader - Identifies whether access port AP0 is present in device"]
pub type Ap0PresentR = crate::BitReader<Ap0Present>;
impl Ap0PresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ap0Present> {
        match self.bits {
            true => Some(Ap0Present::B0x1),
            _ => None,
        }
    }
    #[doc = "AP0 present"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ap0Present::B0x1
    }
}
#[doc = "Identifies whether access port AP0 is open (can be accessed via the debug port) or locked (debug access to the AP is blocked)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ap1Enabled {
    #[doc = "0: AP1 locked"]
    B0x0 = 0,
    #[doc = "1: AP1 enabled"]
    B0x1 = 1,
}
impl From<Ap1Enabled> for bool {
    #[inline(always)]
    fn from(variant: Ap1Enabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AP1_ENABLED` reader - Identifies whether access port AP0 is open (can be accessed via the debug port) or locked (debug access to the AP is blocked)"]
pub type Ap1EnabledR = crate::BitReader<Ap1Enabled>;
impl Ap1EnabledR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ap1Enabled {
        match self.bits {
            false => Ap1Enabled::B0x0,
            true => Ap1Enabled::B0x1,
        }
    }
    #[doc = "AP1 locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ap1Enabled::B0x0
    }
    #[doc = "AP1 enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ap1Enabled::B0x1
    }
}
#[doc = "Identifies whether access port AP0 is open (can be accessed via the debug port) or locked (debug access to the AP is blocked)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ap0Enabled {
    #[doc = "0: AP0 locked"]
    B0x0 = 0,
    #[doc = "1: AP0 enabled"]
    B0x1 = 1,
}
impl From<Ap0Enabled> for bool {
    #[inline(always)]
    fn from(variant: Ap0Enabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AP0_ENABLED` reader - Identifies whether access port AP0 is open (can be accessed via the debug port) or locked (debug access to the AP is blocked)"]
pub type Ap0EnabledR = crate::BitReader<Ap0Enabled>;
impl Ap0EnabledR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ap0Enabled {
        match self.bits {
            false => Ap0Enabled::B0x0,
            true => Ap0Enabled::B0x1,
        }
    }
    #[doc = "AP0 locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ap0Enabled::B0x0
    }
    #[doc = "AP0 enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ap0Enabled::B0x1
    }
}
impl R {
    #[doc = "Bit 0 - Identifies whether access port AP1 is present in device"]
    #[inline(always)]
    pub fn ap1_present(&self) -> Ap1PresentR {
        Ap1PresentR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Identifies whether access port AP0 is present in device"]
    #[inline(always)]
    pub fn ap0_present(&self) -> Ap0PresentR {
        Ap0PresentR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Identifies whether access port AP0 is open (can be accessed via the debug port) or locked (debug access to the AP is blocked)"]
    #[inline(always)]
    pub fn ap1_enabled(&self) -> Ap1EnabledR {
        Ap1EnabledR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Identifies whether access port AP0 is open (can be accessed via the debug port) or locked (debug access to the AP is blocked)"]
    #[inline(always)]
    pub fn ap0_enabled(&self) -> Ap0EnabledR {
        Ap0EnabledR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "DBGMCU status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgmcuSrSpec;
impl crate::RegisterSpec for DbgmcuSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_sr::R`](R) reader structure"]
impl crate::Readable for DbgmcuSrSpec {}
#[doc = "`reset()` method sets DBGMCU_SR to value 0x0001_0003"]
impl crate::Resettable for DbgmcuSrSpec {
    const RESET_VALUE: u32 = 0x0001_0003;
}
