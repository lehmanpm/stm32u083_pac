#[doc = "Register `PWR_SR1` reader"]
pub type R = crate::R<PwrSr1Spec>;
#[doc = "Field `WUF1` reader - Wake-up flag 1 This bit is set when a wake-up event is detected on wake-up pin, WKUP1. It is cleared by writing 1 in the CWUF1 bit of the PWR_SCR register."]
pub type Wuf1R = crate::BitReader;
#[doc = "Field `WUF2` reader - Wake-up flag 2 This bit is set when a wake-up event is detected on wake-up pin, WKUP2. It is cleared by writing 1 in the CWUF2 bit of the PWR_SCR register."]
pub type Wuf2R = crate::BitReader;
#[doc = "Field `WUF3` reader - Wake-up flag 3 This bit is set when a wake-up event is detected on wake-up pin, WKUP3. It is cleared by writing 1 in the CWUF3 bit of the PWR_SCR register."]
pub type Wuf3R = crate::BitReader;
#[doc = "Field `WUF4` reader - Wake-up flag 4 This bit is set when a wake-up event is detected on wake-up pin,WKUP4. It is cleared by writing 1 in the CWUF4 bit of the PWR_SCR register."]
pub type Wuf4R = crate::BitReader;
#[doc = "Field `WUF5` reader - Wake-up flag 5 This bit is set when a wake-up event is detected on wake-up pin, WKUP5. It is cleared by writing 1 in the CWUF5 bit of the PWR_SCR register."]
pub type Wuf5R = crate::BitReader;
#[doc = "Field `WUF7` reader - Wake-up flag 7 This bit is set when a wake-up event is detected on wake-up pin, WKUP7. It is cleared by writing 1 in the CWUF7 bit of the PWR_SCR register."]
pub type Wuf7R = crate::BitReader;
#[doc = "Standby flag This bit is set by hardware when the device enters the Standby mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbf {
    #[doc = "0: The device did not enter the Standby mode"]
    B0x0 = 0,
    #[doc = "1: The device entered the Standby mode"]
    B0x1 = 1,
}
impl From<Sbf> for bool {
    #[inline(always)]
    fn from(variant: Sbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBF` reader - Standby flag This bit is set by hardware when the device enters the Standby mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset."]
pub type SbfR = crate::BitReader<Sbf>;
impl SbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbf {
        match self.bits {
            false => Sbf::B0x0,
            true => Sbf::B0x1,
        }
    }
    #[doc = "The device did not enter the Standby mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sbf::B0x0
    }
    #[doc = "The device entered the Standby mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sbf::B0x1
    }
}
#[doc = "Stop Flags These bits are set by hardware when the device enters any stop mode and are cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stopf {
    #[doc = "0: The device did not enter any Stop mode."]
    B0x0 = 0,
    #[doc = "4: The device entered in Stop 0 mode."]
    B0x4 = 4,
    #[doc = "5: The device entered in Stop 1 mode."]
    B0x5 = 5,
    #[doc = "6: The device entered in Stop 2 mode."]
    B0x6 = 6,
}
impl From<Stopf> for u8 {
    #[inline(always)]
    fn from(variant: Stopf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stopf {
    type Ux = u8;
}
impl crate::IsEnum for Stopf {}
#[doc = "Field `STOPF` reader - Stop Flags These bits are set by hardware when the device enters any stop mode and are cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset."]
pub type StopfR = crate::FieldReader<Stopf>;
impl StopfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stopf> {
        match self.bits {
            0 => Some(Stopf::B0x0),
            4 => Some(Stopf::B0x4),
            5 => Some(Stopf::B0x5),
            6 => Some(Stopf::B0x6),
            _ => None,
        }
    }
    #[doc = "The device did not enter any Stop mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Stopf::B0x0
    }
    #[doc = "The device entered in Stop 0 mode."]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Stopf::B0x4
    }
    #[doc = "The device entered in Stop 1 mode."]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Stopf::B0x5
    }
    #[doc = "The device entered in Stop 2 mode."]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Stopf::B0x6
    }
}
#[doc = "Field `WUFI` reader - Wake-up flag internal This bit is set when a wake-up is detected on the internal wake-up line. It is cleared when all internal wake-up sources are cleared."]
pub type WufiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Wake-up flag 1 This bit is set when a wake-up event is detected on wake-up pin, WKUP1. It is cleared by writing 1 in the CWUF1 bit of the PWR_SCR register."]
    #[inline(always)]
    pub fn wuf1(&self) -> Wuf1R {
        Wuf1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-up flag 2 This bit is set when a wake-up event is detected on wake-up pin, WKUP2. It is cleared by writing 1 in the CWUF2 bit of the PWR_SCR register."]
    #[inline(always)]
    pub fn wuf2(&self) -> Wuf2R {
        Wuf2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-up flag 3 This bit is set when a wake-up event is detected on wake-up pin, WKUP3. It is cleared by writing 1 in the CWUF3 bit of the PWR_SCR register."]
    #[inline(always)]
    pub fn wuf3(&self) -> Wuf3R {
        Wuf3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake-up flag 4 This bit is set when a wake-up event is detected on wake-up pin,WKUP4. It is cleared by writing 1 in the CWUF4 bit of the PWR_SCR register."]
    #[inline(always)]
    pub fn wuf4(&self) -> Wuf4R {
        Wuf4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake-up flag 5 This bit is set when a wake-up event is detected on wake-up pin, WKUP5. It is cleared by writing 1 in the CWUF5 bit of the PWR_SCR register."]
    #[inline(always)]
    pub fn wuf5(&self) -> Wuf5R {
        Wuf5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Wake-up flag 7 This bit is set when a wake-up event is detected on wake-up pin, WKUP7. It is cleared by writing 1 in the CWUF7 bit of the PWR_SCR register."]
    #[inline(always)]
    pub fn wuf7(&self) -> Wuf7R {
        Wuf7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Standby flag This bit is set by hardware when the device enters the Standby mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset."]
    #[inline(always)]
    pub fn sbf(&self) -> SbfR {
        SbfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Stop Flags These bits are set by hardware when the device enters any stop mode and are cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset."]
    #[inline(always)]
    pub fn stopf(&self) -> StopfR {
        StopfR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 15 - Wake-up flag internal This bit is set when a wake-up is detected on the internal wake-up line. It is cleared when all internal wake-up sources are cleared."]
    #[inline(always)]
    pub fn wufi(&self) -> WufiR {
        WufiR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Power status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_sr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrSr1Spec;
impl crate::RegisterSpec for PwrSr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_sr1::R`](R) reader structure"]
impl crate::Readable for PwrSr1Spec {}
#[doc = "`reset()` method sets PWR_SR1 to value 0"]
impl crate::Resettable for PwrSr1Spec {
    const RESET_VALUE: u32 = 0;
}
