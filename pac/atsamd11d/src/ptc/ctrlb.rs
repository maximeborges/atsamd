#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `SYNCFLAG` reader - Synchronisation flag"]
pub type SyncflagR = crate::BitReader;
#[doc = "Field `SYNCFLAG` writer - Synchronisation flag"]
pub type SyncflagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - Synchronisation flag"]
    #[inline(always)]
    pub fn syncflag(&self) -> SyncflagR {
        SyncflagR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Synchronisation flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncflag(&mut self) -> SyncflagW<CtrlbSpec> {
        SyncflagW::new(self, 7)
    }
}
#[doc = "Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {
    const RESET_VALUE: u8 = 0;
}
