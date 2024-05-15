#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lcd_cr: LcdCr,
    lcd_fcr: LcdFcr,
    lcd_sr: LcdSr,
    lcd_clr: LcdClr,
    _reserved4: [u8; 0x04],
    lcd_ram0: LcdRam0,
    lcd_ram1: LcdRam1,
    lcd_ram2: LcdRam2,
    lcd_ram3: LcdRam3,
    lcd_ram4: LcdRam4,
    lcd_ram5: LcdRam5,
    lcd_ram6: LcdRam6,
    lcd_ram7: LcdRam7,
    lcd_ram8: LcdRam8,
    lcd_ram9: LcdRam9,
    lcd_ram10: LcdRam10,
    lcd_ram11: LcdRam11,
    lcd_ram12: LcdRam12,
    lcd_ram13: LcdRam13,
    lcd_ram14: LcdRam14,
    lcd_ram15: LcdRam15,
}
impl RegisterBlock {
    #[doc = "0x00 - LCD control register"]
    #[inline(always)]
    pub const fn lcd_cr(&self) -> &LcdCr {
        &self.lcd_cr
    }
    #[doc = "0x04 - LCD frame control register"]
    #[inline(always)]
    pub const fn lcd_fcr(&self) -> &LcdFcr {
        &self.lcd_fcr
    }
    #[doc = "0x08 - LCD status register"]
    #[inline(always)]
    pub const fn lcd_sr(&self) -> &LcdSr {
        &self.lcd_sr
    }
    #[doc = "0x0c - LCD clear register"]
    #[inline(always)]
    pub const fn lcd_clr(&self) -> &LcdClr {
        &self.lcd_clr
    }
    #[doc = "0x14 - LCD display memory"]
    #[inline(always)]
    pub const fn lcd_ram0(&self) -> &LcdRam0 {
        &self.lcd_ram0
    }
    #[doc = "0x18 - LCD display memory"]
    #[inline(always)]
    pub const fn lcd_ram1(&self) -> &LcdRam1 {
        &self.lcd_ram1
    }
    #[doc = "0x1c - LCD display memory"]
    #[inline(always)]
    pub const fn lcd_ram2(&self) -> &LcdRam2 {
        &self.lcd_ram2
    }
    #[doc = "0x20 - LCD display memory"]
    #[inline(always)]
    pub const fn lcd_ram3(&self) -> &LcdRam3 {
        &self.lcd_ram3
    }
    #[doc = "0x24 - LCD display memory"]
    #[inline(always)]
    pub const fn lcd_ram4(&self) -> &LcdRam4 {
        &self.lcd_ram4
    }
    #[doc = "0x28 - LCD display memory"]
    #[inline(always)]
    pub const fn lcd_ram5(&self) -> &LcdRam5 {
        &self.lcd_ram5
    }
    #[doc = "0x2c - LCD display memory"]
    #[inline(always)]
    pub const fn lcd_ram6(&self) -> &LcdRam6 {
        &self.lcd_ram6
    }
    #[doc = "0x30 - LCD display memory"]
    #[inline(always)]
    pub const fn lcd_ram7(&self) -> &LcdRam7 {
        &self.lcd_ram7
    }
    #[doc = "0x34 - LCD display memory"]
    #[inline(always)]
    pub const fn lcd_ram8(&self) -> &LcdRam8 {
        &self.lcd_ram8
    }
    #[doc = "0x38 - LCD display memory"]
    #[inline(always)]
    pub const fn lcd_ram9(&self) -> &LcdRam9 {
        &self.lcd_ram9
    }
    #[doc = "0x3c - LCD display memory"]
    #[inline(always)]
    pub const fn lcd_ram10(&self) -> &LcdRam10 {
        &self.lcd_ram10
    }
    #[doc = "0x40 - LCD display memory"]
    #[inline(always)]
    pub const fn lcd_ram11(&self) -> &LcdRam11 {
        &self.lcd_ram11
    }
    #[doc = "0x44 - LCD display memory"]
    #[inline(always)]
    pub const fn lcd_ram12(&self) -> &LcdRam12 {
        &self.lcd_ram12
    }
    #[doc = "0x48 - LCD display memory"]
    #[inline(always)]
    pub const fn lcd_ram13(&self) -> &LcdRam13 {
        &self.lcd_ram13
    }
    #[doc = "0x4c - LCD display memory"]
    #[inline(always)]
    pub const fn lcd_ram14(&self) -> &LcdRam14 {
        &self.lcd_ram14
    }
    #[doc = "0x50 - LCD display memory"]
    #[inline(always)]
    pub const fn lcd_ram15(&self) -> &LcdRam15 {
        &self.lcd_ram15
    }
}
#[doc = "LCD_CR (rw) register accessor: LCD control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_cr`]
module"]
#[doc(alias = "LCD_CR")]
pub type LcdCr = crate::Reg<lcd_cr::LcdCrSpec>;
#[doc = "LCD control register"]
pub mod lcd_cr;
#[doc = "LCD_FCR (rw) register accessor: LCD frame control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_fcr`]
module"]
#[doc(alias = "LCD_FCR")]
pub type LcdFcr = crate::Reg<lcd_fcr::LcdFcrSpec>;
#[doc = "LCD frame control register"]
pub mod lcd_fcr;
#[doc = "LCD_SR (rw) register accessor: LCD status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_sr`]
module"]
#[doc(alias = "LCD_SR")]
pub type LcdSr = crate::Reg<lcd_sr::LcdSrSpec>;
#[doc = "LCD status register"]
pub mod lcd_sr;
#[doc = "LCD_CLR (w) register accessor: LCD clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_clr`]
module"]
#[doc(alias = "LCD_CLR")]
pub type LcdClr = crate::Reg<lcd_clr::LcdClrSpec>;
#[doc = "LCD clear register"]
pub mod lcd_clr;
#[doc = "LCD_RAM0 (rw) register accessor: LCD display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ram0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ram0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram0`]
module"]
#[doc(alias = "LCD_RAM0")]
pub type LcdRam0 = crate::Reg<lcd_ram0::LcdRam0Spec>;
#[doc = "LCD display memory"]
pub mod lcd_ram0;
#[doc = "LCD_RAM1 (rw) register accessor: LCD display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ram1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ram1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram1`]
module"]
#[doc(alias = "LCD_RAM1")]
pub type LcdRam1 = crate::Reg<lcd_ram1::LcdRam1Spec>;
#[doc = "LCD display memory"]
pub mod lcd_ram1;
#[doc = "LCD_RAM2 (rw) register accessor: LCD display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ram2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ram2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram2`]
module"]
#[doc(alias = "LCD_RAM2")]
pub type LcdRam2 = crate::Reg<lcd_ram2::LcdRam2Spec>;
#[doc = "LCD display memory"]
pub mod lcd_ram2;
#[doc = "LCD_RAM3 (rw) register accessor: LCD display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ram3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ram3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram3`]
module"]
#[doc(alias = "LCD_RAM3")]
pub type LcdRam3 = crate::Reg<lcd_ram3::LcdRam3Spec>;
#[doc = "LCD display memory"]
pub mod lcd_ram3;
#[doc = "LCD_RAM4 (rw) register accessor: LCD display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ram4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ram4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram4`]
module"]
#[doc(alias = "LCD_RAM4")]
pub type LcdRam4 = crate::Reg<lcd_ram4::LcdRam4Spec>;
#[doc = "LCD display memory"]
pub mod lcd_ram4;
#[doc = "LCD_RAM5 (rw) register accessor: LCD display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ram5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ram5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram5`]
module"]
#[doc(alias = "LCD_RAM5")]
pub type LcdRam5 = crate::Reg<lcd_ram5::LcdRam5Spec>;
#[doc = "LCD display memory"]
pub mod lcd_ram5;
#[doc = "LCD_RAM6 (rw) register accessor: LCD display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ram6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ram6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram6`]
module"]
#[doc(alias = "LCD_RAM6")]
pub type LcdRam6 = crate::Reg<lcd_ram6::LcdRam6Spec>;
#[doc = "LCD display memory"]
pub mod lcd_ram6;
#[doc = "LCD_RAM7 (rw) register accessor: LCD display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ram7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ram7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram7`]
module"]
#[doc(alias = "LCD_RAM7")]
pub type LcdRam7 = crate::Reg<lcd_ram7::LcdRam7Spec>;
#[doc = "LCD display memory"]
pub mod lcd_ram7;
#[doc = "LCD_RAM8 (rw) register accessor: LCD display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ram8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ram8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram8`]
module"]
#[doc(alias = "LCD_RAM8")]
pub type LcdRam8 = crate::Reg<lcd_ram8::LcdRam8Spec>;
#[doc = "LCD display memory"]
pub mod lcd_ram8;
#[doc = "LCD_RAM9 (rw) register accessor: LCD display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ram9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ram9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram9`]
module"]
#[doc(alias = "LCD_RAM9")]
pub type LcdRam9 = crate::Reg<lcd_ram9::LcdRam9Spec>;
#[doc = "LCD display memory"]
pub mod lcd_ram9;
#[doc = "LCD_RAM10 (rw) register accessor: LCD display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ram10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ram10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram10`]
module"]
#[doc(alias = "LCD_RAM10")]
pub type LcdRam10 = crate::Reg<lcd_ram10::LcdRam10Spec>;
#[doc = "LCD display memory"]
pub mod lcd_ram10;
#[doc = "LCD_RAM11 (rw) register accessor: LCD display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ram11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ram11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram11`]
module"]
#[doc(alias = "LCD_RAM11")]
pub type LcdRam11 = crate::Reg<lcd_ram11::LcdRam11Spec>;
#[doc = "LCD display memory"]
pub mod lcd_ram11;
#[doc = "LCD_RAM12 (rw) register accessor: LCD display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ram12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ram12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram12`]
module"]
#[doc(alias = "LCD_RAM12")]
pub type LcdRam12 = crate::Reg<lcd_ram12::LcdRam12Spec>;
#[doc = "LCD display memory"]
pub mod lcd_ram12;
#[doc = "LCD_RAM13 (rw) register accessor: LCD display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ram13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ram13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram13`]
module"]
#[doc(alias = "LCD_RAM13")]
pub type LcdRam13 = crate::Reg<lcd_ram13::LcdRam13Spec>;
#[doc = "LCD display memory"]
pub mod lcd_ram13;
#[doc = "LCD_RAM14 (rw) register accessor: LCD display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ram14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ram14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram14`]
module"]
#[doc(alias = "LCD_RAM14")]
pub type LcdRam14 = crate::Reg<lcd_ram14::LcdRam14Spec>;
#[doc = "LCD display memory"]
pub mod lcd_ram14;
#[doc = "LCD_RAM15 (rw) register accessor: LCD display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ram15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ram15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram15`]
module"]
#[doc(alias = "LCD_RAM15")]
pub type LcdRam15 = crate::Reg<lcd_ram15::LcdRam15Spec>;
#[doc = "LCD display memory"]
pub mod lcd_ram15;
