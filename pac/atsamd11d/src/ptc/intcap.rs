#[doc = "Register `INTCAP` reader"]
pub type R = crate::R<IntcapSpec>;
#[doc = "Register `INTCAP` writer"]
pub type W = crate::W<IntcapSpec>;
#[doc = "Field `VALUE` reader - Value"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `VALUE` writer - Value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<IntcapSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Internal capacitor value\n\nYou can [`read`](crate::Reg::read) this register and get [`intcap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intcap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcapSpec;
impl crate::RegisterSpec for IntcapSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intcap::R`](R) reader structure"]
impl crate::Readable for IntcapSpec {}
#[doc = "`write(|w| ..)` method takes [`intcap::W`](W) writer structure"]
impl crate::Writable for IntcapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTCAP to value 0"]
impl crate::Resettable for IntcapSpec {
    const RESET_VALUE: u8 = 0;
}
