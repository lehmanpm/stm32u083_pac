#[doc = "Register `SYSCFG_CFGR2` reader"]
pub type R = crate::R<SyscfgCfgr2Spec>;
#[doc = "Register `SYSCFG_CFGR2` writer"]
pub type W = crate::W<SyscfgCfgr2Spec>;
#[doc = "Cortex&lt;Superscript>1&lt;Default 1 Font>-M0+ LOCKUP bit enable bit This bit is set by software and cleared by a system reset. It can be use to enable and lock the connection of Cortex&lt;Superscript>1&lt;Default 1 Font>-M0+ LOCKUP (Hardfault) output to TIM1/15/16 Break input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccl {
    #[doc = "0: Cortex&lt;Superscript>1&lt;Default 1 Font>-M0+ LOCKUP output disconnected from TIM1/15/16 Break input"]
    B0x0 = 0,
    #[doc = "1: Cortex&lt;Superscript>1&lt;Default 1 Font>-M0+ LOCKUP output connected to TIM1/15/16 Break input"]
    B0x1 = 1,
}
impl From<Ccl> for bool {
    #[inline(always)]
    fn from(variant: Ccl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCL` reader - Cortex&lt;Superscript>1&lt;Default 1 Font>-M0+ LOCKUP bit enable bit This bit is set by software and cleared by a system reset. It can be use to enable and lock the connection of Cortex&lt;Superscript>1&lt;Default 1 Font>-M0+ LOCKUP (Hardfault) output to TIM1/15/16 Break input."]
pub type CclR = crate::BitReader<Ccl>;
impl CclR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccl {
        match self.bits {
            false => Ccl::B0x0,
            true => Ccl::B0x1,
        }
    }
    #[doc = "Cortex&lt;Superscript>1&lt;Default 1 Font>-M0+ LOCKUP output disconnected from TIM1/15/16 Break input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ccl::B0x0
    }
    #[doc = "Cortex&lt;Superscript>1&lt;Default 1 Font>-M0+ LOCKUP output connected to TIM1/15/16 Break input"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ccl::B0x1
    }
}
#[doc = "Field `CCL` writer - Cortex&lt;Superscript>1&lt;Default 1 Font>-M0+ LOCKUP bit enable bit This bit is set by software and cleared by a system reset. It can be use to enable and lock the connection of Cortex&lt;Superscript>1&lt;Default 1 Font>-M0+ LOCKUP (Hardfault) output to TIM1/15/16 Break input."]
pub type CclW<'a, REG> = crate::BitWriter<'a, REG, Ccl>;
impl<'a, REG> CclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cortex&lt;Superscript>1&lt;Default 1 Font>-M0+ LOCKUP output disconnected from TIM1/15/16 Break input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccl::B0x0)
    }
    #[doc = "Cortex&lt;Superscript>1&lt;Default 1 Font>-M0+ LOCKUP output connected to TIM1/15/16 Break input"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccl::B0x1)
    }
}
#[doc = "SRAM1 parity lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM1 parity error signal connection to TIM1/15/16 Break input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spl {
    #[doc = "0: SRAM1 parity error disconnected from TIM1/15/16 Break input"]
    B0x0 = 0,
    #[doc = "1: SRAM1 parity error connected to TIM1/15/16 Break input"]
    B0x1 = 1,
}
impl From<Spl> for bool {
    #[inline(always)]
    fn from(variant: Spl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPL` reader - SRAM1 parity lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM1 parity error signal connection to TIM1/15/16 Break input."]
pub type SplR = crate::BitReader<Spl>;
impl SplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spl {
        match self.bits {
            false => Spl::B0x0,
            true => Spl::B0x1,
        }
    }
    #[doc = "SRAM1 parity error disconnected from TIM1/15/16 Break input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spl::B0x0
    }
    #[doc = "SRAM1 parity error connected to TIM1/15/16 Break input"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spl::B0x1
    }
}
#[doc = "Field `SPL` writer - SRAM1 parity lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM1 parity error signal connection to TIM1/15/16 Break input."]
pub type SplW<'a, REG> = crate::BitWriter<'a, REG, Spl>;
impl<'a, REG> SplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM1 parity error disconnected from TIM1/15/16 Break input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spl::B0x0)
    }
    #[doc = "SRAM1 parity error connected to TIM1/15/16 Break input"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spl::B0x1)
    }
}
#[doc = "PVD lock enable bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the PVD connection to TIM1/15/16 Break input, as well as the PVDE and PLS\\[2:0\\]
in the PWR_CR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pvdl {
    #[doc = "0: PVD interrupt disconnected from TIM1/15/16 Break input. PVDE and PLS\\[2:0\\]
bits can be programmed by the application."]
    B0x0 = 0,
    #[doc = "1: PVD interrupt connected to TIM1/15/16 Break input, PVDE and PLS\\[2:0\\]
bits are read only."]
    B0x1 = 1,
}
impl From<Pvdl> for bool {
    #[inline(always)]
    fn from(variant: Pvdl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDL` reader - PVD lock enable bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the PVD connection to TIM1/15/16 Break input, as well as the PVDE and PLS\\[2:0\\]
in the PWR_CR register."]
pub type PvdlR = crate::BitReader<Pvdl>;
impl PvdlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pvdl {
        match self.bits {
            false => Pvdl::B0x0,
            true => Pvdl::B0x1,
        }
    }
    #[doc = "PVD interrupt disconnected from TIM1/15/16 Break input. PVDE and PLS\\[2:0\\]
bits can be programmed by the application."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pvdl::B0x0
    }
    #[doc = "PVD interrupt connected to TIM1/15/16 Break input, PVDE and PLS\\[2:0\\]
bits are read only."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pvdl::B0x1
    }
}
#[doc = "Field `PVDL` writer - PVD lock enable bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the PVD connection to TIM1/15/16 Break input, as well as the PVDE and PLS\\[2:0\\]
in the PWR_CR register."]
pub type PvdlW<'a, REG> = crate::BitWriter<'a, REG, Pvdl>;
impl<'a, REG> PvdlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PVD interrupt disconnected from TIM1/15/16 Break input. PVDE and PLS\\[2:0\\]
bits can be programmed by the application."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pvdl::B0x0)
    }
    #[doc = "PVD interrupt connected to TIM1/15/16 Break input, PVDE and PLS\\[2:0\\]
bits are read only."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pvdl::B0x1)
    }
}
#[doc = "ECC error lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the flash ECC 2-bit error detection signal connection to TIM1/15/16 Break input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eccl {
    #[doc = "0: ECC error disconnected from TIM1/15/16 Break input"]
    B0x0 = 0,
    #[doc = "1: ECC error connected to TIM1/15/16 Break input"]
    B0x1 = 1,
}
impl From<Eccl> for bool {
    #[inline(always)]
    fn from(variant: Eccl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCL` reader - ECC error lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the flash ECC 2-bit error detection signal connection to TIM1/15/16 Break input."]
pub type EcclR = crate::BitReader<Eccl>;
impl EcclR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eccl {
        match self.bits {
            false => Eccl::B0x0,
            true => Eccl::B0x1,
        }
    }
    #[doc = "ECC error disconnected from TIM1/15/16 Break input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eccl::B0x0
    }
    #[doc = "ECC error connected to TIM1/15/16 Break input"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eccl::B0x1
    }
}
#[doc = "Field `ECCL` writer - ECC error lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the flash ECC 2-bit error detection signal connection to TIM1/15/16 Break input."]
pub type EcclW<'a, REG> = crate::BitWriter<'a, REG, Eccl>;
impl<'a, REG> EcclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECC error disconnected from TIM1/15/16 Break input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eccl::B0x0)
    }
    #[doc = "ECC error connected to TIM1/15/16 Break input"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eccl::B0x1)
    }
}
#[doc = "Backup SRAM2 parity lock This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM2 parity error signal connection to TIM1/15/16 Break input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bkpl {
    #[doc = "0: SRAM2 parity error disconnected from TIM1/15/16 Break input"]
    B0x0 = 0,
    #[doc = "1: SRAM2 parity error connected to TIM1/15/16 Break input"]
    B0x1 = 1,
}
impl From<Bkpl> for bool {
    #[inline(always)]
    fn from(variant: Bkpl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKPL` reader - Backup SRAM2 parity lock This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM2 parity error signal connection to TIM1/15/16 Break input."]
pub type BkplR = crate::BitReader<Bkpl>;
impl BkplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bkpl {
        match self.bits {
            false => Bkpl::B0x0,
            true => Bkpl::B0x1,
        }
    }
    #[doc = "SRAM2 parity error disconnected from TIM1/15/16 Break input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bkpl::B0x0
    }
    #[doc = "SRAM2 parity error connected to TIM1/15/16 Break input"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bkpl::B0x1
    }
}
#[doc = "Field `BKPL` writer - Backup SRAM2 parity lock This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM2 parity error signal connection to TIM1/15/16 Break input."]
pub type BkplW<'a, REG> = crate::BitWriter<'a, REG, Bkpl>;
impl<'a, REG> BkplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM2 parity error disconnected from TIM1/15/16 Break input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bkpl::B0x0)
    }
    #[doc = "SRAM2 parity error connected to TIM1/15/16 Break input"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bkpl::B0x1)
    }
}
#[doc = "Backup SRAM2 parity error flag This bit is set by hardware when an SRAM2 parity error is detected. It is cleared by software by writing 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bkpf {
    #[doc = "0: No SRAM2 parity error detected"]
    B0x0 = 0,
    #[doc = "1: SRAM2 parity error detected"]
    B0x1 = 1,
}
impl From<Bkpf> for bool {
    #[inline(always)]
    fn from(variant: Bkpf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKPF` reader - Backup SRAM2 parity error flag This bit is set by hardware when an SRAM2 parity error is detected. It is cleared by software by writing 1."]
pub type BkpfR = crate::BitReader<Bkpf>;
impl BkpfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bkpf {
        match self.bits {
            false => Bkpf::B0x0,
            true => Bkpf::B0x1,
        }
    }
    #[doc = "No SRAM2 parity error detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bkpf::B0x0
    }
    #[doc = "SRAM2 parity error detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bkpf::B0x1
    }
}
#[doc = "Field `BKPF` writer - Backup SRAM2 parity error flag This bit is set by hardware when an SRAM2 parity error is detected. It is cleared by software by writing 1."]
pub type BkpfW<'a, REG> = crate::BitWriter<'a, REG, Bkpf>;
impl<'a, REG> BkpfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SRAM2 parity error detected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bkpf::B0x0)
    }
    #[doc = "SRAM2 parity error detected"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bkpf::B0x1)
    }
}
#[doc = "SRAM1 parity error flag This bit is set by hardware when an SRAM1 parity error is detected. It is cleared by software by writing 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spf {
    #[doc = "0: No SRAM1 parity error detected"]
    B0x0 = 0,
    #[doc = "1: SRAM1 parity error detected"]
    B0x1 = 1,
}
impl From<Spf> for bool {
    #[inline(always)]
    fn from(variant: Spf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPF` reader - SRAM1 parity error flag This bit is set by hardware when an SRAM1 parity error is detected. It is cleared by software by writing 1."]
pub type SpfR = crate::BitReader<Spf>;
impl SpfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spf {
        match self.bits {
            false => Spf::B0x0,
            true => Spf::B0x1,
        }
    }
    #[doc = "No SRAM1 parity error detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spf::B0x0
    }
    #[doc = "SRAM1 parity error detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spf::B0x1
    }
}
#[doc = "Field `SPF` writer - SRAM1 parity error flag This bit is set by hardware when an SRAM1 parity error is detected. It is cleared by software by writing 1."]
pub type SpfW<'a, REG> = crate::BitWriter<'a, REG, Spf>;
impl<'a, REG> SpfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SRAM1 parity error detected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spf::B0x0)
    }
    #[doc = "SRAM1 parity error detected"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spf::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Cortex&lt;Superscript>1&lt;Default 1 Font>-M0+ LOCKUP bit enable bit This bit is set by software and cleared by a system reset. It can be use to enable and lock the connection of Cortex&lt;Superscript>1&lt;Default 1 Font>-M0+ LOCKUP (Hardfault) output to TIM1/15/16 Break input."]
    #[inline(always)]
    pub fn ccl(&self) -> CclR {
        CclR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM1 parity lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM1 parity error signal connection to TIM1/15/16 Break input."]
    #[inline(always)]
    pub fn spl(&self) -> SplR {
        SplR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD lock enable bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the PVD connection to TIM1/15/16 Break input, as well as the PVDE and PLS\\[2:0\\]
in the PWR_CR register."]
    #[inline(always)]
    pub fn pvdl(&self) -> PvdlR {
        PvdlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC error lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the flash ECC 2-bit error detection signal connection to TIM1/15/16 Break input."]
    #[inline(always)]
    pub fn eccl(&self) -> EcclR {
        EcclR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Backup SRAM2 parity lock This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM2 parity error signal connection to TIM1/15/16 Break input."]
    #[inline(always)]
    pub fn bkpl(&self) -> BkplR {
        BkplR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Backup SRAM2 parity error flag This bit is set by hardware when an SRAM2 parity error is detected. It is cleared by software by writing 1."]
    #[inline(always)]
    pub fn bkpf(&self) -> BkpfR {
        BkpfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM1 parity error flag This bit is set by hardware when an SRAM1 parity error is detected. It is cleared by software by writing 1."]
    #[inline(always)]
    pub fn spf(&self) -> SpfR {
        SpfR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex&lt;Superscript>1&lt;Default 1 Font>-M0+ LOCKUP bit enable bit This bit is set by software and cleared by a system reset. It can be use to enable and lock the connection of Cortex&lt;Superscript>1&lt;Default 1 Font>-M0+ LOCKUP (Hardfault) output to TIM1/15/16 Break input."]
    #[inline(always)]
    #[must_use]
    pub fn ccl(&mut self) -> CclW<SyscfgCfgr2Spec> {
        CclW::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM1 parity lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM1 parity error signal connection to TIM1/15/16 Break input."]
    #[inline(always)]
    #[must_use]
    pub fn spl(&mut self) -> SplW<SyscfgCfgr2Spec> {
        SplW::new(self, 1)
    }
    #[doc = "Bit 2 - PVD lock enable bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the PVD connection to TIM1/15/16 Break input, as well as the PVDE and PLS\\[2:0\\]
in the PWR_CR register."]
    #[inline(always)]
    #[must_use]
    pub fn pvdl(&mut self) -> PvdlW<SyscfgCfgr2Spec> {
        PvdlW::new(self, 2)
    }
    #[doc = "Bit 3 - ECC error lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the flash ECC 2-bit error detection signal connection to TIM1/15/16 Break input."]
    #[inline(always)]
    #[must_use]
    pub fn eccl(&mut self) -> EcclW<SyscfgCfgr2Spec> {
        EcclW::new(self, 3)
    }
    #[doc = "Bit 4 - Backup SRAM2 parity lock This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM2 parity error signal connection to TIM1/15/16 Break input."]
    #[inline(always)]
    #[must_use]
    pub fn bkpl(&mut self) -> BkplW<SyscfgCfgr2Spec> {
        BkplW::new(self, 4)
    }
    #[doc = "Bit 7 - Backup SRAM2 parity error flag This bit is set by hardware when an SRAM2 parity error is detected. It is cleared by software by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn bkpf(&mut self) -> BkpfW<SyscfgCfgr2Spec> {
        BkpfW::new(self, 7)
    }
    #[doc = "Bit 8 - SRAM1 parity error flag This bit is set by hardware when an SRAM1 parity error is detected. It is cleared by software by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn spf(&mut self) -> SpfW<SyscfgCfgr2Spec> {
        SpfW::new(self, 8)
    }
}
#[doc = "SYSCFG configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgCfgr2Spec;
impl crate::RegisterSpec for SyscfgCfgr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_cfgr2::R`](R) reader structure"]
impl crate::Readable for SyscfgCfgr2Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg_cfgr2::W`](W) writer structure"]
impl crate::Writable for SyscfgCfgr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCFG_CFGR2 to value 0"]
impl crate::Resettable for SyscfgCfgr2Spec {
    const RESET_VALUE: u32 = 0;
}
