#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `EOC` reader - Interrupt on end of comparison"]
pub type EocR = crate::BitReader;
#[doc = "Field `EOC` writer - Interrupt on end of comparison"]
pub type EocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCO` reader - Watch crystal oscillator interrupt"]
pub type WcoR = crate::BitReader;
#[doc = "Field `WCO` writer - Watch crystal oscillator interrupt"]
pub type WcoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt on end of comparison"]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watch crystal oscillator interrupt"]
    #[inline(always)]
    pub fn wco(&self) -> WcoR {
        WcoR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt on end of comparison"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EocW<IntenclrSpec> {
        EocW::new(self, 0)
    }
    #[doc = "Bit 1 - Watch crystal oscillator interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wco(&mut self) -> WcoW<IntenclrSpec> {
        WcoW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u8 = 0;
}
