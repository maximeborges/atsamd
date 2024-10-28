#[doc = "Register `WCOTHRESHOLDBH` reader"]
pub type R = crate::R<WcothresholdbhSpec>;
#[doc = "Register `WCOTHRESHOLDBH` writer"]
pub type W = crate::W<WcothresholdbhSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Set upper WCO threshold for port B\n\nYou can [`read`](crate::Reg::read) this register and get [`wcothresholdbh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcothresholdbh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WcothresholdbhSpec;
impl crate::RegisterSpec for WcothresholdbhSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wcothresholdbh::R`](R) reader structure"]
impl crate::Readable for WcothresholdbhSpec {}
#[doc = "`write(|w| ..)` method takes [`wcothresholdbh::W`](W) writer structure"]
impl crate::Writable for WcothresholdbhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WCOTHRESHOLDBH to value 0"]
impl crate::Resettable for WcothresholdbhSpec {
    const RESET_VALUE: u8 = 0;
}
