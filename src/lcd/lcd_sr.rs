#[doc = "Register `LCD_SR` reader"]
pub type R = crate::R<LcdSrSpec>;
#[doc = "Register `LCD_SR` writer"]
pub type W = crate::W<LcdSrSpec>;
#[doc = "LCD enabled status This bit is set and cleared by hardware. It indicates the LCD controller status. Note: This bit is set immediately when LCDEN in LCD_CR goes from 0 to 1. On deactivation, it reflects the real LCD status. It becomes 0 at the end of the last displayed frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ens {
    #[doc = "0: LCD controller disabled"]
    B0x0 = 0,
    #[doc = "1: LCD controller enabled"]
    B0x1 = 1,
}
impl From<Ens> for bool {
    #[inline(always)]
    fn from(variant: Ens) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENS` reader - LCD enabled status This bit is set and cleared by hardware. It indicates the LCD controller status. Note: This bit is set immediately when LCDEN in LCD_CR goes from 0 to 1. On deactivation, it reflects the real LCD status. It becomes 0 at the end of the last displayed frame."]
pub type EnsR = crate::BitReader<Ens>;
impl EnsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ens {
        match self.bits {
            false => Ens::B0x0,
            true => Ens::B0x1,
        }
    }
    #[doc = "LCD controller disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ens::B0x0
    }
    #[doc = "LCD controller enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ens::B0x1
    }
}
#[doc = "Start-of-frame flag This bit is set by hardware at the beginning of a new frame, at the same time as the display data is updated. It is cleared by writing a 1 to SOFC in LCD_CLR. The bit clear has priority over the set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sof {
    #[doc = "0: No event"]
    B0x0 = 0,
    #[doc = "1: Start-of-frame event occurred. An LCD SOF interrupt is generated if SOFIE is set."]
    B0x1 = 1,
}
impl From<Sof> for bool {
    #[inline(always)]
    fn from(variant: Sof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF` reader - Start-of-frame flag This bit is set by hardware at the beginning of a new frame, at the same time as the display data is updated. It is cleared by writing a 1 to SOFC in LCD_CLR. The bit clear has priority over the set."]
pub type SofR = crate::BitReader<Sof>;
impl SofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sof {
        match self.bits {
            false => Sof::B0x0,
            true => Sof::B0x1,
        }
    }
    #[doc = "No event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sof::B0x0
    }
    #[doc = "Start-of-frame event occurred. An LCD SOF interrupt is generated if SOFIE is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sof::B0x1
    }
}
#[doc = "Update display request Each time software modifies the LCD_RAM, it must set this bit to transfer the updated data to the second level buffer. This bit stays set until the end of the update. During this time, the LCD_RAM is write protected. When the display is disabled, the update is performed for all LCD_DISPLAY locations. When the display is enabled, the update is performed only for locations for which commons are active (depending on DUTY). For example if DUTY = 1/2, Note: only the LCD_DISPLAY of COM0 and COM1 are updated. Note: Writing 0 on this bit or writing 1 when it is already 1 has no effect. This bit can be cleared by hardware only. It can be cleared only when LCDEN = 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Udr {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Update display request"]
    B0x1 = 1,
}
impl From<Udr> for bool {
    #[inline(always)]
    fn from(variant: Udr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDR` reader - Update display request Each time software modifies the LCD_RAM, it must set this bit to transfer the updated data to the second level buffer. This bit stays set until the end of the update. During this time, the LCD_RAM is write protected. When the display is disabled, the update is performed for all LCD_DISPLAY locations. When the display is enabled, the update is performed only for locations for which commons are active (depending on DUTY). For example if DUTY = 1/2, Note: only the LCD_DISPLAY of COM0 and COM1 are updated. Note: Writing 0 on this bit or writing 1 when it is already 1 has no effect. This bit can be cleared by hardware only. It can be cleared only when LCDEN = 1"]
pub type UdrR = crate::BitReader<Udr>;
impl UdrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Udr {
        match self.bits {
            false => Udr::B0x0,
            true => Udr::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Udr::B0x0
    }
    #[doc = "Update display request"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Udr::B0x1
    }
}
#[doc = "Field `UDR` writer - Update display request Each time software modifies the LCD_RAM, it must set this bit to transfer the updated data to the second level buffer. This bit stays set until the end of the update. During this time, the LCD_RAM is write protected. When the display is disabled, the update is performed for all LCD_DISPLAY locations. When the display is enabled, the update is performed only for locations for which commons are active (depending on DUTY). For example if DUTY = 1/2, Note: only the LCD_DISPLAY of COM0 and COM1 are updated. Note: Writing 0 on this bit or writing 1 when it is already 1 has no effect. This bit can be cleared by hardware only. It can be cleared only when LCDEN = 1"]
pub type UdrW<'a, REG> = crate::BitWriter<'a, REG, Udr>;
impl<'a, REG> UdrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Udr::B0x0)
    }
    #[doc = "Update display request"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Udr::B0x1)
    }
}
#[doc = "Update display done This bit is set by hardware. It is cleared by writing 1 to UDDC in LCD_CLR. The bit set has priority over the clear. Note: If the device is in Stop mode (PCLK not provided), UDD does not generate an interrupt even if UDDIE = 1. If the display is not enabled, the UDD interrupt never occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Udd {
    #[doc = "0: No event"]
    B0x0 = 0,
    #[doc = "1: Update display request done. A UDD interrupt is generated if UDDIE = 1 in LCD_FCR."]
    B0x1 = 1,
}
impl From<Udd> for bool {
    #[inline(always)]
    fn from(variant: Udd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDD` reader - Update display done This bit is set by hardware. It is cleared by writing 1 to UDDC in LCD_CLR. The bit set has priority over the clear. Note: If the device is in Stop mode (PCLK not provided), UDD does not generate an interrupt even if UDDIE = 1. If the display is not enabled, the UDD interrupt never occurs."]
pub type UddR = crate::BitReader<Udd>;
impl UddR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Udd {
        match self.bits {
            false => Udd::B0x0,
            true => Udd::B0x1,
        }
    }
    #[doc = "No event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Udd::B0x0
    }
    #[doc = "Update display request done. A UDD interrupt is generated if UDDIE = 1 in LCD_FCR."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Udd::B0x1
    }
}
#[doc = "Ready flag This bit is set and cleared by hardware. It indicates the status of the stepup converter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdy {
    #[doc = "0: Not ready"]
    B0x0 = 0,
    #[doc = "1: Stepup converter enabled and ready to provide the correct voltage"]
    B0x1 = 1,
}
impl From<Rdy> for bool {
    #[inline(always)]
    fn from(variant: Rdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY` reader - Ready flag This bit is set and cleared by hardware. It indicates the status of the stepup converter."]
pub type RdyR = crate::BitReader<Rdy>;
impl RdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdy {
        match self.bits {
            false => Rdy::B0x0,
            true => Rdy::B0x1,
        }
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rdy::B0x0
    }
    #[doc = "Stepup converter enabled and ready to provide the correct voltage"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rdy::B0x1
    }
}
#[doc = "LCD frame control register synchronization flag This bit is set by hardware each time the LCD_FCR register is updated in the LCDCLK domain. It is cleared by hardware when writing to the LCD_FCR register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcrsf {
    #[doc = "0: LCD frame control register not yet synchronized"]
    B0x0 = 0,
    #[doc = "1: LCD frame control register synchronized"]
    B0x1 = 1,
}
impl From<Fcrsf> for bool {
    #[inline(always)]
    fn from(variant: Fcrsf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCRSF` reader - LCD frame control register synchronization flag This bit is set by hardware each time the LCD_FCR register is updated in the LCDCLK domain. It is cleared by hardware when writing to the LCD_FCR register."]
pub type FcrsfR = crate::BitReader<Fcrsf>;
impl FcrsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcrsf {
        match self.bits {
            false => Fcrsf::B0x0,
            true => Fcrsf::B0x1,
        }
    }
    #[doc = "LCD frame control register not yet synchronized"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fcrsf::B0x0
    }
    #[doc = "LCD frame control register synchronized"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fcrsf::B0x1
    }
}
impl R {
    #[doc = "Bit 0 - LCD enabled status This bit is set and cleared by hardware. It indicates the LCD controller status. Note: This bit is set immediately when LCDEN in LCD_CR goes from 0 to 1. On deactivation, it reflects the real LCD status. It becomes 0 at the end of the last displayed frame."]
    #[inline(always)]
    pub fn ens(&self) -> EnsR {
        EnsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start-of-frame flag This bit is set by hardware at the beginning of a new frame, at the same time as the display data is updated. It is cleared by writing a 1 to SOFC in LCD_CLR. The bit clear has priority over the set."]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update display request Each time software modifies the LCD_RAM, it must set this bit to transfer the updated data to the second level buffer. This bit stays set until the end of the update. During this time, the LCD_RAM is write protected. When the display is disabled, the update is performed for all LCD_DISPLAY locations. When the display is enabled, the update is performed only for locations for which commons are active (depending on DUTY). For example if DUTY = 1/2, Note: only the LCD_DISPLAY of COM0 and COM1 are updated. Note: Writing 0 on this bit or writing 1 when it is already 1 has no effect. This bit can be cleared by hardware only. It can be cleared only when LCDEN = 1"]
    #[inline(always)]
    pub fn udr(&self) -> UdrR {
        UdrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Update display done This bit is set by hardware. It is cleared by writing 1 to UDDC in LCD_CLR. The bit set has priority over the clear. Note: If the device is in Stop mode (PCLK not provided), UDD does not generate an interrupt even if UDDIE = 1. If the display is not enabled, the UDD interrupt never occurs."]
    #[inline(always)]
    pub fn udd(&self) -> UddR {
        UddR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ready flag This bit is set and cleared by hardware. It indicates the status of the stepup converter."]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD frame control register synchronization flag This bit is set by hardware each time the LCD_FCR register is updated in the LCDCLK domain. It is cleared by hardware when writing to the LCD_FCR register."]
    #[inline(always)]
    pub fn fcrsf(&self) -> FcrsfR {
        FcrsfR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Update display request Each time software modifies the LCD_RAM, it must set this bit to transfer the updated data to the second level buffer. This bit stays set until the end of the update. During this time, the LCD_RAM is write protected. When the display is disabled, the update is performed for all LCD_DISPLAY locations. When the display is enabled, the update is performed only for locations for which commons are active (depending on DUTY). For example if DUTY = 1/2, Note: only the LCD_DISPLAY of COM0 and COM1 are updated. Note: Writing 0 on this bit or writing 1 when it is already 1 has no effect. This bit can be cleared by hardware only. It can be cleared only when LCDEN = 1"]
    #[inline(always)]
    #[must_use]
    pub fn udr(&mut self) -> UdrW<LcdSrSpec> {
        UdrW::new(self, 2)
    }
}
#[doc = "LCD status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdSrSpec;
impl crate::RegisterSpec for LcdSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_sr::R`](R) reader structure"]
impl crate::Readable for LcdSrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd_sr::W`](W) writer structure"]
impl crate::Writable for LcdSrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCD_SR to value 0x20"]
impl crate::Resettable for LcdSrSpec {
    const RESET_VALUE: u32 = 0x20;
}
