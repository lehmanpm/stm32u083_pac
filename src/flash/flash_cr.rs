#[doc = "Register `FLASH_CR` reader"]
pub type R = crate::R<FlashCrSpec>;
#[doc = "Register `FLASH_CR` writer"]
pub type W = crate::W<FlashCrSpec>;
#[doc = "Flash memory programming enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pg {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Pg> for bool {
    #[inline(always)]
    fn from(variant: Pg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PG` reader - Flash memory programming enable"]
pub type PgR = crate::BitReader<Pg>;
impl PgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pg {
        match self.bits {
            false => Pg::B0x0,
            true => Pg::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pg::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pg::B0x1
    }
}
#[doc = "Field `PG` writer - Flash memory programming enable"]
pub type PgW<'a, REG> = crate::BitWriter<'a, REG, Pg>;
impl<'a, REG> PgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pg::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pg::B0x1)
    }
}
#[doc = "Page erase enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Per {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Per> for bool {
    #[inline(always)]
    fn from(variant: Per) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER` reader - Page erase enable"]
pub type PerR = crate::BitReader<Per>;
impl PerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Per {
        match self.bits {
            false => Per::B0x0,
            true => Per::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Per::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Per::B0x1
    }
}
#[doc = "Field `PER` writer - Page erase enable"]
pub type PerW<'a, REG> = crate::BitWriter<'a, REG, Per>;
impl<'a, REG> PerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Per::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Per::B0x1)
    }
}
#[doc = "Field `MER1` reader - Mass erase When set, this bit triggers the mass erase, that is, all user pages."]
pub type Mer1R = crate::BitReader;
#[doc = "Field `MER1` writer - Mass erase When set, this bit triggers the mass erase, that is, all user pages."]
pub type Mer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Page number selection These bits select the page to erase: ... Note: Values corresponding to addresses outside the main memory are not allowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pnb {
    #[doc = "0: page 0"]
    B0x0 = 0,
    #[doc = "1: page 1"]
    B0x1 = 1,
    #[doc = "15: page 15"]
    B0xF = 15,
}
impl From<Pnb> for u8 {
    #[inline(always)]
    fn from(variant: Pnb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pnb {
    type Ux = u8;
}
impl crate::IsEnum for Pnb {}
#[doc = "Field `PNB` reader - Page number selection These bits select the page to erase: ... Note: Values corresponding to addresses outside the main memory are not allowed."]
pub type PnbR = crate::FieldReader<Pnb>;
impl PnbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pnb> {
        match self.bits {
            0 => Some(Pnb::B0x0),
            1 => Some(Pnb::B0x1),
            15 => Some(Pnb::B0xF),
            _ => None,
        }
    }
    #[doc = "page 0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pnb::B0x0
    }
    #[doc = "page 1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pnb::B0x1
    }
    #[doc = "page 15"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Pnb::B0xF
    }
}
#[doc = "Field `PNB` writer - Page number selection These bits select the page to erase: ... Note: Values corresponding to addresses outside the main memory are not allowed."]
pub type PnbW<'a, REG> = crate::FieldWriter<'a, REG, 7, Pnb>;
impl<'a, REG> PnbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "page 0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pnb::B0x0)
    }
    #[doc = "page 1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pnb::B0x1)
    }
    #[doc = "page 15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Pnb::B0xF)
    }
}
#[doc = "Field `STRT` reader - Start erase operation This bit triggers an erase operation when set. This bit is possible to set only by software and to clear only by hardware. The hardware clears it when one of BSY1 and BSY2 flags in the FLASH_SR register transits to zero."]
pub type StrtR = crate::BitReader;
#[doc = "Field `STRT` writer - Start erase operation This bit triggers an erase operation when set. This bit is possible to set only by software and to clear only by hardware. The hardware clears it when one of BSY1 and BSY2 flags in the FLASH_SR register transits to zero."]
pub type StrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTSTRT` reader - Start of modification of option bytes This bit triggers an options operation when set. This bit is set only by software, and is cleared when the BSY1 bit is cleared in FLASH_SR."]
pub type OptstrtR = crate::BitReader;
#[doc = "Field `OPTSTRT` writer - Start of modification of option bytes This bit triggers an options operation when set. This bit is set only by software, and is cleared when the BSY1 bit is cleared in FLASH_SR."]
pub type OptstrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Fast programming enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fstpg {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Fstpg> for bool {
    #[inline(always)]
    fn from(variant: Fstpg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSTPG` reader - Fast programming enable"]
pub type FstpgR = crate::BitReader<Fstpg>;
impl FstpgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fstpg {
        match self.bits {
            false => Fstpg::B0x0,
            true => Fstpg::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fstpg::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fstpg::B0x1
    }
}
#[doc = "Field `FSTPG` writer - Fast programming enable"]
pub type FstpgW<'a, REG> = crate::BitWriter<'a, REG, Fstpg>;
impl<'a, REG> FstpgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Fstpg::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Fstpg::B0x1)
    }
}
#[doc = "End-of-operation interrupt enable This bit enables the interrupt generation upon setting the EOP flag in the FLASH_SR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eopie {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Eopie> for bool {
    #[inline(always)]
    fn from(variant: Eopie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOPIE` reader - End-of-operation interrupt enable This bit enables the interrupt generation upon setting the EOP flag in the FLASH_SR register."]
pub type EopieR = crate::BitReader<Eopie>;
impl EopieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eopie {
        match self.bits {
            false => Eopie::B0x0,
            true => Eopie::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eopie::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eopie::B0x1
    }
}
#[doc = "Field `EOPIE` writer - End-of-operation interrupt enable This bit enables the interrupt generation upon setting the EOP flag in the FLASH_SR register."]
pub type EopieW<'a, REG> = crate::BitWriter<'a, REG, Eopie>;
impl<'a, REG> EopieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eopie::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eopie::B0x1)
    }
}
#[doc = "Error interrupt enable This bit enables the interrupt generation upon setting the OPERR flag in the FLASH_SR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errie {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Errie> for bool {
    #[inline(always)]
    fn from(variant: Errie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable This bit enables the interrupt generation upon setting the OPERR flag in the FLASH_SR register."]
pub type ErrieR = crate::BitReader<Errie>;
impl ErrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errie {
        match self.bits {
            false => Errie::B0x0,
            true => Errie::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Errie::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Errie::B0x1
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable This bit enables the interrupt generation upon setting the OPERR flag in the FLASH_SR register."]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG, Errie>;
impl<'a, REG> ErrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::B0x1)
    }
}
#[doc = "PCROP read error interrupt enable This bit enables the interrupt generation upon setting the RDERR flag in the FLASH_SR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rderrie {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rderrie> for bool {
    #[inline(always)]
    fn from(variant: Rderrie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDERRIE` reader - PCROP read error interrupt enable This bit enables the interrupt generation upon setting the RDERR flag in the FLASH_SR register."]
pub type RderrieR = crate::BitReader<Rderrie>;
impl RderrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rderrie {
        match self.bits {
            false => Rderrie::B0x0,
            true => Rderrie::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rderrie::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rderrie::B0x1
    }
}
#[doc = "Field `RDERRIE` writer - PCROP read error interrupt enable This bit enables the interrupt generation upon setting the RDERR flag in the FLASH_SR register."]
pub type RderrieW<'a, REG> = crate::BitWriter<'a, REG, Rderrie>;
impl<'a, REG> RderrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rderrie::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rderrie::B0x1)
    }
}
#[doc = "Field `OBL_LAUNCH` reader - Option byte load launch When set, this bit triggers the load of option bytes into option registers. It is automatically cleared upon the completion of the load. The high state of the bit indicates pending option byte load. The bit cannot be cleared by software. It cannot be written as long as OPTLOCK is set."]
pub type OblLaunchR = crate::BitReader;
#[doc = "Field `OBL_LAUNCH` writer - Option byte load launch When set, this bit triggers the load of option bytes into option registers. It is automatically cleared upon the completion of the load. The high state of the bit indicates pending option byte load. The bit cannot be cleared by software. It cannot be written as long as OPTLOCK is set."]
pub type OblLaunchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Securable memory area protection enable This bit enables the protection on securable area, provided that a non-null securable memory area size (SEC_SIZE\\[4:0\\]) is defined in option bytes. This bit is possible to set only by software and to clear only through a system reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecProt {
    #[doc = "0: Disable (securable area accessible)"]
    B0x0 = 0,
    #[doc = "1: Enable (securable area not accessible)"]
    B0x1 = 1,
}
impl From<SecProt> for bool {
    #[inline(always)]
    fn from(variant: SecProt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEC_PROT` reader - Securable memory area protection enable This bit enables the protection on securable area, provided that a non-null securable memory area size (SEC_SIZE\\[4:0\\]) is defined in option bytes. This bit is possible to set only by software and to clear only through a system reset."]
pub type SecProtR = crate::BitReader<SecProt>;
impl SecProtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecProt {
        match self.bits {
            false => SecProt::B0x0,
            true => SecProt::B0x1,
        }
    }
    #[doc = "Disable (securable area accessible)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SecProt::B0x0
    }
    #[doc = "Enable (securable area not accessible)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SecProt::B0x1
    }
}
#[doc = "Field `SEC_PROT` writer - Securable memory area protection enable This bit enables the protection on securable area, provided that a non-null securable memory area size (SEC_SIZE\\[4:0\\]) is defined in option bytes. This bit is possible to set only by software and to clear only through a system reset."]
pub type SecProtW<'a, REG> = crate::BitWriter<'a, REG, SecProt>;
impl<'a, REG> SecProtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (securable area accessible)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SecProt::B0x0)
    }
    #[doc = "Enable (securable area not accessible)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SecProt::B0x1)
    }
}
#[doc = "Field `OPTLOCK` reader - Options Lock This bit is set only. When set, all bits concerning user option in FLASH_CR register and so option page are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset."]
pub type OptlockR = crate::BitReader;
#[doc = "Field `OPTLOCK` writer - Options Lock This bit is set only. When set, all bits concerning user option in FLASH_CR register and so option page are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset."]
pub type OptlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - FLASH_CR Lock This bit is set only. When set, the FLASH_CR register is locked. It is cleared by hardware after detecting the unlock sequence. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - FLASH_CR Lock This bit is set only. When set, the FLASH_CR register is locked. It is cleared by hardware after detecting the unlock sequence. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flash memory programming enable"]
    #[inline(always)]
    pub fn pg(&self) -> PgR {
        PgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page erase enable"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mass erase When set, this bit triggers the mass erase, that is, all user pages."]
    #[inline(always)]
    pub fn mer1(&self) -> Mer1R {
        Mer1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:9 - Page number selection These bits select the page to erase: ... Note: Values corresponding to addresses outside the main memory are not allowed."]
    #[inline(always)]
    pub fn pnb(&self) -> PnbR {
        PnbR::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Start erase operation This bit triggers an erase operation when set. This bit is possible to set only by software and to clear only by hardware. The hardware clears it when one of BSY1 and BSY2 flags in the FLASH_SR register transits to zero."]
    #[inline(always)]
    pub fn strt(&self) -> StrtR {
        StrtR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Start of modification of option bytes This bit triggers an options operation when set. This bit is set only by software, and is cleared when the BSY1 bit is cleared in FLASH_SR."]
    #[inline(always)]
    pub fn optstrt(&self) -> OptstrtR {
        OptstrtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast programming enable"]
    #[inline(always)]
    pub fn fstpg(&self) -> FstpgR {
        FstpgR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - End-of-operation interrupt enable This bit enables the interrupt generation upon setting the EOP flag in the FLASH_SR register."]
    #[inline(always)]
    pub fn eopie(&self) -> EopieR {
        EopieR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Error interrupt enable This bit enables the interrupt generation upon setting the OPERR flag in the FLASH_SR register."]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PCROP read error interrupt enable This bit enables the interrupt generation upon setting the RDERR flag in the FLASH_SR register."]
    #[inline(always)]
    pub fn rderrie(&self) -> RderrieR {
        RderrieR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Option byte load launch When set, this bit triggers the load of option bytes into option registers. It is automatically cleared upon the completion of the load. The high state of the bit indicates pending option byte load. The bit cannot be cleared by software. It cannot be written as long as OPTLOCK is set."]
    #[inline(always)]
    pub fn obl_launch(&self) -> OblLaunchR {
        OblLaunchR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Securable memory area protection enable This bit enables the protection on securable area, provided that a non-null securable memory area size (SEC_SIZE\\[4:0\\]) is defined in option bytes. This bit is possible to set only by software and to clear only through a system reset."]
    #[inline(always)]
    pub fn sec_prot(&self) -> SecProtR {
        SecProtR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Options Lock This bit is set only. When set, all bits concerning user option in FLASH_CR register and so option page are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset."]
    #[inline(always)]
    pub fn optlock(&self) -> OptlockR {
        OptlockR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FLASH_CR Lock This bit is set only. When set, the FLASH_CR register is locked. It is cleared by hardware after detecting the unlock sequence. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash memory programming enable"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PgW<FlashCrSpec> {
        PgW::new(self, 0)
    }
    #[doc = "Bit 1 - Page erase enable"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PerW<FlashCrSpec> {
        PerW::new(self, 1)
    }
    #[doc = "Bit 2 - Mass erase When set, this bit triggers the mass erase, that is, all user pages."]
    #[inline(always)]
    #[must_use]
    pub fn mer1(&mut self) -> Mer1W<FlashCrSpec> {
        Mer1W::new(self, 2)
    }
    #[doc = "Bits 3:9 - Page number selection These bits select the page to erase: ... Note: Values corresponding to addresses outside the main memory are not allowed."]
    #[inline(always)]
    #[must_use]
    pub fn pnb(&mut self) -> PnbW<FlashCrSpec> {
        PnbW::new(self, 3)
    }
    #[doc = "Bit 16 - Start erase operation This bit triggers an erase operation when set. This bit is possible to set only by software and to clear only by hardware. The hardware clears it when one of BSY1 and BSY2 flags in the FLASH_SR register transits to zero."]
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> StrtW<FlashCrSpec> {
        StrtW::new(self, 16)
    }
    #[doc = "Bit 17 - Start of modification of option bytes This bit triggers an options operation when set. This bit is set only by software, and is cleared when the BSY1 bit is cleared in FLASH_SR."]
    #[inline(always)]
    #[must_use]
    pub fn optstrt(&mut self) -> OptstrtW<FlashCrSpec> {
        OptstrtW::new(self, 17)
    }
    #[doc = "Bit 18 - Fast programming enable"]
    #[inline(always)]
    #[must_use]
    pub fn fstpg(&mut self) -> FstpgW<FlashCrSpec> {
        FstpgW::new(self, 18)
    }
    #[doc = "Bit 24 - End-of-operation interrupt enable This bit enables the interrupt generation upon setting the EOP flag in the FLASH_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EopieW<FlashCrSpec> {
        EopieW::new(self, 24)
    }
    #[doc = "Bit 25 - Error interrupt enable This bit enables the interrupt generation upon setting the OPERR flag in the FLASH_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ErrieW<FlashCrSpec> {
        ErrieW::new(self, 25)
    }
    #[doc = "Bit 26 - PCROP read error interrupt enable This bit enables the interrupt generation upon setting the RDERR flag in the FLASH_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn rderrie(&mut self) -> RderrieW<FlashCrSpec> {
        RderrieW::new(self, 26)
    }
    #[doc = "Bit 27 - Option byte load launch When set, this bit triggers the load of option bytes into option registers. It is automatically cleared upon the completion of the load. The high state of the bit indicates pending option byte load. The bit cannot be cleared by software. It cannot be written as long as OPTLOCK is set."]
    #[inline(always)]
    #[must_use]
    pub fn obl_launch(&mut self) -> OblLaunchW<FlashCrSpec> {
        OblLaunchW::new(self, 27)
    }
    #[doc = "Bit 28 - Securable memory area protection enable This bit enables the protection on securable area, provided that a non-null securable memory area size (SEC_SIZE\\[4:0\\]) is defined in option bytes. This bit is possible to set only by software and to clear only through a system reset."]
    #[inline(always)]
    #[must_use]
    pub fn sec_prot(&mut self) -> SecProtW<FlashCrSpec> {
        SecProtW::new(self, 28)
    }
    #[doc = "Bit 30 - Options Lock This bit is set only. When set, all bits concerning user option in FLASH_CR register and so option page are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset."]
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OptlockW<FlashCrSpec> {
        OptlockW::new(self, 30)
    }
    #[doc = "Bit 31 - FLASH_CR Lock This bit is set only. When set, the FLASH_CR register is locked. It is cleared by hardware after detecting the unlock sequence. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<FlashCrSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "FLASH control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashCrSpec;
impl crate::RegisterSpec for FlashCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_cr::R`](R) reader structure"]
impl crate::Readable for FlashCrSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_cr::W`](W) writer structure"]
impl crate::Writable for FlashCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_CR to value 0xc000_0000"]
impl crate::Resettable for FlashCrSpec {
    const RESET_VALUE: u32 = 0xc000_0000;
}
