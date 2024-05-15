#[doc = "Register `I2C_TIMINGR` reader"]
pub type R = crate::R<I2cTimingrSpec>;
#[doc = "Register `I2C_TIMINGR` writer"]
pub type W = crate::W<I2cTimingrSpec>;
#[doc = "Field `SCLL` reader - SCL low period (master mode) This field is used to generate the SCL low period in master mode. t&lt;sub>SCLL &lt;/sub>= (SCLL + 1) x t&lt;sub>PRESC&lt;/sub> Note: SCLL is also used to generate t&lt;sub>BUF &lt;/sub>and t&lt;sub>SU:STA &lt;/sub>timings."]
pub type ScllR = crate::FieldReader;
#[doc = "Field `SCLL` writer - SCL low period (master mode) This field is used to generate the SCL low period in master mode. t&lt;sub>SCLL &lt;/sub>= (SCLL + 1) x t&lt;sub>PRESC&lt;/sub> Note: SCLL is also used to generate t&lt;sub>BUF &lt;/sub>and t&lt;sub>SU:STA &lt;/sub>timings."]
pub type ScllW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLH` reader - SCL high period (master mode) This field is used to generate the SCL high period in master mode. t&lt;sub>SCLH &lt;/sub>= (SCLH + 1) x t&lt;sub>PRESC&lt;/sub> Note: SCLH is also used to generate t&lt;sub>SU:STO &lt;/sub>and t&lt;sub>HD:STA &lt;/sub>timing."]
pub type SclhR = crate::FieldReader;
#[doc = "Field `SCLH` writer - SCL high period (master mode) This field is used to generate the SCL high period in master mode. t&lt;sub>SCLH &lt;/sub>= (SCLH + 1) x t&lt;sub>PRESC&lt;/sub> Note: SCLH is also used to generate t&lt;sub>SU:STO &lt;/sub>and t&lt;sub>HD:STA &lt;/sub>timing."]
pub type SclhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SDADEL` reader - Data hold time This field is used to generate the delay t&lt;sub>SDADEL &lt;/sub>between SCL falling edge and SDA edge. In master and in slave modes with NOSTRETCH = 0, the SCL line is stretched low during t&lt;sub>SDADEL&lt;/sub>. t&lt;sub>SDADEL&lt;/sub>= SDADEL x t&lt;sub>PRESC&lt;/sub> Note: SDADEL is used to generate t&lt;sub>HD:DAT &lt;/sub>timing."]
pub type SdadelR = crate::FieldReader;
#[doc = "Field `SDADEL` writer - Data hold time This field is used to generate the delay t&lt;sub>SDADEL &lt;/sub>between SCL falling edge and SDA edge. In master and in slave modes with NOSTRETCH = 0, the SCL line is stretched low during t&lt;sub>SDADEL&lt;/sub>. t&lt;sub>SDADEL&lt;/sub>= SDADEL x t&lt;sub>PRESC&lt;/sub> Note: SDADEL is used to generate t&lt;sub>HD:DAT &lt;/sub>timing."]
pub type SdadelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCLDEL` reader - Data setup time This field is used to generate a delay t&lt;sub>SCLDEL &lt;/sub>between SDA edge and SCL rising edge. In master and in slave modes with NOSTRETCH = 0, the SCL line is stretched low during t&lt;sub>SCLDEL&lt;/sub>. t&lt;sub>SCLDEL &lt;/sub>= (SCLDEL + 1) x t&lt;sub>PRESC&lt;/sub> Note: t&lt;sub>SCLDEL&lt;/sub> is used to generate t&lt;sub>SU:DAT &lt;/sub>timing."]
pub type ScldelR = crate::FieldReader;
#[doc = "Field `SCLDEL` writer - Data setup time This field is used to generate a delay t&lt;sub>SCLDEL &lt;/sub>between SDA edge and SCL rising edge. In master and in slave modes with NOSTRETCH = 0, the SCL line is stretched low during t&lt;sub>SCLDEL&lt;/sub>. t&lt;sub>SCLDEL &lt;/sub>= (SCLDEL + 1) x t&lt;sub>PRESC&lt;/sub> Note: t&lt;sub>SCLDEL&lt;/sub> is used to generate t&lt;sub>SU:DAT &lt;/sub>timing."]
pub type ScldelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRESC` reader - Timing prescaler This field is used to prescale I2CCLK to generate the clock period t&lt;sub>PRESC &lt;/sub>used for data setup and hold counters (refer to I2C timings) and for SCL high and low level counters (refer to I2C master initialization). t&lt;sub>PRESC &lt;/sub>= (PRESC + 1) x t&lt;sub>I2CCLK&lt;/sub>"]
pub type PrescR = crate::FieldReader;
#[doc = "Field `PRESC` writer - Timing prescaler This field is used to prescale I2CCLK to generate the clock period t&lt;sub>PRESC &lt;/sub>used for data setup and hold counters (refer to I2C timings) and for SCL high and low level counters (refer to I2C master initialization). t&lt;sub>PRESC &lt;/sub>= (PRESC + 1) x t&lt;sub>I2CCLK&lt;/sub>"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - SCL low period (master mode) This field is used to generate the SCL low period in master mode. t&lt;sub>SCLL &lt;/sub>= (SCLL + 1) x t&lt;sub>PRESC&lt;/sub> Note: SCLL is also used to generate t&lt;sub>BUF &lt;/sub>and t&lt;sub>SU:STA &lt;/sub>timings."]
    #[inline(always)]
    pub fn scll(&self) -> ScllR {
        ScllR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCL high period (master mode) This field is used to generate the SCL high period in master mode. t&lt;sub>SCLH &lt;/sub>= (SCLH + 1) x t&lt;sub>PRESC&lt;/sub> Note: SCLH is also used to generate t&lt;sub>SU:STO &lt;/sub>and t&lt;sub>HD:STA &lt;/sub>timing."]
    #[inline(always)]
    pub fn sclh(&self) -> SclhR {
        SclhR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data hold time This field is used to generate the delay t&lt;sub>SDADEL &lt;/sub>between SCL falling edge and SDA edge. In master and in slave modes with NOSTRETCH = 0, the SCL line is stretched low during t&lt;sub>SDADEL&lt;/sub>. t&lt;sub>SDADEL&lt;/sub>= SDADEL x t&lt;sub>PRESC&lt;/sub> Note: SDADEL is used to generate t&lt;sub>HD:DAT &lt;/sub>timing."]
    #[inline(always)]
    pub fn sdadel(&self) -> SdadelR {
        SdadelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Data setup time This field is used to generate a delay t&lt;sub>SCLDEL &lt;/sub>between SDA edge and SCL rising edge. In master and in slave modes with NOSTRETCH = 0, the SCL line is stretched low during t&lt;sub>SCLDEL&lt;/sub>. t&lt;sub>SCLDEL &lt;/sub>= (SCLDEL + 1) x t&lt;sub>PRESC&lt;/sub> Note: t&lt;sub>SCLDEL&lt;/sub> is used to generate t&lt;sub>SU:DAT &lt;/sub>timing."]
    #[inline(always)]
    pub fn scldel(&self) -> ScldelR {
        ScldelR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Timing prescaler This field is used to prescale I2CCLK to generate the clock period t&lt;sub>PRESC &lt;/sub>used for data setup and hold counters (refer to I2C timings) and for SCL high and low level counters (refer to I2C master initialization). t&lt;sub>PRESC &lt;/sub>= (PRESC + 1) x t&lt;sub>I2CCLK&lt;/sub>"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCL low period (master mode) This field is used to generate the SCL low period in master mode. t&lt;sub>SCLL &lt;/sub>= (SCLL + 1) x t&lt;sub>PRESC&lt;/sub> Note: SCLL is also used to generate t&lt;sub>BUF &lt;/sub>and t&lt;sub>SU:STA &lt;/sub>timings."]
    #[inline(always)]
    #[must_use]
    pub fn scll(&mut self) -> ScllW<I2cTimingrSpec> {
        ScllW::new(self, 0)
    }
    #[doc = "Bits 8:15 - SCL high period (master mode) This field is used to generate the SCL high period in master mode. t&lt;sub>SCLH &lt;/sub>= (SCLH + 1) x t&lt;sub>PRESC&lt;/sub> Note: SCLH is also used to generate t&lt;sub>SU:STO &lt;/sub>and t&lt;sub>HD:STA &lt;/sub>timing."]
    #[inline(always)]
    #[must_use]
    pub fn sclh(&mut self) -> SclhW<I2cTimingrSpec> {
        SclhW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Data hold time This field is used to generate the delay t&lt;sub>SDADEL &lt;/sub>between SCL falling edge and SDA edge. In master and in slave modes with NOSTRETCH = 0, the SCL line is stretched low during t&lt;sub>SDADEL&lt;/sub>. t&lt;sub>SDADEL&lt;/sub>= SDADEL x t&lt;sub>PRESC&lt;/sub> Note: SDADEL is used to generate t&lt;sub>HD:DAT &lt;/sub>timing."]
    #[inline(always)]
    #[must_use]
    pub fn sdadel(&mut self) -> SdadelW<I2cTimingrSpec> {
        SdadelW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Data setup time This field is used to generate a delay t&lt;sub>SCLDEL &lt;/sub>between SDA edge and SCL rising edge. In master and in slave modes with NOSTRETCH = 0, the SCL line is stretched low during t&lt;sub>SCLDEL&lt;/sub>. t&lt;sub>SCLDEL &lt;/sub>= (SCLDEL + 1) x t&lt;sub>PRESC&lt;/sub> Note: t&lt;sub>SCLDEL&lt;/sub> is used to generate t&lt;sub>SU:DAT &lt;/sub>timing."]
    #[inline(always)]
    #[must_use]
    pub fn scldel(&mut self) -> ScldelW<I2cTimingrSpec> {
        ScldelW::new(self, 20)
    }
    #[doc = "Bits 28:31 - Timing prescaler This field is used to prescale I2CCLK to generate the clock period t&lt;sub>PRESC &lt;/sub>used for data setup and hold counters (refer to I2C timings) and for SCL high and low level counters (refer to I2C master initialization). t&lt;sub>PRESC &lt;/sub>= (PRESC + 1) x t&lt;sub>I2CCLK&lt;/sub>"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PrescW<I2cTimingrSpec> {
        PrescW::new(self, 28)
    }
}
#[doc = "I2C timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_timingr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_timingr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cTimingrSpec;
impl crate::RegisterSpec for I2cTimingrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_timingr::R`](R) reader structure"]
impl crate::Readable for I2cTimingrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_timingr::W`](W) writer structure"]
impl crate::Writable for I2cTimingrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_TIMINGR to value 0"]
impl crate::Resettable for I2cTimingrSpec {
    const RESET_VALUE: u32 = 0;
}
