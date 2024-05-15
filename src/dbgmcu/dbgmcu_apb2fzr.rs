#[doc = "Register `DBGMCU_APB2FZR` reader"]
pub type R = crate::R<DbgmcuApb2fzrSpec>;
#[doc = "Register `DBGMCU_APB2FZR` writer"]
pub type W = crate::W<DbgmcuApb2fzrSpec>;
#[doc = "TIM1 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgTim1Stop {
    #[doc = "0: normal operation. TIM1 continues to operate while CPU is in debug mode."]
    B0x0 = 0,
    #[doc = "1: stop in debug. TIM1 is frozen while CPU is in debug mode."]
    B0x1 = 1,
}
impl From<DbgTim1Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgTim1Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM1_STOP` reader - TIM1 stop in debug"]
pub type DbgTim1StopR = crate::BitReader<DbgTim1Stop>;
impl DbgTim1StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgTim1Stop {
        match self.bits {
            false => DbgTim1Stop::B0x0,
            true => DbgTim1Stop::B0x1,
        }
    }
    #[doc = "normal operation. TIM1 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgTim1Stop::B0x0
    }
    #[doc = "stop in debug. TIM1 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgTim1Stop::B0x1
    }
}
#[doc = "Field `DBG_TIM1_STOP` writer - TIM1 stop in debug"]
pub type DbgTim1StopW<'a, REG> = crate::BitWriter<'a, REG, DbgTim1Stop>;
impl<'a, REG> DbgTim1StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. TIM1 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim1Stop::B0x0)
    }
    #[doc = "stop in debug. TIM1 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim1Stop::B0x1)
    }
}
#[doc = "TIM14 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgTim14Stop {
    #[doc = "0: normal operation. TIM14 continues to operate while CPU is in debug mode."]
    B0x0 = 0,
    #[doc = "1: stop in debug. TIM14 is frozen while CPU is in debug mode."]
    B0x1 = 1,
}
impl From<DbgTim14Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgTim14Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM14_STOP` reader - TIM14 stop in debug"]
pub type DbgTim14StopR = crate::BitReader<DbgTim14Stop>;
impl DbgTim14StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgTim14Stop {
        match self.bits {
            false => DbgTim14Stop::B0x0,
            true => DbgTim14Stop::B0x1,
        }
    }
    #[doc = "normal operation. TIM14 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgTim14Stop::B0x0
    }
    #[doc = "stop in debug. TIM14 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgTim14Stop::B0x1
    }
}
#[doc = "Field `DBG_TIM14_STOP` writer - TIM14 stop in debug"]
pub type DbgTim14StopW<'a, REG> = crate::BitWriter<'a, REG, DbgTim14Stop>;
impl<'a, REG> DbgTim14StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. TIM14 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim14Stop::B0x0)
    }
    #[doc = "stop in debug. TIM14 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim14Stop::B0x1)
    }
}
#[doc = "TIM15 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgTim15Stop {
    #[doc = "0: normal operation. TIM15 continues to operate while CPU is in debug mode."]
    B0x0 = 0,
    #[doc = "1: stop in debug. TIM15 is frozen while CPU is in debug mode."]
    B0x1 = 1,
}
impl From<DbgTim15Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgTim15Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM15_STOP` reader - TIM15 stop in debug"]
pub type DbgTim15StopR = crate::BitReader<DbgTim15Stop>;
impl DbgTim15StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgTim15Stop {
        match self.bits {
            false => DbgTim15Stop::B0x0,
            true => DbgTim15Stop::B0x1,
        }
    }
    #[doc = "normal operation. TIM15 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgTim15Stop::B0x0
    }
    #[doc = "stop in debug. TIM15 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgTim15Stop::B0x1
    }
}
#[doc = "Field `DBG_TIM15_STOP` writer - TIM15 stop in debug"]
pub type DbgTim15StopW<'a, REG> = crate::BitWriter<'a, REG, DbgTim15Stop>;
impl<'a, REG> DbgTim15StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. TIM15 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim15Stop::B0x0)
    }
    #[doc = "stop in debug. TIM15 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim15Stop::B0x1)
    }
}
#[doc = "TIM16 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgTim16Stop {
    #[doc = "0: normal operation. TIM16 continues to operate while CPU is in debug mode."]
    B0x0 = 0,
    #[doc = "1: stop in debug. TIM16 is frozen while CPU is in debug mode."]
    B0x1 = 1,
}
impl From<DbgTim16Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgTim16Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM16_STOP` reader - TIM16 stop in debug"]
pub type DbgTim16StopR = crate::BitReader<DbgTim16Stop>;
impl DbgTim16StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgTim16Stop {
        match self.bits {
            false => DbgTim16Stop::B0x0,
            true => DbgTim16Stop::B0x1,
        }
    }
    #[doc = "normal operation. TIM16 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgTim16Stop::B0x0
    }
    #[doc = "stop in debug. TIM16 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgTim16Stop::B0x1
    }
}
#[doc = "Field `DBG_TIM16_STOP` writer - TIM16 stop in debug"]
pub type DbgTim16StopW<'a, REG> = crate::BitWriter<'a, REG, DbgTim16Stop>;
impl<'a, REG> DbgTim16StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. TIM16 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim16Stop::B0x0)
    }
    #[doc = "stop in debug. TIM16 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgTim16Stop::B0x1)
    }
}
#[doc = "LPTIM3 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgLptim3Stop {
    #[doc = "0: normal operation. LPTIM3 continues to operate while CPU is in debug mode."]
    B0x0 = 0,
    #[doc = "1: stop in debug. LPTIM3 is frozen while CPU is in debug mode."]
    B0x1 = 1,
}
impl From<DbgLptim3Stop> for bool {
    #[inline(always)]
    fn from(variant: DbgLptim3Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_LPTIM3_STOP` reader - LPTIM3 stop in debug"]
pub type DbgLptim3StopR = crate::BitReader<DbgLptim3Stop>;
impl DbgLptim3StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgLptim3Stop {
        match self.bits {
            false => DbgLptim3Stop::B0x0,
            true => DbgLptim3Stop::B0x1,
        }
    }
    #[doc = "normal operation. LPTIM3 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgLptim3Stop::B0x0
    }
    #[doc = "stop in debug. LPTIM3 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgLptim3Stop::B0x1
    }
}
#[doc = "Field `DBG_LPTIM3_STOP` writer - LPTIM3 stop in debug"]
pub type DbgLptim3StopW<'a, REG> = crate::BitWriter<'a, REG, DbgLptim3Stop>;
impl<'a, REG> DbgLptim3StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. LPTIM3 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgLptim3Stop::B0x0)
    }
    #[doc = "stop in debug. LPTIM3 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgLptim3Stop::B0x1)
    }
}
impl R {
    #[doc = "Bit 11 - TIM1 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DbgTim1StopR {
        DbgTim1StopR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - TIM14 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DbgTim14StopR {
        DbgTim14StopR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DbgTim15StopR {
        DbgTim15StopR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DbgTim16StopR {
        DbgTim16StopR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LPTIM3 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim3_stop(&self) -> DbgLptim3StopR {
        DbgLptim3StopR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim1_stop(&mut self) -> DbgTim1StopW<DbgmcuApb2fzrSpec> {
        DbgTim1StopW::new(self, 11)
    }
    #[doc = "Bit 15 - TIM14 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim14_stop(&mut self) -> DbgTim14StopW<DbgmcuApb2fzrSpec> {
        DbgTim14StopW::new(self, 15)
    }
    #[doc = "Bit 16 - TIM15 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim15_stop(&mut self) -> DbgTim15StopW<DbgmcuApb2fzrSpec> {
        DbgTim15StopW::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim16_stop(&mut self) -> DbgTim16StopW<DbgmcuApb2fzrSpec> {
        DbgTim16StopW::new(self, 17)
    }
    #[doc = "Bit 18 - LPTIM3 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim3_stop(&mut self) -> DbgLptim3StopW<DbgmcuApb2fzrSpec> {
        DbgLptim3StopW::new(self, 18)
    }
}
#[doc = "DBG APB2 freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_apb2fzr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgmcu_apb2fzr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgmcuApb2fzrSpec;
impl crate::RegisterSpec for DbgmcuApb2fzrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_apb2fzr::R`](R) reader structure"]
impl crate::Readable for DbgmcuApb2fzrSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgmcu_apb2fzr::W`](W) writer structure"]
impl crate::Writable for DbgmcuApb2fzrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGMCU_APB2FZR to value 0"]
impl crate::Resettable for DbgmcuApb2fzrSpec {
    const RESET_VALUE: u32 = 0;
}
