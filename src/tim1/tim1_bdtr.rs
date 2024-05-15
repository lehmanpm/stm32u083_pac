#[doc = "Register `TIM1_BDTR` reader"]
pub type R = crate::R<Tim1BdtrSpec>;
#[doc = "Register `TIM1_BDTR` writer"]
pub type W = crate::W<Tim1BdtrSpec>;
#[doc = "Field `DTG` reader - Dead-time generator setup"]
pub type DtgR = crate::FieldReader;
#[doc = "Field `DTG` writer - Dead-time generator setup"]
pub type DtgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lock {
    #[doc = "0: LOCK OFF - No bit is write protected."]
    B0x0 = 0,
    #[doc = "1: LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
bits in TIMx_BDTR register can no longer be written."]
    B0x1 = 1,
    #[doc = "2: LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written."]
    B0x2 = 2,
    #[doc = "3: LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written."]
    B0x3 = 3,
}
impl From<Lock> for u8 {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lock {
    type Ux = u8;
}
impl crate::IsEnum for Lock {}
#[doc = "Field `LOCK` reader - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset."]
pub type LockR = crate::FieldReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            0 => Lock::B0x0,
            1 => Lock::B0x1,
            2 => Lock::B0x2,
            3 => Lock::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "LOCK OFF - No bit is write protected."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lock::B0x0
    }
    #[doc = "LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
bits in TIMx_BDTR register can no longer be written."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lock::B0x1
    }
    #[doc = "LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Lock::B0x2
    }
    #[doc = "LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Lock::B0x3
    }
}
#[doc = "Field `LOCK` writer - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset."]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lock, crate::Safe>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LOCK OFF - No bit is write protected."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::B0x0)
    }
    #[doc = "LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
bits in TIMx_BDTR register can no longer be written."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::B0x1)
    }
    #[doc = "LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::B0x2)
    }
    #[doc = "LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::B0x3)
    }
}
#[doc = "Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (Section122.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ossi {
    #[doc = "0: When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic and which imposes a Hi-Z state)."]
    B0x0 = 0,
    #[doc = "1: When inactive, OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime. The timer maintains its control over the output."]
    B0x1 = 1,
}
impl From<Ossi> for bool {
    #[inline(always)]
    fn from(variant: Ossi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSSI` reader - Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (Section122.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OssiR = crate::BitReader<Ossi>;
impl OssiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ossi {
        match self.bits {
            false => Ossi::B0x0,
            true => Ossi::B0x1,
        }
    }
    #[doc = "When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic and which imposes a Hi-Z state)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ossi::B0x0
    }
    #[doc = "When inactive, OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime. The timer maintains its control over the output."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ossi::B0x1
    }
}
#[doc = "Field `OSSI` writer - Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (Section122.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OssiW<'a, REG> = crate::BitWriter<'a, REG, Ossi>;
impl<'a, REG> OssiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic and which imposes a Hi-Z state)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ossi::B0x0)
    }
    #[doc = "When inactive, OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime. The timer maintains its control over the output."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ossi::B0x1)
    }
}
#[doc = "Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section122.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ossr {
    #[doc = "0: When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic, which forces a Hi-Z state)."]
    B0x0 = 0,
    #[doc = "1: When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer)."]
    B0x1 = 1,
}
impl From<Ossr> for bool {
    #[inline(always)]
    fn from(variant: Ossr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSSR` reader - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section122.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OssrR = crate::BitReader<Ossr>;
impl OssrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ossr {
        match self.bits {
            false => Ossr::B0x0,
            true => Ossr::B0x1,
        }
    }
    #[doc = "When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic, which forces a Hi-Z state)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ossr::B0x0
    }
    #[doc = "When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ossr::B0x1
    }
}
#[doc = "Field `OSSR` writer - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section122.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OssrW<'a, REG> = crate::BitWriter<'a, REG, Ossr>;
impl<'a, REG> OssrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic, which forces a Hi-Z state)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ossr::B0x0)
    }
    #[doc = "When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ossr::B0x1)
    }
}
#[doc = "Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BRK sources, as per Figure1152: Break and Break2 circuitry overview). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bke {
    #[doc = "0: Break function disabled"]
    B0x0 = 0,
    #[doc = "1: Break function enabled"]
    B0x1 = 1,
}
impl From<Bke> for bool {
    #[inline(always)]
    fn from(variant: Bke) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKE` reader - Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BRK sources, as per Figure1152: Break and Break2 circuitry overview). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BkeR = crate::BitReader<Bke>;
impl BkeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bke {
        match self.bits {
            false => Bke::B0x0,
            true => Bke::B0x1,
        }
    }
    #[doc = "Break function disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bke::B0x0
    }
    #[doc = "Break function enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bke::B0x1
    }
}
#[doc = "Field `BKE` writer - Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BRK sources, as per Figure1152: Break and Break2 circuitry overview). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BkeW<'a, REG> = crate::BitWriter<'a, REG, Bke>;
impl<'a, REG> BkeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break function disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bke::B0x0)
    }
    #[doc = "Break function enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bke::B0x1)
    }
}
#[doc = "Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bkp {
    #[doc = "0: Break input BRK is active low"]
    B0x0 = 0,
    #[doc = "1: Break input BRK is active high"]
    B0x1 = 1,
}
impl From<Bkp> for bool {
    #[inline(always)]
    fn from(variant: Bkp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKP` reader - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BkpR = crate::BitReader<Bkp>;
impl BkpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bkp {
        match self.bits {
            false => Bkp::B0x0,
            true => Bkp::B0x1,
        }
    }
    #[doc = "Break input BRK is active low"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bkp::B0x0
    }
    #[doc = "Break input BRK is active high"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bkp::B0x1
    }
}
#[doc = "Field `BKP` writer - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BkpW<'a, REG> = crate::BitWriter<'a, REG, Bkp>;
impl<'a, REG> BkpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input BRK is active low"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bkp::B0x0)
    }
    #[doc = "Break input BRK is active high"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bkp::B0x1)
    }
}
#[doc = "Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aoe {
    #[doc = "0: MOE can be set only by software"]
    B0x0 = 0,
    #[doc = "1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)"]
    B0x1 = 1,
}
impl From<Aoe> for bool {
    #[inline(always)]
    fn from(variant: Aoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AOE` reader - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type AoeR = crate::BitReader<Aoe>;
impl AoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aoe {
        match self.bits {
            false => Aoe::B0x0,
            true => Aoe::B0x1,
        }
    }
    #[doc = "MOE can be set only by software"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Aoe::B0x0
    }
    #[doc = "MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Aoe::B0x1
    }
}
#[doc = "Field `AOE` writer - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type AoeW<'a, REG> = crate::BitWriter<'a, REG, Aoe>;
impl<'a, REG> AoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MOE can be set only by software"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Aoe::B0x0)
    }
    #[doc = "MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Aoe::B0x1)
    }
}
#[doc = "Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (Section122.4.11: TIM1 capture/compare enable register (TIM1_CCER)).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Moe {
    #[doc = "0: In response to a break 2 event. OC and OCN outputs are disabled"]
    B0x0 = 0,
    #[doc = "1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)."]
    B0x1 = 1,
}
impl From<Moe> for bool {
    #[inline(always)]
    fn from(variant: Moe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MOE` reader - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (Section122.4.11: TIM1 capture/compare enable register (TIM1_CCER))."]
pub type MoeR = crate::BitReader<Moe>;
impl MoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Moe {
        match self.bits {
            false => Moe::B0x0,
            true => Moe::B0x1,
        }
    }
    #[doc = "In response to a break 2 event. OC and OCN outputs are disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Moe::B0x0
    }
    #[doc = "OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Moe::B0x1
    }
}
#[doc = "Field `MOE` writer - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (Section122.4.11: TIM1 capture/compare enable register (TIM1_CCER))."]
pub type MoeW<'a, REG> = crate::BitWriter<'a, REG, Moe>;
impl<'a, REG> MoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In response to a break 2 event. OC and OCN outputs are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Moe::B0x0)
    }
    #[doc = "OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Moe::B0x1)
    }
}
#[doc = "Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bkf {
    #[doc = "0: No filter, BRK acts asynchronously"]
    B0x0 = 0,
    #[doc = "1: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=2"]
    B0x1 = 1,
    #[doc = "2: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=4"]
    B0x2 = 2,
    #[doc = "3: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=8"]
    B0x3 = 3,
    #[doc = "4: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/2, N=6"]
    B0x4 = 4,
    #[doc = "5: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/2, N=8"]
    B0x5 = 5,
    #[doc = "6: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/4, N=6"]
    B0x6 = 6,
    #[doc = "7: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/4, N=8"]
    B0x7 = 7,
    #[doc = "8: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/8, N=6"]
    B0x8 = 8,
    #[doc = "9: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/8, N=8"]
    B0x9 = 9,
    #[doc = "10: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=5"]
    B0xA = 10,
    #[doc = "11: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=6"]
    B0xB = 11,
    #[doc = "12: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=8"]
    B0xC = 12,
    #[doc = "13: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=5"]
    B0xD = 13,
    #[doc = "14: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=6"]
    B0xE = 14,
    #[doc = "15: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=8"]
    B0xF = 15,
}
impl From<Bkf> for u8 {
    #[inline(always)]
    fn from(variant: Bkf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bkf {
    type Ux = u8;
}
impl crate::IsEnum for Bkf {}
#[doc = "Field `BKF` reader - Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BkfR = crate::FieldReader<Bkf>;
impl BkfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bkf {
        match self.bits {
            0 => Bkf::B0x0,
            1 => Bkf::B0x1,
            2 => Bkf::B0x2,
            3 => Bkf::B0x3,
            4 => Bkf::B0x4,
            5 => Bkf::B0x5,
            6 => Bkf::B0x6,
            7 => Bkf::B0x7,
            8 => Bkf::B0x8,
            9 => Bkf::B0x9,
            10 => Bkf::B0xA,
            11 => Bkf::B0xB,
            12 => Bkf::B0xC,
            13 => Bkf::B0xD,
            14 => Bkf::B0xE,
            15 => Bkf::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "No filter, BRK acts asynchronously"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bkf::B0x0
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bkf::B0x1
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Bkf::B0x2
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=8"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Bkf::B0x3
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/2, N=6"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Bkf::B0x4
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/2, N=8"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Bkf::B0x5
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/4, N=6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Bkf::B0x6
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/4, N=8"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Bkf::B0x7
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/8, N=6"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Bkf::B0x8
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/8, N=8"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Bkf::B0x9
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=5"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Bkf::B0xA
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=6"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Bkf::B0xB
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=8"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Bkf::B0xC
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=5"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Bkf::B0xD
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=6"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Bkf::B0xE
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=8"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Bkf::B0xF
    }
}
#[doc = "Field `BKF` writer - Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BkfW<'a, REG> = crate::FieldWriter<'a, REG, 4, Bkf, crate::Safe>;
impl<'a, REG> BkfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter, BRK acts asynchronously"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bkf::B0x0)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bkf::B0x1)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Bkf::B0x2)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Bkf::B0x3)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/2, N=6"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Bkf::B0x4)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/2, N=8"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Bkf::B0x5)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/4, N=6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Bkf::B0x6)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/4, N=8"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Bkf::B0x7)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/8, N=6"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Bkf::B0x8)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/8, N=8"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Bkf::B0x9)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=5"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Bkf::B0xA)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=6"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Bkf::B0xB)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=8"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Bkf::B0xC)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=5"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Bkf::B0xD)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=6"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Bkf::B0xE)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=8"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Bkf::B0xF)
    }
}
#[doc = "Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bk2f {
    #[doc = "0: No filter, BRK2 acts asynchronously"]
    B0x0 = 0,
    #[doc = "1: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=2"]
    B0x1 = 1,
    #[doc = "2: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=4"]
    B0x2 = 2,
    #[doc = "3: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=8"]
    B0x3 = 3,
    #[doc = "4: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/2, N=6"]
    B0x4 = 4,
    #[doc = "5: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/2, N=8"]
    B0x5 = 5,
    #[doc = "6: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/4, N=6"]
    B0x6 = 6,
    #[doc = "7: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/4, N=8"]
    B0x7 = 7,
    #[doc = "8: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/8, N=6"]
    B0x8 = 8,
    #[doc = "9: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/8, N=8"]
    B0x9 = 9,
    #[doc = "10: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=5"]
    B0xA = 10,
    #[doc = "11: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=6"]
    B0xB = 11,
    #[doc = "12: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=8"]
    B0xC = 12,
    #[doc = "13: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=5"]
    B0xD = 13,
    #[doc = "14: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=6"]
    B0xE = 14,
    #[doc = "15: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=8"]
    B0xF = 15,
}
impl From<Bk2f> for u8 {
    #[inline(always)]
    fn from(variant: Bk2f) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bk2f {
    type Ux = u8;
}
impl crate::IsEnum for Bk2f {}
#[doc = "Field `BK2F` reader - Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bk2fR = crate::FieldReader<Bk2f>;
impl Bk2fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bk2f {
        match self.bits {
            0 => Bk2f::B0x0,
            1 => Bk2f::B0x1,
            2 => Bk2f::B0x2,
            3 => Bk2f::B0x3,
            4 => Bk2f::B0x4,
            5 => Bk2f::B0x5,
            6 => Bk2f::B0x6,
            7 => Bk2f::B0x7,
            8 => Bk2f::B0x8,
            9 => Bk2f::B0x9,
            10 => Bk2f::B0xA,
            11 => Bk2f::B0xB,
            12 => Bk2f::B0xC,
            13 => Bk2f::B0xD,
            14 => Bk2f::B0xE,
            15 => Bk2f::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "No filter, BRK2 acts asynchronously"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bk2f::B0x0
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bk2f::B0x1
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Bk2f::B0x2
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=8"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Bk2f::B0x3
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/2, N=6"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Bk2f::B0x4
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/2, N=8"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Bk2f::B0x5
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/4, N=6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Bk2f::B0x6
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/4, N=8"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Bk2f::B0x7
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/8, N=6"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Bk2f::B0x8
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/8, N=8"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Bk2f::B0x9
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=5"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Bk2f::B0xA
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=6"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Bk2f::B0xB
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=8"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Bk2f::B0xC
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=5"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Bk2f::B0xD
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=6"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Bk2f::B0xE
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=8"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Bk2f::B0xF
    }
}
#[doc = "Field `BK2F` writer - Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type Bk2fW<'a, REG> = crate::FieldWriter<'a, REG, 4, Bk2f, crate::Safe>;
impl<'a, REG> Bk2fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter, BRK2 acts asynchronously"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2f::B0x0)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2f::B0x1)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2f::B0x2)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2f::B0x3)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/2, N=6"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2f::B0x4)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/2, N=8"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2f::B0x5)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/4, N=6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2f::B0x6)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/4, N=8"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2f::B0x7)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/8, N=6"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2f::B0x8)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/8, N=8"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2f::B0x9)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=5"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2f::B0xA)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=6"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2f::B0xB)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=8"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2f::B0xC)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=5"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2f::B0xD)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=6"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2f::B0xE)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=8"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2f::B0xF)
    }
}
#[doc = "Break 2 enable Note: The BRK2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bk2e {
    #[doc = "0: Break input BRK2 disabled"]
    B0x0 = 0,
    #[doc = "1: Break input BRK2 enabled"]
    B0x1 = 1,
}
impl From<Bk2e> for bool {
    #[inline(always)]
    fn from(variant: Bk2e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2E` reader - Break 2 enable Note: The BRK2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type Bk2eR = crate::BitReader<Bk2e>;
impl Bk2eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bk2e {
        match self.bits {
            false => Bk2e::B0x0,
            true => Bk2e::B0x1,
        }
    }
    #[doc = "Break input BRK2 disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bk2e::B0x0
    }
    #[doc = "Break input BRK2 enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bk2e::B0x1
    }
}
#[doc = "Field `BK2E` writer - Break 2 enable Note: The BRK2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type Bk2eW<'a, REG> = crate::BitWriter<'a, REG, Bk2e>;
impl<'a, REG> Bk2eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input BRK2 disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2e::B0x0)
    }
    #[doc = "Break input BRK2 enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2e::B0x1)
    }
}
#[doc = "Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bk2p {
    #[doc = "0: Break input BRK2 is active low"]
    B0x0 = 0,
    #[doc = "1: Break input BRK2 is active high"]
    B0x1 = 1,
}
impl From<Bk2p> for bool {
    #[inline(always)]
    fn from(variant: Bk2p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2P` reader - Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type Bk2pR = crate::BitReader<Bk2p>;
impl Bk2pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bk2p {
        match self.bits {
            false => Bk2p::B0x0,
            true => Bk2p::B0x1,
        }
    }
    #[doc = "Break input BRK2 is active low"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bk2p::B0x0
    }
    #[doc = "Break input BRK2 is active high"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bk2p::B0x1
    }
}
#[doc = "Field `BK2P` writer - Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type Bk2pW<'a, REG> = crate::BitWriter<'a, REG, Bk2p>;
impl<'a, REG> Bk2pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input BRK2 is active low"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2p::B0x0)
    }
    #[doc = "Break input BRK2 is active high"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bk2p::B0x1)
    }
}
#[doc = "Break Disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bkdsrm {
    #[doc = "0: Break input BRK is armed"]
    B0x0 = 0,
    #[doc = "1: Break input BRK is disarmed"]
    B0x1 = 1,
}
impl From<Bkdsrm> for bool {
    #[inline(always)]
    fn from(variant: Bkdsrm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKDSRM` reader - Break Disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BkdsrmR = crate::BitReader<Bkdsrm>;
impl BkdsrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bkdsrm {
        match self.bits {
            false => Bkdsrm::B0x0,
            true => Bkdsrm::B0x1,
        }
    }
    #[doc = "Break input BRK is armed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bkdsrm::B0x0
    }
    #[doc = "Break input BRK is disarmed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bkdsrm::B0x1
    }
}
#[doc = "Field `BKDSRM` writer - Break Disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BkdsrmW<'a, REG> = crate::BitWriter<'a, REG, Bkdsrm>;
impl<'a, REG> BkdsrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input BRK is armed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bkdsrm::B0x0)
    }
    #[doc = "Break input BRK is disarmed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bkdsrm::B0x1)
    }
}
#[doc = "Field `BK2DSRM` reader - Break2 Disarm Refer to BKDSRM description"]
pub type Bk2dsrmR = crate::BitReader;
#[doc = "Field `BK2DSRM` writer - Break2 Disarm Refer to BKDSRM description"]
pub type Bk2dsrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Break Bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bkbid {
    #[doc = "0: Break input BRK in input mode"]
    B0x0 = 0,
    #[doc = "1: Break input BRK in bidirectional mode"]
    B0x1 = 1,
}
impl From<Bkbid> for bool {
    #[inline(always)]
    fn from(variant: Bkbid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKBID` reader - Break Bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BkbidR = crate::BitReader<Bkbid>;
impl BkbidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bkbid {
        match self.bits {
            false => Bkbid::B0x0,
            true => Bkbid::B0x1,
        }
    }
    #[doc = "Break input BRK in input mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bkbid::B0x0
    }
    #[doc = "Break input BRK in bidirectional mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bkbid::B0x1
    }
}
#[doc = "Field `BKBID` writer - Break Bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BkbidW<'a, REG> = crate::BitWriter<'a, REG, Bkbid>;
impl<'a, REG> BkbidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input BRK in input mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bkbid::B0x0)
    }
    #[doc = "Break input BRK in bidirectional mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bkbid::B0x1)
    }
}
#[doc = "Field `BK2BID` reader - Break2 bidirectional Refer to BKBID description"]
pub type Bk2bidR = crate::BitReader;
#[doc = "Field `BK2BID` writer - Break2 bidirectional Refer to BKBID description"]
pub type Bk2bidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    pub fn dtg(&self) -> DtgR {
        DtgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (Section122.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ossi(&self) -> OssiR {
        OssiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section122.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ossr(&self) -> OssrR {
        OssrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BRK sources, as per Figure1152: Break and Break2 circuitry overview). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bke(&self) -> BkeR {
        BkeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn aoe(&self) -> AoeR {
        AoeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (Section122.4.11: TIM1 capture/compare enable register (TIM1_CCER))."]
    #[inline(always)]
    pub fn moe(&self) -> MoeR {
        MoeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkf(&self) -> BkfR {
        BkfR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2f(&self) -> Bk2fR {
        Bk2fR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Break 2 enable Note: The BRK2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bk2e(&self) -> Bk2eR {
        Bk2eR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bk2p(&self) -> Bk2pR {
        Bk2pR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Break Disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkdsrm(&self) -> BkdsrmR {
        BkdsrmR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Break2 Disarm Refer to BKDSRM description"]
    #[inline(always)]
    pub fn bk2dsrm(&self) -> Bk2dsrmR {
        Bk2dsrmR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Break Bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkbid(&self) -> BkbidR {
        BkbidR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Break2 bidirectional Refer to BKBID description"]
    #[inline(always)]
    pub fn bk2bid(&self) -> Bk2bidR {
        Bk2bidR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    #[must_use]
    pub fn dtg(&mut self) -> DtgW<Tim1BdtrSpec> {
        DtgW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<Tim1BdtrSpec> {
        LockW::new(self, 8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (Section122.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ossi(&mut self) -> OssiW<Tim1BdtrSpec> {
        OssiW::new(self, 10)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section122.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ossr(&mut self) -> OssrW<Tim1BdtrSpec> {
        OssrW::new(self, 11)
    }
    #[doc = "Bit 12 - Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BRK sources, as per Figure1152: Break and Break2 circuitry overview). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    #[must_use]
    pub fn bke(&mut self) -> BkeW<Tim1BdtrSpec> {
        BkeW::new(self, 12)
    }
    #[doc = "Bit 13 - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BkpW<Tim1BdtrSpec> {
        BkpW::new(self, 13)
    }
    #[doc = "Bit 14 - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn aoe(&mut self) -> AoeW<Tim1BdtrSpec> {
        AoeW::new(self, 14)
    }
    #[doc = "Bit 15 - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (Section122.4.11: TIM1 capture/compare enable register (TIM1_CCER))."]
    #[inline(always)]
    #[must_use]
    pub fn moe(&mut self) -> MoeW<Tim1BdtrSpec> {
        MoeW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkf(&mut self) -> BkfW<Tim1BdtrSpec> {
        BkfW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bk2f(&mut self) -> Bk2fW<Tim1BdtrSpec> {
        Bk2fW::new(self, 20)
    }
    #[doc = "Bit 24 - Break 2 enable Note: The BRK2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    #[must_use]
    pub fn bk2e(&mut self) -> Bk2eW<Tim1BdtrSpec> {
        Bk2eW::new(self, 24)
    }
    #[doc = "Bit 25 - Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    #[must_use]
    pub fn bk2p(&mut self) -> Bk2pW<Tim1BdtrSpec> {
        Bk2pW::new(self, 25)
    }
    #[doc = "Bit 26 - Break Disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    #[must_use]
    pub fn bkdsrm(&mut self) -> BkdsrmW<Tim1BdtrSpec> {
        BkdsrmW::new(self, 26)
    }
    #[doc = "Bit 27 - Break2 Disarm Refer to BKDSRM description"]
    #[inline(always)]
    #[must_use]
    pub fn bk2dsrm(&mut self) -> Bk2dsrmW<Tim1BdtrSpec> {
        Bk2dsrmW::new(self, 27)
    }
    #[doc = "Bit 28 - Break Bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    #[must_use]
    pub fn bkbid(&mut self) -> BkbidW<Tim1BdtrSpec> {
        BkbidW::new(self, 28)
    }
    #[doc = "Bit 29 - Break2 bidirectional Refer to BKBID description"]
    #[inline(always)]
    #[must_use]
    pub fn bk2bid(&mut self) -> Bk2bidW<Tim1BdtrSpec> {
        Bk2bidW::new(self, 29)
    }
}
#[doc = "TIM1 break and dead-time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_bdtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_bdtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1BdtrSpec;
impl crate::RegisterSpec for Tim1BdtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_bdtr::R`](R) reader structure"]
impl crate::Readable for Tim1BdtrSpec {}
#[doc = "`write(|w| ..)` method takes [`tim1_bdtr::W`](W) writer structure"]
impl crate::Writable for Tim1BdtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_BDTR to value 0"]
impl crate::Resettable for Tim1BdtrSpec {
    const RESET_VALUE: u32 = 0;
}
