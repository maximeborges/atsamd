#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CtrlcSpec>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CtrlcSpec>;
#[doc = "Field `INIT` reader - Initialize"]
pub type InitR = crate::BitReader;
#[doc = "Field `INIT` writer - Initialize"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Initialize"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialize"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> InitW<CtrlcSpec> {
        InitW::new(self, 0)
    }
}
#[doc = "Control C\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlcSpec;
impl crate::RegisterSpec for CtrlcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CtrlcSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CtrlcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CtrlcSpec {
    const RESET_VALUE: u8 = 0;
}
