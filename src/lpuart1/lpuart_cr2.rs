#[doc = "Register `LPUART_CR2` reader"]
pub type R = crate::R<LpuartCr2Spec>;
#[doc = "Register `LPUART_CR2` writer"]
pub type W = crate::W<LpuartCr2Spec>;
#[doc = "7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the LPUART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\]
and ADD\\[7:0\\]) respectively.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addm7 {
    #[doc = "0: 4-bit address detection"]
    B0x0 = 0,
    #[doc = "1: 7-bit address detection (in 8-bit data mode)"]
    B0x1 = 1,
}
impl From<Addm7> for bool {
    #[inline(always)]
    fn from(variant: Addm7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the LPUART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\]
and ADD\\[7:0\\]) respectively."]
pub type Addm7R = crate::BitReader<Addm7>;
impl Addm7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addm7 {
        match self.bits {
            false => Addm7::B0x0,
            true => Addm7::B0x1,
        }
    }
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Addm7::B0x0
    }
    #[doc = "7-bit address detection (in 8-bit data mode)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Addm7::B0x1
    }
}
#[doc = "Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the LPUART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\]
and ADD\\[7:0\\]) respectively."]
pub type Addm7W<'a, REG> = crate::BitWriter<'a, REG, Addm7>;
impl<'a, REG> Addm7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Addm7::B0x0)
    }
    #[doc = "7-bit address detection (in 8-bit data mode)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Addm7::B0x1)
    }
}
#[doc = "STOP bits These bits are used for programming the stop bits. This bitfield can only be written when the LPUART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stop {
    #[doc = "0: 1 stop bit"]
    B0x0 = 0,
    #[doc = "1: Reserved."]
    B0x1 = 1,
    #[doc = "2: 2 stop bits"]
    B0x2 = 2,
}
impl From<Stop> for u8 {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stop {
    type Ux = u8;
}
impl crate::IsEnum for Stop {}
#[doc = "Field `STOP` reader - STOP bits These bits are used for programming the stop bits. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type StopR = crate::FieldReader<Stop>;
impl StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stop> {
        match self.bits {
            0 => Some(Stop::B0x0),
            1 => Some(Stop::B0x1),
            2 => Some(Stop::B0x2),
            _ => None,
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Stop::B0x0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Stop::B0x1
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Stop::B0x2
    }
}
#[doc = "Field `STOP` writer - STOP bits These bits are used for programming the stop bits. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 2, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::B0x0)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::B0x1)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::B0x2)
    }
}
#[doc = "Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swap {
    #[doc = "0: TX/RX pins are used as defined in standard pinout"]
    B0x0 = 0,
    #[doc = "1: The TX and RX pins functions are swapped. This enables to work in the case of a cross-wired connection to another UART."]
    B0x1 = 1,
}
impl From<Swap> for bool {
    #[inline(always)]
    fn from(variant: Swap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAP` reader - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type SwapR = crate::BitReader<Swap>;
impl SwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swap {
        match self.bits {
            false => Swap::B0x0,
            true => Swap::B0x1,
        }
    }
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swap::B0x0
    }
    #[doc = "The TX and RX pins functions are swapped. This enables to work in the case of a cross-wired connection to another UART."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swap::B0x1
    }
}
#[doc = "Field `SWAP` writer - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG, Swap>;
impl<'a, REG> SwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swap::B0x0)
    }
    #[doc = "The TX and RX pins functions are swapped. This enables to work in the case of a cross-wired connection to another UART."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swap::B0x1)
    }
}
#[doc = "RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the LPUART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxinv {
    #[doc = "0: RX pin signal works using the standard logic levels (V&lt;sub>DD&lt;/sub> =1/idle, Gnd=0/mark)"]
    B0x0 = 0,
    #[doc = "1: RX pin signal values are inverted. ((V&lt;sub>DD&lt;/sub> =0/mark, Gnd=1/idle)."]
    B0x1 = 1,
}
impl From<Rxinv> for bool {
    #[inline(always)]
    fn from(variant: Rxinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINV` reader - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type RxinvR = crate::BitReader<Rxinv>;
impl RxinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxinv {
        match self.bits {
            false => Rxinv::B0x0,
            true => Rxinv::B0x1,
        }
    }
    #[doc = "RX pin signal works using the standard logic levels (V&lt;sub>DD&lt;/sub> =1/idle, Gnd=0/mark)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxinv::B0x0
    }
    #[doc = "RX pin signal values are inverted. ((V&lt;sub>DD&lt;/sub> =0/mark, Gnd=1/idle)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxinv::B0x1
    }
}
#[doc = "Field `RXINV` writer - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG, Rxinv>;
impl<'a, REG> RxinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX pin signal works using the standard logic levels (V&lt;sub>DD&lt;/sub> =1/idle, Gnd=0/mark)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinv::B0x0)
    }
    #[doc = "RX pin signal values are inverted. ((V&lt;sub>DD&lt;/sub> =0/mark, Gnd=1/idle)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinv::B0x1)
    }
}
#[doc = "TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the LPUART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txinv {
    #[doc = "0: TX pin signal works using the standard logic levels (V&lt;sub>DD&lt;/sub> =1/idle, Gnd=0/mark)"]
    B0x0 = 0,
    #[doc = "1: TX pin signal values are inverted. ((V&lt;sub>DD&lt;/sub> =0/mark, Gnd=1/idle)."]
    B0x1 = 1,
}
impl From<Txinv> for bool {
    #[inline(always)]
    fn from(variant: Txinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINV` reader - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type TxinvR = crate::BitReader<Txinv>;
impl TxinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txinv {
        match self.bits {
            false => Txinv::B0x0,
            true => Txinv::B0x1,
        }
    }
    #[doc = "TX pin signal works using the standard logic levels (V&lt;sub>DD&lt;/sub> =1/idle, Gnd=0/mark)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txinv::B0x0
    }
    #[doc = "TX pin signal values are inverted. ((V&lt;sub>DD&lt;/sub> =0/mark, Gnd=1/idle)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txinv::B0x1
    }
}
#[doc = "Field `TXINV` writer - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG, Txinv>;
impl<'a, REG> TxinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX pin signal works using the standard logic levels (V&lt;sub>DD&lt;/sub> =1/idle, Gnd=0/mark)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Txinv::B0x0)
    }
    #[doc = "TX pin signal values are inverted. ((V&lt;sub>DD&lt;/sub> =0/mark, Gnd=1/idle)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Txinv::B0x1)
    }
}
#[doc = "Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datainv {
    #[doc = "0: Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L)"]
    B0x0 = 0,
    #[doc = "1: Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted."]
    B0x1 = 1,
}
impl From<Datainv> for bool {
    #[inline(always)]
    fn from(variant: Datainv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAINV` reader - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type DatainvR = crate::BitReader<Datainv>;
impl DatainvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datainv {
        match self.bits {
            false => Datainv::B0x0,
            true => Datainv::B0x1,
        }
    }
    #[doc = "Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Datainv::B0x0
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Datainv::B0x1
    }
}
#[doc = "Field `DATAINV` writer - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type DatainvW<'a, REG> = crate::BitWriter<'a, REG, Datainv>;
impl<'a, REG> DatainvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Datainv::B0x0)
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Datainv::B0x1)
    }
}
#[doc = "Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msbfirst {
    #[doc = "0: data is transmitted/received with data bit 0 first, following the start bit."]
    B0x0 = 0,
    #[doc = "1: data is transmitted/received with the MSB (bit 7/8) first, following the start bit."]
    B0x1 = 1,
}
impl From<Msbfirst> for bool {
    #[inline(always)]
    fn from(variant: Msbfirst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBFIRST` reader - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type MsbfirstR = crate::BitReader<Msbfirst>;
impl MsbfirstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msbfirst {
        match self.bits {
            false => Msbfirst::B0x0,
            true => Msbfirst::B0x1,
        }
    }
    #[doc = "data is transmitted/received with data bit 0 first, following the start bit."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Msbfirst::B0x0
    }
    #[doc = "data is transmitted/received with the MSB (bit 7/8) first, following the start bit."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Msbfirst::B0x1
    }
}
#[doc = "Field `MSBFIRST` writer - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type MsbfirstW<'a, REG> = crate::BitWriter<'a, REG, Msbfirst>;
impl<'a, REG> MsbfirstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "data is transmitted/received with data bit 0 first, following the start bit."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Msbfirst::B0x0)
    }
    #[doc = "data is transmitted/received with the MSB (bit 7/8) first, following the start bit."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Msbfirst::B0x1)
    }
}
#[doc = "Field `ADD` reader - Address of the LPUART node These bits give the address of the LPUART node in Mute mode or a character code to be recognized in low-power or Run mode: In Mute mode: they are used in multiprocessor communication to wake up from Mute mode with 4-bit/7-bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. In 4-bit address mark detection, only ADD\\[3:0\\]
bits are used. In low-power mode: they are used for wake up from low-power mode on character match. When WUS\\[1:0\\]
is programmed to 0b00 (WUF active on address match), the wake-up from low-power mode is performed when the received character corresponds to the character programmed through ADD\\[6:0\\]
or ADD\\[3:0\\]
bitfield (depending on ADDM7 bit), and WUF interrupt is enabled by setting WUFIE bit. The MSB of the character sent by transmitter should be equal to 1. In Run mode with Mute mode inactive (for example, end-of-block detection in ModBus protocol): the whole received character (8 bits) is compared to ADD\\[7:0\\]
value and CMF flag is set on match. An interrupt is generated if the CMIE bit is set. These bits can only be written when the reception is disabled (RE1=10) or when the USART is disabled (UE1=10)."]
pub type AddR = crate::FieldReader;
#[doc = "Field `ADD` writer - Address of the LPUART node These bits give the address of the LPUART node in Mute mode or a character code to be recognized in low-power or Run mode: In Mute mode: they are used in multiprocessor communication to wake up from Mute mode with 4-bit/7-bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. In 4-bit address mark detection, only ADD\\[3:0\\]
bits are used. In low-power mode: they are used for wake up from low-power mode on character match. When WUS\\[1:0\\]
is programmed to 0b00 (WUF active on address match), the wake-up from low-power mode is performed when the received character corresponds to the character programmed through ADD\\[6:0\\]
or ADD\\[3:0\\]
bitfield (depending on ADDM7 bit), and WUF interrupt is enabled by setting WUFIE bit. The MSB of the character sent by transmitter should be equal to 1. In Run mode with Mute mode inactive (for example, end-of-block detection in ModBus protocol): the whole received character (8 bits) is compared to ADD\\[7:0\\]
value and CMF flag is set on match. An interrupt is generated if the CMIE bit is set. These bits can only be written when the reception is disabled (RE1=10) or when the USART is disabled (UE1=10)."]
pub type AddW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the LPUART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\]
and ADD\\[7:0\\]) respectively."]
    #[inline(always)]
    pub fn addm7(&self) -> Addm7R {
        Addm7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits These bits are used for programming the stop bits. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn datainv(&self) -> DatainvR {
        DatainvR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn msbfirst(&self) -> MsbfirstR {
        MsbfirstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Address of the LPUART node These bits give the address of the LPUART node in Mute mode or a character code to be recognized in low-power or Run mode: In Mute mode: they are used in multiprocessor communication to wake up from Mute mode with 4-bit/7-bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. In 4-bit address mark detection, only ADD\\[3:0\\]
bits are used. In low-power mode: they are used for wake up from low-power mode on character match. When WUS\\[1:0\\]
is programmed to 0b00 (WUF active on address match), the wake-up from low-power mode is performed when the received character corresponds to the character programmed through ADD\\[6:0\\]
or ADD\\[3:0\\]
bitfield (depending on ADDM7 bit), and WUF interrupt is enabled by setting WUFIE bit. The MSB of the character sent by transmitter should be equal to 1. In Run mode with Mute mode inactive (for example, end-of-block detection in ModBus protocol): the whole received character (8 bits) is compared to ADD\\[7:0\\]
value and CMF flag is set on match. An interrupt is generated if the CMIE bit is set. These bits can only be written when the reception is disabled (RE1=10) or when the USART is disabled (UE1=10)."]
    #[inline(always)]
    pub fn add(&self) -> AddR {
        AddR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the LPUART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\]
and ADD\\[7:0\\]) respectively."]
    #[inline(always)]
    #[must_use]
    pub fn addm7(&mut self) -> Addm7W<LpuartCr2Spec> {
        Addm7W::new(self, 4)
    }
    #[doc = "Bits 12:13 - STOP bits These bits are used for programming the stop bits. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<LpuartCr2Spec> {
        StopW::new(self, 12)
    }
    #[doc = "Bit 15 - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SwapW<LpuartCr2Spec> {
        SwapW::new(self, 15)
    }
    #[doc = "Bit 16 - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RxinvW<LpuartCr2Spec> {
        RxinvW::new(self, 16)
    }
    #[doc = "Bit 17 - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TxinvW<LpuartCr2Spec> {
        TxinvW::new(self, 17)
    }
    #[doc = "Bit 18 - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn datainv(&mut self) -> DatainvW<LpuartCr2Spec> {
        DatainvW::new(self, 18)
    }
    #[doc = "Bit 19 - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn msbfirst(&mut self) -> MsbfirstW<LpuartCr2Spec> {
        MsbfirstW::new(self, 19)
    }
    #[doc = "Bits 24:31 - Address of the LPUART node These bits give the address of the LPUART node in Mute mode or a character code to be recognized in low-power or Run mode: In Mute mode: they are used in multiprocessor communication to wake up from Mute mode with 4-bit/7-bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. In 4-bit address mark detection, only ADD\\[3:0\\]
bits are used. In low-power mode: they are used for wake up from low-power mode on character match. When WUS\\[1:0\\]
is programmed to 0b00 (WUF active on address match), the wake-up from low-power mode is performed when the received character corresponds to the character programmed through ADD\\[6:0\\]
or ADD\\[3:0\\]
bitfield (depending on ADDM7 bit), and WUF interrupt is enabled by setting WUFIE bit. The MSB of the character sent by transmitter should be equal to 1. In Run mode with Mute mode inactive (for example, end-of-block detection in ModBus protocol): the whole received character (8 bits) is compared to ADD\\[7:0\\]
value and CMF flag is set on match. An interrupt is generated if the CMIE bit is set. These bits can only be written when the reception is disabled (RE1=10) or when the USART is disabled (UE1=10)."]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> AddW<LpuartCr2Spec> {
        AddW::new(self, 24)
    }
}
#[doc = "LPUART control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpuart_cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpuart_cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpuartCr2Spec;
impl crate::RegisterSpec for LpuartCr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpuart_cr2::R`](R) reader structure"]
impl crate::Readable for LpuartCr2Spec {}
#[doc = "`write(|w| ..)` method takes [`lpuart_cr2::W`](W) writer structure"]
impl crate::Writable for LpuartCr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPUART_CR2 to value 0"]
impl crate::Resettable for LpuartCr2Spec {
    const RESET_VALUE: u32 = 0;
}
