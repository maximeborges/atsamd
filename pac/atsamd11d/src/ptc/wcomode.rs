#[doc = "Register `WCOMODE` reader"]
pub type R = crate::R<WcomodeSpec>;
#[doc = "Register `WCOMODE` writer"]
pub type W = crate::W<WcomodeSpec>;
#[doc = "Field `MODE` reader - Set WCO mode"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - Set WCO mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Set WCO mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Set WCO mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<WcomodeSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "Set WCO mode\n\nYou can [`read`](crate::Reg::read) this register and get [`wcomode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcomode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WcomodeSpec;
impl crate::RegisterSpec for WcomodeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wcomode::R`](R) reader structure"]
impl crate::Readable for WcomodeSpec {}
#[doc = "`write(|w| ..)` method takes [`wcomode::W`](W) writer structure"]
impl crate::Writable for WcomodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WCOMODE to value 0"]
impl crate::Resettable for WcomodeSpec {
    const RESET_VALUE: u8 = 0;
}
