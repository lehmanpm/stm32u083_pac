#[doc = "Register `DBGMCU_APB1FZR` reader"]
pub type R = crate::R<DbgmcuApb1fzrSpec>;
#[doc = "Register `DBGMCU_APB1FZR` writer"]
pub type W = crate::W<DbgmcuApb1fzrSpec>;
#[doc = "TIM2 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgTim2Stop {
    #[doc = "0: normal operation. TIM2 continues to operate while CPU is in debug mode."]
    B0x0 = 0,
    #[doc = "1: stop in debug. TIM2 is frozen while CPU is in debug mode."]
    B0x1 = 1,
}
impl From<DbgTim2Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgTim2Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM2_STOP` reader - TIM2 stop in debug"]
pub type DbgTim2StopR = crate::BitReader<DbgTim2Stop>;
impl DbgTim2StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgTim2Stop {
        match self.bits {
            false => DbgTim2Stop::B0x0,
            true => DbgTim2Stop::B0x1,
        }
    }
    #[doc = "normal operation. TIM2 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgTim2Stop::B0x0
    }
    #[doc = "stop in debug. TIM2 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgTim2Stop::B0x1
    }
}
#[doc = "Field `DBG_TIM2_STOP` writer - TIM2 stop in debug"]
pub type DbgTim2StopW<'a, REG> = crate::BitWriter<'a, REG, DbgTim2Stop>;
impl<'a, REG> DbgTim2StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. TIM2 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim2Stop::B0x0)
    }
    #[doc = "stop in debug. TIM2 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim2Stop::B0x1)
    }
}
#[doc = "TIM3 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgTim3Stop {
    #[doc = "0: normal operation. TIM3 continues to operate while CPU is in debug mode."]
    B0x0 = 0,
    #[doc = "1: stop in debug. TIM3 is frozen while CPU is in debug mode."]
    B0x1 = 1,
}
impl From<DbgTim3Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgTim3Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM3_STOP` reader - TIM3 stop in debug"]
pub type DbgTim3StopR = crate::BitReader<DbgTim3Stop>;
impl DbgTim3StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgTim3Stop {
        match self.bits {
            false => DbgTim3Stop::B0x0,
            true => DbgTim3Stop::B0x1,
        }
    }
    #[doc = "normal operation. TIM3 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgTim3Stop::B0x0
    }
    #[doc = "stop in debug. TIM3 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgTim3Stop::B0x1
    }
}
#[doc = "Field `DBG_TIM3_STOP` writer - TIM3 stop in debug"]
pub type DbgTim3StopW<'a, REG> = crate::BitWriter<'a, REG, DbgTim3Stop>;
impl<'a, REG> DbgTim3StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. TIM3 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim3Stop::B0x0)
    }
    #[doc = "stop in debug. TIM3 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim3Stop::B0x1)
    }
}
#[doc = "TIM4 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgTim4Stop {
    #[doc = "0: normal operation. TIM4 continues to operate while CPU is in debug mode."]
    B0x0 = 0,
    #[doc = "1: stop in debug. TIM34 is frozen while CPU is in debug mode"]
    B0x1 = 1,
}
impl From<DbgTim4Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgTim4Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM4_STOP` reader - TIM4 stop in debug"]
pub type DbgTim4StopR = crate::BitReader<DbgTim4Stop>;
impl DbgTim4StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgTim4Stop {
        match self.bits {
            false => DbgTim4Stop::B0x0,
            true => DbgTim4Stop::B0x1,
        }
    }
    #[doc = "normal operation. TIM4 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgTim4Stop::B0x0
    }
    #[doc = "stop in debug. TIM34 is frozen while CPU is in debug mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgTim4Stop::B0x1
    }
}
#[doc = "Field `DBG_TIM4_STOP` writer - TIM4 stop in debug"]
pub type DbgTim4StopW<'a, REG> = crate::BitWriter<'a, REG, DbgTim4Stop>;
impl<'a, REG> DbgTim4StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. TIM4 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim4Stop::B0x0)
    }
    #[doc = "stop in debug. TIM34 is frozen while CPU is in debug mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim4Stop::B0x1)
    }
}
#[doc = "TIM6 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgTim6Stop {
    #[doc = "0: normal operation. TIM6 continues to operate while CPU is in debug mode."]
    B0x0 = 0,
    #[doc = "1: stop in debug. TIM6 is frozen while CPU is in debug mode."]
    B0x1 = 1,
}
impl From<DbgTim6Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgTim6Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM6_STOP` reader - TIM6 stop in debug"]
pub type DbgTim6StopR = crate::BitReader<DbgTim6Stop>;
impl DbgTim6StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgTim6Stop {
        match self.bits {
            false => DbgTim6Stop::B0x0,
            true => DbgTim6Stop::B0x1,
        }
    }
    #[doc = "normal operation. TIM6 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgTim6Stop::B0x0
    }
    #[doc = "stop in debug. TIM6 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgTim6Stop::B0x1
    }
}
#[doc = "Field `DBG_TIM6_STOP` writer - TIM6 stop in debug"]
pub type DbgTim6StopW<'a, REG> = crate::BitWriter<'a, REG, DbgTim6Stop>;
impl<'a, REG> DbgTim6StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. TIM6 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim6Stop::B0x0)
    }
    #[doc = "stop in debug. TIM6 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim6Stop::B0x1)
    }
}
#[doc = "TIM7 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgTim7Stop {
    #[doc = "0: normal operation. TIM7 continues to operate while CPU is in debug mode."]
    B0x0 = 0,
    #[doc = "1: stop in debug. TIM7 is frozen while CPU is in debug mode."]
    B0x1 = 1,
}
impl From<DbgTim7Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgTim7Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM7_STOP` reader - TIM7 stop in debug"]
pub type DbgTim7StopR = crate::BitReader<DbgTim7Stop>;
impl DbgTim7StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgTim7Stop {
        match self.bits {
            false => DbgTim7Stop::B0x0,
            true => DbgTim7Stop::B0x1,
        }
    }
    #[doc = "normal operation. TIM7 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgTim7Stop::B0x0
    }
    #[doc = "stop in debug. TIM7 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgTim7Stop::B0x1
    }
}
#[doc = "Field `DBG_TIM7_STOP` writer - TIM7 stop in debug"]
pub type DbgTim7StopW<'a, REG> = crate::BitWriter<'a, REG, DbgTim7Stop>;
impl<'a, REG> DbgTim7StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. TIM7 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim7Stop::B0x0)
    }
    #[doc = "stop in debug. TIM7 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim7Stop::B0x1)
    }
}
#[doc = "RTC stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgRtcStop {
    #[doc = "0: normal operation. RTC counter continues to operate while CPU is in debug mode."]
    B0x0 = 0,
    #[doc = "1: stop in debug. RTC counter is frozen while CPU is in debug mode."]
    B0x1 = 1,
}
impl From<DbgRtcStop> for bool {
    #[inline(always)]
    fn from(variant: DbgRtcStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_RTC_STOP` reader - RTC stop in debug"]
pub type DbgRtcStopR = crate::BitReader<DbgRtcStop>;
impl DbgRtcStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgRtcStop {
        match self.bits {
            false => DbgRtcStop::B0x0,
            true => DbgRtcStop::B0x1,
        }
    }
    #[doc = "normal operation. RTC counter continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgRtcStop::B0x0
    }
    #[doc = "stop in debug. RTC counter is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgRtcStop::B0x1
    }
}
#[doc = "Field `DBG_RTC_STOP` writer - RTC stop in debug"]
pub type DbgRtcStopW<'a, REG> = crate::BitWriter<'a, REG, DbgRtcStop>;
impl<'a, REG> DbgRtcStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. RTC counter continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgRtcStop::B0x0)
    }
    #[doc = "stop in debug. RTC counter is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgRtcStop::B0x1)
    }
}
#[doc = "WWDG stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgWwdgStop {
    #[doc = "0: normal operation. WWDG continues to operate while CPU is in debug mode."]
    B0x0 = 0,
    #[doc = "1: stop in debug. WWDG is frozen while CPU is in debug mode."]
    B0x1 = 1,
}
impl From<DbgWwdgStop> for bool {
    #[inline(always)]
    fn from(variant: DbgWwdgStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_WWDG_STOP` reader - WWDG stop in debug"]
pub type DbgWwdgStopR = crate::BitReader<DbgWwdgStop>;
impl DbgWwdgStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgWwdgStop {
        match self.bits {
            false => DbgWwdgStop::B0x0,
            true => DbgWwdgStop::B0x1,
        }
    }
    #[doc = "normal operation. WWDG continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgWwdgStop::B0x0
    }
    #[doc = "stop in debug. WWDG is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgWwdgStop::B0x1
    }
}
#[doc = "Field `DBG_WWDG_STOP` writer - WWDG stop in debug"]
pub type DbgWwdgStopW<'a, REG> = crate::BitWriter<'a, REG, DbgWwdgStop>;
impl<'a, REG> DbgWwdgStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. WWDG continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgWwdgStop::B0x0)
    }
    #[doc = "stop in debug. WWDG is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgWwdgStop::B0x1)
    }
}
#[doc = "IWDG stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgIwdgStop {
    #[doc = "0: normal operation. IWDG continues to operate while CPU is in debug mode."]
    B0x0 = 0,
    #[doc = "1: stop in debug. IWDG is frozen while CPU is in debug mode."]
    B0x1 = 1,
}
impl From<DbgIwdgStop> for bool {
    #[inline(always)]
    fn from(variant: DbgIwdgStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_IWDG_STOP` reader - IWDG stop in debug"]
pub type DbgIwdgStopR = crate::BitReader<DbgIwdgStop>;
impl DbgIwdgStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgIwdgStop {
        match self.bits {
            false => DbgIwdgStop::B0x0,
            true => DbgIwdgStop::B0x1,
        }
    }
    #[doc = "normal operation. IWDG continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgIwdgStop::B0x0
    }
    #[doc = "stop in debug. IWDG is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgIwdgStop::B0x1
    }
}
#[doc = "Field `DBG_IWDG_STOP` writer - IWDG stop in debug"]
pub type DbgIwdgStopW<'a, REG> = crate::BitWriter<'a, REG, DbgIwdgStop>;
impl<'a, REG> DbgIwdgStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. IWDG continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgIwdgStop::B0x0)
    }
    #[doc = "stop in debug. IWDG is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgIwdgStop::B0x1)
    }
}
#[doc = "I2C3 SMBUS timeout stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgI2c3Stop {
    #[doc = "0: normal operation. I2C3 SMBUS timeout continues to operate while CPU is in debug mode."]
    B0x0 = 0,
    #[doc = "1: stop in debug. I2C3 SMBUS timeout is frozen while CPU is in debug mode."]
    B0x1 = 1,
}
impl From<DbgI2c3Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgI2c3Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_I2C3_STOP` reader - I2C3 SMBUS timeout stop in debug"]
pub type DbgI2c3StopR = crate::BitReader<DbgI2c3Stop>;
impl DbgI2c3StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgI2c3Stop {
        match self.bits {
            false => DbgI2c3Stop::B0x0,
            true => DbgI2c3Stop::B0x1,
        }
    }
    #[doc = "normal operation. I2C3 SMBUS timeout continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgI2c3Stop::B0x0
    }
    #[doc = "stop in debug. I2C3 SMBUS timeout is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgI2c3Stop::B0x1
    }
}
#[doc = "Field `DBG_I2C3_STOP` writer - I2C3 SMBUS timeout stop in debug"]
pub type DbgI2c3StopW<'a, REG> = crate::BitWriter<'a, REG, DbgI2c3Stop>;
impl<'a, REG> DbgI2c3StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. I2C3 SMBUS timeout continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgI2c3Stop::B0x0)
    }
    #[doc = "stop in debug. I2C3 SMBUS timeout is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgI2c3Stop::B0x1)
    }
}
#[doc = "I2C1 SMBUS timeout stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgI2c1Stop {
    #[doc = "0: normal operation. I2C1 SMBUS timeout continues to operate while CPU is in debug mode."]
    B0x0 = 0,
    #[doc = "1: stop in debug. I2C1 SMBUS timeout is frozen while CPU is in debug mode."]
    B0x1 = 1,
}
impl From<DbgI2c1Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgI2c1Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout stop in debug"]
pub type DbgI2c1StopR = crate::BitReader<DbgI2c1Stop>;
impl DbgI2c1StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgI2c1Stop {
        match self.bits {
            false => DbgI2c1Stop::B0x0,
            true => DbgI2c1Stop::B0x1,
        }
    }
    #[doc = "normal operation. I2C1 SMBUS timeout continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgI2c1Stop::B0x0
    }
    #[doc = "stop in debug. I2C1 SMBUS timeout is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgI2c1Stop::B0x1
    }
}
#[doc = "Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout stop in debug"]
pub type DbgI2c1StopW<'a, REG> = crate::BitWriter<'a, REG, DbgI2c1Stop>;
impl<'a, REG> DbgI2c1StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. I2C1 SMBUS timeout continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgI2c1Stop::B0x0)
    }
    #[doc = "stop in debug. I2C1 SMBUS timeout is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgI2c1Stop::B0x1)
    }
}
#[doc = "LPTIM2 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgLptim2Stop {
    #[doc = "0: normal operation. LPTIM2 continues to operate while CPU is in debug mode."]
    B0x0 = 0,
    #[doc = "1: stop in debug. LPTIM2 is frozen while CPU is in debug mode."]
    B0x1 = 1,
}
impl From<DbgLptim2Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgLptim2Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_LPTIM2_STOP` reader - LPTIM2 stop in debug"]
pub type DbgLptim2StopR = crate::BitReader<DbgLptim2Stop>;
impl DbgLptim2StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgLptim2Stop {
        match self.bits {
            false => DbgLptim2Stop::B0x0,
            true => DbgLptim2Stop::B0x1,
        }
    }
    #[doc = "normal operation. LPTIM2 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgLptim2Stop::B0x0
    }
    #[doc = "stop in debug. LPTIM2 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgLptim2Stop::B0x1
    }
}
#[doc = "Field `DBG_LPTIM2_STOP` writer - LPTIM2 stop in debug"]
pub type DbgLptim2StopW<'a, REG> = crate::BitWriter<'a, REG, DbgLptim2Stop>;
impl<'a, REG> DbgLptim2StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. LPTIM2 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgLptim2Stop::B0x0)
    }
    #[doc = "stop in debug. LPTIM2 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgLptim2Stop::B0x1)
    }
}
#[doc = "LPTIM1 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgLptim1Stop {
    #[doc = "0: normal operation. LPTIM1 continues to operate while CPU is in debug mode."]
    B0x0 = 0,
    #[doc = "1: stop in debug. LPTIM1 is frozen while CPU is in debug mode."]
    B0x1 = 1,
}
impl From<DbgLptim1Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgLptim1Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_LPTIM1_STOP` reader - LPTIM1 stop in debug"]
pub type DbgLptim1StopR = crate::BitReader<DbgLptim1Stop>;
impl DbgLptim1StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgLptim1Stop {
        match self.bits {
            false => DbgLptim1Stop::B0x0,
            true => DbgLptim1Stop::B0x1,
        }
    }
    #[doc = "normal operation. LPTIM1 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgLptim1Stop::B0x0
    }
    #[doc = "stop in debug. LPTIM1 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgLptim1Stop::B0x1
    }
}
#[doc = "Field `DBG_LPTIM1_STOP` writer - LPTIM1 stop in debug"]
pub type DbgLptim1StopW<'a, REG> = crate::BitWriter<'a, REG, DbgLptim1Stop>;
impl<'a, REG> DbgLptim1StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. LPTIM1 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgLptim1Stop::B0x0)
    }
    #[doc = "stop in debug. LPTIM1 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgLptim1Stop::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - TIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DbgTim2StopR {
        DbgTim2StopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DbgTim3StopR {
        DbgTim3StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DbgTim4StopR {
        DbgTim4StopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DbgTim6StopR {
        DbgTim6StopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DbgTim7StopR {
        DbgTim7StopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC stop in debug"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DbgRtcStopR {
        DbgRtcStopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG stop in debug"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DbgWwdgStopR {
        DbgWwdgStopR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IWDG stop in debug"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DbgIwdgStopR {
        DbgIwdgStopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C3 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DbgI2c3StopR {
        DbgI2c3StopR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DbgI2c1StopR {
        DbgI2c1StopR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 30 - LPTIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DbgLptim2StopR {
        DbgLptim2StopR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LPTIM1 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DbgLptim1StopR {
        DbgLptim1StopR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim2_stop(&mut self) -> DbgTim2StopW<DbgmcuApb1fzrSpec> {
        DbgTim2StopW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim3_stop(&mut self) -> DbgTim3StopW<DbgmcuApb1fzrSpec> {
        DbgTim3StopW::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim4_stop(&mut self) -> DbgTim4StopW<DbgmcuApb1fzrSpec> {
        DbgTim4StopW::new(self, 2)
    }
    #[doc = "Bit 4 - TIM6 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim6_stop(&mut self) -> DbgTim6StopW<DbgmcuApb1fzrSpec> {
        DbgTim6StopW::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim7_stop(&mut self) -> DbgTim7StopW<DbgmcuApb1fzrSpec> {
        DbgTim7StopW::new(self, 5)
    }
    #[doc = "Bit 10 - RTC stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DbgRtcStopW<DbgmcuApb1fzrSpec> {
        DbgRtcStopW::new(self, 10)
    }
    #[doc = "Bit 11 - WWDG stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DbgWwdgStopW<DbgmcuApb1fzrSpec> {
        DbgWwdgStopW::new(self, 11)
    }
    #[doc = "Bit 12 - IWDG stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DbgIwdgStopW<DbgmcuApb1fzrSpec> {
        DbgIwdgStopW::new(self, 12)
    }
    #[doc = "Bit 21 - I2C3 SMBUS timeout stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c3_stop(&mut self) -> DbgI2c3StopW<DbgmcuApb1fzrSpec> {
        DbgI2c3StopW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C1 SMBUS timeout stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c1_stop(&mut self) -> DbgI2c1StopW<DbgmcuApb1fzrSpec> {
        DbgI2c1StopW::new(self, 22)
    }
    #[doc = "Bit 30 - LPTIM2 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim2_stop(&mut self) -> DbgLptim2StopW<DbgmcuApb1fzrSpec> {
        DbgLptim2StopW::new(self, 30)
    }
    #[doc = "Bit 31 - LPTIM1 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim1_stop(&mut self) -> DbgLptim1StopW<DbgmcuApb1fzrSpec> {
        DbgLptim1StopW::new(self, 31)
    }
}
#[doc = "DBGMCU APB1 freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_apb1fzr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgmcu_apb1fzr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgmcuApb1fzrSpec;
impl crate::RegisterSpec for DbgmcuApb1fzrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_apb1fzr::R`](R) reader structure"]
impl crate::Readable for DbgmcuApb1fzrSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgmcu_apb1fzr::W`](W) writer structure"]
impl crate::Writable for DbgmcuApb1fzrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGMCU_APB1FZR to value 0"]
impl crate::Resettable for DbgmcuApb1fzrSpec {
    const RESET_VALUE: u32 = 0;
}
