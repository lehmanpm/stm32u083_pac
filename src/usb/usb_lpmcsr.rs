#[doc = "Register `USB_LPMCSR` reader"]
pub type R = crate::R<UsbLpmcsrSpec>;
#[doc = "Register `USB_LPMCSR` writer"]
pub type W = crate::W<UsbLpmcsrSpec>;
#[doc = "Field `LPMEN` reader - LPM support enable Device mode This bit is set by the software to enable the LPM support within the USB Device. If this bit is at 0 no LPM transactions are handled."]
pub type LpmenR = crate::BitReader;
#[doc = "Field `LPMEN` writer - LPM support enable Device mode This bit is set by the software to enable the LPM support within the USB Device. If this bit is at 0 no LPM transactions are handled."]
pub type LpmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpmack {
    #[doc = "0: the valid LPM token is NYET."]
    B0x0 = 0,
    #[doc = "1: the valid LPM token is ACK."]
    B0x1 = 1,
}
impl From<Lpmack> for bool {
    #[inline(always)]
    fn from(variant: Lpmack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPMACK` reader - LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)"]
pub type LpmackR = crate::BitReader<Lpmack>;
impl LpmackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpmack {
        match self.bits {
            false => Lpmack::B0x0,
            true => Lpmack::B0x1,
        }
    }
    #[doc = "the valid LPM token is NYET."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpmack::B0x0
    }
    #[doc = "the valid LPM token is ACK."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpmack::B0x1
    }
}
#[doc = "Field `LPMACK` writer - LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)"]
pub type LpmackW<'a, REG> = crate::BitWriter<'a, REG, Lpmack>;
impl<'a, REG> LpmackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the valid LPM token is NYET."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmack::B0x0)
    }
    #[doc = "the valid LPM token is ACK."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmack::B0x1)
    }
}
#[doc = "Field `REMWAKE` reader - bRemoteWake value Device mode This bit contains the bRemoteWake value received with last ACKed LPM Token"]
pub type RemwakeR = crate::BitReader;
#[doc = "Field `BESL` reader - BESL value Device mode These bits contain the BESL value received with last ACKed LPM Token"]
pub type BeslR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - LPM support enable Device mode This bit is set by the software to enable the LPM support within the USB Device. If this bit is at 0 no LPM transactions are handled."]
    #[inline(always)]
    pub fn lpmen(&self) -> LpmenR {
        LpmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)"]
    #[inline(always)]
    pub fn lpmack(&self) -> LpmackR {
        LpmackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - bRemoteWake value Device mode This bit contains the bRemoteWake value received with last ACKed LPM Token"]
    #[inline(always)]
    pub fn remwake(&self) -> RemwakeR {
        RemwakeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - BESL value Device mode These bits contain the BESL value received with last ACKed LPM Token"]
    #[inline(always)]
    pub fn besl(&self) -> BeslR {
        BeslR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LPM support enable Device mode This bit is set by the software to enable the LPM support within the USB Device. If this bit is at 0 no LPM transactions are handled."]
    #[inline(always)]
    #[must_use]
    pub fn lpmen(&mut self) -> LpmenW<UsbLpmcsrSpec> {
        LpmenW::new(self, 0)
    }
    #[doc = "Bit 1 - LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)"]
    #[inline(always)]
    #[must_use]
    pub fn lpmack(&mut self) -> LpmackW<UsbLpmcsrSpec> {
        LpmackW::new(self, 1)
    }
}
#[doc = "LPM control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_lpmcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_lpmcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbLpmcsrSpec;
impl crate::RegisterSpec for UsbLpmcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_lpmcsr::R`](R) reader structure"]
impl crate::Readable for UsbLpmcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_lpmcsr::W`](W) writer structure"]
impl crate::Writable for UsbLpmcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB_LPMCSR to value 0"]
impl crate::Resettable for UsbLpmcsrSpec {
    const RESET_VALUE: u32 = 0;
}
