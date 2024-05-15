#[doc = "Register `SYSCFG_SCSR` reader"]
pub type R = crate::R<SyscfgScsrSpec>;
#[doc = "Register `SYSCFG_SCSR` writer"]
pub type W = crate::W<SyscfgScsrSpec>;
#[doc = "Field `SRAM2ER` reader - SRAM2 erase Setting this bit starts a hardware SRAM2 erase operation. This bit is automatically cleared at the end of the SRAM2 erase operation. Note: This bit is write-protected: setting this bit is possible only after the correct key sequence is written in the SYSCFG_SKR register."]
pub type Sram2erR = crate::BitReader;
#[doc = "Field `SRAM2ER` writer - SRAM2 erase Setting this bit starts a hardware SRAM2 erase operation. This bit is automatically cleared at the end of the SRAM2 erase operation. Note: This bit is write-protected: setting this bit is possible only after the correct key sequence is written in the SYSCFG_SKR register."]
pub type Sram2erW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SRAM2 busy by erase operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram2bsy {
    #[doc = "0: No SRAM2 erase operation is ongoing"]
    B0x0 = 0,
    #[doc = "1: SRAM2 erase operation is ongoing"]
    B0x1 = 1,
}
impl From<Sram2bsy> for bool {
    #[inline(always)]
    fn from(variant: Sram2bsy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2BSY` reader - SRAM2 busy by erase operation"]
pub type Sram2bsyR = crate::BitReader<Sram2bsy>;
impl Sram2bsyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram2bsy {
        match self.bits {
            false => Sram2bsy::B0x0,
            true => Sram2bsy::B0x1,
        }
    }
    #[doc = "No SRAM2 erase operation is ongoing"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sram2bsy::B0x0
    }
    #[doc = "SRAM2 erase operation is ongoing"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sram2bsy::B0x1
    }
}
impl R {
    #[doc = "Bit 0 - SRAM2 erase Setting this bit starts a hardware SRAM2 erase operation. This bit is automatically cleared at the end of the SRAM2 erase operation. Note: This bit is write-protected: setting this bit is possible only after the correct key sequence is written in the SYSCFG_SKR register."]
    #[inline(always)]
    pub fn sram2er(&self) -> Sram2erR {
        Sram2erR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM2 busy by erase operation"]
    #[inline(always)]
    pub fn sram2bsy(&self) -> Sram2bsyR {
        Sram2bsyR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM2 erase Setting this bit starts a hardware SRAM2 erase operation. This bit is automatically cleared at the end of the SRAM2 erase operation. Note: This bit is write-protected: setting this bit is possible only after the correct key sequence is written in the SYSCFG_SKR register."]
    #[inline(always)]
    #[must_use]
    pub fn sram2er(&mut self) -> Sram2erW<SyscfgScsrSpec> {
        Sram2erW::new(self, 0)
    }
}
#[doc = "SYSCFG SRAM2 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_scsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_scsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgScsrSpec;
impl crate::RegisterSpec for SyscfgScsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_scsr::R`](R) reader structure"]
impl crate::Readable for SyscfgScsrSpec {}
#[doc = "`write(|w| ..)` method takes [`syscfg_scsr::W`](W) writer structure"]
impl crate::Writable for SyscfgScsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCFG_SCSR to value 0"]
impl crate::Resettable for SyscfgScsrSpec {
    const RESET_VALUE: u32 = 0;
}
