#[doc = "Register `DAC_CR` reader"]
pub type R = crate::R<DacCrSpec>;
#[doc = "Register `DAC_CR` writer"]
pub type W = crate::W<DacCrSpec>;
#[doc = "DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En1 {
    #[doc = "0: DAC channel1 disabled"]
    B0x0 = 0,
    #[doc = "1: DAC channel1 enabled"]
    B0x1 = 1,
}
impl From<En1> for bool {
    #[inline(always)]
    fn from(variant: En1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN1` reader - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
pub type En1R = crate::BitReader<En1>;
impl En1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En1 {
        match self.bits {
            false => En1::B0x0,
            true => En1::B0x1,
        }
    }
    #[doc = "DAC channel1 disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == En1::B0x0
    }
    #[doc = "DAC channel1 enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == En1::B0x1
    }
}
#[doc = "Field `EN1` writer - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
pub type En1W<'a, REG> = crate::BitWriter<'a, REG, En1>;
impl<'a, REG> En1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel1 disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(En1::B0x0)
    }
    #[doc = "DAC channel1 enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(En1::B0x1)
    }
}
#[doc = "DAC channel1 trigger enable This bit is set and cleared by software to enable/disable DAC channel1 trigger. Note: When software trigger is selected, the transfer from the DAC_DHR1 register to the DAC_DOR1 register takes only one dac_pclk clock cycle.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ten1 {
    #[doc = "0: DAC channel1 trigger disabled and data written into the DAC_DHR1 register are transferred one dac_pclk clock cycle later to the DAC_DOR1 register"]
    B0x0 = 0,
    #[doc = "1: DAC channel1 trigger enabled and data from the DAC_DHR1 register are transferred three dac_pclk clock cycles later to the DAC_DOR1 register"]
    B0x1 = 1,
}
impl From<Ten1> for bool {
    #[inline(always)]
    fn from(variant: Ten1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN1` reader - DAC channel1 trigger enable This bit is set and cleared by software to enable/disable DAC channel1 trigger. Note: When software trigger is selected, the transfer from the DAC_DHR1 register to the DAC_DOR1 register takes only one dac_pclk clock cycle."]
pub type Ten1R = crate::BitReader<Ten1>;
impl Ten1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ten1 {
        match self.bits {
            false => Ten1::B0x0,
            true => Ten1::B0x1,
        }
    }
    #[doc = "DAC channel1 trigger disabled and data written into the DAC_DHR1 register are transferred one dac_pclk clock cycle later to the DAC_DOR1 register"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ten1::B0x0
    }
    #[doc = "DAC channel1 trigger enabled and data from the DAC_DHR1 register are transferred three dac_pclk clock cycles later to the DAC_DOR1 register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ten1::B0x1
    }
}
#[doc = "Field `TEN1` writer - DAC channel1 trigger enable This bit is set and cleared by software to enable/disable DAC channel1 trigger. Note: When software trigger is selected, the transfer from the DAC_DHR1 register to the DAC_DOR1 register takes only one dac_pclk clock cycle."]
pub type Ten1W<'a, REG> = crate::BitWriter<'a, REG, Ten1>;
impl<'a, REG> Ten1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel1 trigger disabled and data written into the DAC_DHR1 register are transferred one dac_pclk clock cycle later to the DAC_DOR1 register"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ten1::B0x0)
    }
    #[doc = "DAC channel1 trigger enabled and data from the DAC_DHR1 register are transferred three dac_pclk clock cycles later to the DAC_DOR1 register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ten1::B0x1)
    }
}
#[doc = "DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1 ... Refer to the trigger selection tables in Section114.4.2: DAC pins and internal signals for details on trigger configuration and mapping. Note: Only used if bit TEN11=11 (DAC channel1 trigger enabled).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsel1 {
    #[doc = "0: SWTRIG1"]
    B0x0 = 0,
    #[doc = "1: dac_ch1_trg1"]
    B0x1 = 1,
    #[doc = "2: dac_ch1_trg2"]
    B0x2 = 2,
    #[doc = "15: dac_ch1_trg15"]
    B0xF = 15,
}
impl From<Tsel1> for u8 {
    #[inline(always)]
    fn from(variant: Tsel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsel1 {
    type Ux = u8;
}
impl crate::IsEnum for Tsel1 {}
#[doc = "Field `TSEL1` reader - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1 ... Refer to the trigger selection tables in Section114.4.2: DAC pins and internal signals for details on trigger configuration and mapping. Note: Only used if bit TEN11=11 (DAC channel1 trigger enabled)."]
pub type Tsel1R = crate::FieldReader<Tsel1>;
impl Tsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tsel1> {
        match self.bits {
            0 => Some(Tsel1::B0x0),
            1 => Some(Tsel1::B0x1),
            2 => Some(Tsel1::B0x2),
            15 => Some(Tsel1::B0xF),
            _ => None,
        }
    }
    #[doc = "SWTRIG1"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tsel1::B0x0
    }
    #[doc = "dac_ch1_trg1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tsel1::B0x1
    }
    #[doc = "dac_ch1_trg2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Tsel1::B0x2
    }
    #[doc = "dac_ch1_trg15"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Tsel1::B0xF
    }
}
#[doc = "Field `TSEL1` writer - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1 ... Refer to the trigger selection tables in Section114.4.2: DAC pins and internal signals for details on trigger configuration and mapping. Note: Only used if bit TEN11=11 (DAC channel1 trigger enabled)."]
pub type Tsel1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Tsel1>;
impl<'a, REG> Tsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SWTRIG1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel1::B0x0)
    }
    #[doc = "dac_ch1_trg1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel1::B0x1)
    }
    #[doc = "dac_ch1_trg2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel1::B0x2)
    }
    #[doc = "dac_ch1_trg15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel1::B0xF)
    }
}
#[doc = "DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. 1x: Triangle wave generation enabled Only used if bit TEN11=11 (DAC channel1 trigger enabled).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wave1 {
    #[doc = "0: wave generation disabled"]
    B0x0 = 0,
    #[doc = "1: Noise wave generation enabled"]
    B0x1 = 1,
}
impl From<Wave1> for u8 {
    #[inline(always)]
    fn from(variant: Wave1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wave1 {
    type Ux = u8;
}
impl crate::IsEnum for Wave1 {}
#[doc = "Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. 1x: Triangle wave generation enabled Only used if bit TEN11=11 (DAC channel1 trigger enabled)."]
pub type Wave1R = crate::FieldReader<Wave1>;
impl Wave1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wave1> {
        match self.bits {
            0 => Some(Wave1::B0x0),
            1 => Some(Wave1::B0x1),
            _ => None,
        }
    }
    #[doc = "wave generation disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wave1::B0x0
    }
    #[doc = "Noise wave generation enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wave1::B0x1
    }
}
#[doc = "Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. 1x: Triangle wave generation enabled Only used if bit TEN11=11 (DAC channel1 trigger enabled)."]
pub type Wave1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wave1>;
impl<'a, REG> Wave1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "wave generation disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wave1::B0x0)
    }
    #[doc = "Noise wave generation enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wave1::B0x1)
    }
}
#[doc = "DAC channel1 mask/amplitude selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mamp1 {
    #[doc = "0: Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    B0x0 = 0,
    #[doc = "1: Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    B0x1 = 1,
    #[doc = "2: Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    B0x2 = 2,
    #[doc = "3: Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    B0x3 = 3,
    #[doc = "4: Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    B0x4 = 4,
    #[doc = "5: Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal to 63"]
    B0x5 = 5,
    #[doc = "6: Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    B0x6 = 6,
    #[doc = "7: Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    B0x7 = 7,
    #[doc = "8: Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    B0x8 = 8,
    #[doc = "9: Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    B0x9 = 9,
    #[doc = "10: Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    B0xA = 10,
}
impl From<Mamp1> for u8 {
    #[inline(always)]
    fn from(variant: Mamp1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mamp1 {
    type Ux = u8;
}
impl crate::IsEnum for Mamp1 {}
#[doc = "Field `MAMP1` reader - DAC channel1 mask/amplitude selector"]
pub type Mamp1R = crate::FieldReader<Mamp1>;
impl Mamp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mamp1> {
        match self.bits {
            0 => Some(Mamp1::B0x0),
            1 => Some(Mamp1::B0x1),
            2 => Some(Mamp1::B0x2),
            3 => Some(Mamp1::B0x3),
            4 => Some(Mamp1::B0x4),
            5 => Some(Mamp1::B0x5),
            6 => Some(Mamp1::B0x6),
            7 => Some(Mamp1::B0x7),
            8 => Some(Mamp1::B0x8),
            9 => Some(Mamp1::B0x9),
            10 => Some(Mamp1::B0xA),
            _ => None,
        }
    }
    #[doc = "Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mamp1::B0x0
    }
    #[doc = "Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mamp1::B0x1
    }
    #[doc = "Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mamp1::B0x2
    }
    #[doc = "Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mamp1::B0x3
    }
    #[doc = "Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Mamp1::B0x4
    }
    #[doc = "Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal to 63"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Mamp1::B0x5
    }
    #[doc = "Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Mamp1::B0x6
    }
    #[doc = "Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Mamp1::B0x7
    }
    #[doc = "Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Mamp1::B0x8
    }
    #[doc = "Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Mamp1::B0x9
    }
    #[doc = "Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Mamp1::B0xA
    }
}
#[doc = "Field `MAMP1` writer - DAC channel1 mask/amplitude selector"]
pub type Mamp1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mamp1>;
impl<'a, REG> Mamp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mamp1::B0x0)
    }
    #[doc = "Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mamp1::B0x1)
    }
    #[doc = "Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mamp1::B0x2)
    }
    #[doc = "Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mamp1::B0x3)
    }
    #[doc = "Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Mamp1::B0x4)
    }
    #[doc = "Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal to 63"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Mamp1::B0x5)
    }
    #[doc = "Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Mamp1::B0x6)
    }
    #[doc = "Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Mamp1::B0x7)
    }
    #[doc = "Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Mamp1::B0x8)
    }
    #[doc = "Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Mamp1::B0x9)
    }
    #[doc = "Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Mamp1::B0xA)
    }
}
#[doc = "DAC channel1 DMA enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen1 {
    #[doc = "0: DAC channel1 DMA mode disabled"]
    B0x0 = 0,
    #[doc = "1: DAC channel1 DMA mode enabled"]
    B0x1 = 1,
}
impl From<Dmaen1> for bool {
    #[inline(always)]
    fn from(variant: Dmaen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN1` reader - DAC channel1 DMA enable This bit is set and cleared by software."]
pub type Dmaen1R = crate::BitReader<Dmaen1>;
impl Dmaen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaen1 {
        match self.bits {
            false => Dmaen1::B0x0,
            true => Dmaen1::B0x1,
        }
    }
    #[doc = "DAC channel1 DMA mode disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dmaen1::B0x0
    }
    #[doc = "DAC channel1 DMA mode enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dmaen1::B0x1
    }
}
#[doc = "Field `DMAEN1` writer - DAC channel1 DMA enable This bit is set and cleared by software."]
pub type Dmaen1W<'a, REG> = crate::BitWriter<'a, REG, Dmaen1>;
impl<'a, REG> Dmaen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel1 DMA mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen1::B0x0)
    }
    #[doc = "DAC channel1 DMA mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen1::B0x1)
    }
}
#[doc = "DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaudrie1 {
    #[doc = "0: DAC channel1 DMA Underrun Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: DAC channel1 DMA Underrun Interrupt enabled"]
    B0x1 = 1,
}
impl From<Dmaudrie1> for bool {
    #[inline(always)]
    fn from(variant: Dmaudrie1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAUDRIE1` reader - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
pub type Dmaudrie1R = crate::BitReader<Dmaudrie1>;
impl Dmaudrie1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaudrie1 {
        match self.bits {
            false => Dmaudrie1::B0x0,
            true => Dmaudrie1::B0x1,
        }
    }
    #[doc = "DAC channel1 DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dmaudrie1::B0x0
    }
    #[doc = "DAC channel1 DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dmaudrie1::B0x1
    }
}
#[doc = "Field `DMAUDRIE1` writer - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
pub type Dmaudrie1W<'a, REG> = crate::BitWriter<'a, REG, Dmaudrie1>;
impl<'a, REG> Dmaudrie1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel1 DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaudrie1::B0x0)
    }
    #[doc = "DAC channel1 DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaudrie1::B0x1)
    }
}
#[doc = "DAC channel1 calibration enable This bit is set and cleared by software to enable/disable DAC channel1 calibration, it can be written only if bit EN11=10 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cen1 {
    #[doc = "0: DAC channel1 in Normal operating mode"]
    B0x0 = 0,
    #[doc = "1: DAC channel1 in calibration mode"]
    B0x1 = 1,
}
impl From<Cen1> for bool {
    #[inline(always)]
    fn from(variant: Cen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN1` reader - DAC channel1 calibration enable This bit is set and cleared by software to enable/disable DAC channel1 calibration, it can be written only if bit EN11=10 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
pub type Cen1R = crate::BitReader<Cen1>;
impl Cen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cen1 {
        match self.bits {
            false => Cen1::B0x0,
            true => Cen1::B0x1,
        }
    }
    #[doc = "DAC channel1 in Normal operating mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cen1::B0x0
    }
    #[doc = "DAC channel1 in calibration mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cen1::B0x1
    }
}
#[doc = "Field `CEN1` writer - DAC channel1 calibration enable This bit is set and cleared by software to enable/disable DAC channel1 calibration, it can be written only if bit EN11=10 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
pub type Cen1W<'a, REG> = crate::BitWriter<'a, REG, Cen1>;
impl<'a, REG> Cen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel1 in Normal operating mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cen1::B0x0)
    }
    #[doc = "DAC channel1 in calibration mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cen1::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
    #[inline(always)]
    pub fn en1(&self) -> En1R {
        En1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC channel1 trigger enable This bit is set and cleared by software to enable/disable DAC channel1 trigger. Note: When software trigger is selected, the transfer from the DAC_DHR1 register to the DAC_DOR1 register takes only one dac_pclk clock cycle."]
    #[inline(always)]
    pub fn ten1(&self) -> Ten1R {
        Ten1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1 ... Refer to the trigger selection tables in Section114.4.2: DAC pins and internal signals for details on trigger configuration and mapping. Note: Only used if bit TEN11=11 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub fn tsel1(&self) -> Tsel1R {
        Tsel1R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. 1x: Triangle wave generation enabled Only used if bit TEN11=11 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub fn wave1(&self) -> Wave1R {
        Wave1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp1(&self) -> Mamp1R {
        Mamp1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dmaen1(&self) -> Dmaen1R {
        Dmaen1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dmaudrie1(&self) -> Dmaudrie1R {
        Dmaudrie1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DAC channel1 calibration enable This bit is set and cleared by software to enable/disable DAC channel1 calibration, it can be written only if bit EN11=10 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
    #[inline(always)]
    pub fn cen1(&self) -> Cen1R {
        Cen1R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> En1W<DacCrSpec> {
        En1W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC channel1 trigger enable This bit is set and cleared by software to enable/disable DAC channel1 trigger. Note: When software trigger is selected, the transfer from the DAC_DHR1 register to the DAC_DOR1 register takes only one dac_pclk clock cycle."]
    #[inline(always)]
    #[must_use]
    pub fn ten1(&mut self) -> Ten1W<DacCrSpec> {
        Ten1W::new(self, 1)
    }
    #[doc = "Bits 2:5 - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1 ... Refer to the trigger selection tables in Section114.4.2: DAC pins and internal signals for details on trigger configuration and mapping. Note: Only used if bit TEN11=11 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    #[must_use]
    pub fn tsel1(&mut self) -> Tsel1W<DacCrSpec> {
        Tsel1W::new(self, 2)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. 1x: Triangle wave generation enabled Only used if bit TEN11=11 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    #[must_use]
    pub fn wave1(&mut self) -> Wave1W<DacCrSpec> {
        Wave1W::new(self, 6)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    #[must_use]
    pub fn mamp1(&mut self) -> Mamp1W<DacCrSpec> {
        Mamp1W::new(self, 8)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dmaen1(&mut self) -> Dmaen1W<DacCrSpec> {
        Dmaen1W::new(self, 12)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dmaudrie1(&mut self) -> Dmaudrie1W<DacCrSpec> {
        Dmaudrie1W::new(self, 13)
    }
    #[doc = "Bit 14 - DAC channel1 calibration enable This bit is set and cleared by software to enable/disable DAC channel1 calibration, it can be written only if bit EN11=10 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn cen1(&mut self) -> Cen1W<DacCrSpec> {
        Cen1W::new(self, 14)
    }
}
#[doc = "DAC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacCrSpec;
impl crate::RegisterSpec for DacCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_cr::R`](R) reader structure"]
impl crate::Readable for DacCrSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_cr::W`](W) writer structure"]
impl crate::Writable for DacCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_CR to value 0"]
impl crate::Resettable for DacCrSpec {
    const RESET_VALUE: u32 = 0;
}
