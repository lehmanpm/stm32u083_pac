#[doc = "Register `TIM6_DIER` reader"]
pub type R = crate::R<Tim6DierSpec>;
#[doc = "Register `TIM6_DIER` writer"]
pub type W = crate::W<Tim6DierSpec>;
#[doc = "Update interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uie {
    #[doc = "0: Update interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: Update interrupt enabled."]
    B0x1 = 1,
}
impl From<Uie> for bool {
    #[inline(always)]
    fn from(variant: Uie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIE` reader - Update interrupt enable"]
pub type UieR = crate::BitReader<Uie>;
impl UieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uie {
        match self.bits {
            false => Uie::B0x0,
            true => Uie::B0x1,
        }
    }
    #[doc = "Update interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Uie::B0x0
    }
    #[doc = "Update interrupt enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Uie::B0x1
    }
}
#[doc = "Field `UIE` writer - Update interrupt enable"]
pub type UieW<'a, REG> = crate::BitWriter<'a, REG, Uie>;
impl<'a, REG> UieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Uie::B0x0)
    }
    #[doc = "Update interrupt enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Uie::B0x1)
    }
}
#[doc = "Update DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ude {
    #[doc = "0: Update DMA request disabled."]
    B0x0 = 0,
    #[doc = "1: Update DMA request enabled."]
    B0x1 = 1,
}
impl From<Ude> for bool {
    #[inline(always)]
    fn from(variant: Ude) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDE` reader - Update DMA request enable"]
pub type UdeR = crate::BitReader<Ude>;
impl UdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ude {
        match self.bits {
            false => Ude::B0x0,
            true => Ude::B0x1,
        }
    }
    #[doc = "Update DMA request disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ude::B0x0
    }
    #[doc = "Update DMA request enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ude::B0x1
    }
}
#[doc = "Field `UDE` writer - Update DMA request enable"]
pub type UdeW<'a, REG> = crate::BitWriter<'a, REG, Ude>;
impl<'a, REG> UdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update DMA request disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ude::B0x0)
    }
    #[doc = "Update DMA request enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ude::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uie(&self) -> UieR {
        UieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn ude(&self) -> UdeR {
        UdeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UieW<Tim6DierSpec> {
        UieW::new(self, 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ude(&mut self) -> UdeW<Tim6DierSpec> {
        UdeW::new(self, 8)
    }
}
#[doc = "TIM6 DMA/Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim6_dier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim6_dier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim6DierSpec;
impl crate::RegisterSpec for Tim6DierSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim6_dier::R`](R) reader structure"]
impl crate::Readable for Tim6DierSpec {}
#[doc = "`write(|w| ..)` method takes [`tim6_dier::W`](W) writer structure"]
impl crate::Writable for Tim6DierSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM6_DIER to value 0"]
impl crate::Resettable for Tim6DierSpec {
    const RESET_VALUE: u16 = 0;
}
