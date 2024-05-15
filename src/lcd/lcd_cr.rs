#[doc = "Register `LCD_CR` reader"]
pub type R = crate::R<LcdCrSpec>;
#[doc = "Register `LCD_CR` writer"]
pub type W = crate::W<LcdCrSpec>;
#[doc = "LCD controller enable This bit is set by software to enable the LCD controller/driver. It is cleared by software to turn off the LCD at the beginning of the next frame. When the LCD is disabled, all COM and SEG pins are driven to V&lt;sub>SS&lt;/sub>.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcden {
    #[doc = "0: LCD controller disabled"]
    B0x0 = 0,
    #[doc = "1: LCD controller enabled"]
    B0x1 = 1,
}
impl From<Lcden> for bool {
    #[inline(always)]
    fn from(variant: Lcden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDEN` reader - LCD controller enable This bit is set by software to enable the LCD controller/driver. It is cleared by software to turn off the LCD at the beginning of the next frame. When the LCD is disabled, all COM and SEG pins are driven to V&lt;sub>SS&lt;/sub>."]
pub type LcdenR = crate::BitReader<Lcden>;
impl LcdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcden {
        match self.bits {
            false => Lcden::B0x0,
            true => Lcden::B0x1,
        }
    }
    #[doc = "LCD controller disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lcden::B0x0
    }
    #[doc = "LCD controller enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lcden::B0x1
    }
}
#[doc = "Field `LCDEN` writer - LCD controller enable This bit is set by software to enable the LCD controller/driver. It is cleared by software to turn off the LCD at the beginning of the next frame. When the LCD is disabled, all COM and SEG pins are driven to V&lt;sub>SS&lt;/sub>."]
pub type LcdenW<'a, REG> = crate::BitWriter<'a, REG, Lcden>;
impl<'a, REG> LcdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD controller disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcden::B0x0)
    }
    #[doc = "LCD controller enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcden::B0x1)
    }
}
#[doc = "Voltage source selection This bit determines the voltage source for the LCD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vsel {
    #[doc = "0: Internal source (voltage stepup converter)"]
    B0x0 = 0,
    #[doc = "1: External source (VLCD pin)"]
    B0x1 = 1,
}
impl From<Vsel> for bool {
    #[inline(always)]
    fn from(variant: Vsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSEL` reader - Voltage source selection This bit determines the voltage source for the LCD."]
pub type VselR = crate::BitReader<Vsel>;
impl VselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vsel {
        match self.bits {
            false => Vsel::B0x0,
            true => Vsel::B0x1,
        }
    }
    #[doc = "Internal source (voltage stepup converter)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Vsel::B0x0
    }
    #[doc = "External source (VLCD pin)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Vsel::B0x1
    }
}
#[doc = "Field `VSEL` writer - Voltage source selection This bit determines the voltage source for the LCD."]
pub type VselW<'a, REG> = crate::BitWriter<'a, REG, Vsel>;
impl<'a, REG> VselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal source (voltage stepup converter)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Vsel::B0x0)
    }
    #[doc = "External source (VLCD pin)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Vsel::B0x1)
    }
}
#[doc = "Duty selection These bits determine the duty cycle. Values 101, 110 and 111 are forbidden. Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Duty {
    #[doc = "0: Static duty"]
    B0x0 = 0,
    #[doc = "1: 1/2 duty"]
    B0x1 = 1,
    #[doc = "2: 1/3 duty"]
    B0x2 = 2,
    #[doc = "3: 1/4 duty"]
    B0x3 = 3,
    #[doc = "4: 1/8 duty"]
    B0x4 = 4,
}
impl From<Duty> for u8 {
    #[inline(always)]
    fn from(variant: Duty) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Duty {
    type Ux = u8;
}
impl crate::IsEnum for Duty {}
#[doc = "Field `DUTY` reader - Duty selection These bits determine the duty cycle. Values 101, 110 and 111 are forbidden. Others: Reserved"]
pub type DutyR = crate::FieldReader<Duty>;
impl DutyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Duty> {
        match self.bits {
            0 => Some(Duty::B0x0),
            1 => Some(Duty::B0x1),
            2 => Some(Duty::B0x2),
            3 => Some(Duty::B0x3),
            4 => Some(Duty::B0x4),
            _ => None,
        }
    }
    #[doc = "Static duty"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Duty::B0x0
    }
    #[doc = "1/2 duty"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Duty::B0x1
    }
    #[doc = "1/3 duty"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Duty::B0x2
    }
    #[doc = "1/4 duty"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Duty::B0x3
    }
    #[doc = "1/8 duty"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Duty::B0x4
    }
}
#[doc = "Field `DUTY` writer - Duty selection These bits determine the duty cycle. Values 101, 110 and 111 are forbidden. Others: Reserved"]
pub type DutyW<'a, REG> = crate::FieldWriter<'a, REG, 3, Duty>;
impl<'a, REG> DutyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Static duty"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Duty::B0x0)
    }
    #[doc = "1/2 duty"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Duty::B0x1)
    }
    #[doc = "1/3 duty"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Duty::B0x2)
    }
    #[doc = "1/4 duty"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Duty::B0x3)
    }
    #[doc = "1/8 duty"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Duty::B0x4)
    }
}
#[doc = "Bias selector These bits determine the bias used. Value 11 is forbidden.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bias {
    #[doc = "0: Bias 1/4"]
    B0x0 = 0,
    #[doc = "1: Bias 1/2"]
    B0x1 = 1,
    #[doc = "2: Bias 1/3"]
    B0x2 = 2,
}
impl From<Bias> for u8 {
    #[inline(always)]
    fn from(variant: Bias) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bias {
    type Ux = u8;
}
impl crate::IsEnum for Bias {}
#[doc = "Field `BIAS` reader - Bias selector These bits determine the bias used. Value 11 is forbidden."]
pub type BiasR = crate::FieldReader<Bias>;
impl BiasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bias> {
        match self.bits {
            0 => Some(Bias::B0x0),
            1 => Some(Bias::B0x1),
            2 => Some(Bias::B0x2),
            _ => None,
        }
    }
    #[doc = "Bias 1/4"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bias::B0x0
    }
    #[doc = "Bias 1/2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bias::B0x1
    }
    #[doc = "Bias 1/3"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Bias::B0x2
    }
}
#[doc = "Field `BIAS` writer - Bias selector These bits determine the bias used. Value 11 is forbidden."]
pub type BiasW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bias>;
impl<'a, REG> BiasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bias 1/4"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bias::B0x0)
    }
    #[doc = "Bias 1/2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bias::B0x1)
    }
    #[doc = "Bias 1/3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Bias::B0x2)
    }
}
#[doc = "Mux segment enable This bit is used to enable SEG pin remapping. Four SEG pins can be multiplexed with1SEG\\[31:28\\]
or SEG\\[15:12\\]. See Section118.3.7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MuxSeg {
    #[doc = "0: SEG pin multiplexing disabled"]
    B0x0 = 0,
    #[doc = "1: SEG\\[31:28\\]
multiplexed with SEG\\[43:40\\]"]
    B0x1 = 1,
}
impl From<MuxSeg> for bool {
    #[inline(always)]
    fn from(variant: MuxSeg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUX_SEG` reader - Mux segment enable This bit is used to enable SEG pin remapping. Four SEG pins can be multiplexed with1SEG\\[31:28\\]
or SEG\\[15:12\\]. See Section118.3.7."]
pub type MuxSegR = crate::BitReader<MuxSeg>;
impl MuxSegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MuxSeg {
        match self.bits {
            false => MuxSeg::B0x0,
            true => MuxSeg::B0x1,
        }
    }
    #[doc = "SEG pin multiplexing disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MuxSeg::B0x0
    }
    #[doc = "SEG\\[31:28\\]
multiplexed with SEG\\[43:40\\]"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MuxSeg::B0x1
    }
}
#[doc = "Field `MUX_SEG` writer - Mux segment enable This bit is used to enable SEG pin remapping. Four SEG pins can be multiplexed with1SEG\\[31:28\\]
or SEG\\[15:12\\]. See Section118.3.7."]
pub type MuxSegW<'a, REG> = crate::BitWriter<'a, REG, MuxSeg>;
impl<'a, REG> MuxSegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SEG pin multiplexing disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MuxSeg::B0x0)
    }
    #[doc = "SEG\\[31:28\\]
multiplexed with SEG\\[43:40\\]"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MuxSeg::B0x1)
    }
}
#[doc = "Voltage output buffer enable This bit is used to enable/disable the voltage output buffer for higher driving capability.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufen {
    #[doc = "0: Output buffer disabled"]
    B0x0 = 0,
    #[doc = "1: Output buffer enabled"]
    B0x1 = 1,
}
impl From<Bufen> for bool {
    #[inline(always)]
    fn from(variant: Bufen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFEN` reader - Voltage output buffer enable This bit is used to enable/disable the voltage output buffer for higher driving capability."]
pub type BufenR = crate::BitReader<Bufen>;
impl BufenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufen {
        match self.bits {
            false => Bufen::B0x0,
            true => Bufen::B0x1,
        }
    }
    #[doc = "Output buffer disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bufen::B0x0
    }
    #[doc = "Output buffer enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bufen::B0x1
    }
}
#[doc = "Field `BUFEN` writer - Voltage output buffer enable This bit is used to enable/disable the voltage output buffer for higher driving capability."]
pub type BufenW<'a, REG> = crate::BitWriter<'a, REG, Bufen>;
impl<'a, REG> BufenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output buffer disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bufen::B0x0)
    }
    #[doc = "Output buffer enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bufen::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - LCD controller enable This bit is set by software to enable the LCD controller/driver. It is cleared by software to turn off the LCD at the beginning of the next frame. When the LCD is disabled, all COM and SEG pins are driven to V&lt;sub>SS&lt;/sub>."]
    #[inline(always)]
    pub fn lcden(&self) -> LcdenR {
        LcdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Voltage source selection This bit determines the voltage source for the LCD."]
    #[inline(always)]
    pub fn vsel(&self) -> VselR {
        VselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Duty selection These bits determine the duty cycle. Values 101, 110 and 111 are forbidden. Others: Reserved"]
    #[inline(always)]
    pub fn duty(&self) -> DutyR {
        DutyR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6 - Bias selector These bits determine the bias used. Value 11 is forbidden."]
    #[inline(always)]
    pub fn bias(&self) -> BiasR {
        BiasR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Mux segment enable This bit is used to enable SEG pin remapping. Four SEG pins can be multiplexed with1SEG\\[31:28\\]
or SEG\\[15:12\\]. See Section118.3.7."]
    #[inline(always)]
    pub fn mux_seg(&self) -> MuxSegR {
        MuxSegR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Voltage output buffer enable This bit is used to enable/disable the voltage output buffer for higher driving capability."]
    #[inline(always)]
    pub fn bufen(&self) -> BufenR {
        BufenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD controller enable This bit is set by software to enable the LCD controller/driver. It is cleared by software to turn off the LCD at the beginning of the next frame. When the LCD is disabled, all COM and SEG pins are driven to V&lt;sub>SS&lt;/sub>."]
    #[inline(always)]
    #[must_use]
    pub fn lcden(&mut self) -> LcdenW<LcdCrSpec> {
        LcdenW::new(self, 0)
    }
    #[doc = "Bit 1 - Voltage source selection This bit determines the voltage source for the LCD."]
    #[inline(always)]
    #[must_use]
    pub fn vsel(&mut self) -> VselW<LcdCrSpec> {
        VselW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Duty selection These bits determine the duty cycle. Values 101, 110 and 111 are forbidden. Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DutyW<LcdCrSpec> {
        DutyW::new(self, 2)
    }
    #[doc = "Bits 5:6 - Bias selector These bits determine the bias used. Value 11 is forbidden."]
    #[inline(always)]
    #[must_use]
    pub fn bias(&mut self) -> BiasW<LcdCrSpec> {
        BiasW::new(self, 5)
    }
    #[doc = "Bit 7 - Mux segment enable This bit is used to enable SEG pin remapping. Four SEG pins can be multiplexed with1SEG\\[31:28\\]
or SEG\\[15:12\\]. See Section118.3.7."]
    #[inline(always)]
    #[must_use]
    pub fn mux_seg(&mut self) -> MuxSegW<LcdCrSpec> {
        MuxSegW::new(self, 7)
    }
    #[doc = "Bit 8 - Voltage output buffer enable This bit is used to enable/disable the voltage output buffer for higher driving capability."]
    #[inline(always)]
    #[must_use]
    pub fn bufen(&mut self) -> BufenW<LcdCrSpec> {
        BufenW::new(self, 8)
    }
}
#[doc = "LCD control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdCrSpec;
impl crate::RegisterSpec for LcdCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_cr::R`](R) reader structure"]
impl crate::Readable for LcdCrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd_cr::W`](W) writer structure"]
impl crate::Writable for LcdCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCD_CR to value 0"]
impl crate::Resettable for LcdCrSpec {
    const RESET_VALUE: u32 = 0;
}
