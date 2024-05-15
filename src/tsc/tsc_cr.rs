#[doc = "Register `TSC_CR` reader"]
pub type R = crate::R<TscCrSpec>;
#[doc = "Register `TSC_CR` writer"]
pub type W = crate::W<TscCrSpec>;
#[doc = "Touch sensing controller enable This bit is set and cleared by software to enable/disable the touch sensing controller. Note: When the touch sensing controller is disabled, TSC registers settings have no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsce {
    #[doc = "0: Touch sensing controller disabled"]
    B0x0 = 0,
    #[doc = "1: Touch sensing controller enabled"]
    B0x1 = 1,
}
impl From<Tsce> for bool {
    #[inline(always)]
    fn from(variant: Tsce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCE` reader - Touch sensing controller enable This bit is set and cleared by software to enable/disable the touch sensing controller. Note: When the touch sensing controller is disabled, TSC registers settings have no effect."]
pub type TsceR = crate::BitReader<Tsce>;
impl TsceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsce {
        match self.bits {
            false => Tsce::B0x0,
            true => Tsce::B0x1,
        }
    }
    #[doc = "Touch sensing controller disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tsce::B0x0
    }
    #[doc = "Touch sensing controller enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tsce::B0x1
    }
}
#[doc = "Field `TSCE` writer - Touch sensing controller enable This bit is set and cleared by software to enable/disable the touch sensing controller. Note: When the touch sensing controller is disabled, TSC registers settings have no effect."]
pub type TsceW<'a, REG> = crate::BitWriter<'a, REG, Tsce>;
impl<'a, REG> TsceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Touch sensing controller disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsce::B0x0)
    }
    #[doc = "Touch sensing controller enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsce::B0x1)
    }
}
#[doc = "Start a new acquisition This bit is set by software to start a new acquisition. It is cleared by hardware as soon as the acquisition is complete or by software to cancel the ongoing acquisition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "0: Acquisition not started"]
    B0x0 = 0,
    #[doc = "1: Start a new acquisition"]
    B0x1 = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start a new acquisition This bit is set by software to start a new acquisition. It is cleared by hardware as soon as the acquisition is complete or by software to cancel the ongoing acquisition."]
pub type StartR = crate::BitReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            false => Start::B0x0,
            true => Start::B0x1,
        }
    }
    #[doc = "Acquisition not started"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Start::B0x0
    }
    #[doc = "Start a new acquisition"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Start::B0x1
    }
}
#[doc = "Field `START` writer - Start a new acquisition This bit is set by software to start a new acquisition. It is cleared by hardware as soon as the acquisition is complete or by software to cancel the ongoing acquisition."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Acquisition not started"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Start::B0x0)
    }
    #[doc = "Start a new acquisition"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Start::B0x1)
    }
}
#[doc = "Acquisition mode This bit is set and cleared by software to select the acquisition mode. Note: This bit must not be modified when an acquisition is ongoing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Am {
    #[doc = "0: Normal acquisition mode (acquisition starts as soon as START bit is set)"]
    B0x0 = 0,
    #[doc = "1: Synchronized acquisition mode (acquisition starts if START bit is set and when the selected signal is detected on the SYNC input pin)"]
    B0x1 = 1,
}
impl From<Am> for bool {
    #[inline(always)]
    fn from(variant: Am) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AM` reader - Acquisition mode This bit is set and cleared by software to select the acquisition mode. Note: This bit must not be modified when an acquisition is ongoing."]
pub type AmR = crate::BitReader<Am>;
impl AmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Am {
        match self.bits {
            false => Am::B0x0,
            true => Am::B0x1,
        }
    }
    #[doc = "Normal acquisition mode (acquisition starts as soon as START bit is set)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Am::B0x0
    }
    #[doc = "Synchronized acquisition mode (acquisition starts if START bit is set and when the selected signal is detected on the SYNC input pin)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Am::B0x1
    }
}
#[doc = "Field `AM` writer - Acquisition mode This bit is set and cleared by software to select the acquisition mode. Note: This bit must not be modified when an acquisition is ongoing."]
pub type AmW<'a, REG> = crate::BitWriter<'a, REG, Am>;
impl<'a, REG> AmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal acquisition mode (acquisition starts as soon as START bit is set)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Am::B0x0)
    }
    #[doc = "Synchronized acquisition mode (acquisition starts if START bit is set and when the selected signal is detected on the SYNC input pin)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Am::B0x1)
    }
}
#[doc = "Synchronization pin polarity This bit is set and cleared by software to select the polarity of the synchronization input pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syncpol {
    #[doc = "0: Falling edge only"]
    B0x0 = 0,
    #[doc = "1: Rising edge and high level"]
    B0x1 = 1,
}
impl From<Syncpol> for bool {
    #[inline(always)]
    fn from(variant: Syncpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCPOL` reader - Synchronization pin polarity This bit is set and cleared by software to select the polarity of the synchronization input pin."]
pub type SyncpolR = crate::BitReader<Syncpol>;
impl SyncpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syncpol {
        match self.bits {
            false => Syncpol::B0x0,
            true => Syncpol::B0x1,
        }
    }
    #[doc = "Falling edge only"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Syncpol::B0x0
    }
    #[doc = "Rising edge and high level"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Syncpol::B0x1
    }
}
#[doc = "Field `SYNCPOL` writer - Synchronization pin polarity This bit is set and cleared by software to select the polarity of the synchronization input pin."]
pub type SyncpolW<'a, REG> = crate::BitWriter<'a, REG, Syncpol>;
impl<'a, REG> SyncpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge only"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Syncpol::B0x0)
    }
    #[doc = "Rising edge and high level"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Syncpol::B0x1)
    }
}
#[doc = "I/O Default mode This bit is set and cleared by software. It defines the configuration of all the TSC I/Os when there is no ongoing acquisition. When there is an ongoing acquisition, it defines the configuration of all unused I/Os (not defined as sampling capacitor I/O or as channel I/O). Note: This bit must not be modified when an acquisition is ongoing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iodef {
    #[doc = "0: I/Os are forced to output push-pull low"]
    B0x0 = 0,
    #[doc = "1: I/Os are in input floating"]
    B0x1 = 1,
}
impl From<Iodef> for bool {
    #[inline(always)]
    fn from(variant: Iodef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IODEF` reader - I/O Default mode This bit is set and cleared by software. It defines the configuration of all the TSC I/Os when there is no ongoing acquisition. When there is an ongoing acquisition, it defines the configuration of all unused I/Os (not defined as sampling capacitor I/O or as channel I/O). Note: This bit must not be modified when an acquisition is ongoing."]
pub type IodefR = crate::BitReader<Iodef>;
impl IodefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iodef {
        match self.bits {
            false => Iodef::B0x0,
            true => Iodef::B0x1,
        }
    }
    #[doc = "I/Os are forced to output push-pull low"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Iodef::B0x0
    }
    #[doc = "I/Os are in input floating"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Iodef::B0x1
    }
}
#[doc = "Field `IODEF` writer - I/O Default mode This bit is set and cleared by software. It defines the configuration of all the TSC I/Os when there is no ongoing acquisition. When there is an ongoing acquisition, it defines the configuration of all unused I/Os (not defined as sampling capacitor I/O or as channel I/O). Note: This bit must not be modified when an acquisition is ongoing."]
pub type IodefW<'a, REG> = crate::BitWriter<'a, REG, Iodef>;
impl<'a, REG> IodefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/Os are forced to output push-pull low"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Iodef::B0x0)
    }
    #[doc = "I/Os are in input floating"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Iodef::B0x1)
    }
}
#[doc = "Max count value These bits are set and cleared by software. They define the maximum number of charge transfer pulses that can be generated before a max count error is generated. Note: These bits must not be modified when an acquisition is ongoing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mcv {
    #[doc = "0: 255"]
    B0x0 = 0,
    #[doc = "1: 511"]
    B0x1 = 1,
    #[doc = "2: 1023"]
    B0x2 = 2,
    #[doc = "3: 2047"]
    B0x3 = 3,
    #[doc = "4: 4095"]
    B0x4 = 4,
    #[doc = "5: 8191"]
    B0x5 = 5,
    #[doc = "6: 16383"]
    B0x6 = 6,
}
impl From<Mcv> for u8 {
    #[inline(always)]
    fn from(variant: Mcv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mcv {
    type Ux = u8;
}
impl crate::IsEnum for Mcv {}
#[doc = "Field `MCV` reader - Max count value These bits are set and cleared by software. They define the maximum number of charge transfer pulses that can be generated before a max count error is generated. Note: These bits must not be modified when an acquisition is ongoing."]
pub type McvR = crate::FieldReader<Mcv>;
impl McvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mcv> {
        match self.bits {
            0 => Some(Mcv::B0x0),
            1 => Some(Mcv::B0x1),
            2 => Some(Mcv::B0x2),
            3 => Some(Mcv::B0x3),
            4 => Some(Mcv::B0x4),
            5 => Some(Mcv::B0x5),
            6 => Some(Mcv::B0x6),
            _ => None,
        }
    }
    #[doc = "255"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mcv::B0x0
    }
    #[doc = "511"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mcv::B0x1
    }
    #[doc = "1023"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mcv::B0x2
    }
    #[doc = "2047"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mcv::B0x3
    }
    #[doc = "4095"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Mcv::B0x4
    }
    #[doc = "8191"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Mcv::B0x5
    }
    #[doc = "16383"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Mcv::B0x6
    }
}
#[doc = "Field `MCV` writer - Max count value These bits are set and cleared by software. They define the maximum number of charge transfer pulses that can be generated before a max count error is generated. Note: These bits must not be modified when an acquisition is ongoing."]
pub type McvW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mcv>;
impl<'a, REG> McvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "255"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mcv::B0x0)
    }
    #[doc = "511"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mcv::B0x1)
    }
    #[doc = "1023"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mcv::B0x2)
    }
    #[doc = "2047"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mcv::B0x3)
    }
    #[doc = "4095"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Mcv::B0x4)
    }
    #[doc = "8191"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Mcv::B0x5)
    }
    #[doc = "16383"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Mcv::B0x6)
    }
}
#[doc = "Pulse generator prescaler These bits are set and cleared by software.They select the AHB clock divider used to generate the pulse generator clock (PGCLK). Note: These bits must not be modified when an acquisition is ongoing. Note: Some configurations are forbidden. Refer to the Section119.4.4: Charge transfer acquisition sequence for details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pgpsc {
    #[doc = "0: f&lt;sub>HCLK&lt;/sub>"]
    B0x0 = 0,
    #[doc = "1: f&lt;sub>HCLK&lt;/sub> /2"]
    B0x1 = 1,
    #[doc = "2: f&lt;sub>HCLK&lt;/sub> /4"]
    B0x2 = 2,
    #[doc = "3: f&lt;sub>HCLK&lt;/sub> /8"]
    B0x3 = 3,
    #[doc = "4: f&lt;sub>HCLK&lt;/sub> /16"]
    B0x4 = 4,
    #[doc = "5: f&lt;sub>HCLK&lt;/sub> /32"]
    B0x5 = 5,
    #[doc = "6: f&lt;sub>HCLK&lt;/sub> /64"]
    B0x6 = 6,
    #[doc = "7: f&lt;sub>HCLK&lt;/sub> /128"]
    B0x7 = 7,
}
impl From<Pgpsc> for u8 {
    #[inline(always)]
    fn from(variant: Pgpsc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pgpsc {
    type Ux = u8;
}
impl crate::IsEnum for Pgpsc {}
#[doc = "Field `PGPSC` reader - Pulse generator prescaler These bits are set and cleared by software.They select the AHB clock divider used to generate the pulse generator clock (PGCLK). Note: These bits must not be modified when an acquisition is ongoing. Note: Some configurations are forbidden. Refer to the Section119.4.4: Charge transfer acquisition sequence for details."]
pub type PgpscR = crate::FieldReader<Pgpsc>;
impl PgpscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pgpsc {
        match self.bits {
            0 => Pgpsc::B0x0,
            1 => Pgpsc::B0x1,
            2 => Pgpsc::B0x2,
            3 => Pgpsc::B0x3,
            4 => Pgpsc::B0x4,
            5 => Pgpsc::B0x5,
            6 => Pgpsc::B0x6,
            7 => Pgpsc::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "f&lt;sub>HCLK&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pgpsc::B0x0
    }
    #[doc = "f&lt;sub>HCLK&lt;/sub> /2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pgpsc::B0x1
    }
    #[doc = "f&lt;sub>HCLK&lt;/sub> /4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pgpsc::B0x2
    }
    #[doc = "f&lt;sub>HCLK&lt;/sub> /8"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Pgpsc::B0x3
    }
    #[doc = "f&lt;sub>HCLK&lt;/sub> /16"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Pgpsc::B0x4
    }
    #[doc = "f&lt;sub>HCLK&lt;/sub> /32"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Pgpsc::B0x5
    }
    #[doc = "f&lt;sub>HCLK&lt;/sub> /64"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Pgpsc::B0x6
    }
    #[doc = "f&lt;sub>HCLK&lt;/sub> /128"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Pgpsc::B0x7
    }
}
#[doc = "Field `PGPSC` writer - Pulse generator prescaler These bits are set and cleared by software.They select the AHB clock divider used to generate the pulse generator clock (PGCLK). Note: These bits must not be modified when an acquisition is ongoing. Note: Some configurations are forbidden. Refer to the Section119.4.4: Charge transfer acquisition sequence for details."]
pub type PgpscW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pgpsc, crate::Safe>;
impl<'a, REG> PgpscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "f&lt;sub>HCLK&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pgpsc::B0x0)
    }
    #[doc = "f&lt;sub>HCLK&lt;/sub> /2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pgpsc::B0x1)
    }
    #[doc = "f&lt;sub>HCLK&lt;/sub> /4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pgpsc::B0x2)
    }
    #[doc = "f&lt;sub>HCLK&lt;/sub> /8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Pgpsc::B0x3)
    }
    #[doc = "f&lt;sub>HCLK&lt;/sub> /16"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Pgpsc::B0x4)
    }
    #[doc = "f&lt;sub>HCLK&lt;/sub> /32"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Pgpsc::B0x5)
    }
    #[doc = "f&lt;sub>HCLK&lt;/sub> /64"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Pgpsc::B0x6)
    }
    #[doc = "f&lt;sub>HCLK&lt;/sub> /128"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Pgpsc::B0x7)
    }
}
#[doc = "Spread spectrum prescaler This bit is set and cleared by software. It selects the AHB clock divider used to generate the spread spectrum clock (SSCLK). Note: This bit must not be modified when an acquisition is ongoing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sspsc {
    #[doc = "0: f&lt;sub>HCLK&lt;/sub>"]
    B0x0 = 0,
    #[doc = "1: f&lt;sub>HCLK&lt;/sub> /2"]
    B0x1 = 1,
}
impl From<Sspsc> for bool {
    #[inline(always)]
    fn from(variant: Sspsc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSPSC` reader - Spread spectrum prescaler This bit is set and cleared by software. It selects the AHB clock divider used to generate the spread spectrum clock (SSCLK). Note: This bit must not be modified when an acquisition is ongoing."]
pub type SspscR = crate::BitReader<Sspsc>;
impl SspscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sspsc {
        match self.bits {
            false => Sspsc::B0x0,
            true => Sspsc::B0x1,
        }
    }
    #[doc = "f&lt;sub>HCLK&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sspsc::B0x0
    }
    #[doc = "f&lt;sub>HCLK&lt;/sub> /2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sspsc::B0x1
    }
}
#[doc = "Field `SSPSC` writer - Spread spectrum prescaler This bit is set and cleared by software. It selects the AHB clock divider used to generate the spread spectrum clock (SSCLK). Note: This bit must not be modified when an acquisition is ongoing."]
pub type SspscW<'a, REG> = crate::BitWriter<'a, REG, Sspsc>;
impl<'a, REG> SspscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "f&lt;sub>HCLK&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sspsc::B0x0)
    }
    #[doc = "f&lt;sub>HCLK&lt;/sub> /2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sspsc::B0x1)
    }
}
#[doc = "Spread spectrum enable This bit is set and cleared by software to enable/disable the spread spectrum feature. Note: This bit must not be modified when an acquisition is ongoing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sse {
    #[doc = "0: Spread spectrum disabled"]
    B0x0 = 0,
    #[doc = "1: Spread spectrum enabled"]
    B0x1 = 1,
}
impl From<Sse> for bool {
    #[inline(always)]
    fn from(variant: Sse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSE` reader - Spread spectrum enable This bit is set and cleared by software to enable/disable the spread spectrum feature. Note: This bit must not be modified when an acquisition is ongoing."]
pub type SseR = crate::BitReader<Sse>;
impl SseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sse {
        match self.bits {
            false => Sse::B0x0,
            true => Sse::B0x1,
        }
    }
    #[doc = "Spread spectrum disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sse::B0x0
    }
    #[doc = "Spread spectrum enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sse::B0x1
    }
}
#[doc = "Field `SSE` writer - Spread spectrum enable This bit is set and cleared by software to enable/disable the spread spectrum feature. Note: This bit must not be modified when an acquisition is ongoing."]
pub type SseW<'a, REG> = crate::BitWriter<'a, REG, Sse>;
impl<'a, REG> SseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Spread spectrum disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sse::B0x0)
    }
    #[doc = "Spread spectrum enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sse::B0x1)
    }
}
#[doc = "Spread spectrum deviation These bits are set and cleared by software. They define the spread spectrum deviation which consists in adding a variable number of periods of the SSCLK clock to the charge transfer pulse high state. ... Note: These bits must not be modified when an acquisition is ongoing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ssd {
    #[doc = "0: 1x t&lt;sub>SSCLK&lt;/sub>"]
    B0x0 = 0,
    #[doc = "1: 2x t&lt;sub>SSCLK&lt;/sub>"]
    B0x1 = 1,
    #[doc = "127: 128x t&lt;sub>SSCLK&lt;/sub>"]
    B0x7f = 127,
}
impl From<Ssd> for u8 {
    #[inline(always)]
    fn from(variant: Ssd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ssd {
    type Ux = u8;
}
impl crate::IsEnum for Ssd {}
#[doc = "Field `SSD` reader - Spread spectrum deviation These bits are set and cleared by software. They define the spread spectrum deviation which consists in adding a variable number of periods of the SSCLK clock to the charge transfer pulse high state. ... Note: These bits must not be modified when an acquisition is ongoing."]
pub type SsdR = crate::FieldReader<Ssd>;
impl SsdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ssd> {
        match self.bits {
            0 => Some(Ssd::B0x0),
            1 => Some(Ssd::B0x1),
            127 => Some(Ssd::B0x7f),
            _ => None,
        }
    }
    #[doc = "1x t&lt;sub>SSCLK&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ssd::B0x0
    }
    #[doc = "2x t&lt;sub>SSCLK&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ssd::B0x1
    }
    #[doc = "128x t&lt;sub>SSCLK&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x7f(&self) -> bool {
        *self == Ssd::B0x7f
    }
}
#[doc = "Field `SSD` writer - Spread spectrum deviation These bits are set and cleared by software. They define the spread spectrum deviation which consists in adding a variable number of periods of the SSCLK clock to the charge transfer pulse high state. ... Note: These bits must not be modified when an acquisition is ongoing."]
pub type SsdW<'a, REG> = crate::FieldWriter<'a, REG, 7, Ssd>;
impl<'a, REG> SsdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1x t&lt;sub>SSCLK&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssd::B0x0)
    }
    #[doc = "2x t&lt;sub>SSCLK&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssd::B0x1)
    }
    #[doc = "128x t&lt;sub>SSCLK&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x7f(self) -> &'a mut crate::W<REG> {
        self.variant(Ssd::B0x7f)
    }
}
#[doc = "Charge transfer pulse low These bits are set and cleared by software. They define the duration of the low state of the charge transfer pulse (transfer of charge from C&lt;sub>X&lt;/sub> to C&lt;sub>S&lt;/sub>). ... Note: These bits must not be modified when an acquisition is ongoing. Note: Some configurations are forbidden. Refer to the Section119.4.4: Charge transfer acquisition sequence for details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctpl {
    #[doc = "0: 1x t&lt;sub>PGCLK&lt;/sub>"]
    B0x0 = 0,
    #[doc = "1: 2x t&lt;sub>PGCLK&lt;/sub>"]
    B0x1 = 1,
    #[doc = "15: 16x t&lt;sub>PGCLK&lt;/sub>"]
    B0xF = 15,
}
impl From<Ctpl> for u8 {
    #[inline(always)]
    fn from(variant: Ctpl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctpl {
    type Ux = u8;
}
impl crate::IsEnum for Ctpl {}
#[doc = "Field `CTPL` reader - Charge transfer pulse low These bits are set and cleared by software. They define the duration of the low state of the charge transfer pulse (transfer of charge from C&lt;sub>X&lt;/sub> to C&lt;sub>S&lt;/sub>). ... Note: These bits must not be modified when an acquisition is ongoing. Note: Some configurations are forbidden. Refer to the Section119.4.4: Charge transfer acquisition sequence for details."]
pub type CtplR = crate::FieldReader<Ctpl>;
impl CtplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctpl> {
        match self.bits {
            0 => Some(Ctpl::B0x0),
            1 => Some(Ctpl::B0x1),
            15 => Some(Ctpl::B0xF),
            _ => None,
        }
    }
    #[doc = "1x t&lt;sub>PGCLK&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ctpl::B0x0
    }
    #[doc = "2x t&lt;sub>PGCLK&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ctpl::B0x1
    }
    #[doc = "16x t&lt;sub>PGCLK&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Ctpl::B0xF
    }
}
#[doc = "Field `CTPL` writer - Charge transfer pulse low These bits are set and cleared by software. They define the duration of the low state of the charge transfer pulse (transfer of charge from C&lt;sub>X&lt;/sub> to C&lt;sub>S&lt;/sub>). ... Note: These bits must not be modified when an acquisition is ongoing. Note: Some configurations are forbidden. Refer to the Section119.4.4: Charge transfer acquisition sequence for details."]
pub type CtplW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ctpl>;
impl<'a, REG> CtplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1x t&lt;sub>PGCLK&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctpl::B0x0)
    }
    #[doc = "2x t&lt;sub>PGCLK&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctpl::B0x1)
    }
    #[doc = "16x t&lt;sub>PGCLK&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Ctpl::B0xF)
    }
}
#[doc = "Charge transfer pulse high These bits are set and cleared by software. They define the duration of the high state of the charge transfer pulse (charge of C&lt;sub>X&lt;/sub>). ... Note: These bits must not be modified when an acquisition is ongoing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctph {
    #[doc = "0: 1x t&lt;sub>PGCLK&lt;/sub>"]
    B0x0 = 0,
    #[doc = "1: 2x t&lt;sub>PGCLK&lt;/sub>"]
    B0x1 = 1,
    #[doc = "15: 16x t&lt;sub>PGCLK&lt;/sub>"]
    B0xF = 15,
}
impl From<Ctph> for u8 {
    #[inline(always)]
    fn from(variant: Ctph) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctph {
    type Ux = u8;
}
impl crate::IsEnum for Ctph {}
#[doc = "Field `CTPH` reader - Charge transfer pulse high These bits are set and cleared by software. They define the duration of the high state of the charge transfer pulse (charge of C&lt;sub>X&lt;/sub>). ... Note: These bits must not be modified when an acquisition is ongoing."]
pub type CtphR = crate::FieldReader<Ctph>;
impl CtphR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctph> {
        match self.bits {
            0 => Some(Ctph::B0x0),
            1 => Some(Ctph::B0x1),
            15 => Some(Ctph::B0xF),
            _ => None,
        }
    }
    #[doc = "1x t&lt;sub>PGCLK&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ctph::B0x0
    }
    #[doc = "2x t&lt;sub>PGCLK&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ctph::B0x1
    }
    #[doc = "16x t&lt;sub>PGCLK&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Ctph::B0xF
    }
}
#[doc = "Field `CTPH` writer - Charge transfer pulse high These bits are set and cleared by software. They define the duration of the high state of the charge transfer pulse (charge of C&lt;sub>X&lt;/sub>). ... Note: These bits must not be modified when an acquisition is ongoing."]
pub type CtphW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ctph>;
impl<'a, REG> CtphW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1x t&lt;sub>PGCLK&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctph::B0x0)
    }
    #[doc = "2x t&lt;sub>PGCLK&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctph::B0x1)
    }
    #[doc = "16x t&lt;sub>PGCLK&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Ctph::B0xF)
    }
}
impl R {
    #[doc = "Bit 0 - Touch sensing controller enable This bit is set and cleared by software to enable/disable the touch sensing controller. Note: When the touch sensing controller is disabled, TSC registers settings have no effect."]
    #[inline(always)]
    pub fn tsce(&self) -> TsceR {
        TsceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start a new acquisition This bit is set by software to start a new acquisition. It is cleared by hardware as soon as the acquisition is complete or by software to cancel the ongoing acquisition."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acquisition mode This bit is set and cleared by software to select the acquisition mode. Note: This bit must not be modified when an acquisition is ongoing."]
    #[inline(always)]
    pub fn am(&self) -> AmR {
        AmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization pin polarity This bit is set and cleared by software to select the polarity of the synchronization input pin."]
    #[inline(always)]
    pub fn syncpol(&self) -> SyncpolR {
        SyncpolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O Default mode This bit is set and cleared by software. It defines the configuration of all the TSC I/Os when there is no ongoing acquisition. When there is an ongoing acquisition, it defines the configuration of all unused I/Os (not defined as sampling capacitor I/O or as channel I/O). Note: This bit must not be modified when an acquisition is ongoing."]
    #[inline(always)]
    pub fn iodef(&self) -> IodefR {
        IodefR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Max count value These bits are set and cleared by software. They define the maximum number of charge transfer pulses that can be generated before a max count error is generated. Note: These bits must not be modified when an acquisition is ongoing."]
    #[inline(always)]
    pub fn mcv(&self) -> McvR {
        McvR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Pulse generator prescaler These bits are set and cleared by software.They select the AHB clock divider used to generate the pulse generator clock (PGCLK). Note: These bits must not be modified when an acquisition is ongoing. Note: Some configurations are forbidden. Refer to the Section119.4.4: Charge transfer acquisition sequence for details."]
    #[inline(always)]
    pub fn pgpsc(&self) -> PgpscR {
        PgpscR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Spread spectrum prescaler This bit is set and cleared by software. It selects the AHB clock divider used to generate the spread spectrum clock (SSCLK). Note: This bit must not be modified when an acquisition is ongoing."]
    #[inline(always)]
    pub fn sspsc(&self) -> SspscR {
        SspscR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Spread spectrum enable This bit is set and cleared by software to enable/disable the spread spectrum feature. Note: This bit must not be modified when an acquisition is ongoing."]
    #[inline(always)]
    pub fn sse(&self) -> SseR {
        SseR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Spread spectrum deviation These bits are set and cleared by software. They define the spread spectrum deviation which consists in adding a variable number of periods of the SSCLK clock to the charge transfer pulse high state. ... Note: These bits must not be modified when an acquisition is ongoing."]
    #[inline(always)]
    pub fn ssd(&self) -> SsdR {
        SsdR::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - Charge transfer pulse low These bits are set and cleared by software. They define the duration of the low state of the charge transfer pulse (transfer of charge from C&lt;sub>X&lt;/sub> to C&lt;sub>S&lt;/sub>). ... Note: These bits must not be modified when an acquisition is ongoing. Note: Some configurations are forbidden. Refer to the Section119.4.4: Charge transfer acquisition sequence for details."]
    #[inline(always)]
    pub fn ctpl(&self) -> CtplR {
        CtplR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Charge transfer pulse high These bits are set and cleared by software. They define the duration of the high state of the charge transfer pulse (charge of C&lt;sub>X&lt;/sub>). ... Note: These bits must not be modified when an acquisition is ongoing."]
    #[inline(always)]
    pub fn ctph(&self) -> CtphR {
        CtphR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Touch sensing controller enable This bit is set and cleared by software to enable/disable the touch sensing controller. Note: When the touch sensing controller is disabled, TSC registers settings have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tsce(&mut self) -> TsceW<TscCrSpec> {
        TsceW::new(self, 0)
    }
    #[doc = "Bit 1 - Start a new acquisition This bit is set by software to start a new acquisition. It is cleared by hardware as soon as the acquisition is complete or by software to cancel the ongoing acquisition."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<TscCrSpec> {
        StartW::new(self, 1)
    }
    #[doc = "Bit 2 - Acquisition mode This bit is set and cleared by software to select the acquisition mode. Note: This bit must not be modified when an acquisition is ongoing."]
    #[inline(always)]
    #[must_use]
    pub fn am(&mut self) -> AmW<TscCrSpec> {
        AmW::new(self, 2)
    }
    #[doc = "Bit 3 - Synchronization pin polarity This bit is set and cleared by software to select the polarity of the synchronization input pin."]
    #[inline(always)]
    #[must_use]
    pub fn syncpol(&mut self) -> SyncpolW<TscCrSpec> {
        SyncpolW::new(self, 3)
    }
    #[doc = "Bit 4 - I/O Default mode This bit is set and cleared by software. It defines the configuration of all the TSC I/Os when there is no ongoing acquisition. When there is an ongoing acquisition, it defines the configuration of all unused I/Os (not defined as sampling capacitor I/O or as channel I/O). Note: This bit must not be modified when an acquisition is ongoing."]
    #[inline(always)]
    #[must_use]
    pub fn iodef(&mut self) -> IodefW<TscCrSpec> {
        IodefW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Max count value These bits are set and cleared by software. They define the maximum number of charge transfer pulses that can be generated before a max count error is generated. Note: These bits must not be modified when an acquisition is ongoing."]
    #[inline(always)]
    #[must_use]
    pub fn mcv(&mut self) -> McvW<TscCrSpec> {
        McvW::new(self, 5)
    }
    #[doc = "Bits 12:14 - Pulse generator prescaler These bits are set and cleared by software.They select the AHB clock divider used to generate the pulse generator clock (PGCLK). Note: These bits must not be modified when an acquisition is ongoing. Note: Some configurations are forbidden. Refer to the Section119.4.4: Charge transfer acquisition sequence for details."]
    #[inline(always)]
    #[must_use]
    pub fn pgpsc(&mut self) -> PgpscW<TscCrSpec> {
        PgpscW::new(self, 12)
    }
    #[doc = "Bit 15 - Spread spectrum prescaler This bit is set and cleared by software. It selects the AHB clock divider used to generate the spread spectrum clock (SSCLK). Note: This bit must not be modified when an acquisition is ongoing."]
    #[inline(always)]
    #[must_use]
    pub fn sspsc(&mut self) -> SspscW<TscCrSpec> {
        SspscW::new(self, 15)
    }
    #[doc = "Bit 16 - Spread spectrum enable This bit is set and cleared by software to enable/disable the spread spectrum feature. Note: This bit must not be modified when an acquisition is ongoing."]
    #[inline(always)]
    #[must_use]
    pub fn sse(&mut self) -> SseW<TscCrSpec> {
        SseW::new(self, 16)
    }
    #[doc = "Bits 17:23 - Spread spectrum deviation These bits are set and cleared by software. They define the spread spectrum deviation which consists in adding a variable number of periods of the SSCLK clock to the charge transfer pulse high state. ... Note: These bits must not be modified when an acquisition is ongoing."]
    #[inline(always)]
    #[must_use]
    pub fn ssd(&mut self) -> SsdW<TscCrSpec> {
        SsdW::new(self, 17)
    }
    #[doc = "Bits 24:27 - Charge transfer pulse low These bits are set and cleared by software. They define the duration of the low state of the charge transfer pulse (transfer of charge from C&lt;sub>X&lt;/sub> to C&lt;sub>S&lt;/sub>). ... Note: These bits must not be modified when an acquisition is ongoing. Note: Some configurations are forbidden. Refer to the Section119.4.4: Charge transfer acquisition sequence for details."]
    #[inline(always)]
    #[must_use]
    pub fn ctpl(&mut self) -> CtplW<TscCrSpec> {
        CtplW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Charge transfer pulse high These bits are set and cleared by software. They define the duration of the high state of the charge transfer pulse (charge of C&lt;sub>X&lt;/sub>). ... Note: These bits must not be modified when an acquisition is ongoing."]
    #[inline(always)]
    #[must_use]
    pub fn ctph(&mut self) -> CtphW<TscCrSpec> {
        CtphW::new(self, 28)
    }
}
#[doc = "TSC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsc_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TscCrSpec;
impl crate::RegisterSpec for TscCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsc_cr::R`](R) reader structure"]
impl crate::Readable for TscCrSpec {}
#[doc = "`write(|w| ..)` method takes [`tsc_cr::W`](W) writer structure"]
impl crate::Writable for TscCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSC_CR to value 0"]
impl crate::Resettable for TscCrSpec {
    const RESET_VALUE: u32 = 0;
}
