#[doc = "Register `CC%s` reader"]
pub type R = crate::R<CcSpec>;
#[doc = "Register `CC%s` writer"]
pub type W = crate::W<CcSpec>;
#[doc = "Field `CC` reader - Compare/Capture Value"]
pub type CcR = crate::FieldReader<u16>;
#[doc = "Field `CC` writer - Compare/Capture Value"]
pub type CcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare/Capture Value"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare/Capture Value"]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CcW<CcSpec> {
        CcW::new(self, 0)
    }
}
#[doc = "COUNT16 Compare/Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcSpec;
impl crate::RegisterSpec for CcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cc::R`](R) reader structure"]
impl crate::Readable for CcSpec {}
#[doc = "`write(|w| ..)` method takes [`cc::W`](W) writer structure"]
impl crate::Writable for CcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CC%s to value 0"]
impl crate::Resettable for CcSpec {
    const RESET_VALUE: u16 = 0;
}
