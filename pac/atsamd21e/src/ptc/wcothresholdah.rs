#[doc = "Register `WCOTHRESHOLDAH` reader"]
pub type R = crate::R<WcothresholdahSpec>;
#[doc = "Register `WCOTHRESHOLDAH` writer"]
pub type W = crate::W<WcothresholdahSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Set upper WCO threshold for port A\n\nYou can [`read`](crate::Reg::read) this register and get [`wcothresholdah::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcothresholdah::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WcothresholdahSpec;
impl crate::RegisterSpec for WcothresholdahSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wcothresholdah::R`](R) reader structure"]
impl crate::Readable for WcothresholdahSpec {}
#[doc = "`write(|w| ..)` method takes [`wcothresholdah::W`](W) writer structure"]
impl crate::Writable for WcothresholdahSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WCOTHRESHOLDAH to value 0"]
impl crate::Resettable for WcothresholdahSpec {
    const RESET_VALUE: u8 = 0;
}
