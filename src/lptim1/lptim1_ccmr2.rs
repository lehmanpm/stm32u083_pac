#[doc = "Register `LPTIM1_CCMR2` reader"]
pub type R = crate::R<Lptim1Ccmr2Spec>;
#[doc = "Register `LPTIM1_CCMR2` writer"]
pub type W = crate::W<Lptim1Ccmr2Spec>;
#[doc = "Capture/compare 3 selection This bitfield defines the direction of the channel input (capture) or output mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc3sel {
    #[doc = "0: CC3 channel is configured in output PWM mode"]
    B0x0 = 0,
    #[doc = "1: CC3 channel is configured in input capture mode"]
    B0x1 = 1,
}
impl From<Cc3sel> for bool {
    #[inline(always)]
    fn from(variant: Cc3sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC3SEL` reader - Capture/compare 3 selection This bitfield defines the direction of the channel input (capture) or output mode."]
pub type Cc3selR = crate::BitReader<Cc3sel>;
impl Cc3selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc3sel {
        match self.bits {
            false => Cc3sel::B0x0,
            true => Cc3sel::B0x1,
        }
    }
    #[doc = "CC3 channel is configured in output PWM mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc3sel::B0x0
    }
    #[doc = "CC3 channel is configured in input capture mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc3sel::B0x1
    }
}
#[doc = "Field `CC3SEL` writer - Capture/compare 3 selection This bitfield defines the direction of the channel input (capture) or output mode."]
pub type Cc3selW<'a, REG> = crate::BitWriter<'a, REG, Cc3sel>;
impl<'a, REG> Cc3selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC3 channel is configured in output PWM mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3sel::B0x0)
    }
    #[doc = "CC3 channel is configured in input capture mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3sel::B0x1)
    }
}
#[doc = "Capture/compare 3 output enable. Condition: CC3 as output: Condition: CC3 as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 3 (LPTIM1_CCR3) or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc3e {
    #[doc = "0: Capture disabled. Writing '0' to the CC3E bit resets the associated ic3_dma_req signal."]
    B0x0 = 0,
    #[doc = "1: Capture enabled."]
    B0x1 = 1,
}
impl From<Cc3e> for bool {
    #[inline(always)]
    fn from(variant: Cc3e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC3E` reader - Capture/compare 3 output enable. Condition: CC3 as output: Condition: CC3 as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 3 (LPTIM1_CCR3) or not."]
pub type Cc3eR = crate::BitReader<Cc3e>;
impl Cc3eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc3e {
        match self.bits {
            false => Cc3e::B0x0,
            true => Cc3e::B0x1,
        }
    }
    #[doc = "Capture disabled. Writing '0' to the CC3E bit resets the associated ic3_dma_req signal."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc3e::B0x0
    }
    #[doc = "Capture enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc3e::B0x1
    }
}
#[doc = "Field `CC3E` writer - Capture/compare 3 output enable. Condition: CC3 as output: Condition: CC3 as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 3 (LPTIM1_CCR3) or not."]
pub type Cc3eW<'a, REG> = crate::BitWriter<'a, REG, Cc3e>;
impl<'a, REG> Cc3eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture disabled. Writing '0' to the CC3E bit resets the associated ic3_dma_req signal."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3e::B0x0)
    }
    #[doc = "Capture enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3e::B0x1)
    }
}
#[doc = "Capture/compare 3 output polarity. Condition: CC3 as output: Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. Condition: CC3 as input: This field is used to select the IC3 polarity for capture operations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc3p {
    #[doc = "0: rising edge, circuit is sensitive to IC3 rising edge"]
    B0x0 = 0,
    #[doc = "1: falling edge, circuit is sensitive to IC3 falling edge"]
    B0x1 = 1,
    #[doc = "2: reserved, do not use this configuration."]
    B0x2 = 2,
    #[doc = "3: both edges, circuit is sensitive to both IC3 rising and falling edges."]
    B0x3 = 3,
}
impl From<Cc3p> for u8 {
    #[inline(always)]
    fn from(variant: Cc3p) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc3p {
    type Ux = u8;
}
impl crate::IsEnum for Cc3p {}
#[doc = "Field `CC3P` reader - Capture/compare 3 output polarity. Condition: CC3 as output: Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. Condition: CC3 as input: This field is used to select the IC3 polarity for capture operations."]
pub type Cc3pR = crate::FieldReader<Cc3p>;
impl Cc3pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc3p {
        match self.bits {
            0 => Cc3p::B0x0,
            1 => Cc3p::B0x1,
            2 => Cc3p::B0x2,
            3 => Cc3p::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "rising edge, circuit is sensitive to IC3 rising edge"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc3p::B0x0
    }
    #[doc = "falling edge, circuit is sensitive to IC3 falling edge"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc3p::B0x1
    }
    #[doc = "reserved, do not use this configuration."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Cc3p::B0x2
    }
    #[doc = "both edges, circuit is sensitive to both IC3 rising and falling edges."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Cc3p::B0x3
    }
}
#[doc = "Field `CC3P` writer - Capture/compare 3 output polarity. Condition: CC3 as output: Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. Condition: CC3 as input: This field is used to select the IC3 polarity for capture operations."]
pub type Cc3pW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc3p, crate::Safe>;
impl<'a, REG> Cc3pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rising edge, circuit is sensitive to IC3 rising edge"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3p::B0x0)
    }
    #[doc = "falling edge, circuit is sensitive to IC3 falling edge"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3p::B0x1)
    }
    #[doc = "reserved, do not use this configuration."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3p::B0x2)
    }
    #[doc = "both edges, circuit is sensitive to both IC3 rising and falling edges."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3p::B0x3)
    }
}
#[doc = "Input capture 3 prescaler This bitfield defines the ratio of the prescaler acting on the CC3 input (IC3).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ic3psc {
    #[doc = "0: no prescaler, capture is done each time an edge is detected on the capture input"]
    B0x0 = 0,
    #[doc = "1: capture is done once every 2 events"]
    B0x1 = 1,
    #[doc = "2: capture is done once every 4 events"]
    B0x2 = 2,
    #[doc = "3: capture is done once every 8 events"]
    B0x3 = 3,
}
impl From<Ic3psc> for u8 {
    #[inline(always)]
    fn from(variant: Ic3psc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ic3psc {
    type Ux = u8;
}
impl crate::IsEnum for Ic3psc {}
#[doc = "Field `IC3PSC` reader - Input capture 3 prescaler This bitfield defines the ratio of the prescaler acting on the CC3 input (IC3)."]
pub type Ic3pscR = crate::FieldReader<Ic3psc>;
impl Ic3pscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ic3psc {
        match self.bits {
            0 => Ic3psc::B0x0,
            1 => Ic3psc::B0x1,
            2 => Ic3psc::B0x2,
            3 => Ic3psc::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ic3psc::B0x0
    }
    #[doc = "capture is done once every 2 events"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ic3psc::B0x1
    }
    #[doc = "capture is done once every 4 events"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ic3psc::B0x2
    }
    #[doc = "capture is done once every 8 events"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ic3psc::B0x3
    }
}
#[doc = "Field `IC3PSC` writer - Input capture 3 prescaler This bitfield defines the ratio of the prescaler acting on the CC3 input (IC3)."]
pub type Ic3pscW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ic3psc, crate::Safe>;
impl<'a, REG> Ic3pscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ic3psc::B0x0)
    }
    #[doc = "capture is done once every 2 events"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ic3psc::B0x1)
    }
    #[doc = "capture is done once every 4 events"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ic3psc::B0x2)
    }
    #[doc = "capture is done once every 8 events"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ic3psc::B0x3)
    }
}
#[doc = "Input capture 3 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ic3f {
    #[doc = "0: any external input capture signal level change is considered as a valid transition"]
    B0x0 = 0,
    #[doc = "1: external input capture signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    B0x1 = 1,
    #[doc = "2: external input capture signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    B0x2 = 2,
    #[doc = "3: external input capture signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    B0x3 = 3,
}
impl From<Ic3f> for u8 {
    #[inline(always)]
    fn from(variant: Ic3f) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ic3f {
    type Ux = u8;
}
impl crate::IsEnum for Ic3f {}
#[doc = "Field `IC3F` reader - Input capture 3 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
pub type Ic3fR = crate::FieldReader<Ic3f>;
impl Ic3fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ic3f {
        match self.bits {
            0 => Ic3f::B0x0,
            1 => Ic3f::B0x1,
            2 => Ic3f::B0x2,
            3 => Ic3f::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "any external input capture signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ic3f::B0x0
    }
    #[doc = "external input capture signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ic3f::B0x1
    }
    #[doc = "external input capture signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ic3f::B0x2
    }
    #[doc = "external input capture signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ic3f::B0x3
    }
}
#[doc = "Field `IC3F` writer - Input capture 3 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
pub type Ic3fW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ic3f, crate::Safe>;
impl<'a, REG> Ic3fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "any external input capture signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ic3f::B0x0)
    }
    #[doc = "external input capture signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ic3f::B0x1)
    }
    #[doc = "external input capture signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ic3f::B0x2)
    }
    #[doc = "external input capture signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ic3f::B0x3)
    }
}
#[doc = "Capture/compare 4 selection This bitfield defines the direction of the channel, input (capture) or output mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc4sel {
    #[doc = "0: CC4 channel is configured in output PWM mode"]
    B0x0 = 0,
    #[doc = "1: CC4 channel is configured in input capture mode"]
    B0x1 = 1,
}
impl From<Cc4sel> for bool {
    #[inline(always)]
    fn from(variant: Cc4sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC4SEL` reader - Capture/compare 4 selection This bitfield defines the direction of the channel, input (capture) or output mode."]
pub type Cc4selR = crate::BitReader<Cc4sel>;
impl Cc4selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc4sel {
        match self.bits {
            false => Cc4sel::B0x0,
            true => Cc4sel::B0x1,
        }
    }
    #[doc = "CC4 channel is configured in output PWM mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc4sel::B0x0
    }
    #[doc = "CC4 channel is configured in input capture mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc4sel::B0x1
    }
}
#[doc = "Field `CC4SEL` writer - Capture/compare 4 selection This bitfield defines the direction of the channel, input (capture) or output mode."]
pub type Cc4selW<'a, REG> = crate::BitWriter<'a, REG, Cc4sel>;
impl<'a, REG> Cc4selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC4 channel is configured in output PWM mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc4sel::B0x0)
    }
    #[doc = "CC4 channel is configured in input capture mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc4sel::B0x1)
    }
}
#[doc = "Capture/compare 4 output enable. Condition: CC4 as output: Condition: CC4 as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 4 (LPTIM1_CCR4) or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc4e {
    #[doc = "0: Capture disabled. Writing '0' to the CC4E bit resets the associated ic4_dma_req signal."]
    B0x0 = 0,
    #[doc = "1: Capture enabled."]
    B0x1 = 1,
}
impl From<Cc4e> for bool {
    #[inline(always)]
    fn from(variant: Cc4e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC4E` reader - Capture/compare 4 output enable. Condition: CC4 as output: Condition: CC4 as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 4 (LPTIM1_CCR4) or not."]
pub type Cc4eR = crate::BitReader<Cc4e>;
impl Cc4eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc4e {
        match self.bits {
            false => Cc4e::B0x0,
            true => Cc4e::B0x1,
        }
    }
    #[doc = "Capture disabled. Writing '0' to the CC4E bit resets the associated ic4_dma_req signal."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc4e::B0x0
    }
    #[doc = "Capture enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc4e::B0x1
    }
}
#[doc = "Field `CC4E` writer - Capture/compare 4 output enable. Condition: CC4 as output: Condition: CC4 as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 4 (LPTIM1_CCR4) or not."]
pub type Cc4eW<'a, REG> = crate::BitWriter<'a, REG, Cc4e>;
impl<'a, REG> Cc4eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture disabled. Writing '0' to the CC4E bit resets the associated ic4_dma_req signal."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc4e::B0x0)
    }
    #[doc = "Capture enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc4e::B0x1)
    }
}
#[doc = "Capture/compare 4 output polarity. Condition: CC4 as output: Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. Condition: CC4 as input: This field is used to select the IC4 polarity for capture operations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc4p {
    #[doc = "0: rising edge, circuit is sensitive to IC4 rising edge"]
    B0x0 = 0,
    #[doc = "1: falling edge, circuit is sensitive to IC4 falling edge"]
    B0x1 = 1,
    #[doc = "2: reserved, do not use this configuration."]
    B0x2 = 2,
    #[doc = "3: both edges, circuit is sensitive to both IC4 rising and falling edges."]
    B0x3 = 3,
}
impl From<Cc4p> for u8 {
    #[inline(always)]
    fn from(variant: Cc4p) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc4p {
    type Ux = u8;
}
impl crate::IsEnum for Cc4p {}
#[doc = "Field `CC4P` reader - Capture/compare 4 output polarity. Condition: CC4 as output: Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. Condition: CC4 as input: This field is used to select the IC4 polarity for capture operations."]
pub type Cc4pR = crate::FieldReader<Cc4p>;
impl Cc4pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc4p {
        match self.bits {
            0 => Cc4p::B0x0,
            1 => Cc4p::B0x1,
            2 => Cc4p::B0x2,
            3 => Cc4p::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "rising edge, circuit is sensitive to IC4 rising edge"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc4p::B0x0
    }
    #[doc = "falling edge, circuit is sensitive to IC4 falling edge"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc4p::B0x1
    }
    #[doc = "reserved, do not use this configuration."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Cc4p::B0x2
    }
    #[doc = "both edges, circuit is sensitive to both IC4 rising and falling edges."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Cc4p::B0x3
    }
}
#[doc = "Field `CC4P` writer - Capture/compare 4 output polarity. Condition: CC4 as output: Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. Condition: CC4 as input: This field is used to select the IC4 polarity for capture operations."]
pub type Cc4pW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc4p, crate::Safe>;
impl<'a, REG> Cc4pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rising edge, circuit is sensitive to IC4 rising edge"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc4p::B0x0)
    }
    #[doc = "falling edge, circuit is sensitive to IC4 falling edge"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc4p::B0x1)
    }
    #[doc = "reserved, do not use this configuration."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc4p::B0x2)
    }
    #[doc = "both edges, circuit is sensitive to both IC4 rising and falling edges."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Cc4p::B0x3)
    }
}
#[doc = "Input capture 4 prescaler This bitfield defines the ratio of the prescaler acting on the CC4 input (IC4).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ic4psc {
    #[doc = "0: no prescaler, capture is done each time an edge is detected on the capture input"]
    B0x0 = 0,
    #[doc = "1: capture is done once every 2 events"]
    B0x1 = 1,
    #[doc = "2: capture is done once every 4 events"]
    B0x2 = 2,
    #[doc = "3: capture is done once every 8 events"]
    B0x3 = 3,
}
impl From<Ic4psc> for u8 {
    #[inline(always)]
    fn from(variant: Ic4psc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ic4psc {
    type Ux = u8;
}
impl crate::IsEnum for Ic4psc {}
#[doc = "Field `IC4PSC` reader - Input capture 4 prescaler This bitfield defines the ratio of the prescaler acting on the CC4 input (IC4)."]
pub type Ic4pscR = crate::FieldReader<Ic4psc>;
impl Ic4pscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ic4psc {
        match self.bits {
            0 => Ic4psc::B0x0,
            1 => Ic4psc::B0x1,
            2 => Ic4psc::B0x2,
            3 => Ic4psc::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ic4psc::B0x0
    }
    #[doc = "capture is done once every 2 events"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ic4psc::B0x1
    }
    #[doc = "capture is done once every 4 events"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ic4psc::B0x2
    }
    #[doc = "capture is done once every 8 events"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ic4psc::B0x3
    }
}
#[doc = "Field `IC4PSC` writer - Input capture 4 prescaler This bitfield defines the ratio of the prescaler acting on the CC4 input (IC4)."]
pub type Ic4pscW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ic4psc, crate::Safe>;
impl<'a, REG> Ic4pscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ic4psc::B0x0)
    }
    #[doc = "capture is done once every 2 events"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ic4psc::B0x1)
    }
    #[doc = "capture is done once every 4 events"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ic4psc::B0x2)
    }
    #[doc = "capture is done once every 8 events"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ic4psc::B0x3)
    }
}
#[doc = "Input capture 4 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ic4f {
    #[doc = "0: any external input capture signal level change is considered as a valid transition"]
    B0x0 = 0,
    #[doc = "1: external input capture signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    B0x1 = 1,
    #[doc = "2: external input capture signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    B0x2 = 2,
    #[doc = "3: external input capture signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    B0x3 = 3,
}
impl From<Ic4f> for u8 {
    #[inline(always)]
    fn from(variant: Ic4f) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ic4f {
    type Ux = u8;
}
impl crate::IsEnum for Ic4f {}
#[doc = "Field `IC4F` reader - Input capture 4 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
pub type Ic4fR = crate::FieldReader<Ic4f>;
impl Ic4fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ic4f {
        match self.bits {
            0 => Ic4f::B0x0,
            1 => Ic4f::B0x1,
            2 => Ic4f::B0x2,
            3 => Ic4f::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "any external input capture signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ic4f::B0x0
    }
    #[doc = "external input capture signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ic4f::B0x1
    }
    #[doc = "external input capture signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ic4f::B0x2
    }
    #[doc = "external input capture signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ic4f::B0x3
    }
}
#[doc = "Field `IC4F` writer - Input capture 4 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
pub type Ic4fW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ic4f, crate::Safe>;
impl<'a, REG> Ic4fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "any external input capture signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ic4f::B0x0)
    }
    #[doc = "external input capture signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ic4f::B0x1)
    }
    #[doc = "external input capture signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ic4f::B0x2)
    }
    #[doc = "external input capture signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ic4f::B0x3)
    }
}
impl R {
    #[doc = "Bit 0 - Capture/compare 3 selection This bitfield defines the direction of the channel input (capture) or output mode."]
    #[inline(always)]
    pub fn cc3sel(&self) -> Cc3selR {
        Cc3selR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 3 output enable. Condition: CC3 as output: Condition: CC3 as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 3 (LPTIM1_CCR3) or not."]
    #[inline(always)]
    pub fn cc3e(&self) -> Cc3eR {
        Cc3eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Capture/compare 3 output polarity. Condition: CC3 as output: Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. Condition: CC3 as input: This field is used to select the IC3 polarity for capture operations."]
    #[inline(always)]
    pub fn cc3p(&self) -> Cc3pR {
        Cc3pR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Input capture 3 prescaler This bitfield defines the ratio of the prescaler acting on the CC3 input (IC3)."]
    #[inline(always)]
    pub fn ic3psc(&self) -> Ic3pscR {
        Ic3pscR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Input capture 3 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    #[inline(always)]
    pub fn ic3f(&self) -> Ic3fR {
        Ic3fR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - Capture/compare 4 selection This bitfield defines the direction of the channel, input (capture) or output mode."]
    #[inline(always)]
    pub fn cc4sel(&self) -> Cc4selR {
        Cc4selR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Capture/compare 4 output enable. Condition: CC4 as output: Condition: CC4 as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 4 (LPTIM1_CCR4) or not."]
    #[inline(always)]
    pub fn cc4e(&self) -> Cc4eR {
        Cc4eR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Capture/compare 4 output polarity. Condition: CC4 as output: Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. Condition: CC4 as input: This field is used to select the IC4 polarity for capture operations."]
    #[inline(always)]
    pub fn cc4p(&self) -> Cc4pR {
        Cc4pR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Input capture 4 prescaler This bitfield defines the ratio of the prescaler acting on the CC4 input (IC4)."]
    #[inline(always)]
    pub fn ic4psc(&self) -> Ic4pscR {
        Ic4pscR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Input capture 4 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    #[inline(always)]
    pub fn ic4f(&self) -> Ic4fR {
        Ic4fR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare 3 selection This bitfield defines the direction of the channel input (capture) or output mode."]
    #[inline(always)]
    #[must_use]
    pub fn cc3sel(&mut self) -> Cc3selW<Lptim1Ccmr2Spec> {
        Cc3selW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 3 output enable. Condition: CC3 as output: Condition: CC3 as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 3 (LPTIM1_CCR3) or not."]
    #[inline(always)]
    #[must_use]
    pub fn cc3e(&mut self) -> Cc3eW<Lptim1Ccmr2Spec> {
        Cc3eW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Capture/compare 3 output polarity. Condition: CC3 as output: Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. Condition: CC3 as input: This field is used to select the IC3 polarity for capture operations."]
    #[inline(always)]
    #[must_use]
    pub fn cc3p(&mut self) -> Cc3pW<Lptim1Ccmr2Spec> {
        Cc3pW::new(self, 2)
    }
    #[doc = "Bits 8:9 - Input capture 3 prescaler This bitfield defines the ratio of the prescaler acting on the CC3 input (IC3)."]
    #[inline(always)]
    #[must_use]
    pub fn ic3psc(&mut self) -> Ic3pscW<Lptim1Ccmr2Spec> {
        Ic3pscW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Input capture 3 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    #[inline(always)]
    #[must_use]
    pub fn ic3f(&mut self) -> Ic3fW<Lptim1Ccmr2Spec> {
        Ic3fW::new(self, 12)
    }
    #[doc = "Bit 16 - Capture/compare 4 selection This bitfield defines the direction of the channel, input (capture) or output mode."]
    #[inline(always)]
    #[must_use]
    pub fn cc4sel(&mut self) -> Cc4selW<Lptim1Ccmr2Spec> {
        Cc4selW::new(self, 16)
    }
    #[doc = "Bit 17 - Capture/compare 4 output enable. Condition: CC4 as output: Condition: CC4 as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 4 (LPTIM1_CCR4) or not."]
    #[inline(always)]
    #[must_use]
    pub fn cc4e(&mut self) -> Cc4eW<Lptim1Ccmr2Spec> {
        Cc4eW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Capture/compare 4 output polarity. Condition: CC4 as output: Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. Condition: CC4 as input: This field is used to select the IC4 polarity for capture operations."]
    #[inline(always)]
    #[must_use]
    pub fn cc4p(&mut self) -> Cc4pW<Lptim1Ccmr2Spec> {
        Cc4pW::new(self, 18)
    }
    #[doc = "Bits 24:25 - Input capture 4 prescaler This bitfield defines the ratio of the prescaler acting on the CC4 input (IC4)."]
    #[inline(always)]
    #[must_use]
    pub fn ic4psc(&mut self) -> Ic4pscW<Lptim1Ccmr2Spec> {
        Ic4pscW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Input capture 4 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    #[inline(always)]
    #[must_use]
    pub fn ic4f(&mut self) -> Ic4fW<Lptim1Ccmr2Spec> {
        Ic4fW::new(self, 28)
    }
}
#[doc = "LPTIM capture/compare mode register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_ccmr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim1_ccmr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lptim1Ccmr2Spec;
impl crate::RegisterSpec for Lptim1Ccmr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim1_ccmr2::R`](R) reader structure"]
impl crate::Readable for Lptim1Ccmr2Spec {}
#[doc = "`write(|w| ..)` method takes [`lptim1_ccmr2::W`](W) writer structure"]
impl crate::Writable for Lptim1Ccmr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTIM1_CCMR2 to value 0"]
impl crate::Resettable for Lptim1Ccmr2Spec {
    const RESET_VALUE: u32 = 0;
}
