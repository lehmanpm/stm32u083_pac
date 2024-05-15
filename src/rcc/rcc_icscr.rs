#[doc = "Register `RCC_ICSCR` reader"]
pub type R = crate::R<RccIcscrSpec>;
#[doc = "Register `RCC_ICSCR` writer"]
pub type W = crate::W<RccIcscrSpec>;
#[doc = "Field `MSICAL` reader - MSI clock calibration These bits are initialized at startup with the factory-programmed MSI calibration trim value. When MSITRIM is written, MSICAL is updated with the sum of MSITRIM and the factory trim value."]
pub type MsicalR = crate::FieldReader;
#[doc = "Field `MSITRIM` reader - MSI clock trimming These bits provide an additional user-programmable trimming value that is added to the MSICAL\\[7:0\\]
bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the MSI."]
pub type MsitrimR = crate::FieldReader;
#[doc = "Field `MSITRIM` writer - MSI clock trimming These bits provide an additional user-programmable trimming value that is added to the MSICAL\\[7:0\\]
bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the MSI."]
pub type MsitrimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HSICAL` reader - HSI16 clock calibration These bits are initialized at startup with the factory-programmed HSI16 calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value."]
pub type HsicalR = crate::FieldReader;
#[doc = "Field `HSITRIM` reader - HSI16 clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[7:0\\]
bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI16. The default value is 64 when added to the HSICAL value, trim the HSI16 to 161MHz 1 11%."]
pub type HsitrimR = crate::FieldReader;
#[doc = "Field `HSITRIM` writer - HSI16 clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[7:0\\]
bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI16. The default value is 64 when added to the HSICAL value, trim the HSI16 to 161MHz 1 11%."]
pub type HsitrimW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:7 - MSI clock calibration These bits are initialized at startup with the factory-programmed MSI calibration trim value. When MSITRIM is written, MSICAL is updated with the sum of MSITRIM and the factory trim value."]
    #[inline(always)]
    pub fn msical(&self) -> MsicalR {
        MsicalR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MSI clock trimming These bits provide an additional user-programmable trimming value that is added to the MSICAL\\[7:0\\]
bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the MSI."]
    #[inline(always)]
    pub fn msitrim(&self) -> MsitrimR {
        MsitrimR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - HSI16 clock calibration These bits are initialized at startup with the factory-programmed HSI16 calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value."]
    #[inline(always)]
    pub fn hsical(&self) -> HsicalR {
        HsicalR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - HSI16 clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[7:0\\]
bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI16. The default value is 64 when added to the HSICAL value, trim the HSI16 to 161MHz 1 11%."]
    #[inline(always)]
    pub fn hsitrim(&self) -> HsitrimR {
        HsitrimR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - MSI clock trimming These bits provide an additional user-programmable trimming value that is added to the MSICAL\\[7:0\\]
bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the MSI."]
    #[inline(always)]
    #[must_use]
    pub fn msitrim(&mut self) -> MsitrimW<RccIcscrSpec> {
        MsitrimW::new(self, 8)
    }
    #[doc = "Bits 24:30 - HSI16 clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[7:0\\]
bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI16. The default value is 64 when added to the HSICAL value, trim the HSI16 to 161MHz 1 11%."]
    #[inline(always)]
    #[must_use]
    pub fn hsitrim(&mut self) -> HsitrimW<RccIcscrSpec> {
        HsitrimW::new(self, 24)
    }
}
#[doc = "Internal clock sources calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_icscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_icscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccIcscrSpec;
impl crate::RegisterSpec for RccIcscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_icscr::R`](R) reader structure"]
impl crate::Readable for RccIcscrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_icscr::W`](W) writer structure"]
impl crate::Writable for RccIcscrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_ICSCR to value 0x4000_4000"]
impl crate::Resettable for RccIcscrSpec {
    const RESET_VALUE: u32 = 0x4000_4000;
}
