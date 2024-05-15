#[doc = "Register `I2C_CR2` reader"]
pub type R = crate::R<I2cCr2Spec>;
#[doc = "Register `I2C_CR2` writer"]
pub type W = crate::W<I2cCr2Spec>;
#[doc = "Field `SADD` reader - Slave address (master mode) In 7-bit addressing mode (ADD10 = 0): SADD\\[7:1\\]
must be written with the 7-bit slave address to be sent. Bits SADD\\[9\\], SADD\\[8\\]
and SADD\\[0\\]
are don't care. In 10-bit addressing mode (ADD10 = 1): SADD\\[9:0\\]
must be written with the 10-bit slave address to be sent. Note: Changing these bits when the START bit is set is not allowed."]
pub type SaddR = crate::FieldReader<u16>;
#[doc = "Field `SADD` writer - Slave address (master mode) In 7-bit addressing mode (ADD10 = 0): SADD\\[7:1\\]
must be written with the 7-bit slave address to be sent. Bits SADD\\[9\\], SADD\\[8\\]
and SADD\\[0\\]
are don't care. In 10-bit addressing mode (ADD10 = 1): SADD\\[9:0\\]
must be written with the 10-bit slave address to be sent. Note: Changing these bits when the START bit is set is not allowed."]
pub type SaddW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RdWrn {
    #[doc = "0: Master requests a write transfer"]
    B0x0 = 0,
    #[doc = "1: Master requests a read transfer"]
    B0x1 = 1,
}
impl From<RdWrn> for bool {
    #[inline(always)]
    fn from(variant: RdWrn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RD_WRN` reader - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed."]
pub type RdWrnR = crate::BitReader<RdWrn>;
impl RdWrnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RdWrn {
        match self.bits {
            false => RdWrn::B0x0,
            true => RdWrn::B0x1,
        }
    }
    #[doc = "Master requests a write transfer"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RdWrn::B0x0
    }
    #[doc = "Master requests a read transfer"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RdWrn::B0x1
    }
}
#[doc = "Field `RD_WRN` writer - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed."]
pub type RdWrnW<'a, REG> = crate::BitWriter<'a, REG, RdWrn>;
impl<'a, REG> RdWrnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master requests a write transfer"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RdWrn::B0x0)
    }
    #[doc = "Master requests a read transfer"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RdWrn::B0x1)
    }
}
#[doc = "10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Add10 {
    #[doc = "0: The master operates in 7-bit addressing mode"]
    B0x0 = 0,
    #[doc = "1: The master operates in 10-bit addressing mode"]
    B0x1 = 1,
}
impl From<Add10> for bool {
    #[inline(always)]
    fn from(variant: Add10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD10` reader - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed."]
pub type Add10R = crate::BitReader<Add10>;
impl Add10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Add10 {
        match self.bits {
            false => Add10::B0x0,
            true => Add10::B0x1,
        }
    }
    #[doc = "The master operates in 7-bit addressing mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Add10::B0x0
    }
    #[doc = "The master operates in 10-bit addressing mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Add10::B0x1
    }
}
#[doc = "Field `ADD10` writer - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed."]
pub type Add10W<'a, REG> = crate::BitWriter<'a, REG, Add10>;
impl<'a, REG> Add10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The master operates in 7-bit addressing mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Add10::B0x0)
    }
    #[doc = "The master operates in 10-bit addressing mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Add10::B0x1)
    }
}
#[doc = "10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Head10r {
    #[doc = "0: The master sends the complete 10-bit slave address read sequence: Start + 2 bytes 10-bit address in write direction + Restart + first seven bits of the 10-bit address in read direction."]
    B0x0 = 0,
    #[doc = "1: The master only sends the first seven bits of the 10-bit address, followed by Read direction."]
    B0x1 = 1,
}
impl From<Head10r> for bool {
    #[inline(always)]
    fn from(variant: Head10r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HEAD10R` reader - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed."]
pub type Head10rR = crate::BitReader<Head10r>;
impl Head10rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Head10r {
        match self.bits {
            false => Head10r::B0x0,
            true => Head10r::B0x1,
        }
    }
    #[doc = "The master sends the complete 10-bit slave address read sequence: Start + 2 bytes 10-bit address in write direction + Restart + first seven bits of the 10-bit address in read direction."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Head10r::B0x0
    }
    #[doc = "The master only sends the first seven bits of the 10-bit address, followed by Read direction."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Head10r::B0x1
    }
}
#[doc = "Field `HEAD10R` writer - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed."]
pub type Head10rW<'a, REG> = crate::BitWriter<'a, REG, Head10r>;
impl<'a, REG> Head10rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The master sends the complete 10-bit slave address read sequence: Start + 2 bytes 10-bit address in write direction + Restart + first seven bits of the 10-bit address in read direction."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Head10r::B0x0)
    }
    #[doc = "The master only sends the first seven bits of the 10-bit address, followed by Read direction."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Head10r::B0x1)
    }
}
#[doc = "Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated start condition when RELOAD = 0, after the end of the NBYTES transfer. Otherwise, setting this bit generates a START condition once the bus is free. Note: Writing 0 to this bit has no effect. Note: The START bit can be set even if the bus is BUSY or I2C is in slave mode. Note: This bit has no effect when RELOAD is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "0: No Start generation"]
    B0x0 = 0,
    #[doc = "1: Restart/Start generation:"]
    B0x1 = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated start condition when RELOAD = 0, after the end of the NBYTES transfer. Otherwise, setting this bit generates a START condition once the bus is free. Note: Writing 0 to this bit has no effect. Note: The START bit can be set even if the bus is BUSY or I2C is in slave mode. Note: This bit has no effect when RELOAD is set."]
pub type StartR = crate::BitReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            false => Start::B0x0,
            true => Start::B0x1,
        }
    }
    #[doc = "No Start generation"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Start::B0x0
    }
    #[doc = "Restart/Start generation:"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Start::B0x1
    }
}
#[doc = "Field `START` writer - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated start condition when RELOAD = 0, after the end of the NBYTES transfer. Otherwise, setting this bit generates a START condition once the bus is free. Note: Writing 0 to this bit has no effect. Note: The START bit can be set even if the bus is BUSY or I2C is in slave mode. Note: This bit has no effect when RELOAD is set."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Start generation"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Start::B0x0)
    }
    #[doc = "Restart/Start generation:"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Start::B0x1)
    }
}
#[doc = "Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In master mode: Note: Writing 0 to this bit has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    #[doc = "0: No Stop generation"]
    B0x0 = 0,
    #[doc = "1: Stop generation after current byte transfer"]
    B0x1 = 1,
}
impl From<Stop> for bool {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In master mode: Note: Writing 0 to this bit has no effect."]
pub type StopR = crate::BitReader<Stop>;
impl StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop {
        match self.bits {
            false => Stop::B0x0,
            true => Stop::B0x1,
        }
    }
    #[doc = "No Stop generation"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Stop::B0x0
    }
    #[doc = "Stop generation after current byte transfer"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Stop::B0x1
    }
}
#[doc = "Field `STOP` writer - Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In master mode: Note: Writing 0 to this bit has no effect."]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Stop generation"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::B0x0)
    }
    #[doc = "Stop generation after current byte transfer"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::B0x1)
    }
}
#[doc = "NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. Note: When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated, whatever the NACK bit value. Note: When hardware PEC checking is enabled (PECBYTE = 1), the PEC acknowledge value does not depend on the NACK value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nack {
    #[doc = "0: an ACK is sent after current received byte."]
    B0x0 = 0,
    #[doc = "1: a NACK is sent after current received byte."]
    B0x1 = 1,
}
impl From<Nack> for bool {
    #[inline(always)]
    fn from(variant: Nack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK` reader - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. Note: When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated, whatever the NACK bit value. Note: When hardware PEC checking is enabled (PECBYTE = 1), the PEC acknowledge value does not depend on the NACK value."]
pub type NackR = crate::BitReader<Nack>;
impl NackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nack {
        match self.bits {
            false => Nack::B0x0,
            true => Nack::B0x1,
        }
    }
    #[doc = "an ACK is sent after current received byte."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Nack::B0x0
    }
    #[doc = "a NACK is sent after current received byte."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Nack::B0x1
    }
}
#[doc = "Field `NACK` writer - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. Note: When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated, whatever the NACK bit value. Note: When hardware PEC checking is enabled (PECBYTE = 1), the PEC acknowledge value does not depend on the NACK value."]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG, Nack>;
impl<'a, REG> NackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "an ACK is sent after current received byte."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Nack::B0x0)
    }
    #[doc = "a NACK is sent after current received byte."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Nack::B0x1)
    }
}
#[doc = "Field `NBYTES` reader - Number of bytes"]
pub type NbytesR = crate::FieldReader;
#[doc = "Field `NBYTES` writer - Number of bytes"]
pub type NbytesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "NBYTES reload mode This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reload {
    #[doc = "0: The transfer is completed after the NBYTES data transfer (STOP or RESTART follows)."]
    B0x0 = 0,
    #[doc = "1: The transfer is not completed after the NBYTES data transfer (NBYTES is reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low."]
    B0x1 = 1,
}
impl From<Reload> for bool {
    #[inline(always)]
    fn from(variant: Reload) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RELOAD` reader - NBYTES reload mode This bit is set and cleared by software."]
pub type ReloadR = crate::BitReader<Reload>;
impl ReloadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reload {
        match self.bits {
            false => Reload::B0x0,
            true => Reload::B0x1,
        }
    }
    #[doc = "The transfer is completed after the NBYTES data transfer (STOP or RESTART follows)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Reload::B0x0
    }
    #[doc = "The transfer is not completed after the NBYTES data transfer (NBYTES is reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Reload::B0x1
    }
}
#[doc = "Field `RELOAD` writer - NBYTES reload mode This bit is set and cleared by software."]
pub type ReloadW<'a, REG> = crate::BitWriter<'a, REG, Reload>;
impl<'a, REG> ReloadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transfer is completed after the NBYTES data transfer (STOP or RESTART follows)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Reload::B0x0)
    }
    #[doc = "The transfer is not completed after the NBYTES data transfer (NBYTES is reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Reload::B0x1)
    }
}
#[doc = "Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autoend {
    #[doc = "0: software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low."]
    B0x0 = 0,
    #[doc = "1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred."]
    B0x1 = 1,
}
impl From<Autoend> for bool {
    #[inline(always)]
    fn from(variant: Autoend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOEND` reader - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set."]
pub type AutoendR = crate::BitReader<Autoend>;
impl AutoendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autoend {
        match self.bits {
            false => Autoend::B0x0,
            true => Autoend::B0x1,
        }
    }
    #[doc = "software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Autoend::B0x0
    }
    #[doc = "Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Autoend::B0x1
    }
}
#[doc = "Field `AUTOEND` writer - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set."]
pub type AutoendW<'a, REG> = crate::BitWriter<'a, REG, Autoend>;
impl<'a, REG> AutoendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Autoend::B0x0)
    }
    #[doc = "Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Autoend::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:9 - Slave address (master mode) In 7-bit addressing mode (ADD10 = 0): SADD\\[7:1\\]
must be written with the 7-bit slave address to be sent. Bits SADD\\[9\\], SADD\\[8\\]
and SADD\\[0\\]
are don't care. In 10-bit addressing mode (ADD10 = 1): SADD\\[9:0\\]
must be written with the 10-bit slave address to be sent. Note: Changing these bits when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn sadd(&self) -> SaddR {
        SaddR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn rd_wrn(&self) -> RdWrnR {
        RdWrnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn add10(&self) -> Add10R {
        Add10R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn head10r(&self) -> Head10rR {
        Head10rR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated start condition when RELOAD = 0, after the end of the NBYTES transfer. Otherwise, setting this bit generates a START condition once the bus is free. Note: Writing 0 to this bit has no effect. Note: The START bit can be set even if the bus is BUSY or I2C is in slave mode. Note: This bit has no effect when RELOAD is set."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In master mode: Note: Writing 0 to this bit has no effect."]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. Note: When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated, whatever the NACK bit value. Note: When hardware PEC checking is enabled (PECBYTE = 1), the PEC acknowledge value does not depend on the NACK value."]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Number of bytes"]
    #[inline(always)]
    pub fn nbytes(&self) -> NbytesR {
        NbytesR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - NBYTES reload mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set."]
    #[inline(always)]
    pub fn autoend(&self) -> AutoendR {
        AutoendR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave address (master mode) In 7-bit addressing mode (ADD10 = 0): SADD\\[7:1\\]
must be written with the 7-bit slave address to be sent. Bits SADD\\[9\\], SADD\\[8\\]
and SADD\\[0\\]
are don't care. In 10-bit addressing mode (ADD10 = 1): SADD\\[9:0\\]
must be written with the 10-bit slave address to be sent. Note: Changing these bits when the START bit is set is not allowed."]
    #[inline(always)]
    #[must_use]
    pub fn sadd(&mut self) -> SaddW<I2cCr2Spec> {
        SaddW::new(self, 0)
    }
    #[doc = "Bit 10 - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed."]
    #[inline(always)]
    #[must_use]
    pub fn rd_wrn(&mut self) -> RdWrnW<I2cCr2Spec> {
        RdWrnW::new(self, 10)
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed."]
    #[inline(always)]
    #[must_use]
    pub fn add10(&mut self) -> Add10W<I2cCr2Spec> {
        Add10W::new(self, 11)
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed."]
    #[inline(always)]
    #[must_use]
    pub fn head10r(&mut self) -> Head10rW<I2cCr2Spec> {
        Head10rW::new(self, 12)
    }
    #[doc = "Bit 13 - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated start condition when RELOAD = 0, after the end of the NBYTES transfer. Otherwise, setting this bit generates a START condition once the bus is free. Note: Writing 0 to this bit has no effect. Note: The START bit can be set even if the bus is BUSY or I2C is in slave mode. Note: This bit has no effect when RELOAD is set."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<I2cCr2Spec> {
        StartW::new(self, 13)
    }
    #[doc = "Bit 14 - Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In master mode: Note: Writing 0 to this bit has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<I2cCr2Spec> {
        StopW::new(self, 14)
    }
    #[doc = "Bit 15 - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. Note: When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated, whatever the NACK bit value. Note: When hardware PEC checking is enabled (PECBYTE = 1), the PEC acknowledge value does not depend on the NACK value."]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NackW<I2cCr2Spec> {
        NackW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Number of bytes"]
    #[inline(always)]
    #[must_use]
    pub fn nbytes(&mut self) -> NbytesW<I2cCr2Spec> {
        NbytesW::new(self, 16)
    }
    #[doc = "Bit 24 - NBYTES reload mode This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> ReloadW<I2cCr2Spec> {
        ReloadW::new(self, 24)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn autoend(&mut self) -> AutoendW<I2cCr2Spec> {
        AutoendW::new(self, 25)
    }
}
#[doc = "I2C control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cCr2Spec;
impl crate::RegisterSpec for I2cCr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_cr2::R`](R) reader structure"]
impl crate::Readable for I2cCr2Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_cr2::W`](W) writer structure"]
impl crate::Writable for I2cCr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_CR2 to value 0"]
impl crate::Resettable for I2cCr2Spec {
    const RESET_VALUE: u32 = 0;
}
