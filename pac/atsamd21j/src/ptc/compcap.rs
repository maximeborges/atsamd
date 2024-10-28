#[doc = "Register `COMPCAP` reader"]
pub type R = crate::R<CompcapSpec>;
#[doc = "Register `COMPCAP` writer"]
pub type W = crate::W<CompcapSpec>;
#[doc = "Field `VALUE` reader - Value"]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - Value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits & 0x3fff)
    }
}
impl W {
    #[doc = "Bits 0:13 - Value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<CompcapSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Compensation capacitor value\n\nYou can [`read`](crate::Reg::read) this register and get [`compcap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compcap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompcapSpec;
impl crate::RegisterSpec for CompcapSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`compcap::R`](R) reader structure"]
impl crate::Readable for CompcapSpec {}
#[doc = "`write(|w| ..)` method takes [`compcap::W`](W) writer structure"]
impl crate::Writable for CompcapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets COMPCAP to value 0"]
impl crate::Resettable for CompcapSpec {
    const RESET_VALUE: u16 = 0;
}
