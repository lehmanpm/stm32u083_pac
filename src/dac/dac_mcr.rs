#[doc = "Register `DAC_MCR` reader"]
pub type R = crate::R<DacMcrSpec>;
#[doc = "Register `DAC_MCR` writer"]
pub type W = crate::W<DacMcrSpec>;
#[doc = "DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN11=10 and bit CEN11=10 in the DAC_CR register). If EN11=11 or CEN11=11 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample &amp; hold mode Note: This register can be modified only when EN11=10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode1 {
    #[doc = "0: DAC channel1 is connected to external pin with Buffer enabled"]
    B0x0 = 0,
    #[doc = "1: DAC channel1 is connected to external pin and to on chip peripherals with Buffer enabled"]
    B0x1 = 1,
    #[doc = "2: DAC channel1 is connected to external pin with Buffer disabled"]
    B0x2 = 2,
    #[doc = "3: DAC channel1 is connected to on chip peripherals with Buffer disabled"]
    B0x3 = 3,
    #[doc = "4: DAC channel1 is connected to external pin with Buffer enabled"]
    B0x4 = 4,
    #[doc = "5: DAC channel1 is connected to external pin and to on chip peripherals with Buffer enabled"]
    B0x5 = 5,
    #[doc = "6: DAC channel1 is connected to external pin and to on chip peripherals with Buffer disabled"]
    B0x6 = 6,
    #[doc = "7: DAC channel1 is connected to on chip peripherals with Buffer disabled"]
    B0x7 = 7,
}
impl From<Mode1> for u8 {
    #[inline(always)]
    fn from(variant: Mode1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode1 {
    type Ux = u8;
}
impl crate::IsEnum for Mode1 {}
#[doc = "Field `MODE1` reader - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN11=10 and bit CEN11=10 in the DAC_CR register). If EN11=11 or CEN11=11 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample &amp; hold mode Note: This register can be modified only when EN11=10."]
pub type Mode1R = crate::FieldReader<Mode1>;
impl Mode1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode1 {
        match self.bits {
            0 => Mode1::B0x0,
            1 => Mode1::B0x1,
            2 => Mode1::B0x2,
            3 => Mode1::B0x3,
            4 => Mode1::B0x4,
            5 => Mode1::B0x5,
            6 => Mode1::B0x6,
            7 => Mode1::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "DAC channel1 is connected to external pin with Buffer enabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mode1::B0x0
    }
    #[doc = "DAC channel1 is connected to external pin and to on chip peripherals with Buffer enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mode1::B0x1
    }
    #[doc = "DAC channel1 is connected to external pin with Buffer disabled"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mode1::B0x2
    }
    #[doc = "DAC channel1 is connected to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mode1::B0x3
    }
    #[doc = "DAC channel1 is connected to external pin with Buffer enabled"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Mode1::B0x4
    }
    #[doc = "DAC channel1 is connected to external pin and to on chip peripherals with Buffer enabled"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Mode1::B0x5
    }
    #[doc = "DAC channel1 is connected to external pin and to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Mode1::B0x6
    }
    #[doc = "DAC channel1 is connected to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Mode1::B0x7
    }
}
#[doc = "Field `MODE1` writer - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN11=10 and bit CEN11=10 in the DAC_CR register). If EN11=11 or CEN11=11 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample &amp; hold mode Note: This register can be modified only when EN11=10."]
pub type Mode1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Mode1, crate::Safe>;
impl<'a, REG> Mode1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DAC channel1 is connected to external pin with Buffer enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::B0x0)
    }
    #[doc = "DAC channel1 is connected to external pin and to on chip peripherals with Buffer enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::B0x1)
    }
    #[doc = "DAC channel1 is connected to external pin with Buffer disabled"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::B0x2)
    }
    #[doc = "DAC channel1 is connected to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::B0x3)
    }
    #[doc = "DAC channel1 is connected to external pin with Buffer enabled"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::B0x4)
    }
    #[doc = "DAC channel1 is connected to external pin and to on chip peripherals with Buffer enabled"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::B0x5)
    }
    #[doc = "DAC channel1 is connected to external pin and to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::B0x6)
    }
    #[doc = "DAC channel1 is connected to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::B0x7)
    }
}
impl R {
    #[doc = "Bits 0:2 - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN11=10 and bit CEN11=10 in the DAC_CR register). If EN11=11 or CEN11=11 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample &amp; hold mode Note: This register can be modified only when EN11=10."]
    #[inline(always)]
    pub fn mode1(&self) -> Mode1R {
        Mode1R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN11=10 and bit CEN11=10 in the DAC_CR register). If EN11=11 or CEN11=11 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample &amp; hold mode Note: This register can be modified only when EN11=10."]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> Mode1W<DacMcrSpec> {
        Mode1W::new(self, 0)
    }
}
#[doc = "DAC mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacMcrSpec;
impl crate::RegisterSpec for DacMcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_mcr::R`](R) reader structure"]
impl crate::Readable for DacMcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_mcr::W`](W) writer structure"]
impl crate::Writable for DacMcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_MCR to value 0"]
impl crate::Resettable for DacMcrSpec {
    const RESET_VALUE: u32 = 0;
}
