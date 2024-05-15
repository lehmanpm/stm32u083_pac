#[doc = "Register `I2C_TIMEOUTR` reader"]
pub type R = crate::R<I2cTimeoutrSpec>;
#[doc = "Register `I2C_TIMEOUTR` writer"]
pub type W = crate::W<I2cTimeoutrSpec>;
#[doc = "Field `TIMEOUTA` reader - Bus timeout A This field is used to configure: The SCL low timeout condition t&lt;sub>TIMEOUT&lt;/sub> when TIDLE = 0 t&lt;sub>TIMEOUT&lt;/sub>= (TIMEOUTA + 1) x 2048 x t&lt;sub>I2CCLK&lt;/sub> The bus idle condition (both SCL and SDA high) when TIDLE = 1 t&lt;sub>IDLE&lt;/sub>= (TIMEOUTA + 1) x 4 x t&lt;sub>I2CCLK&lt;/sub> Note: These bits can be written only when TIMOUTEN = 0."]
pub type TimeoutaR = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUTA` writer - Bus timeout A This field is used to configure: The SCL low timeout condition t&lt;sub>TIMEOUT&lt;/sub> when TIDLE = 0 t&lt;sub>TIMEOUT&lt;/sub>= (TIMEOUTA + 1) x 2048 x t&lt;sub>I2CCLK&lt;/sub> The bus idle condition (both SCL and SDA high) when TIDLE = 1 t&lt;sub>IDLE&lt;/sub>= (TIMEOUTA + 1) x 4 x t&lt;sub>I2CCLK&lt;/sub> Note: These bits can be written only when TIMOUTEN = 0."]
pub type TimeoutaW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Idle clock timeout detection Note: This bit can be written only when TIMOUTEN = 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tidle {
    #[doc = "0: TIMEOUTA is used to detect SCL low timeout"]
    B0x0 = 0,
    #[doc = "1: TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)"]
    B0x1 = 1,
}
impl From<Tidle> for bool {
    #[inline(always)]
    fn from(variant: Tidle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIDLE` reader - Idle clock timeout detection Note: This bit can be written only when TIMOUTEN = 0."]
pub type TidleR = crate::BitReader<Tidle>;
impl TidleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tidle {
        match self.bits {
            false => Tidle::B0x0,
            true => Tidle::B0x1,
        }
    }
    #[doc = "TIMEOUTA is used to detect SCL low timeout"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tidle::B0x0
    }
    #[doc = "TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tidle::B0x1
    }
}
#[doc = "Field `TIDLE` writer - Idle clock timeout detection Note: This bit can be written only when TIMOUTEN = 0."]
pub type TidleW<'a, REG> = crate::BitWriter<'a, REG, Tidle>;
impl<'a, REG> TidleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMEOUTA is used to detect SCL low timeout"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tidle::B0x0)
    }
    #[doc = "TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tidle::B0x1)
    }
}
#[doc = "Clock timeout enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timouten {
    #[doc = "0: SCL timeout detection is disabled"]
    B0x0 = 0,
    #[doc = "1: SCL timeout detection is enabled: when SCL is low for more than t&lt;sub>TIMEOUT&lt;/sub> (TIDLE = 0) or high for more than t&lt;sub>IDLE &lt;/sub>(TIDLE = 1), a timeout error is detected (TIMEOUT = 1)."]
    B0x1 = 1,
}
impl From<Timouten> for bool {
    #[inline(always)]
    fn from(variant: Timouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMOUTEN` reader - Clock timeout enable"]
pub type TimoutenR = crate::BitReader<Timouten>;
impl TimoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timouten {
        match self.bits {
            false => Timouten::B0x0,
            true => Timouten::B0x1,
        }
    }
    #[doc = "SCL timeout detection is disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Timouten::B0x0
    }
    #[doc = "SCL timeout detection is enabled: when SCL is low for more than t&lt;sub>TIMEOUT&lt;/sub> (TIDLE = 0) or high for more than t&lt;sub>IDLE &lt;/sub>(TIDLE = 1), a timeout error is detected (TIMEOUT = 1)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Timouten::B0x1
    }
}
#[doc = "Field `TIMOUTEN` writer - Clock timeout enable"]
pub type TimoutenW<'a, REG> = crate::BitWriter<'a, REG, Timouten>;
impl<'a, REG> TimoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCL timeout detection is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Timouten::B0x0)
    }
    #[doc = "SCL timeout detection is enabled: when SCL is low for more than t&lt;sub>TIMEOUT&lt;/sub> (TIDLE = 0) or high for more than t&lt;sub>IDLE &lt;/sub>(TIDLE = 1), a timeout error is detected (TIMEOUT = 1)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Timouten::B0x1)
    }
}
#[doc = "Field `TIMEOUTB` reader - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (t&lt;sub>LOW:MEXT&lt;/sub>) is detected In slave mode, the slave cumulative clock low extend time (t&lt;sub>LOW:SEXT&lt;/sub>) is detected t&lt;sub>LOW:EXT &lt;/sub>= (TIMEOUTB + TIDLE = 01) x 2048 x t&lt;sub>I2CCLK&lt;/sub> Note: These bits can be written only when TEXTEN = 0."]
pub type TimeoutbR = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUTB` writer - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (t&lt;sub>LOW:MEXT&lt;/sub>) is detected In slave mode, the slave cumulative clock low extend time (t&lt;sub>LOW:SEXT&lt;/sub>) is detected t&lt;sub>LOW:EXT &lt;/sub>= (TIMEOUTB + TIDLE = 01) x 2048 x t&lt;sub>I2CCLK&lt;/sub> Note: These bits can be written only when TEXTEN = 0."]
pub type TimeoutbW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Extended clock timeout enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Texten {
    #[doc = "0: Extended clock timeout detection is disabled"]
    B0x0 = 0,
    #[doc = "1: Extended clock timeout detection is enabled. When a cumulative SCL stretch for more than t&lt;sub>LOW:EXT &lt;/sub>is done by the I2C interface, a timeout error is detected (TIMEOUT = 1)."]
    B0x1 = 1,
}
impl From<Texten> for bool {
    #[inline(always)]
    fn from(variant: Texten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEXTEN` reader - Extended clock timeout enable"]
pub type TextenR = crate::BitReader<Texten>;
impl TextenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Texten {
        match self.bits {
            false => Texten::B0x0,
            true => Texten::B0x1,
        }
    }
    #[doc = "Extended clock timeout detection is disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Texten::B0x0
    }
    #[doc = "Extended clock timeout detection is enabled. When a cumulative SCL stretch for more than t&lt;sub>LOW:EXT &lt;/sub>is done by the I2C interface, a timeout error is detected (TIMEOUT = 1)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Texten::B0x1
    }
}
#[doc = "Field `TEXTEN` writer - Extended clock timeout enable"]
pub type TextenW<'a, REG> = crate::BitWriter<'a, REG, Texten>;
impl<'a, REG> TextenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Extended clock timeout detection is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Texten::B0x0)
    }
    #[doc = "Extended clock timeout detection is enabled. When a cumulative SCL stretch for more than t&lt;sub>LOW:EXT &lt;/sub>is done by the I2C interface, a timeout error is detected (TIMEOUT = 1)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Texten::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:11 - Bus timeout A This field is used to configure: The SCL low timeout condition t&lt;sub>TIMEOUT&lt;/sub> when TIDLE = 0 t&lt;sub>TIMEOUT&lt;/sub>= (TIMEOUTA + 1) x 2048 x t&lt;sub>I2CCLK&lt;/sub> The bus idle condition (both SCL and SDA high) when TIDLE = 1 t&lt;sub>IDLE&lt;/sub>= (TIMEOUTA + 1) x 4 x t&lt;sub>I2CCLK&lt;/sub> Note: These bits can be written only when TIMOUTEN = 0."]
    #[inline(always)]
    pub fn timeouta(&self) -> TimeoutaR {
        TimeoutaR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Idle clock timeout detection Note: This bit can be written only when TIMOUTEN = 0."]
    #[inline(always)]
    pub fn tidle(&self) -> TidleR {
        TidleR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock timeout enable"]
    #[inline(always)]
    pub fn timouten(&self) -> TimoutenR {
        TimoutenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (t&lt;sub>LOW:MEXT&lt;/sub>) is detected In slave mode, the slave cumulative clock low extend time (t&lt;sub>LOW:SEXT&lt;/sub>) is detected t&lt;sub>LOW:EXT &lt;/sub>= (TIMEOUTB + TIDLE = 01) x 2048 x t&lt;sub>I2CCLK&lt;/sub> Note: These bits can be written only when TEXTEN = 0."]
    #[inline(always)]
    pub fn timeoutb(&self) -> TimeoutbR {
        TimeoutbR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Extended clock timeout enable"]
    #[inline(always)]
    pub fn texten(&self) -> TextenR {
        TextenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Bus timeout A This field is used to configure: The SCL low timeout condition t&lt;sub>TIMEOUT&lt;/sub> when TIDLE = 0 t&lt;sub>TIMEOUT&lt;/sub>= (TIMEOUTA + 1) x 2048 x t&lt;sub>I2CCLK&lt;/sub> The bus idle condition (both SCL and SDA high) when TIDLE = 1 t&lt;sub>IDLE&lt;/sub>= (TIMEOUTA + 1) x 4 x t&lt;sub>I2CCLK&lt;/sub> Note: These bits can be written only when TIMOUTEN = 0."]
    #[inline(always)]
    #[must_use]
    pub fn timeouta(&mut self) -> TimeoutaW<I2cTimeoutrSpec> {
        TimeoutaW::new(self, 0)
    }
    #[doc = "Bit 12 - Idle clock timeout detection Note: This bit can be written only when TIMOUTEN = 0."]
    #[inline(always)]
    #[must_use]
    pub fn tidle(&mut self) -> TidleW<I2cTimeoutrSpec> {
        TidleW::new(self, 12)
    }
    #[doc = "Bit 15 - Clock timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn timouten(&mut self) -> TimoutenW<I2cTimeoutrSpec> {
        TimoutenW::new(self, 15)
    }
    #[doc = "Bits 16:27 - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (t&lt;sub>LOW:MEXT&lt;/sub>) is detected In slave mode, the slave cumulative clock low extend time (t&lt;sub>LOW:SEXT&lt;/sub>) is detected t&lt;sub>LOW:EXT &lt;/sub>= (TIMEOUTB + TIDLE = 01) x 2048 x t&lt;sub>I2CCLK&lt;/sub> Note: These bits can be written only when TEXTEN = 0."]
    #[inline(always)]
    #[must_use]
    pub fn timeoutb(&mut self) -> TimeoutbW<I2cTimeoutrSpec> {
        TimeoutbW::new(self, 16)
    }
    #[doc = "Bit 31 - Extended clock timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn texten(&mut self) -> TextenW<I2cTimeoutrSpec> {
        TextenW::new(self, 31)
    }
}
#[doc = "I2C timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_timeoutr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_timeoutr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cTimeoutrSpec;
impl crate::RegisterSpec for I2cTimeoutrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_timeoutr::R`](R) reader structure"]
impl crate::Readable for I2cTimeoutrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_timeoutr::W`](W) writer structure"]
impl crate::Writable for I2cTimeoutrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_TIMEOUTR to value 0"]
impl crate::Resettable for I2cTimeoutrSpec {
    const RESET_VALUE: u32 = 0;
}
