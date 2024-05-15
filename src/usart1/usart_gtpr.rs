#[doc = "Register `USART_GTPR` reader"]
pub type R = crate::R<UsartGtprSpec>;
#[doc = "Register `USART_GTPR` writer"]
pub type W = crate::W<UsartGtprSpec>;
#[doc = "Field `PSC` reader - Prescaler value PSC\\[7:0\\]
= IrDA Normal and Low-power baud rate This bitfield is used for programming the prescaler for dividing the USART source clock to achieve the low-power frequency: The source clock is divided by the value given in the register (8 significant bits): ... PSC\\[4:0\\]: Prescaler value This bitfield is used for programming the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... This bitfield can only be written when the USART is disabled (UE=0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. Note: This bitfield is reserved and forced by hardware to 0 when the Smartcard and IrDA modes are not supported. Refer to Section131.4: USART implementation on page1826."]
pub type PscR = crate::FieldReader;
#[doc = "Field `PSC` writer - Prescaler value PSC\\[7:0\\]
= IrDA Normal and Low-power baud rate This bitfield is used for programming the prescaler for dividing the USART source clock to achieve the low-power frequency: The source clock is divided by the value given in the register (8 significant bits): ... PSC\\[4:0\\]: Prescaler value This bitfield is used for programming the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... This bitfield can only be written when the USART is disabled (UE=0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. Note: This bitfield is reserved and forced by hardware to 0 when the Smartcard and IrDA modes are not supported. Refer to Section131.4: USART implementation on page1826."]
pub type PscW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GT` reader - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE=0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type GtR = crate::FieldReader;
#[doc = "Field `GT` writer - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE=0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type GtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Prescaler value PSC\\[7:0\\]
= IrDA Normal and Low-power baud rate This bitfield is used for programming the prescaler for dividing the USART source clock to achieve the low-power frequency: The source clock is divided by the value given in the register (8 significant bits): ... PSC\\[4:0\\]: Prescaler value This bitfield is used for programming the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... This bitfield can only be written when the USART is disabled (UE=0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. Note: This bitfield is reserved and forced by hardware to 0 when the Smartcard and IrDA modes are not supported. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn psc(&self) -> PscR {
        PscR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE=0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn gt(&self) -> GtR {
        GtR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prescaler value PSC\\[7:0\\]
= IrDA Normal and Low-power baud rate This bitfield is used for programming the prescaler for dividing the USART source clock to achieve the low-power frequency: The source clock is divided by the value given in the register (8 significant bits): ... PSC\\[4:0\\]: Prescaler value This bitfield is used for programming the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... This bitfield can only be written when the USART is disabled (UE=0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. Note: This bitfield is reserved and forced by hardware to 0 when the Smartcard and IrDA modes are not supported. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PscW<UsartGtprSpec> {
        PscW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE=0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn gt(&mut self) -> GtW<UsartGtprSpec> {
        GtW::new(self, 8)
    }
}
#[doc = "USART guard time and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_gtpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_gtpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsartGtprSpec;
impl crate::RegisterSpec for UsartGtprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart_gtpr::R`](R) reader structure"]
impl crate::Readable for UsartGtprSpec {}
#[doc = "`write(|w| ..)` method takes [`usart_gtpr::W`](W) writer structure"]
impl crate::Writable for UsartGtprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USART_GTPR to value 0"]
impl crate::Resettable for UsartGtprSpec {
    const RESET_VALUE: u32 = 0;
}
