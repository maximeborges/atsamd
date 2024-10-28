#[doc = "Register `WCOTHRESHOLDAL` reader"]
pub type R = crate::R<WcothresholdalSpec>;
#[doc = "Register `WCOTHRESHOLDAL` writer"]
pub type W = crate::W<WcothresholdalSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Set lower WCO threshold for port A\n\nYou can [`read`](crate::Reg::read) this register and get [`wcothresholdal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcothresholdal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WcothresholdalSpec;
impl crate::RegisterSpec for WcothresholdalSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wcothresholdal::R`](R) reader structure"]
impl crate::Readable for WcothresholdalSpec {}
#[doc = "`write(|w| ..)` method takes [`wcothresholdal::W`](W) writer structure"]
impl crate::Writable for WcothresholdalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WCOTHRESHOLDAL to value 0"]
impl crate::Resettable for WcothresholdalSpec {
    const RESET_VALUE: u8 = 0;
}
