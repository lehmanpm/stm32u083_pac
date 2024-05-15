#[doc = "Register `WWDG_CFR` reader"]
pub type R = crate::R<WwdgCfrSpec>;
#[doc = "Register `WWDG_CFR` writer"]
pub type W = crate::W<WwdgCfrSpec>;
#[doc = "Field `W` reader - 7-bit window value These bits contain the window value to be compared with the down-counter."]
pub type WR = crate::FieldReader;
#[doc = "Field `W` writer - 7-bit window value These bits contain the window value to be compared with the down-counter."]
pub type WW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EWI` reader - Early wake-up interrupt enable Set by software and cleared by hardware after a reset. When set, an interrupt occurs whenever the counter reaches the value 0x40."]
pub type EwiR = crate::BitReader;
#[doc = "Field `EWI` writer - Early wake-up interrupt enable Set by software and cleared by hardware after a reset. When set, an interrupt occurs whenever the counter reaches the value 0x40."]
pub type EwiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timer base The timebase of the prescaler can be modified as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wdgtb {
    #[doc = "0: CK counter clock (PCLK div 4096) div 1"]
    B0x0 = 0,
    #[doc = "1: CK counter clock (PCLK div 4096) div 2"]
    B0x1 = 1,
    #[doc = "2: CK counter clock (PCLK div 4096) div 4"]
    B0x2 = 2,
    #[doc = "3: CK counter clock (PCLK div 4096) div 8"]
    B0x3 = 3,
    #[doc = "4: CK counter clock (PCLK div 4096) div 16"]
    B0x4 = 4,
    #[doc = "5: CK counter clock (PCLK div 4096) div 32"]
    B0x5 = 5,
    #[doc = "6: CK counter clock (PCLK div 4096) div 64"]
    B0x6 = 6,
    #[doc = "7: CK counter clock (PCLK div 4096) div 128"]
    B0x7 = 7,
}
impl From<Wdgtb> for u8 {
    #[inline(always)]
    fn from(variant: Wdgtb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wdgtb {
    type Ux = u8;
}
impl crate::IsEnum for Wdgtb {}
#[doc = "Field `WDGTB` reader - Timer base The timebase of the prescaler can be modified as follows:"]
pub type WdgtbR = crate::FieldReader<Wdgtb>;
impl WdgtbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdgtb {
        match self.bits {
            0 => Wdgtb::B0x0,
            1 => Wdgtb::B0x1,
            2 => Wdgtb::B0x2,
            3 => Wdgtb::B0x3,
            4 => Wdgtb::B0x4,
            5 => Wdgtb::B0x5,
            6 => Wdgtb::B0x6,
            7 => Wdgtb::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "CK counter clock (PCLK div 4096) div 1"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wdgtb::B0x0
    }
    #[doc = "CK counter clock (PCLK div 4096) div 2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wdgtb::B0x1
    }
    #[doc = "CK counter clock (PCLK div 4096) div 4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Wdgtb::B0x2
    }
    #[doc = "CK counter clock (PCLK div 4096) div 8"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Wdgtb::B0x3
    }
    #[doc = "CK counter clock (PCLK div 4096) div 16"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Wdgtb::B0x4
    }
    #[doc = "CK counter clock (PCLK div 4096) div 32"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Wdgtb::B0x5
    }
    #[doc = "CK counter clock (PCLK div 4096) div 64"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Wdgtb::B0x6
    }
    #[doc = "CK counter clock (PCLK div 4096) div 128"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Wdgtb::B0x7
    }
}
#[doc = "Field `WDGTB` writer - Timer base The timebase of the prescaler can be modified as follows:"]
pub type WdgtbW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wdgtb, crate::Safe>;
impl<'a, REG> WdgtbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CK counter clock (PCLK div 4096) div 1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdgtb::B0x0)
    }
    #[doc = "CK counter clock (PCLK div 4096) div 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdgtb::B0x1)
    }
    #[doc = "CK counter clock (PCLK div 4096) div 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Wdgtb::B0x2)
    }
    #[doc = "CK counter clock (PCLK div 4096) div 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Wdgtb::B0x3)
    }
    #[doc = "CK counter clock (PCLK div 4096) div 16"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Wdgtb::B0x4)
    }
    #[doc = "CK counter clock (PCLK div 4096) div 32"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Wdgtb::B0x5)
    }
    #[doc = "CK counter clock (PCLK div 4096) div 64"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Wdgtb::B0x6)
    }
    #[doc = "CK counter clock (PCLK div 4096) div 128"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Wdgtb::B0x7)
    }
}
impl R {
    #[doc = "Bits 0:6 - 7-bit window value These bits contain the window value to be compared with the down-counter."]
    #[inline(always)]
    pub fn w(&self) -> WR {
        WR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 9 - Early wake-up interrupt enable Set by software and cleared by hardware after a reset. When set, an interrupt occurs whenever the counter reaches the value 0x40."]
    #[inline(always)]
    pub fn ewi(&self) -> EwiR {
        EwiR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Timer base The timebase of the prescaler can be modified as follows:"]
    #[inline(always)]
    pub fn wdgtb(&self) -> WdgtbR {
        WdgtbR::new(((self.bits >> 11) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit window value These bits contain the window value to be compared with the down-counter."]
    #[inline(always)]
    #[must_use]
    pub fn w(&mut self) -> WW<WwdgCfrSpec> {
        WW::new(self, 0)
    }
    #[doc = "Bit 9 - Early wake-up interrupt enable Set by software and cleared by hardware after a reset. When set, an interrupt occurs whenever the counter reaches the value 0x40."]
    #[inline(always)]
    #[must_use]
    pub fn ewi(&mut self) -> EwiW<WwdgCfrSpec> {
        EwiW::new(self, 9)
    }
    #[doc = "Bits 11:13 - Timer base The timebase of the prescaler can be modified as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn wdgtb(&mut self) -> WdgtbW<WwdgCfrSpec> {
        WdgtbW::new(self, 11)
    }
}
#[doc = "WWDG configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_cfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wwdg_cfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WwdgCfrSpec;
impl crate::RegisterSpec for WwdgCfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdg_cfr::R`](R) reader structure"]
impl crate::Readable for WwdgCfrSpec {}
#[doc = "`write(|w| ..)` method takes [`wwdg_cfr::W`](W) writer structure"]
impl crate::Writable for WwdgCfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WWDG_CFR to value 0x7f"]
impl crate::Resettable for WwdgCfrSpec {
    const RESET_VALUE: u32 = 0x7f;
}
