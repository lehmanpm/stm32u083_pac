#[doc = "Register `AES_CR` reader"]
pub type R = crate::R<AesCrSpec>;
#[doc = "Register `AES_CR` writer"]
pub type W = crate::W<AesCrSpec>;
#[doc = "Enable This bit enables/disables the AES peripheral. At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (MODE\\[1:0\\]
at 0x1) and upon the completion of GCM/GMAC/CCM initialization phase. The bit cannot be set as long as KEYVALID1is cleared\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Enable This bit enables/disables the AES peripheral. At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (MODE\\[1:0\\]
at 0x1) and upon the completion of GCM/GMAC/CCM initialization phase. The bit cannot be set as long as KEYVALID1is cleared"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::B0x0,
            true => En::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == En::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == En::B0x1
    }
}
#[doc = "Field `EN` writer - Enable This bit enables/disables the AES peripheral. At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (MODE\\[1:0\\]
at 0x1) and upon the completion of GCM/GMAC/CCM initialization phase. The bit cannot be set as long as KEYVALID1is cleared"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(En::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(En::B0x1)
    }
}
#[doc = "Data type This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping. This swapping is defined in Section121.4.14: AES data registers and data swapping. Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datatype {
    #[doc = "0: No swapping (32-bit data)."]
    B0x0 = 0,
    #[doc = "1: Half-word swapping (16-bit data)"]
    B0x1 = 1,
    #[doc = "2: Byte swapping (8-bit data)"]
    B0x2 = 2,
    #[doc = "3: Bit-level swapping"]
    B0x3 = 3,
}
impl From<Datatype> for u8 {
    #[inline(always)]
    fn from(variant: Datatype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datatype {
    type Ux = u8;
}
impl crate::IsEnum for Datatype {}
#[doc = "Field `DATATYPE` reader - Data type This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping. This swapping is defined in Section121.4.14: AES data registers and data swapping. Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access."]
pub type DatatypeR = crate::FieldReader<Datatype>;
impl DatatypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datatype {
        match self.bits {
            0 => Datatype::B0x0,
            1 => Datatype::B0x1,
            2 => Datatype::B0x2,
            3 => Datatype::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "No swapping (32-bit data)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Datatype::B0x0
    }
    #[doc = "Half-word swapping (16-bit data)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Datatype::B0x1
    }
    #[doc = "Byte swapping (8-bit data)"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Datatype::B0x2
    }
    #[doc = "Bit-level swapping"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Datatype::B0x3
    }
}
#[doc = "Field `DATATYPE` writer - Data type This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping. This swapping is defined in Section121.4.14: AES data registers and data swapping. Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access."]
pub type DatatypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Datatype, crate::Safe>;
impl<'a, REG> DatatypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No swapping (32-bit data)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0x0)
    }
    #[doc = "Half-word swapping (16-bit data)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0x1)
    }
    #[doc = "Byte swapping (8-bit data)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0x2)
    }
    #[doc = "Bit-level swapping"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0x3)
    }
}
#[doc = "Operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Encryption"]
    B0x0 = 0,
    #[doc = "1: Key derivation (or key preparation), for ECB/CBC decryption only"]
    B0x1 = 1,
    #[doc = "2: Decryption"]
    B0x2 = 2,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::B0x0),
            1 => Some(Mode::B0x1),
            2 => Some(Mode::B0x2),
            _ => None,
        }
    }
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mode::B0x0
    }
    #[doc = "Key derivation (or key preparation), for ECB/CBC decryption only"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mode::B0x1
    }
    #[doc = "Decryption"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mode::B0x2
    }
}
#[doc = "Field `MODE` writer - Operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::B0x0)
    }
    #[doc = "Key derivation (or key preparation), for ECB/CBC decryption only"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::B0x1)
    }
    #[doc = "Decryption"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::B0x2)
    }
}
#[doc = "CHMOD\\[1:0\\]: Chaining mode This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chmod {
    #[doc = "0: Electronic codebook (ECB)"]
    B0x0 = 0,
    #[doc = "1: Cipher-block chaining (CBC)"]
    B0x1 = 1,
    #[doc = "2: Counter mode (CTR)"]
    B0x2 = 2,
    #[doc = "3: Galois counter mode (GCM) and Galois message authentication code (GMAC)"]
    B0x3 = 3,
}
impl From<Chmod> for u8 {
    #[inline(always)]
    fn from(variant: Chmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chmod {
    type Ux = u8;
}
impl crate::IsEnum for Chmod {}
#[doc = "Field `CHMOD` reader - CHMOD\\[1:0\\]: Chaining mode This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access."]
pub type ChmodR = crate::FieldReader<Chmod>;
impl ChmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chmod {
        match self.bits {
            0 => Chmod::B0x0,
            1 => Chmod::B0x1,
            2 => Chmod::B0x2,
            3 => Chmod::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Electronic codebook (ECB)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Chmod::B0x0
    }
    #[doc = "Cipher-block chaining (CBC)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Chmod::B0x1
    }
    #[doc = "Counter mode (CTR)"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Chmod::B0x2
    }
    #[doc = "Galois counter mode (GCM) and Galois message authentication code (GMAC)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Chmod::B0x3
    }
}
#[doc = "Field `CHMOD` writer - CHMOD\\[1:0\\]: Chaining mode This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access."]
pub type ChmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Chmod, crate::Safe>;
impl<'a, REG> ChmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Electronic codebook (ECB)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chmod::B0x0)
    }
    #[doc = "Cipher-block chaining (CBC)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chmod::B0x1)
    }
    #[doc = "Counter mode (CTR)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Chmod::B0x2)
    }
    #[doc = "Galois counter mode (GCM) and Galois message authentication code (GMAC)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Chmod::B0x3)
    }
}
#[doc = "DMA input enable This bit enables automatic generation of DMA requests during the data phase, for incoming data transfers to AES via DMA. Setting this bit is ignored when MODE\\[1:0\\]
is at 0x1 (key derivation).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmainen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Dmainen> for bool {
    #[inline(always)]
    fn from(variant: Dmainen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAINEN` reader - DMA input enable This bit enables automatic generation of DMA requests during the data phase, for incoming data transfers to AES via DMA. Setting this bit is ignored when MODE\\[1:0\\]
is at 0x1 (key derivation)."]
pub type DmainenR = crate::BitReader<Dmainen>;
impl DmainenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmainen {
        match self.bits {
            false => Dmainen::B0x0,
            true => Dmainen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dmainen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dmainen::B0x1
    }
}
#[doc = "Field `DMAINEN` writer - DMA input enable This bit enables automatic generation of DMA requests during the data phase, for incoming data transfers to AES via DMA. Setting this bit is ignored when MODE\\[1:0\\]
is at 0x1 (key derivation)."]
pub type DmainenW<'a, REG> = crate::BitWriter<'a, REG, Dmainen>;
impl<'a, REG> DmainenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmainen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmainen::B0x1)
    }
}
#[doc = "DMA output enable This bit enables automatic generation of DMA requests during the data phase, for outgoing data transfers from AES via DMA. Setting this bit is ignored when MODE\\[1:0\\]
is at 0x1 (key derivation).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaouten {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Dmaouten> for bool {
    #[inline(always)]
    fn from(variant: Dmaouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAOUTEN` reader - DMA output enable This bit enables automatic generation of DMA requests during the data phase, for outgoing data transfers from AES via DMA. Setting this bit is ignored when MODE\\[1:0\\]
is at 0x1 (key derivation)."]
pub type DmaoutenR = crate::BitReader<Dmaouten>;
impl DmaoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaouten {
        match self.bits {
            false => Dmaouten::B0x0,
            true => Dmaouten::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dmaouten::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dmaouten::B0x1
    }
}
#[doc = "Field `DMAOUTEN` writer - DMA output enable This bit enables automatic generation of DMA requests during the data phase, for outgoing data transfers from AES via DMA. Setting this bit is ignored when MODE\\[1:0\\]
is at 0x1 (key derivation)."]
pub type DmaoutenW<'a, REG> = crate::BitWriter<'a, REG, Dmaouten>;
impl<'a, REG> DmaoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaouten::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaouten::B0x1)
    }
}
#[doc = "GCM or CCM phase selection This bitfield selects the phase, applicable only with GCM, GMAC or CCM chaining modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gcmph {
    #[doc = "0: Initialization phase"]
    B0x0 = 0,
    #[doc = "1: Header phase"]
    B0x1 = 1,
    #[doc = "2: Payload phase"]
    B0x2 = 2,
    #[doc = "3: Final phase"]
    B0x3 = 3,
}
impl From<Gcmph> for u8 {
    #[inline(always)]
    fn from(variant: Gcmph) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gcmph {
    type Ux = u8;
}
impl crate::IsEnum for Gcmph {}
#[doc = "Field `GCMPH` reader - GCM or CCM phase selection This bitfield selects the phase, applicable only with GCM, GMAC or CCM chaining modes."]
pub type GcmphR = crate::FieldReader<Gcmph>;
impl GcmphR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gcmph {
        match self.bits {
            0 => Gcmph::B0x0,
            1 => Gcmph::B0x1,
            2 => Gcmph::B0x2,
            3 => Gcmph::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Initialization phase"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gcmph::B0x0
    }
    #[doc = "Header phase"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gcmph::B0x1
    }
    #[doc = "Payload phase"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Gcmph::B0x2
    }
    #[doc = "Final phase"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Gcmph::B0x3
    }
}
#[doc = "Field `GCMPH` writer - GCM or CCM phase selection This bitfield selects the phase, applicable only with GCM, GMAC or CCM chaining modes."]
pub type GcmphW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gcmph, crate::Safe>;
impl<'a, REG> GcmphW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Initialization phase"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gcmph::B0x0)
    }
    #[doc = "Header phase"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gcmph::B0x1)
    }
    #[doc = "Payload phase"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Gcmph::B0x2)
    }
    #[doc = "Final phase"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Gcmph::B0x3)
    }
}
#[doc = "Field `CHMOD_1` reader - CHMOD\\[2\\]"]
pub type Chmod1R = crate::BitReader;
#[doc = "Field `CHMOD_1` writer - CHMOD\\[2\\]"]
pub type Chmod1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Key size selection This bitfield defines the key length in bits of the key used by AES. Attempts to write the bit are ignored when the EN is set before the write access and it is not cleared by that write access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keysize {
    #[doc = "0: 128-bit"]
    B0x0 = 0,
    #[doc = "1: 256-bit"]
    B0x1 = 1,
}
impl From<Keysize> for bool {
    #[inline(always)]
    fn from(variant: Keysize) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSIZE` reader - Key size selection This bitfield defines the key length in bits of the key used by AES. Attempts to write the bit are ignored when the EN is set before the write access and it is not cleared by that write access."]
pub type KeysizeR = crate::BitReader<Keysize>;
impl KeysizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Keysize {
        match self.bits {
            false => Keysize::B0x0,
            true => Keysize::B0x1,
        }
    }
    #[doc = "128-bit"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Keysize::B0x0
    }
    #[doc = "256-bit"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Keysize::B0x1
    }
}
#[doc = "Field `KEYSIZE` writer - Key size selection This bitfield defines the key length in bits of the key used by AES. Attempts to write the bit are ignored when the EN is set before the write access and it is not cleared by that write access."]
pub type KeysizeW<'a, REG> = crate::BitWriter<'a, REG, Keysize>;
impl<'a, REG> KeysizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "128-bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Keysize::B0x0)
    }
    #[doc = "256-bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Keysize::B0x1)
    }
}
#[doc = "Number of padding bytes in last block This padding information must be filled by software before processing the last block of GCM payload encryption or CCM payload decryption, otherwise authentication tag computation is incorrect. ...\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Npblb {
    #[doc = "0: All bytes are valid (no padding)"]
    B0x0 = 0,
    #[doc = "1: Padding for the last LSB byte"]
    B0x1 = 1,
    #[doc = "15: Padding for the 15 LSB bytes of last block."]
    B0xF = 15,
}
impl From<Npblb> for u8 {
    #[inline(always)]
    fn from(variant: Npblb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Npblb {
    type Ux = u8;
}
impl crate::IsEnum for Npblb {}
#[doc = "Field `NPBLB` reader - Number of padding bytes in last block This padding information must be filled by software before processing the last block of GCM payload encryption or CCM payload decryption, otherwise authentication tag computation is incorrect. ..."]
pub type NpblbR = crate::FieldReader<Npblb>;
impl NpblbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Npblb> {
        match self.bits {
            0 => Some(Npblb::B0x0),
            1 => Some(Npblb::B0x1),
            15 => Some(Npblb::B0xF),
            _ => None,
        }
    }
    #[doc = "All bytes are valid (no padding)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Npblb::B0x0
    }
    #[doc = "Padding for the last LSB byte"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Npblb::B0x1
    }
    #[doc = "Padding for the 15 LSB bytes of last block."]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Npblb::B0xF
    }
}
#[doc = "Field `NPBLB` writer - Number of padding bytes in last block This padding information must be filled by software before processing the last block of GCM payload encryption or CCM payload decryption, otherwise authentication tag computation is incorrect. ..."]
pub type NpblbW<'a, REG> = crate::FieldWriter<'a, REG, 4, Npblb>;
impl<'a, REG> NpblbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All bytes are valid (no padding)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Npblb::B0x0)
    }
    #[doc = "Padding for the last LSB byte"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Npblb::B0x1)
    }
    #[doc = "Padding for the 15 LSB bytes of last block."]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Npblb::B0xF)
    }
}
#[doc = "Field `IPRST` reader - AES peripheral software reset Setting the bit resets the AES peripheral, putting all registers to their default values, except the IPRST bit itself. Hence, any key-relative data are lost. For this reason, it is recommended to set the bit before handing over the AES to a less secure application. The bit must be kept low while writing any configuration registers."]
pub type IprstR = crate::BitReader;
#[doc = "Field `IPRST` writer - AES peripheral software reset Setting the bit resets the AES peripheral, putting all registers to their default values, except the IPRST bit itself. Hence, any key-relative data are lost. For this reason, it is recommended to set the bit before handing over the AES to a less secure application. The bit must be kept low while writing any configuration registers."]
pub type IprstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable This bit enables/disables the AES peripheral. At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (MODE\\[1:0\\]
at 0x1) and upon the completion of GCM/GMAC/CCM initialization phase. The bit cannot be set as long as KEYVALID1is cleared"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data type This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping. This swapping is defined in Section121.4.14: AES data registers and data swapping. Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn datatype(&self) -> DatatypeR {
        DatatypeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - CHMOD\\[1:0\\]: Chaining mode This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn chmod(&self) -> ChmodR {
        ChmodR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 11 - DMA input enable This bit enables automatic generation of DMA requests during the data phase, for incoming data transfers to AES via DMA. Setting this bit is ignored when MODE\\[1:0\\]
is at 0x1 (key derivation)."]
    #[inline(always)]
    pub fn dmainen(&self) -> DmainenR {
        DmainenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA output enable This bit enables automatic generation of DMA requests during the data phase, for outgoing data transfers from AES via DMA. Setting this bit is ignored when MODE\\[1:0\\]
is at 0x1 (key derivation)."]
    #[inline(always)]
    pub fn dmaouten(&self) -> DmaoutenR {
        DmaoutenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - GCM or CCM phase selection This bitfield selects the phase, applicable only with GCM, GMAC or CCM chaining modes."]
    #[inline(always)]
    pub fn gcmph(&self) -> GcmphR {
        GcmphR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 16 - CHMOD\\[2\\]"]
    #[inline(always)]
    pub fn chmod_1(&self) -> Chmod1R {
        Chmod1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Key size selection This bitfield defines the key length in bits of the key used by AES. Attempts to write the bit are ignored when the EN is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn keysize(&self) -> KeysizeR {
        KeysizeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Number of padding bytes in last block This padding information must be filled by software before processing the last block of GCM payload encryption or CCM payload decryption, otherwise authentication tag computation is incorrect. ..."]
    #[inline(always)]
    pub fn npblb(&self) -> NpblbR {
        NpblbR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - AES peripheral software reset Setting the bit resets the AES peripheral, putting all registers to their default values, except the IPRST bit itself. Hence, any key-relative data are lost. For this reason, it is recommended to set the bit before handing over the AES to a less secure application. The bit must be kept low while writing any configuration registers."]
    #[inline(always)]
    pub fn iprst(&self) -> IprstR {
        IprstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable This bit enables/disables the AES peripheral. At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (MODE\\[1:0\\]
at 0x1) and upon the completion of GCM/GMAC/CCM initialization phase. The bit cannot be set as long as KEYVALID1is cleared"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<AesCrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Data type This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping. This swapping is defined in Section121.4.14: AES data registers and data swapping. Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    #[must_use]
    pub fn datatype(&mut self) -> DatatypeW<AesCrSpec> {
        DatatypeW::new(self, 1)
    }
    #[doc = "Bits 3:4 - Operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<AesCrSpec> {
        ModeW::new(self, 3)
    }
    #[doc = "Bits 5:6 - CHMOD\\[1:0\\]: Chaining mode This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    #[must_use]
    pub fn chmod(&mut self) -> ChmodW<AesCrSpec> {
        ChmodW::new(self, 5)
    }
    #[doc = "Bit 11 - DMA input enable This bit enables automatic generation of DMA requests during the data phase, for incoming data transfers to AES via DMA. Setting this bit is ignored when MODE\\[1:0\\]
is at 0x1 (key derivation)."]
    #[inline(always)]
    #[must_use]
    pub fn dmainen(&mut self) -> DmainenW<AesCrSpec> {
        DmainenW::new(self, 11)
    }
    #[doc = "Bit 12 - DMA output enable This bit enables automatic generation of DMA requests during the data phase, for outgoing data transfers from AES via DMA. Setting this bit is ignored when MODE\\[1:0\\]
is at 0x1 (key derivation)."]
    #[inline(always)]
    #[must_use]
    pub fn dmaouten(&mut self) -> DmaoutenW<AesCrSpec> {
        DmaoutenW::new(self, 12)
    }
    #[doc = "Bits 13:14 - GCM or CCM phase selection This bitfield selects the phase, applicable only with GCM, GMAC or CCM chaining modes."]
    #[inline(always)]
    #[must_use]
    pub fn gcmph(&mut self) -> GcmphW<AesCrSpec> {
        GcmphW::new(self, 13)
    }
    #[doc = "Bit 16 - CHMOD\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn chmod_1(&mut self) -> Chmod1W<AesCrSpec> {
        Chmod1W::new(self, 16)
    }
    #[doc = "Bit 18 - Key size selection This bitfield defines the key length in bits of the key used by AES. Attempts to write the bit are ignored when the EN is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    #[must_use]
    pub fn keysize(&mut self) -> KeysizeW<AesCrSpec> {
        KeysizeW::new(self, 18)
    }
    #[doc = "Bits 20:23 - Number of padding bytes in last block This padding information must be filled by software before processing the last block of GCM payload encryption or CCM payload decryption, otherwise authentication tag computation is incorrect. ..."]
    #[inline(always)]
    #[must_use]
    pub fn npblb(&mut self) -> NpblbW<AesCrSpec> {
        NpblbW::new(self, 20)
    }
    #[doc = "Bit 31 - AES peripheral software reset Setting the bit resets the AES peripheral, putting all registers to their default values, except the IPRST bit itself. Hence, any key-relative data are lost. For this reason, it is recommended to set the bit before handing over the AES to a less secure application. The bit must be kept low while writing any configuration registers."]
    #[inline(always)]
    #[must_use]
    pub fn iprst(&mut self) -> IprstW<AesCrSpec> {
        IprstW::new(self, 31)
    }
}
#[doc = "AES control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesCrSpec;
impl crate::RegisterSpec for AesCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_cr::R`](R) reader structure"]
impl crate::Readable for AesCrSpec {}
#[doc = "`write(|w| ..)` method takes [`aes_cr::W`](W) writer structure"]
impl crate::Writable for AesCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_CR to value 0"]
impl crate::Resettable for AesCrSpec {
    const RESET_VALUE: u32 = 0;
}
