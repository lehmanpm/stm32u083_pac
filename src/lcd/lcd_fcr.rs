#[doc = "Register `LCD_FCR` reader"]
pub type R = crate::R<LcdFcrSpec>;
#[doc = "Register `LCD_FCR` writer"]
pub type W = crate::W<LcdFcrSpec>;
#[doc = "High drive enable This bit is written by software to enable a low resistance divider. Displays with high internal resistance may need a longer drive time to achieve satisfactory contrast. This bit is useful in this case if some additional power consumption can be tolerated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hd {
    #[doc = "0: Permanent high drive disabled"]
    B0x0 = 0,
    #[doc = "1: Permanent high drive enabled. When HD = 1, PON\\[2:0\\]
must be programmed to 001."]
    B0x1 = 1,
}
impl From<Hd> for bool {
    #[inline(always)]
    fn from(variant: Hd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HD` reader - High drive enable This bit is written by software to enable a low resistance divider. Displays with high internal resistance may need a longer drive time to achieve satisfactory contrast. This bit is useful in this case if some additional power consumption can be tolerated."]
pub type HdR = crate::BitReader<Hd>;
impl HdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hd {
        match self.bits {
            false => Hd::B0x0,
            true => Hd::B0x1,
        }
    }
    #[doc = "Permanent high drive disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hd::B0x0
    }
    #[doc = "Permanent high drive enabled. When HD = 1, PON\\[2:0\\]
must be programmed to 001."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hd::B0x1
    }
}
#[doc = "Field `HD` writer - High drive enable This bit is written by software to enable a low resistance divider. Displays with high internal resistance may need a longer drive time to achieve satisfactory contrast. This bit is useful in this case if some additional power consumption can be tolerated."]
pub type HdW<'a, REG> = crate::BitWriter<'a, REG, Hd>;
impl<'a, REG> HdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Permanent high drive disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hd::B0x0)
    }
    #[doc = "Permanent high drive enabled. When HD = 1, PON\\[2:0\\]
must be programmed to 001."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hd::B0x1)
    }
}
#[doc = "Start of frame interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sofie {
    #[doc = "0: LCD start-of-frame interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: LCD start-of-frame interrupt enabled"]
    B0x1 = 1,
}
impl From<Sofie> for bool {
    #[inline(always)]
    fn from(variant: Sofie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFIE` reader - Start of frame interrupt enable This bit is set and cleared by software."]
pub type SofieR = crate::BitReader<Sofie>;
impl SofieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sofie {
        match self.bits {
            false => Sofie::B0x0,
            true => Sofie::B0x1,
        }
    }
    #[doc = "LCD start-of-frame interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sofie::B0x0
    }
    #[doc = "LCD start-of-frame interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sofie::B0x1
    }
}
#[doc = "Field `SOFIE` writer - Start of frame interrupt enable This bit is set and cleared by software."]
pub type SofieW<'a, REG> = crate::BitWriter<'a, REG, Sofie>;
impl<'a, REG> SofieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD start-of-frame interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sofie::B0x0)
    }
    #[doc = "LCD start-of-frame interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sofie::B0x1)
    }
}
#[doc = "Update display done interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uddie {
    #[doc = "0: LCD update display done interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: LCD update display done interrupt enabled"]
    B0x1 = 1,
}
impl From<Uddie> for bool {
    #[inline(always)]
    fn from(variant: Uddie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDDIE` reader - Update display done interrupt enable This bit is set and cleared by software."]
pub type UddieR = crate::BitReader<Uddie>;
impl UddieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uddie {
        match self.bits {
            false => Uddie::B0x0,
            true => Uddie::B0x1,
        }
    }
    #[doc = "LCD update display done interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Uddie::B0x0
    }
    #[doc = "LCD update display done interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Uddie::B0x1
    }
}
#[doc = "Field `UDDIE` writer - Update display done interrupt enable This bit is set and cleared by software."]
pub type UddieW<'a, REG> = crate::BitWriter<'a, REG, Uddie>;
impl<'a, REG> UddieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD update display done interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Uddie::B0x0)
    }
    #[doc = "LCD update display done interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Uddie::B0x1)
    }
}
#[doc = "Pulse ON duration These bits are written by software to define the pulse duration in terms of ck_ps pulses. A1short pulse leads to lower power consumption, but displays with high internal resistance may need a longer pulse to achieve satisfactory contrast. Note that the pulse is never longer than one half prescaled LCD clock period. PON duration example with LCDCLK = 32.7681kHz and PS=0x03:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pon {
    #[doc = "0: 0 1s"]
    B0x0 = 0,
    #[doc = "1: 244 1s"]
    B0x1 = 1,
    #[doc = "2: 488 1s"]
    B0x2 = 2,
    #[doc = "3: 782 1s"]
    B0x3 = 3,
    #[doc = "4: 976 1s"]
    B0x4 = 4,
    #[doc = "5: 1.22 ms"]
    B0x5 = 5,
    #[doc = "6: 1.46 ms"]
    B0x6 = 6,
    #[doc = "7: 1.71 ms"]
    B0x7 = 7,
}
impl From<Pon> for u8 {
    #[inline(always)]
    fn from(variant: Pon) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pon {
    type Ux = u8;
}
impl crate::IsEnum for Pon {}
#[doc = "Field `PON` reader - Pulse ON duration These bits are written by software to define the pulse duration in terms of ck_ps pulses. A1short pulse leads to lower power consumption, but displays with high internal resistance may need a longer pulse to achieve satisfactory contrast. Note that the pulse is never longer than one half prescaled LCD clock period. PON duration example with LCDCLK = 32.7681kHz and PS=0x03:"]
pub type PonR = crate::FieldReader<Pon>;
impl PonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pon {
        match self.bits {
            0 => Pon::B0x0,
            1 => Pon::B0x1,
            2 => Pon::B0x2,
            3 => Pon::B0x3,
            4 => Pon::B0x4,
            5 => Pon::B0x5,
            6 => Pon::B0x6,
            7 => Pon::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "0 1s"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pon::B0x0
    }
    #[doc = "244 1s"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pon::B0x1
    }
    #[doc = "488 1s"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pon::B0x2
    }
    #[doc = "782 1s"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Pon::B0x3
    }
    #[doc = "976 1s"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Pon::B0x4
    }
    #[doc = "1.22 ms"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Pon::B0x5
    }
    #[doc = "1.46 ms"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Pon::B0x6
    }
    #[doc = "1.71 ms"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Pon::B0x7
    }
}
#[doc = "Field `PON` writer - Pulse ON duration These bits are written by software to define the pulse duration in terms of ck_ps pulses. A1short pulse leads to lower power consumption, but displays with high internal resistance may need a longer pulse to achieve satisfactory contrast. Note that the pulse is never longer than one half prescaled LCD clock period. PON duration example with LCDCLK = 32.7681kHz and PS=0x03:"]
pub type PonW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pon, crate::Safe>;
impl<'a, REG> PonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 1s"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pon::B0x0)
    }
    #[doc = "244 1s"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pon::B0x1)
    }
    #[doc = "488 1s"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pon::B0x2)
    }
    #[doc = "782 1s"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Pon::B0x3)
    }
    #[doc = "976 1s"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Pon::B0x4)
    }
    #[doc = "1.22 ms"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Pon::B0x5)
    }
    #[doc = "1.46 ms"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Pon::B0x6)
    }
    #[doc = "1.71 ms"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Pon::B0x7)
    }
}
#[doc = "Dead time duration These bits are written by software to configure the length of the dead time between frames. During the dead time the COM and SEG voltage levels are held at 0 V to reduce the contrast without modifying the frame rate. ......\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dead {
    #[doc = "0: No dead time"]
    B0x0 = 0,
    #[doc = "1: 1 phase period dead time"]
    B0x1 = 1,
    #[doc = "2: 2 phase period dead time"]
    B0x2 = 2,
    #[doc = "7: 7 phase period dead time"]
    B0x7 = 7,
}
impl From<Dead> for u8 {
    #[inline(always)]
    fn from(variant: Dead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dead {
    type Ux = u8;
}
impl crate::IsEnum for Dead {}
#[doc = "Field `DEAD` reader - Dead time duration These bits are written by software to configure the length of the dead time between frames. During the dead time the COM and SEG voltage levels are held at 0 V to reduce the contrast without modifying the frame rate. ......"]
pub type DeadR = crate::FieldReader<Dead>;
impl DeadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dead> {
        match self.bits {
            0 => Some(Dead::B0x0),
            1 => Some(Dead::B0x1),
            2 => Some(Dead::B0x2),
            7 => Some(Dead::B0x7),
            _ => None,
        }
    }
    #[doc = "No dead time"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dead::B0x0
    }
    #[doc = "1 phase period dead time"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dead::B0x1
    }
    #[doc = "2 phase period dead time"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Dead::B0x2
    }
    #[doc = "7 phase period dead time"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Dead::B0x7
    }
}
#[doc = "Field `DEAD` writer - Dead time duration These bits are written by software to configure the length of the dead time between frames. During the dead time the COM and SEG voltage levels are held at 0 V to reduce the contrast without modifying the frame rate. ......"]
pub type DeadW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dead>;
impl<'a, REG> DeadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No dead time"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dead::B0x0)
    }
    #[doc = "1 phase period dead time"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dead::B0x1)
    }
    #[doc = "2 phase period dead time"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Dead::B0x2)
    }
    #[doc = "7 phase period dead time"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Dead::B0x7)
    }
}
#[doc = "Contrast control These bits specify one of the V&lt;sub>LCD &lt;/sub>maximum voltages (independent of V&lt;sub>DD&lt;/sub>). It ranges from12.60 V to 3.51V. Note: Refer to the datasheet for the V&lt;sub>LCDx&lt;/sub> values.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc {
    #[doc = "0: V&lt;sub>LCD0&lt;/sub>"]
    B0x0 = 0,
    #[doc = "1: V&lt;sub>LCD1&lt;/sub>"]
    B0x1 = 1,
    #[doc = "2: V&lt;sub>LCD2&lt;/sub>"]
    B0x2 = 2,
    #[doc = "3: V&lt;sub>LCD3&lt;/sub>"]
    B0x3 = 3,
    #[doc = "4: V&lt;sub>LCD4&lt;/sub>"]
    B0x4 = 4,
    #[doc = "5: V&lt;sub>LCD5&lt;/sub>"]
    B0x5 = 5,
    #[doc = "6: V&lt;sub>LCD6&lt;/sub>"]
    B0x6 = 6,
    #[doc = "7: V&lt;sub>LCD7&lt;/sub>"]
    B0x7 = 7,
}
impl From<Cc> for u8 {
    #[inline(always)]
    fn from(variant: Cc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc {
    type Ux = u8;
}
impl crate::IsEnum for Cc {}
#[doc = "Field `CC` reader - Contrast control These bits specify one of the V&lt;sub>LCD &lt;/sub>maximum voltages (independent of V&lt;sub>DD&lt;/sub>). It ranges from12.60 V to 3.51V. Note: Refer to the datasheet for the V&lt;sub>LCDx&lt;/sub> values."]
pub type CcR = crate::FieldReader<Cc>;
impl CcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc {
        match self.bits {
            0 => Cc::B0x0,
            1 => Cc::B0x1,
            2 => Cc::B0x2,
            3 => Cc::B0x3,
            4 => Cc::B0x4,
            5 => Cc::B0x5,
            6 => Cc::B0x6,
            7 => Cc::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "V&lt;sub>LCD0&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc::B0x0
    }
    #[doc = "V&lt;sub>LCD1&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc::B0x1
    }
    #[doc = "V&lt;sub>LCD2&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Cc::B0x2
    }
    #[doc = "V&lt;sub>LCD3&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Cc::B0x3
    }
    #[doc = "V&lt;sub>LCD4&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Cc::B0x4
    }
    #[doc = "V&lt;sub>LCD5&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Cc::B0x5
    }
    #[doc = "V&lt;sub>LCD6&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Cc::B0x6
    }
    #[doc = "V&lt;sub>LCD7&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Cc::B0x7
    }
}
#[doc = "Field `CC` writer - Contrast control These bits specify one of the V&lt;sub>LCD &lt;/sub>maximum voltages (independent of V&lt;sub>DD&lt;/sub>). It ranges from12.60 V to 3.51V. Note: Refer to the datasheet for the V&lt;sub>LCDx&lt;/sub> values."]
pub type CcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cc, crate::Safe>;
impl<'a, REG> CcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "V&lt;sub>LCD0&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::B0x0)
    }
    #[doc = "V&lt;sub>LCD1&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::B0x1)
    }
    #[doc = "V&lt;sub>LCD2&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::B0x2)
    }
    #[doc = "V&lt;sub>LCD3&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::B0x3)
    }
    #[doc = "V&lt;sub>LCD4&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::B0x4)
    }
    #[doc = "V&lt;sub>LCD5&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::B0x5)
    }
    #[doc = "V&lt;sub>LCD6&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::B0x6)
    }
    #[doc = "V&lt;sub>LCD7&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::B0x7)
    }
}
#[doc = "Blink frequency selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Blinkf {
    #[doc = "0: f&lt;sub>LCD&lt;/sub>/8"]
    B0x0 = 0,
    #[doc = "1: f&lt;sub>LCD&lt;/sub>/16"]
    B0x1 = 1,
    #[doc = "2: f&lt;sub>LCD&lt;/sub>/32"]
    B0x2 = 2,
    #[doc = "3: f&lt;sub>LCD&lt;/sub>/64"]
    B0x3 = 3,
    #[doc = "4: f&lt;sub>LCD&lt;/sub>/128"]
    B0x4 = 4,
    #[doc = "5: f&lt;sub>LCD&lt;/sub>/256"]
    B0x5 = 5,
    #[doc = "6: f&lt;sub>LCD&lt;/sub>/512"]
    B0x6 = 6,
    #[doc = "7: f&lt;sub>LCD&lt;/sub>/1024"]
    B0x7 = 7,
}
impl From<Blinkf> for u8 {
    #[inline(always)]
    fn from(variant: Blinkf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Blinkf {
    type Ux = u8;
}
impl crate::IsEnum for Blinkf {}
#[doc = "Field `BLINKF` reader - Blink frequency selection"]
pub type BlinkfR = crate::FieldReader<Blinkf>;
impl BlinkfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blinkf {
        match self.bits {
            0 => Blinkf::B0x0,
            1 => Blinkf::B0x1,
            2 => Blinkf::B0x2,
            3 => Blinkf::B0x3,
            4 => Blinkf::B0x4,
            5 => Blinkf::B0x5,
            6 => Blinkf::B0x6,
            7 => Blinkf::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "f&lt;sub>LCD&lt;/sub>/8"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Blinkf::B0x0
    }
    #[doc = "f&lt;sub>LCD&lt;/sub>/16"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Blinkf::B0x1
    }
    #[doc = "f&lt;sub>LCD&lt;/sub>/32"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Blinkf::B0x2
    }
    #[doc = "f&lt;sub>LCD&lt;/sub>/64"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Blinkf::B0x3
    }
    #[doc = "f&lt;sub>LCD&lt;/sub>/128"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Blinkf::B0x4
    }
    #[doc = "f&lt;sub>LCD&lt;/sub>/256"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Blinkf::B0x5
    }
    #[doc = "f&lt;sub>LCD&lt;/sub>/512"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Blinkf::B0x6
    }
    #[doc = "f&lt;sub>LCD&lt;/sub>/1024"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Blinkf::B0x7
    }
}
#[doc = "Field `BLINKF` writer - Blink frequency selection"]
pub type BlinkfW<'a, REG> = crate::FieldWriter<'a, REG, 3, Blinkf, crate::Safe>;
impl<'a, REG> BlinkfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "f&lt;sub>LCD&lt;/sub>/8"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Blinkf::B0x0)
    }
    #[doc = "f&lt;sub>LCD&lt;/sub>/16"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Blinkf::B0x1)
    }
    #[doc = "f&lt;sub>LCD&lt;/sub>/32"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Blinkf::B0x2)
    }
    #[doc = "f&lt;sub>LCD&lt;/sub>/64"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Blinkf::B0x3)
    }
    #[doc = "f&lt;sub>LCD&lt;/sub>/128"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Blinkf::B0x4)
    }
    #[doc = "f&lt;sub>LCD&lt;/sub>/256"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Blinkf::B0x5)
    }
    #[doc = "f&lt;sub>LCD&lt;/sub>/512"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Blinkf::B0x6)
    }
    #[doc = "f&lt;sub>LCD&lt;/sub>/1024"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Blinkf::B0x7)
    }
}
#[doc = "Blink mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Blink {
    #[doc = "0: Blink disabled"]
    B0x0 = 0,
    #[doc = "1: Blink enabled on SEG\\[0\\], COM\\[0\\]
(1 pixel)"]
    B0x1 = 1,
    #[doc = "2: Blink enabled on SEG\\[0\\], all COMs (up to 8 pixels depending on the programmed duty)"]
    B0x2 = 2,
    #[doc = "3: Blink enabled on all SEGs and all COMs (all pixels)"]
    B0x3 = 3,
}
impl From<Blink> for u8 {
    #[inline(always)]
    fn from(variant: Blink) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Blink {
    type Ux = u8;
}
impl crate::IsEnum for Blink {}
#[doc = "Field `BLINK` reader - Blink mode selection"]
pub type BlinkR = crate::FieldReader<Blink>;
impl BlinkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blink {
        match self.bits {
            0 => Blink::B0x0,
            1 => Blink::B0x1,
            2 => Blink::B0x2,
            3 => Blink::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Blink disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Blink::B0x0
    }
    #[doc = "Blink enabled on SEG\\[0\\], COM\\[0\\]
(1 pixel)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Blink::B0x1
    }
    #[doc = "Blink enabled on SEG\\[0\\], all COMs (up to 8 pixels depending on the programmed duty)"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Blink::B0x2
    }
    #[doc = "Blink enabled on all SEGs and all COMs (all pixels)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Blink::B0x3
    }
}
#[doc = "Field `BLINK` writer - Blink mode selection"]
pub type BlinkW<'a, REG> = crate::FieldWriter<'a, REG, 2, Blink, crate::Safe>;
impl<'a, REG> BlinkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Blink disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Blink::B0x0)
    }
    #[doc = "Blink enabled on SEG\\[0\\], COM\\[0\\]
(1 pixel)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Blink::B0x1)
    }
    #[doc = "Blink enabled on SEG\\[0\\], all COMs (up to 8 pixels depending on the programmed duty)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Blink::B0x2)
    }
    #[doc = "Blink enabled on all SEGs and all COMs (all pixels)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Blink::B0x3)
    }
}
#[doc = "DIV clock divider These bits are written by software to define the division factor of the DIV divider (see1Section118.3.2.) ...\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Div {
    #[doc = "0: ck_div = ck_ps/16"]
    B0x0 = 0,
    #[doc = "1: ck_div = ck_ps/17"]
    B0x1 = 1,
    #[doc = "15: ck_div = ck_ps/31"]
    B0xF = 15,
}
impl From<Div> for u8 {
    #[inline(always)]
    fn from(variant: Div) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Div {
    type Ux = u8;
}
impl crate::IsEnum for Div {}
#[doc = "Field `DIV` reader - DIV clock divider These bits are written by software to define the division factor of the DIV divider (see1Section118.3.2.) ..."]
pub type DivR = crate::FieldReader<Div>;
impl DivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Div> {
        match self.bits {
            0 => Some(Div::B0x0),
            1 => Some(Div::B0x1),
            15 => Some(Div::B0xF),
            _ => None,
        }
    }
    #[doc = "ck_div = ck_ps/16"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Div::B0x0
    }
    #[doc = "ck_div = ck_ps/17"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Div::B0x1
    }
    #[doc = "ck_div = ck_ps/31"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Div::B0xF
    }
}
#[doc = "Field `DIV` writer - DIV clock divider These bits are written by software to define the division factor of the DIV divider (see1Section118.3.2.) ..."]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 4, Div>;
impl<'a, REG> DivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ck_div = ck_ps/16"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Div::B0x0)
    }
    #[doc = "ck_div = ck_ps/17"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Div::B0x1)
    }
    #[doc = "ck_div = ck_ps/31"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Div::B0xF)
    }
}
#[doc = "PS 16-bit prescaler These bits are written by software to define the division factor of the PS 16-bit prescaler. ck_ps = LCDCLK/(2&lt;sup>PS\\[3:0\\]&lt;/sup>). See&lt;sub> &lt;/sub>Section118.3.2. ...\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ps {
    #[doc = "0: ck_ps = LCDCLK"]
    B0x0 = 0,
    #[doc = "1: ck_ps = LCDCLK/2"]
    B0x1 = 1,
    #[doc = "15: ck_ps = LCDCLK/32768"]
    B0xF = 15,
}
impl From<Ps> for u8 {
    #[inline(always)]
    fn from(variant: Ps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ps {
    type Ux = u8;
}
impl crate::IsEnum for Ps {}
#[doc = "Field `PS` reader - PS 16-bit prescaler These bits are written by software to define the division factor of the PS 16-bit prescaler. ck_ps = LCDCLK/(2&lt;sup>PS\\[3:0\\]&lt;/sup>). See&lt;sub> &lt;/sub>Section118.3.2. ..."]
pub type PsR = crate::FieldReader<Ps>;
impl PsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ps> {
        match self.bits {
            0 => Some(Ps::B0x0),
            1 => Some(Ps::B0x1),
            15 => Some(Ps::B0xF),
            _ => None,
        }
    }
    #[doc = "ck_ps = LCDCLK"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ps::B0x0
    }
    #[doc = "ck_ps = LCDCLK/2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ps::B0x1
    }
    #[doc = "ck_ps = LCDCLK/32768"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Ps::B0xF
    }
}
#[doc = "Field `PS` writer - PS 16-bit prescaler These bits are written by software to define the division factor of the PS 16-bit prescaler. ck_ps = LCDCLK/(2&lt;sup>PS\\[3:0\\]&lt;/sup>). See&lt;sub> &lt;/sub>Section118.3.2. ..."]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ps>;
impl<'a, REG> PsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ck_ps = LCDCLK"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::B0x0)
    }
    #[doc = "ck_ps = LCDCLK/2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::B0x1)
    }
    #[doc = "ck_ps = LCDCLK/32768"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::B0xF)
    }
}
impl R {
    #[doc = "Bit 0 - High drive enable This bit is written by software to enable a low resistance divider. Displays with high internal resistance may need a longer drive time to achieve satisfactory contrast. This bit is useful in this case if some additional power consumption can be tolerated."]
    #[inline(always)]
    pub fn hd(&self) -> HdR {
        HdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of frame interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn sofie(&self) -> SofieR {
        SofieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Update display done interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn uddie(&self) -> UddieR {
        UddieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Pulse ON duration These bits are written by software to define the pulse duration in terms of ck_ps pulses. A1short pulse leads to lower power consumption, but displays with high internal resistance may need a longer pulse to achieve satisfactory contrast. Note that the pulse is never longer than one half prescaled LCD clock period. PON duration example with LCDCLK = 32.7681kHz and PS=0x03:"]
    #[inline(always)]
    pub fn pon(&self) -> PonR {
        PonR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Dead time duration These bits are written by software to configure the length of the dead time between frames. During the dead time the COM and SEG voltage levels are held at 0 V to reduce the contrast without modifying the frame rate. ......"]
    #[inline(always)]
    pub fn dead(&self) -> DeadR {
        DeadR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:12 - Contrast control These bits specify one of the V&lt;sub>LCD &lt;/sub>maximum voltages (independent of V&lt;sub>DD&lt;/sub>). It ranges from12.60 V to 3.51V. Note: Refer to the datasheet for the V&lt;sub>LCDx&lt;/sub> values."]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Blink frequency selection"]
    #[inline(always)]
    pub fn blinkf(&self) -> BlinkfR {
        BlinkfR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Blink mode selection"]
    #[inline(always)]
    pub fn blink(&self) -> BlinkR {
        BlinkR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - DIV clock divider These bits are written by software to define the division factor of the DIV divider (see1Section118.3.2.) ..."]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - PS 16-bit prescaler These bits are written by software to define the division factor of the PS 16-bit prescaler. ck_ps = LCDCLK/(2&lt;sup>PS\\[3:0\\]&lt;/sup>). See&lt;sub> &lt;/sub>Section118.3.2. ..."]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - High drive enable This bit is written by software to enable a low resistance divider. Displays with high internal resistance may need a longer drive time to achieve satisfactory contrast. This bit is useful in this case if some additional power consumption can be tolerated."]
    #[inline(always)]
    #[must_use]
    pub fn hd(&mut self) -> HdW<LcdFcrSpec> {
        HdW::new(self, 0)
    }
    #[doc = "Bit 1 - Start of frame interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sofie(&mut self) -> SofieW<LcdFcrSpec> {
        SofieW::new(self, 1)
    }
    #[doc = "Bit 3 - Update display done interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn uddie(&mut self) -> UddieW<LcdFcrSpec> {
        UddieW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Pulse ON duration These bits are written by software to define the pulse duration in terms of ck_ps pulses. A1short pulse leads to lower power consumption, but displays with high internal resistance may need a longer pulse to achieve satisfactory contrast. Note that the pulse is never longer than one half prescaled LCD clock period. PON duration example with LCDCLK = 32.7681kHz and PS=0x03:"]
    #[inline(always)]
    #[must_use]
    pub fn pon(&mut self) -> PonW<LcdFcrSpec> {
        PonW::new(self, 4)
    }
    #[doc = "Bits 7:9 - Dead time duration These bits are written by software to configure the length of the dead time between frames. During the dead time the COM and SEG voltage levels are held at 0 V to reduce the contrast without modifying the frame rate. ......"]
    #[inline(always)]
    #[must_use]
    pub fn dead(&mut self) -> DeadW<LcdFcrSpec> {
        DeadW::new(self, 7)
    }
    #[doc = "Bits 10:12 - Contrast control These bits specify one of the V&lt;sub>LCD &lt;/sub>maximum voltages (independent of V&lt;sub>DD&lt;/sub>). It ranges from12.60 V to 3.51V. Note: Refer to the datasheet for the V&lt;sub>LCDx&lt;/sub> values."]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CcW<LcdFcrSpec> {
        CcW::new(self, 10)
    }
    #[doc = "Bits 13:15 - Blink frequency selection"]
    #[inline(always)]
    #[must_use]
    pub fn blinkf(&mut self) -> BlinkfW<LcdFcrSpec> {
        BlinkfW::new(self, 13)
    }
    #[doc = "Bits 16:17 - Blink mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn blink(&mut self) -> BlinkW<LcdFcrSpec> {
        BlinkW::new(self, 16)
    }
    #[doc = "Bits 18:21 - DIV clock divider These bits are written by software to define the division factor of the DIV divider (see1Section118.3.2.) ..."]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<LcdFcrSpec> {
        DivW::new(self, 18)
    }
    #[doc = "Bits 22:25 - PS 16-bit prescaler These bits are written by software to define the division factor of the PS 16-bit prescaler. ck_ps = LCDCLK/(2&lt;sup>PS\\[3:0\\]&lt;/sup>). See&lt;sub> &lt;/sub>Section118.3.2. ..."]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PsW<LcdFcrSpec> {
        PsW::new(self, 22)
    }
}
#[doc = "LCD frame control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdFcrSpec;
impl crate::RegisterSpec for LcdFcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_fcr::R`](R) reader structure"]
impl crate::Readable for LcdFcrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd_fcr::W`](W) writer structure"]
impl crate::Writable for LcdFcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCD_FCR to value 0"]
impl crate::Resettable for LcdFcrSpec {
    const RESET_VALUE: u32 = 0;
}
