#[doc = "Register `WCOTHRESHOLDBL` reader"]
pub type R = crate::R<WcothresholdblSpec>;
#[doc = "Register `WCOTHRESHOLDBL` writer"]
pub type W = crate::W<WcothresholdblSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Set lower WCO threshold for port B\n\nYou can [`read`](crate::Reg::read) this register and get [`wcothresholdbl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcothresholdbl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WcothresholdblSpec;
impl crate::RegisterSpec for WcothresholdblSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wcothresholdbl::R`](R) reader structure"]
impl crate::Readable for WcothresholdblSpec {}
#[doc = "`write(|w| ..)` method takes [`wcothresholdbl::W`](W) writer structure"]
impl crate::Writable for WcothresholdblSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WCOTHRESHOLDBL to value 0"]
impl crate::Resettable for WcothresholdblSpec {
    const RESET_VALUE: u8 = 0;
}
