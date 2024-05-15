#[doc = "Register `FLASH_SR` reader"]
pub type R = crate::R<FlashSrSpec>;
#[doc = "Register `FLASH_SR` writer"]
pub type W = crate::W<FlashSrSpec>;
#[doc = "Field `EOP` reader - End of operation Set by hardware when one or more flash memory operation (programming / erase) has been completed successfully. This bit is set only if the end of operation interrupts are enabled (EOPIE=1). Cleared by writing 1."]
pub type EopR = crate::BitReader;
#[doc = "Field `EOP` writer - End of operation Set by hardware when one or more flash memory operation (programming / erase) has been completed successfully. This bit is set only if the end of operation interrupts are enabled (EOPIE=1). Cleared by writing 1."]
pub type EopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPERR` reader - Operation error Set by hardware when a flash memory operation (program / erase) completes unsuccessfully. This bit is set only if error interrupts are enabled (ERRIE=1). Cleared by writing 1."]
pub type OperrR = crate::BitReader;
#[doc = "Field `OPERR` writer - Operation error Set by hardware when a flash memory operation (program / erase) completes unsuccessfully. This bit is set only if error interrupts are enabled (ERRIE=1). Cleared by writing 1."]
pub type OperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROGERR` reader - Programming error Set by hardware when a double-word address to be programmed contains a value different from '0xFFFF FFFF' before programming, except if the data to write is '0x0000 0000'. Cleared by writing 1."]
pub type ProgerrR = crate::BitReader;
#[doc = "Field `PROGERR` writer - Programming error Set by hardware when a double-word address to be programmed contains a value different from '0xFFFF FFFF' before programming, except if the data to write is '0x0000 0000'. Cleared by writing 1."]
pub type ProgerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPERR` reader - Write protection error Set by hardware when an address to be erased/programmed belongs to a write-protected part (by WRP, PCROP or RDP Level 1) of the flash memory. Cleared by writing 1."]
pub type WrperrR = crate::BitReader;
#[doc = "Field `WRPERR` writer - Write protection error Set by hardware when an address to be erased/programmed belongs to a write-protected part (by WRP, PCROP or RDP Level 1) of the flash memory. Cleared by writing 1."]
pub type WrperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGAERR` reader - Programming alignment error Set by hardware when the data to program cannot be contained in the same double word (64-bit) flash memory in case of standard programming, or if there is a change of page during fast programming. Cleared by writing 1."]
pub type PgaerrR = crate::BitReader;
#[doc = "Field `PGAERR` writer - Programming alignment error Set by hardware when the data to program cannot be contained in the same double word (64-bit) flash memory in case of standard programming, or if there is a change of page during fast programming. Cleared by writing 1."]
pub type PgaerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIZERR` reader - Size error Set by hardware when the size of the access is a byte or half-word during a program or a fast program sequence. Only double word programming is allowed (consequently: word access). Cleared by writing 1."]
pub type SizerrR = crate::BitReader;
#[doc = "Field `SIZERR` writer - Size error Set by hardware when the size of the access is a byte or half-word during a program or a fast program sequence. Only double word programming is allowed (consequently: word access). Cleared by writing 1."]
pub type SizerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGSERR` reader - Programming sequence error Set by hardware when a write access to the flash memory is performed by the code while PG or FSTPG have not been set previously. Set also by hardware when PROGERR, SIZERR, PGAERR, WRPERR, MISSERR or FASTERR is set due to a previous programming error. Cleared by writing 1."]
pub type PgserrR = crate::BitReader;
#[doc = "Field `PGSERR` writer - Programming sequence error Set by hardware when a write access to the flash memory is performed by the code while PG or FSTPG have not been set previously. Set also by hardware when PROGERR, SIZERR, PGAERR, WRPERR, MISSERR or FASTERR is set due to a previous programming error. Cleared by writing 1."]
pub type PgserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISSERR` reader - Fast programming data miss error In Fast programming mode, 16 double words (128 bytes) must be sent to flash memory successively, and the new data must be sent to the logic control before the current data is fully programmed. MISSERR is set by hardware when the new data is not present in time. Cleared by writing 1."]
pub type MisserrR = crate::BitReader;
#[doc = "Field `MISSERR` writer - Fast programming data miss error In Fast programming mode, 16 double words (128 bytes) must be sent to flash memory successively, and the new data must be sent to the logic control before the current data is fully programmed. MISSERR is set by hardware when the new data is not present in time. Cleared by writing 1."]
pub type MisserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FASTERR` reader - Fast programming error Set by hardware when a fast programming sequence (activated by FSTPG) is interrupted due to an error (alignment, size, write protection or data miss). The corresponding status bit (PGAERR, SIZERR, WRPERR or MISSERR) is set at the same time. Cleared by writing 1."]
pub type FasterrR = crate::BitReader;
#[doc = "Field `FASTERR` writer - Fast programming error Set by hardware when a fast programming sequence (activated by FSTPG) is interrupted due to an error (alignment, size, write protection or data miss). The corresponding status bit (PGAERR, SIZERR, WRPERR or MISSERR) is set at the same time. Cleared by writing 1."]
pub type FasterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDERR` reader - PCROP read error Set by hardware when an address to be read belongs to a read protected area of the flash memory (PCROP protection). An interrupt is generated if RDERRIE is set in FLASH_CR. Cleared by writing 1."]
pub type RderrR = crate::BitReader;
#[doc = "Field `RDERR` writer - PCROP read error Set by hardware when an address to be read belongs to a read protected area of the flash memory (PCROP protection). An interrupt is generated if RDERRIE is set in FLASH_CR. Cleared by writing 1."]
pub type RderrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTVERR` reader - Option and Engineering bits loading validity error"]
pub type OptverrR = crate::BitReader;
#[doc = "Field `OPTVERR` writer - Option and Engineering bits loading validity error"]
pub type OptverrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSY1` reader - Busy This flag indicates that a flash memory operation requested by FLASH control register (FLASH_CR) is in progress. This bit is set at the beginning of the flash memory operation, and cleared when the operation finishes or when an error occurs."]
pub type Bsy1R = crate::BitReader;
#[doc = "Field `CFGBSY` reader - Programming or erase configuration busy. This flag is set and cleared by hardware. It is set when the first word is sent for program or when setting the STRT bit of FLASH control register (FLASH_CR) for erase. It is cleared when the flash memory program or erase operation completes or ends with an error. When set, launching any other operation through the FLASH control register (FLASH_CR) is impossible, and must be postponed (a programming or erase operation is ongoing). When cleared, the program and erase settings in the FLASH control register (FLASH_CR) can be modified."]
pub type CfgbsyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of operation Set by hardware when one or more flash memory operation (programming / erase) has been completed successfully. This bit is set only if the end of operation interrupts are enabled (EOPIE=1). Cleared by writing 1."]
    #[inline(always)]
    pub fn eop(&self) -> EopR {
        EopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operation error Set by hardware when a flash memory operation (program / erase) completes unsuccessfully. This bit is set only if error interrupts are enabled (ERRIE=1). Cleared by writing 1."]
    #[inline(always)]
    pub fn operr(&self) -> OperrR {
        OperrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Programming error Set by hardware when a double-word address to be programmed contains a value different from '0xFFFF FFFF' before programming, except if the data to write is '0x0000 0000'. Cleared by writing 1."]
    #[inline(always)]
    pub fn progerr(&self) -> ProgerrR {
        ProgerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protection error Set by hardware when an address to be erased/programmed belongs to a write-protected part (by WRP, PCROP or RDP Level 1) of the flash memory. Cleared by writing 1."]
    #[inline(always)]
    pub fn wrperr(&self) -> WrperrR {
        WrperrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Programming alignment error Set by hardware when the data to program cannot be contained in the same double word (64-bit) flash memory in case of standard programming, or if there is a change of page during fast programming. Cleared by writing 1."]
    #[inline(always)]
    pub fn pgaerr(&self) -> PgaerrR {
        PgaerrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Size error Set by hardware when the size of the access is a byte or half-word during a program or a fast program sequence. Only double word programming is allowed (consequently: word access). Cleared by writing 1."]
    #[inline(always)]
    pub fn sizerr(&self) -> SizerrR {
        SizerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Programming sequence error Set by hardware when a write access to the flash memory is performed by the code while PG or FSTPG have not been set previously. Set also by hardware when PROGERR, SIZERR, PGAERR, WRPERR, MISSERR or FASTERR is set due to a previous programming error. Cleared by writing 1."]
    #[inline(always)]
    pub fn pgserr(&self) -> PgserrR {
        PgserrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fast programming data miss error In Fast programming mode, 16 double words (128 bytes) must be sent to flash memory successively, and the new data must be sent to the logic control before the current data is fully programmed. MISSERR is set by hardware when the new data is not present in time. Cleared by writing 1."]
    #[inline(always)]
    pub fn misserr(&self) -> MisserrR {
        MisserrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast programming error Set by hardware when a fast programming sequence (activated by FSTPG) is interrupted due to an error (alignment, size, write protection or data miss). The corresponding status bit (PGAERR, SIZERR, WRPERR or MISSERR) is set at the same time. Cleared by writing 1."]
    #[inline(always)]
    pub fn fasterr(&self) -> FasterrR {
        FasterrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - PCROP read error Set by hardware when an address to be read belongs to a read protected area of the flash memory (PCROP protection). An interrupt is generated if RDERRIE is set in FLASH_CR. Cleared by writing 1."]
    #[inline(always)]
    pub fn rderr(&self) -> RderrR {
        RderrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Option and Engineering bits loading validity error"]
    #[inline(always)]
    pub fn optverr(&self) -> OptverrR {
        OptverrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy This flag indicates that a flash memory operation requested by FLASH control register (FLASH_CR) is in progress. This bit is set at the beginning of the flash memory operation, and cleared when the operation finishes or when an error occurs."]
    #[inline(always)]
    pub fn bsy1(&self) -> Bsy1R {
        Bsy1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Programming or erase configuration busy. This flag is set and cleared by hardware. It is set when the first word is sent for program or when setting the STRT bit of FLASH control register (FLASH_CR) for erase. It is cleared when the flash memory program or erase operation completes or ends with an error. When set, launching any other operation through the FLASH control register (FLASH_CR) is impossible, and must be postponed (a programming or erase operation is ongoing). When cleared, the program and erase settings in the FLASH control register (FLASH_CR) can be modified."]
    #[inline(always)]
    pub fn cfgbsy(&self) -> CfgbsyR {
        CfgbsyR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of operation Set by hardware when one or more flash memory operation (programming / erase) has been completed successfully. This bit is set only if the end of operation interrupts are enabled (EOPIE=1). Cleared by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn eop(&mut self) -> EopW<FlashSrSpec> {
        EopW::new(self, 0)
    }
    #[doc = "Bit 1 - Operation error Set by hardware when a flash memory operation (program / erase) completes unsuccessfully. This bit is set only if error interrupts are enabled (ERRIE=1). Cleared by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn operr(&mut self) -> OperrW<FlashSrSpec> {
        OperrW::new(self, 1)
    }
    #[doc = "Bit 3 - Programming error Set by hardware when a double-word address to be programmed contains a value different from '0xFFFF FFFF' before programming, except if the data to write is '0x0000 0000'. Cleared by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn progerr(&mut self) -> ProgerrW<FlashSrSpec> {
        ProgerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Write protection error Set by hardware when an address to be erased/programmed belongs to a write-protected part (by WRP, PCROP or RDP Level 1) of the flash memory. Cleared by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn wrperr(&mut self) -> WrperrW<FlashSrSpec> {
        WrperrW::new(self, 4)
    }
    #[doc = "Bit 5 - Programming alignment error Set by hardware when the data to program cannot be contained in the same double word (64-bit) flash memory in case of standard programming, or if there is a change of page during fast programming. Cleared by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn pgaerr(&mut self) -> PgaerrW<FlashSrSpec> {
        PgaerrW::new(self, 5)
    }
    #[doc = "Bit 6 - Size error Set by hardware when the size of the access is a byte or half-word during a program or a fast program sequence. Only double word programming is allowed (consequently: word access). Cleared by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn sizerr(&mut self) -> SizerrW<FlashSrSpec> {
        SizerrW::new(self, 6)
    }
    #[doc = "Bit 7 - Programming sequence error Set by hardware when a write access to the flash memory is performed by the code while PG or FSTPG have not been set previously. Set also by hardware when PROGERR, SIZERR, PGAERR, WRPERR, MISSERR or FASTERR is set due to a previous programming error. Cleared by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn pgserr(&mut self) -> PgserrW<FlashSrSpec> {
        PgserrW::new(self, 7)
    }
    #[doc = "Bit 8 - Fast programming data miss error In Fast programming mode, 16 double words (128 bytes) must be sent to flash memory successively, and the new data must be sent to the logic control before the current data is fully programmed. MISSERR is set by hardware when the new data is not present in time. Cleared by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn misserr(&mut self) -> MisserrW<FlashSrSpec> {
        MisserrW::new(self, 8)
    }
    #[doc = "Bit 9 - Fast programming error Set by hardware when a fast programming sequence (activated by FSTPG) is interrupted due to an error (alignment, size, write protection or data miss). The corresponding status bit (PGAERR, SIZERR, WRPERR or MISSERR) is set at the same time. Cleared by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn fasterr(&mut self) -> FasterrW<FlashSrSpec> {
        FasterrW::new(self, 9)
    }
    #[doc = "Bit 14 - PCROP read error Set by hardware when an address to be read belongs to a read protected area of the flash memory (PCROP protection). An interrupt is generated if RDERRIE is set in FLASH_CR. Cleared by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn rderr(&mut self) -> RderrW<FlashSrSpec> {
        RderrW::new(self, 14)
    }
    #[doc = "Bit 15 - Option and Engineering bits loading validity error"]
    #[inline(always)]
    #[must_use]
    pub fn optverr(&mut self) -> OptverrW<FlashSrSpec> {
        OptverrW::new(self, 15)
    }
}
#[doc = "FLASH status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashSrSpec;
impl crate::RegisterSpec for FlashSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_sr::R`](R) reader structure"]
impl crate::Readable for FlashSrSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_sr::W`](W) writer structure"]
impl crate::Writable for FlashSrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_SR to value 0"]
impl crate::Resettable for FlashSrSpec {
    const RESET_VALUE: u32 = 0;
}
