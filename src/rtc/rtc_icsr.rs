#[doc = "Register `RTC_ICSR` reader"]
pub type R = crate::R<RtcIcsrSpec>;
#[doc = "Register `RTC_ICSR` writer"]
pub type W = crate::W<RtcIcsrSpec>;
#[doc = "Wake-up timer write flag This bit is set by hardware when WUT value can be changed, after the WUTE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wutwf {
    #[doc = "0: Wake-up timer configuration update not allowed except in initialization mode"]
    B0x0 = 0,
    #[doc = "1: Wake-up timer configuration update allowed"]
    B0x1 = 1,
}
impl From<Wutwf> for bool {
    #[inline(always)]
    fn from(variant: Wutwf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTWF` reader - Wake-up timer write flag This bit is set by hardware when WUT value can be changed, after the WUTE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode."]
pub type WutwfR = crate::BitReader<Wutwf>;
impl WutwfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wutwf {
        match self.bits {
            false => Wutwf::B0x0,
            true => Wutwf::B0x1,
        }
    }
    #[doc = "Wake-up timer configuration update not allowed except in initialization mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wutwf::B0x0
    }
    #[doc = "Wake-up timer configuration update allowed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wutwf::B0x1
    }
}
#[doc = "Shift operation pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shpf {
    #[doc = "0: No shift operation is pending"]
    B0x0 = 0,
    #[doc = "1: A shift operation is pending"]
    B0x1 = 1,
}
impl From<Shpf> for bool {
    #[inline(always)]
    fn from(variant: Shpf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHPF` reader - Shift operation pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect."]
pub type ShpfR = crate::BitReader<Shpf>;
impl ShpfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shpf {
        match self.bits {
            false => Shpf::B0x0,
            true => Shpf::B0x1,
        }
    }
    #[doc = "No shift operation is pending"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Shpf::B0x0
    }
    #[doc = "A shift operation is pending"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Shpf::B0x1
    }
}
#[doc = "Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (Backup domain reset state).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inits {
    #[doc = "0: Calendar has not been initialized"]
    B0x0 = 0,
    #[doc = "1: Calendar has been initialized"]
    B0x1 = 1,
}
impl From<Inits> for bool {
    #[inline(always)]
    fn from(variant: Inits) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITS` reader - Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (Backup domain reset state)."]
pub type InitsR = crate::BitReader<Inits>;
impl InitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inits {
        match self.bits {
            false => Inits::B0x0,
            true => Inits::B0x1,
        }
    }
    #[doc = "Calendar has not been initialized"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Inits::B0x0
    }
    #[doc = "Calendar has been initialized"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Inits::B0x1
    }
}
#[doc = "Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsf {
    #[doc = "0: Calendar shadow registers not yet synchronized"]
    B0x0 = 0,
    #[doc = "1: Calendar shadow registers synchronized"]
    B0x1 = 1,
}
impl From<Rsf> for bool {
    #[inline(always)]
    fn from(variant: Rsf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` reader - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode."]
pub type RsfR = crate::BitReader<Rsf>;
impl RsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rsf {
        match self.bits {
            false => Rsf::B0x0,
            true => Rsf::B0x1,
        }
    }
    #[doc = "Calendar shadow registers not yet synchronized"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rsf::B0x0
    }
    #[doc = "Calendar shadow registers synchronized"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rsf::B0x1
    }
}
#[doc = "Field `RSF` writer - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode."]
pub type RsfW<'a, REG> = crate::BitWriter<'a, REG, Rsf>;
impl<'a, REG> RsfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calendar shadow registers not yet synchronized"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rsf::B0x0)
    }
    #[doc = "Calendar shadow registers synchronized"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsf::B0x1)
    }
}
#[doc = "Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Initf {
    #[doc = "0: Calendar registers update is not allowed"]
    B0x0 = 0,
    #[doc = "1: Calendar registers update is allowed"]
    B0x1 = 1,
}
impl From<Initf> for bool {
    #[inline(always)]
    fn from(variant: Initf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITF` reader - Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated."]
pub type InitfR = crate::BitReader<Initf>;
impl InitfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Initf {
        match self.bits {
            false => Initf::B0x0,
            true => Initf::B0x1,
        }
    }
    #[doc = "Calendar registers update is not allowed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Initf::B0x0
    }
    #[doc = "Calendar registers update is allowed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Initf::B0x1
    }
}
#[doc = "Initialization mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Init {
    #[doc = "0: Free running mode"]
    B0x0 = 0,
    #[doc = "1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER), plus BIN and BCDU fields. Counters are stopped and start counting from the new value when INIT is reset."]
    B0x1 = 1,
}
impl From<Init> for bool {
    #[inline(always)]
    fn from(variant: Init) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT` reader - Initialization mode"]
pub type InitR = crate::BitReader<Init>;
impl InitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Init {
        match self.bits {
            false => Init::B0x0,
            true => Init::B0x1,
        }
    }
    #[doc = "Free running mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Init::B0x0
    }
    #[doc = "Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER), plus BIN and BCDU fields. Counters are stopped and start counting from the new value when INIT is reset."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Init::B0x1
    }
}
#[doc = "Field `INIT` writer - Initialization mode"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG, Init>;
impl<'a, REG> InitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Free running mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Init::B0x0)
    }
    #[doc = "Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER), plus BIN and BCDU fields. Counters are stopped and start counting from the new value when INIT is reset."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Init::B0x1)
    }
}
#[doc = "Binary mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bin {
    #[doc = "0: Free running BCD calendar mode (Binary mode disabled)."]
    B0x0 = 0,
    #[doc = "1: Free running Binary mode (BCD mode disabled)"]
    B0x1 = 1,
    #[doc = "2: Free running BCD calendar and Binary modes"]
    B0x2 = 2,
    #[doc = "3: Free running BCD calendar and Binary modes"]
    B0x3 = 3,
}
impl From<Bin> for u8 {
    #[inline(always)]
    fn from(variant: Bin) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bin {
    type Ux = u8;
}
impl crate::IsEnum for Bin {}
#[doc = "Field `BIN` reader - Binary mode"]
pub type BinR = crate::FieldReader<Bin>;
impl BinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bin {
        match self.bits {
            0 => Bin::B0x0,
            1 => Bin::B0x1,
            2 => Bin::B0x2,
            3 => Bin::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Free running BCD calendar mode (Binary mode disabled)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bin::B0x0
    }
    #[doc = "Free running Binary mode (BCD mode disabled)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bin::B0x1
    }
    #[doc = "Free running BCD calendar and Binary modes"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Bin::B0x2
    }
    #[doc = "Free running BCD calendar and Binary modes"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Bin::B0x3
    }
}
#[doc = "Field `BIN` writer - Binary mode"]
pub type BinW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bin, crate::Safe>;
impl<'a, REG> BinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Free running BCD calendar mode (Binary mode disabled)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bin::B0x0)
    }
    #[doc = "Free running Binary mode (BCD mode disabled)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bin::B0x1)
    }
    #[doc = "Free running BCD calendar and Binary modes"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Bin::B0x2)
    }
    #[doc = "Free running BCD calendar and Binary modes"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Bin::B0x3)
    }
}
#[doc = "BCD update (BIN = 10 or 11) In mixed mode when both BCD calendar and binary extended counter are used (BIN = 10 or 11), the calendar second is incremented using the SSR Least Significant Bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bcdu {
    #[doc = "0: 1s calendar increment is generated each time SS\\[7:0\\]
= 0"]
    B0x0 = 0,
    #[doc = "1: 1s calendar increment is generated each time SS\\[8:0\\]
= 0"]
    B0x1 = 1,
    #[doc = "2: 1s calendar increment is generated each time SS\\[9:0\\]
= 0"]
    B0x2 = 2,
    #[doc = "3: 1s calendar increment is generated each time SS\\[10:0\\]
= 0"]
    B0x3 = 3,
    #[doc = "4: 1s calendar increment is generated each time SS\\[11:0\\]
= 0"]
    B0x4 = 4,
    #[doc = "5: 1s calendar increment is generated each time SS\\[12:0\\]
= 0"]
    B0x5 = 5,
    #[doc = "6: 1s calendar increment is generated each time SS\\[13:0\\]
= 0"]
    B0x6 = 6,
    #[doc = "7: 1s calendar increment is generated each time SS\\[14:0\\]
= 0"]
    B0x7 = 7,
}
impl From<Bcdu> for u8 {
    #[inline(always)]
    fn from(variant: Bcdu) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bcdu {
    type Ux = u8;
}
impl crate::IsEnum for Bcdu {}
#[doc = "Field `BCDU` reader - BCD update (BIN = 10 or 11) In mixed mode when both BCD calendar and binary extended counter are used (BIN = 10 or 11), the calendar second is incremented using the SSR Least Significant Bits."]
pub type BcduR = crate::FieldReader<Bcdu>;
impl BcduR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bcdu {
        match self.bits {
            0 => Bcdu::B0x0,
            1 => Bcdu::B0x1,
            2 => Bcdu::B0x2,
            3 => Bcdu::B0x3,
            4 => Bcdu::B0x4,
            5 => Bcdu::B0x5,
            6 => Bcdu::B0x6,
            7 => Bcdu::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "1s calendar increment is generated each time SS\\[7:0\\]
= 0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bcdu::B0x0
    }
    #[doc = "1s calendar increment is generated each time SS\\[8:0\\]
= 0"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bcdu::B0x1
    }
    #[doc = "1s calendar increment is generated each time SS\\[9:0\\]
= 0"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Bcdu::B0x2
    }
    #[doc = "1s calendar increment is generated each time SS\\[10:0\\]
= 0"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Bcdu::B0x3
    }
    #[doc = "1s calendar increment is generated each time SS\\[11:0\\]
= 0"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Bcdu::B0x4
    }
    #[doc = "1s calendar increment is generated each time SS\\[12:0\\]
= 0"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Bcdu::B0x5
    }
    #[doc = "1s calendar increment is generated each time SS\\[13:0\\]
= 0"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Bcdu::B0x6
    }
    #[doc = "1s calendar increment is generated each time SS\\[14:0\\]
= 0"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Bcdu::B0x7
    }
}
#[doc = "Field `BCDU` writer - BCD update (BIN = 10 or 11) In mixed mode when both BCD calendar and binary extended counter are used (BIN = 10 or 11), the calendar second is incremented using the SSR Least Significant Bits."]
pub type BcduW<'a, REG> = crate::FieldWriter<'a, REG, 3, Bcdu, crate::Safe>;
impl<'a, REG> BcduW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1s calendar increment is generated each time SS\\[7:0\\]
= 0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bcdu::B0x0)
    }
    #[doc = "1s calendar increment is generated each time SS\\[8:0\\]
= 0"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bcdu::B0x1)
    }
    #[doc = "1s calendar increment is generated each time SS\\[9:0\\]
= 0"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Bcdu::B0x2)
    }
    #[doc = "1s calendar increment is generated each time SS\\[10:0\\]
= 0"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Bcdu::B0x3)
    }
    #[doc = "1s calendar increment is generated each time SS\\[11:0\\]
= 0"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Bcdu::B0x4)
    }
    #[doc = "1s calendar increment is generated each time SS\\[12:0\\]
= 0"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Bcdu::B0x5)
    }
    #[doc = "1s calendar increment is generated each time SS\\[13:0\\]
= 0"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Bcdu::B0x6)
    }
    #[doc = "1s calendar increment is generated each time SS\\[14:0\\]
= 0"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Bcdu::B0x7)
    }
}
#[doc = "Field `RECALPF` reader - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0. Refer to Re-calibration on-the-fly."]
pub type RecalpfR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - Wake-up timer write flag This bit is set by hardware when WUT value can be changed, after the WUTE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode."]
    #[inline(always)]
    pub fn wutwf(&self) -> WutwfR {
        WutwfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shift operation pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect."]
    #[inline(always)]
    pub fn shpf(&self) -> ShpfR {
        ShpfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (Backup domain reset state)."]
    #[inline(always)]
    pub fn inits(&self) -> InitsR {
        InitsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode."]
    #[inline(always)]
    pub fn rsf(&self) -> RsfR {
        RsfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated."]
    #[inline(always)]
    pub fn initf(&self) -> InitfR {
        InitfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Binary mode"]
    #[inline(always)]
    pub fn bin(&self) -> BinR {
        BinR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - BCD update (BIN = 10 or 11) In mixed mode when both BCD calendar and binary extended counter are used (BIN = 10 or 11), the calendar second is incremented using the SSR Least Significant Bits."]
    #[inline(always)]
    pub fn bcdu(&self) -> BcduR {
        BcduR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 16 - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0. Refer to Re-calibration on-the-fly."]
    #[inline(always)]
    pub fn recalpf(&self) -> RecalpfR {
        RecalpfR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode."]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RsfW<RtcIcsrSpec> {
        RsfW::new(self, 5)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> InitW<RtcIcsrSpec> {
        InitW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Binary mode"]
    #[inline(always)]
    #[must_use]
    pub fn bin(&mut self) -> BinW<RtcIcsrSpec> {
        BinW::new(self, 8)
    }
    #[doc = "Bits 10:12 - BCD update (BIN = 10 or 11) In mixed mode when both BCD calendar and binary extended counter are used (BIN = 10 or 11), the calendar second is incremented using the SSR Least Significant Bits."]
    #[inline(always)]
    #[must_use]
    pub fn bcdu(&mut self) -> BcduW<RtcIcsrSpec> {
        BcduW::new(self, 10)
    }
}
#[doc = "RTC initialization control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_icsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_icsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcIcsrSpec;
impl crate::RegisterSpec for RtcIcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_icsr::R`](R) reader structure"]
impl crate::Readable for RtcIcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_icsr::W`](W) writer structure"]
impl crate::Writable for RtcIcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_ICSR to value 0x07"]
impl crate::Resettable for RtcIcsrSpec {
    const RESET_VALUE: u32 = 0x07;
}
