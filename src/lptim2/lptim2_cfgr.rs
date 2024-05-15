#[doc = "Register `LPTIM2_CFGR` reader"]
pub type R = crate::R<Lptim2CfgrSpec>;
#[doc = "Register `LPTIM2_CFGR` writer"]
pub type W = crate::W<Lptim2CfgrSpec>;
#[doc = "Clock selector The CKSEL bit selects which clock source the LPTIM uses:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cksel {
    #[doc = "0: LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)"]
    B0x0 = 0,
    #[doc = "1: LPTIM is clocked by an external clock source through the LPTIM external Input1"]
    B0x1 = 1,
}
impl From<Cksel> for bool {
    #[inline(always)]
    fn from(variant: Cksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKSEL` reader - Clock selector The CKSEL bit selects which clock source the LPTIM uses:"]
pub type CkselR = crate::BitReader<Cksel>;
impl CkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cksel {
        match self.bits {
            false => Cksel::B0x0,
            true => Cksel::B0x1,
        }
    }
    #[doc = "LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cksel::B0x0
    }
    #[doc = "LPTIM is clocked by an external clock source through the LPTIM external Input1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cksel::B0x1
    }
}
#[doc = "Field `CKSEL` writer - Clock selector The CKSEL bit selects which clock source the LPTIM uses:"]
pub type CkselW<'a, REG> = crate::BitWriter<'a, REG, Cksel>;
impl<'a, REG> CkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::B0x0)
    }
    #[doc = "LPTIM is clocked by an external clock source through the LPTIM external Input1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::B0x1)
    }
}
#[doc = "Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active. Refer to Section125.4.15: Encoder mode for more details about Encoder mode sub-modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckpol {
    #[doc = "0: the rising edge is the active edge used for counting."]
    B0x0 = 0,
    #[doc = "1: the falling edge is the active edge used for counting."]
    B0x1 = 1,
    #[doc = "2: both edges are active edges. When both external clock signal edges are considered active ones, the LPTIM must also be clocked by an internal clock source with a frequency equal to at least four times the external clock frequency."]
    B0x2 = 2,
    #[doc = "3: not allowed"]
    B0x3 = 3,
}
impl From<Ckpol> for u8 {
    #[inline(always)]
    fn from(variant: Ckpol) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckpol {
    type Ux = u8;
}
impl crate::IsEnum for Ckpol {}
#[doc = "Field `CKPOL` reader - Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active. Refer to Section125.4.15: Encoder mode for more details about Encoder mode sub-modes."]
pub type CkpolR = crate::FieldReader<Ckpol>;
impl CkpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckpol {
        match self.bits {
            0 => Ckpol::B0x0,
            1 => Ckpol::B0x1,
            2 => Ckpol::B0x2,
            3 => Ckpol::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "the rising edge is the active edge used for counting."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ckpol::B0x0
    }
    #[doc = "the falling edge is the active edge used for counting."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ckpol::B0x1
    }
    #[doc = "both edges are active edges. When both external clock signal edges are considered active ones, the LPTIM must also be clocked by an internal clock source with a frequency equal to at least four times the external clock frequency."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ckpol::B0x2
    }
    #[doc = "not allowed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ckpol::B0x3
    }
}
#[doc = "Field `CKPOL` writer - Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active. Refer to Section125.4.15: Encoder mode for more details about Encoder mode sub-modes."]
pub type CkpolW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ckpol, crate::Safe>;
impl<'a, REG> CkpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "the rising edge is the active edge used for counting."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ckpol::B0x0)
    }
    #[doc = "the falling edge is the active edge used for counting."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ckpol::B0x1)
    }
    #[doc = "both edges are active edges. When both external clock signal edges are considered active ones, the LPTIM must also be clocked by an internal clock source with a frequency equal to at least four times the external clock frequency."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ckpol::B0x2)
    }
    #[doc = "not allowed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ckpol::B0x3)
    }
}
#[doc = "Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that are detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckflt {
    #[doc = "0: any external clock signal level change is considered as a valid transition"]
    B0x0 = 0,
    #[doc = "1: external clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    B0x1 = 1,
    #[doc = "2: external clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    B0x2 = 2,
    #[doc = "3: external clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    B0x3 = 3,
}
impl From<Ckflt> for u8 {
    #[inline(always)]
    fn from(variant: Ckflt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckflt {
    type Ux = u8;
}
impl crate::IsEnum for Ckflt {}
#[doc = "Field `CKFLT` reader - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that are detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub type CkfltR = crate::FieldReader<Ckflt>;
impl CkfltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckflt {
        match self.bits {
            0 => Ckflt::B0x0,
            1 => Ckflt::B0x1,
            2 => Ckflt::B0x2,
            3 => Ckflt::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "any external clock signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ckflt::B0x0
    }
    #[doc = "external clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ckflt::B0x1
    }
    #[doc = "external clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ckflt::B0x2
    }
    #[doc = "external clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ckflt::B0x3
    }
}
#[doc = "Field `CKFLT` writer - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that are detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub type CkfltW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ckflt, crate::Safe>;
impl<'a, REG> CkfltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "any external clock signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ckflt::B0x0)
    }
    #[doc = "external clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ckflt::B0x1)
    }
    #[doc = "external clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ckflt::B0x2)
    }
    #[doc = "external clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ckflt::B0x3)
    }
}
#[doc = "Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that are detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgflt {
    #[doc = "0: any trigger active level change is considered as a valid trigger"]
    B0x0 = 0,
    #[doc = "1: trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger."]
    B0x1 = 1,
    #[doc = "2: trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger."]
    B0x2 = 2,
    #[doc = "3: trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger."]
    B0x3 = 3,
}
impl From<Trgflt> for u8 {
    #[inline(always)]
    fn from(variant: Trgflt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgflt {
    type Ux = u8;
}
impl crate::IsEnum for Trgflt {}
#[doc = "Field `TRGFLT` reader - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that are detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub type TrgfltR = crate::FieldReader<Trgflt>;
impl TrgfltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgflt {
        match self.bits {
            0 => Trgflt::B0x0,
            1 => Trgflt::B0x1,
            2 => Trgflt::B0x2,
            3 => Trgflt::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "any trigger active level change is considered as a valid trigger"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Trgflt::B0x0
    }
    #[doc = "trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Trgflt::B0x1
    }
    #[doc = "trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Trgflt::B0x2
    }
    #[doc = "trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Trgflt::B0x3
    }
}
#[doc = "Field `TRGFLT` writer - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that are detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub type TrgfltW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trgflt, crate::Safe>;
impl<'a, REG> TrgfltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "any trigger active level change is considered as a valid trigger"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgflt::B0x0)
    }
    #[doc = "trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgflt::B0x1)
    }
    #[doc = "trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgflt::B0x2)
    }
    #[doc = "trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgflt::B0x3)
    }
}
#[doc = "Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Presc {
    #[doc = "0: /1"]
    B0x0 = 0,
    #[doc = "1: /2"]
    B0x1 = 1,
    #[doc = "2: /4"]
    B0x2 = 2,
    #[doc = "3: /8"]
    B0x3 = 3,
    #[doc = "4: /16"]
    B0x4 = 4,
    #[doc = "5: /32"]
    B0x5 = 5,
    #[doc = "6: /64"]
    B0x6 = 6,
    #[doc = "7: /128"]
    B0x7 = 7,
}
impl From<Presc> for u8 {
    #[inline(always)]
    fn from(variant: Presc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Presc {
    type Ux = u8;
}
impl crate::IsEnum for Presc {}
#[doc = "Field `PRESC` reader - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
pub type PrescR = crate::FieldReader<Presc>;
impl PrescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Presc {
        match self.bits {
            0 => Presc::B0x0,
            1 => Presc::B0x1,
            2 => Presc::B0x2,
            3 => Presc::B0x3,
            4 => Presc::B0x4,
            5 => Presc::B0x5,
            6 => Presc::B0x6,
            7 => Presc::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Presc::B0x0
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Presc::B0x1
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Presc::B0x2
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Presc::B0x3
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Presc::B0x4
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Presc::B0x5
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Presc::B0x6
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Presc::B0x7
    }
}
#[doc = "Field `PRESC` writer - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 3, Presc, crate::Safe>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0x0)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0x1)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0x2)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0x3)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0x4)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0x5)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0x6)
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::B0x7)
    }
}
#[doc = "Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See Section125.4.3: LPTIM input and trigger mapping for details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigsel {
    #[doc = "0: LPTIM2_ext_trig0"]
    B0x0 = 0,
    #[doc = "1: LPTIM2_ext_trig1"]
    B0x1 = 1,
    #[doc = "2: LPTIM2_ext_trig2"]
    B0x2 = 2,
    #[doc = "3: LPTIM2_ext_trig3"]
    B0x3 = 3,
    #[doc = "4: LPTIM2_ext_trig4"]
    B0x4 = 4,
    #[doc = "5: LPTIM2_ext_trig5"]
    B0x5 = 5,
    #[doc = "6: LPTIM2_ext_trig6"]
    B0x6 = 6,
    #[doc = "7: LPTIM2_ext_trig7"]
    B0x7 = 7,
}
impl From<Trigsel> for u8 {
    #[inline(always)]
    fn from(variant: Trigsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigsel {
    type Ux = u8;
}
impl crate::IsEnum for Trigsel {}
#[doc = "Field `TRIGSEL` reader - Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See Section125.4.3: LPTIM input and trigger mapping for details."]
pub type TrigselR = crate::FieldReader<Trigsel>;
impl TrigselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigsel {
        match self.bits {
            0 => Trigsel::B0x0,
            1 => Trigsel::B0x1,
            2 => Trigsel::B0x2,
            3 => Trigsel::B0x3,
            4 => Trigsel::B0x4,
            5 => Trigsel::B0x5,
            6 => Trigsel::B0x6,
            7 => Trigsel::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "LPTIM2_ext_trig0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Trigsel::B0x0
    }
    #[doc = "LPTIM2_ext_trig1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Trigsel::B0x1
    }
    #[doc = "LPTIM2_ext_trig2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Trigsel::B0x2
    }
    #[doc = "LPTIM2_ext_trig3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Trigsel::B0x3
    }
    #[doc = "LPTIM2_ext_trig4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Trigsel::B0x4
    }
    #[doc = "LPTIM2_ext_trig5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Trigsel::B0x5
    }
    #[doc = "LPTIM2_ext_trig6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Trigsel::B0x6
    }
    #[doc = "LPTIM2_ext_trig7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Trigsel::B0x7
    }
}
#[doc = "Field `TRIGSEL` writer - Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See Section125.4.3: LPTIM input and trigger mapping for details."]
pub type TrigselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Trigsel, crate::Safe>;
impl<'a, REG> TrigselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPTIM2_ext_trig0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::B0x0)
    }
    #[doc = "LPTIM2_ext_trig1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::B0x1)
    }
    #[doc = "LPTIM2_ext_trig2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::B0x2)
    }
    #[doc = "LPTIM2_ext_trig3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::B0x3)
    }
    #[doc = "LPTIM2_ext_trig4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::B0x4)
    }
    #[doc = "LPTIM2_ext_trig5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::B0x5)
    }
    #[doc = "LPTIM2_ext_trig6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::B0x6)
    }
    #[doc = "LPTIM2_ext_trig7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::B0x7)
    }
}
#[doc = "Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigen {
    #[doc = "0: software trigger (counting start is initiated by software)"]
    B0x0 = 0,
    #[doc = "1: rising edge is the active edge"]
    B0x1 = 1,
    #[doc = "2: falling edge is the active edge"]
    B0x2 = 2,
    #[doc = "3: both edges are active edges"]
    B0x3 = 3,
}
impl From<Trigen> for u8 {
    #[inline(always)]
    fn from(variant: Trigen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigen {
    type Ux = u8;
}
impl crate::IsEnum for Trigen {}
#[doc = "Field `TRIGEN` reader - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
pub type TrigenR = crate::FieldReader<Trigen>;
impl TrigenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigen {
        match self.bits {
            0 => Trigen::B0x0,
            1 => Trigen::B0x1,
            2 => Trigen::B0x2,
            3 => Trigen::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "software trigger (counting start is initiated by software)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Trigen::B0x0
    }
    #[doc = "rising edge is the active edge"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Trigen::B0x1
    }
    #[doc = "falling edge is the active edge"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Trigen::B0x2
    }
    #[doc = "both edges are active edges"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Trigen::B0x3
    }
}
#[doc = "Field `TRIGEN` writer - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
pub type TrigenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trigen, crate::Safe>;
impl<'a, REG> TrigenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "software trigger (counting start is initiated by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigen::B0x0)
    }
    #[doc = "rising edge is the active edge"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigen::B0x1)
    }
    #[doc = "falling edge is the active edge"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Trigen::B0x2)
    }
    #[doc = "both edges are active edges"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Trigen::B0x3)
    }
}
#[doc = "Timeout enable The TIMOUT bit controls the Timeout feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timout {
    #[doc = "0: A trigger event arriving when the timer is already started is ignored"]
    B0x0 = 0,
    #[doc = "1: A trigger event arriving when the timer is already started resets and restarts the LPTIM counter and the repetition counter"]
    B0x1 = 1,
}
impl From<Timout> for bool {
    #[inline(always)]
    fn from(variant: Timout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMOUT` reader - Timeout enable The TIMOUT bit controls the Timeout feature"]
pub type TimoutR = crate::BitReader<Timout>;
impl TimoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timout {
        match self.bits {
            false => Timout::B0x0,
            true => Timout::B0x1,
        }
    }
    #[doc = "A trigger event arriving when the timer is already started is ignored"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Timout::B0x0
    }
    #[doc = "A trigger event arriving when the timer is already started resets and restarts the LPTIM counter and the repetition counter"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Timout::B0x1
    }
}
#[doc = "Field `TIMOUT` writer - Timeout enable The TIMOUT bit controls the Timeout feature"]
pub type TimoutW<'a, REG> = crate::BitWriter<'a, REG, Timout>;
impl<'a, REG> TimoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A trigger event arriving when the timer is already started is ignored"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Timout::B0x0)
    }
    #[doc = "A trigger event arriving when the timer is already started resets and restarts the LPTIM counter and the repetition counter"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Timout::B0x1)
    }
}
#[doc = "Waveform shape The WAVE bit controls the output shape\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wave {
    #[doc = "0: Deactivate Set-once mode"]
    B0x0 = 0,
    #[doc = "1: Activate the Set-once mode"]
    B0x1 = 1,
}
impl From<Wave> for bool {
    #[inline(always)]
    fn from(variant: Wave) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAVE` reader - Waveform shape The WAVE bit controls the output shape"]
pub type WaveR = crate::BitReader<Wave>;
impl WaveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wave {
        match self.bits {
            false => Wave::B0x0,
            true => Wave::B0x1,
        }
    }
    #[doc = "Deactivate Set-once mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wave::B0x0
    }
    #[doc = "Activate the Set-once mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wave::B0x1
    }
}
#[doc = "Field `WAVE` writer - Waveform shape The WAVE bit controls the output shape"]
pub type WaveW<'a, REG> = crate::BitWriter<'a, REG, Wave>;
impl<'a, REG> WaveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deactivate Set-once mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wave::B0x0)
    }
    #[doc = "Activate the Set-once mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wave::B0x1)
    }
}
#[doc = "Registers update mode The PRELOAD bit controls the LPTIM2_ARR, LPTIM2_RCR and the LPTIM2_CCRx registers update modality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preload {
    #[doc = "0: Registers are updated after each APB bus write access"]
    B0x0 = 0,
    #[doc = "1: Registers are updated at the end of the current LPTIM period"]
    B0x1 = 1,
}
impl From<Preload> for bool {
    #[inline(always)]
    fn from(variant: Preload) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRELOAD` reader - Registers update mode The PRELOAD bit controls the LPTIM2_ARR, LPTIM2_RCR and the LPTIM2_CCRx registers update modality"]
pub type PreloadR = crate::BitReader<Preload>;
impl PreloadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Preload {
        match self.bits {
            false => Preload::B0x0,
            true => Preload::B0x1,
        }
    }
    #[doc = "Registers are updated after each APB bus write access"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Preload::B0x0
    }
    #[doc = "Registers are updated at the end of the current LPTIM period"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Preload::B0x1
    }
}
#[doc = "Field `PRELOAD` writer - Registers update mode The PRELOAD bit controls the LPTIM2_ARR, LPTIM2_RCR and the LPTIM2_CCRx registers update modality"]
pub type PreloadW<'a, REG> = crate::BitWriter<'a, REG, Preload>;
impl<'a, REG> PreloadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Registers are updated after each APB bus write access"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Preload::B0x0)
    }
    #[doc = "Registers are updated at the end of the current LPTIM period"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Preload::B0x1)
    }
}
#[doc = "counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Countmode {
    #[doc = "0: the counter is incremented following each internal clock pulse"]
    B0x0 = 0,
    #[doc = "1: the counter is incremented following each valid clock pulse on the LPTIM external Input1"]
    B0x1 = 1,
}
impl From<Countmode> for bool {
    #[inline(always)]
    fn from(variant: Countmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COUNTMODE` reader - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
pub type CountmodeR = crate::BitReader<Countmode>;
impl CountmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Countmode {
        match self.bits {
            false => Countmode::B0x0,
            true => Countmode::B0x1,
        }
    }
    #[doc = "the counter is incremented following each internal clock pulse"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Countmode::B0x0
    }
    #[doc = "the counter is incremented following each valid clock pulse on the LPTIM external Input1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Countmode::B0x1
    }
}
#[doc = "Field `COUNTMODE` writer - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
pub type CountmodeW<'a, REG> = crate::BitWriter<'a, REG, Countmode>;
impl<'a, REG> CountmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the counter is incremented following each internal clock pulse"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Countmode::B0x0)
    }
    #[doc = "the counter is incremented following each valid clock pulse on the LPTIM external Input1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Countmode::B0x1)
    }
}
#[doc = "Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enc {
    #[doc = "0: Encoder mode disabled"]
    B0x0 = 0,
    #[doc = "1: Encoder mode enabled"]
    B0x1 = 1,
}
impl From<Enc> for bool {
    #[inline(always)]
    fn from(variant: Enc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENC` reader - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
pub type EncR = crate::BitReader<Enc>;
impl EncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enc {
        match self.bits {
            false => Enc::B0x0,
            true => Enc::B0x1,
        }
    }
    #[doc = "Encoder mode disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Enc::B0x0
    }
    #[doc = "Encoder mode enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Enc::B0x1
    }
}
#[doc = "Field `ENC` writer - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
pub type EncW<'a, REG> = crate::BitWriter<'a, REG, Enc>;
impl<'a, REG> EncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Encoder mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Enc::B0x0)
    }
    #[doc = "Encoder mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Enc::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Clock selector The CKSEL bit selects which clock source the LPTIM uses:"]
    #[inline(always)]
    pub fn cksel(&self) -> CkselR {
        CkselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active. Refer to Section125.4.15: Encoder mode for more details about Encoder mode sub-modes."]
    #[inline(always)]
    pub fn ckpol(&self) -> CkpolR {
        CkpolR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that are detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    pub fn ckflt(&self) -> CkfltR {
        CkfltR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that are detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    pub fn trgflt(&self) -> TrgfltR {
        TrgfltR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 9:11 - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See Section125.4.3: LPTIM input and trigger mapping for details."]
    #[inline(always)]
    pub fn trigsel(&self) -> TrigselR {
        TrigselR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 17:18 - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
    #[inline(always)]
    pub fn trigen(&self) -> TrigenR {
        TrigenR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Timeout enable The TIMOUT bit controls the Timeout feature"]
    #[inline(always)]
    pub fn timout(&self) -> TimoutR {
        TimoutR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Waveform shape The WAVE bit controls the output shape"]
    #[inline(always)]
    pub fn wave(&self) -> WaveR {
        WaveR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Registers update mode The PRELOAD bit controls the LPTIM2_ARR, LPTIM2_RCR and the LPTIM2_CCRx registers update modality"]
    #[inline(always)]
    pub fn preload(&self) -> PreloadR {
        PreloadR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
    #[inline(always)]
    pub fn countmode(&self) -> CountmodeR {
        CountmodeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn enc(&self) -> EncR {
        EncR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock selector The CKSEL bit selects which clock source the LPTIM uses:"]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CkselW<Lptim2CfgrSpec> {
        CkselW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active. Refer to Section125.4.15: Encoder mode for more details about Encoder mode sub-modes."]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CkpolW<Lptim2CfgrSpec> {
        CkpolW::new(self, 1)
    }
    #[doc = "Bits 3:4 - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that are detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    #[must_use]
    pub fn ckflt(&mut self) -> CkfltW<Lptim2CfgrSpec> {
        CkfltW::new(self, 3)
    }
    #[doc = "Bits 6:7 - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that are detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    #[must_use]
    pub fn trgflt(&mut self) -> TrgfltW<Lptim2CfgrSpec> {
        TrgfltW::new(self, 6)
    }
    #[doc = "Bits 9:11 - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PrescW<Lptim2CfgrSpec> {
        PrescW::new(self, 9)
    }
    #[doc = "Bits 13:15 - Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See Section125.4.3: LPTIM input and trigger mapping for details."]
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TrigselW<Lptim2CfgrSpec> {
        TrigselW::new(self, 13)
    }
    #[doc = "Bits 17:18 - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
    #[inline(always)]
    #[must_use]
    pub fn trigen(&mut self) -> TrigenW<Lptim2CfgrSpec> {
        TrigenW::new(self, 17)
    }
    #[doc = "Bit 19 - Timeout enable The TIMOUT bit controls the Timeout feature"]
    #[inline(always)]
    #[must_use]
    pub fn timout(&mut self) -> TimoutW<Lptim2CfgrSpec> {
        TimoutW::new(self, 19)
    }
    #[doc = "Bit 20 - Waveform shape The WAVE bit controls the output shape"]
    #[inline(always)]
    #[must_use]
    pub fn wave(&mut self) -> WaveW<Lptim2CfgrSpec> {
        WaveW::new(self, 20)
    }
    #[doc = "Bit 22 - Registers update mode The PRELOAD bit controls the LPTIM2_ARR, LPTIM2_RCR and the LPTIM2_CCRx registers update modality"]
    #[inline(always)]
    #[must_use]
    pub fn preload(&mut self) -> PreloadW<Lptim2CfgrSpec> {
        PreloadW::new(self, 22)
    }
    #[doc = "Bit 23 - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
    #[inline(always)]
    #[must_use]
    pub fn countmode(&mut self) -> CountmodeW<Lptim2CfgrSpec> {
        CountmodeW::new(self, 23)
    }
    #[doc = "Bit 24 - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn enc(&mut self) -> EncW<Lptim2CfgrSpec> {
        EncW::new(self, 24)
    }
}
#[doc = "LPTIM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lptim2CfgrSpec;
impl crate::RegisterSpec for Lptim2CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim2_cfgr::R`](R) reader structure"]
impl crate::Readable for Lptim2CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`lptim2_cfgr::W`](W) writer structure"]
impl crate::Writable for Lptim2CfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTIM2_CFGR to value 0"]
impl crate::Resettable for Lptim2CfgrSpec {
    const RESET_VALUE: u32 = 0;
}
