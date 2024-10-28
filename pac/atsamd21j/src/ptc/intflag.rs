#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `EOC` reader - Interrupt end of comparison"]
pub type EocR = crate::BitReader;
#[doc = "Field `EOC` writer - Interrupt end of comparison"]
pub type EocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCO` reader - Watch crystal oscillator interrupt"]
pub type WcoR = crate::BitReader;
#[doc = "Field `WCO` writer - Watch crystal oscillator interrupt"]
pub type WcoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt end of comparison"]
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
    #[doc = "Bit 0 - Interrupt end of comparison"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EocW<IntflagSpec> {
        EocW::new(self, 0)
    }
    #[doc = "Bit 1 - Watch crystal oscillator interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wco(&mut self) -> WcoW<IntflagSpec> {
        WcoW::new(self, 1)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {
    const RESET_VALUE: u8 = 0;
}
