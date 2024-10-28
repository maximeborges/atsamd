#[doc = "Register `XSELECTEN` reader"]
pub type R = crate::R<XselectenSpec>;
#[doc = "Register `XSELECTEN` writer"]
pub type W = crate::W<XselectenSpec>;
#[doc = "Field `X0EN` reader - PTC X0 pin enable"]
pub type X0enR = crate::BitReader;
#[doc = "Field `X0EN` writer - PTC X0 pin enable"]
pub type X0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X1EN` reader - PTC X1 pin enable"]
pub type X1enR = crate::BitReader;
#[doc = "Field `X1EN` writer - PTC X1 pin enable"]
pub type X1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X2EN` reader - PTC X2 pin enable"]
pub type X2enR = crate::BitReader;
#[doc = "Field `X2EN` writer - PTC X2 pin enable"]
pub type X2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X3EN` reader - PTC X3 pin enable"]
pub type X3enR = crate::BitReader;
#[doc = "Field `X3EN` writer - PTC X3 pin enable"]
pub type X3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X4EN` reader - PTC X4 pin enable"]
pub type X4enR = crate::BitReader;
#[doc = "Field `X4EN` writer - PTC X4 pin enable"]
pub type X4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X5EN` reader - PTC X5 pin enable"]
pub type X5enR = crate::BitReader;
#[doc = "Field `X5EN` writer - PTC X5 pin enable"]
pub type X5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X6EN` reader - PTC X6 pin enable"]
pub type X6enR = crate::BitReader;
#[doc = "Field `X6EN` writer - PTC X6 pin enable"]
pub type X6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X7EN` reader - PTC X7 pin enable"]
pub type X7enR = crate::BitReader;
#[doc = "Field `X7EN` writer - PTC X7 pin enable"]
pub type X7enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X8EN` reader - PTC X8 pin enable"]
pub type X8enR = crate::BitReader;
#[doc = "Field `X8EN` writer - PTC X8 pin enable"]
pub type X8enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X9EN` reader - PTC X9 pin enable"]
pub type X9enR = crate::BitReader;
#[doc = "Field `X9EN` writer - PTC X9 pin enable"]
pub type X9enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X10EN` reader - PTC X10 pin enable"]
pub type X10enR = crate::BitReader;
#[doc = "Field `X10EN` writer - PTC X10 pin enable"]
pub type X10enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X11EN` reader - PTC X11 pin enable"]
pub type X11enR = crate::BitReader;
#[doc = "Field `X11EN` writer - PTC X11 pin enable"]
pub type X11enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X12EN` reader - PTC X12 pin enable"]
pub type X12enR = crate::BitReader;
#[doc = "Field `X12EN` writer - PTC X12 pin enable"]
pub type X12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X13EN` reader - PTC X13 pin enable"]
pub type X13enR = crate::BitReader;
#[doc = "Field `X13EN` writer - PTC X13 pin enable"]
pub type X13enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X14EN` reader - PTC X14 pin enable"]
pub type X14enR = crate::BitReader;
#[doc = "Field `X14EN` writer - PTC X14 pin enable"]
pub type X14enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X15EN` reader - PTC X15 pin enable"]
pub type X15enR = crate::BitReader;
#[doc = "Field `X15EN` writer - PTC X15 pin enable"]
pub type X15enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PTC X0 pin enable"]
    #[inline(always)]
    pub fn x0en(&self) -> X0enR {
        X0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PTC X1 pin enable"]
    #[inline(always)]
    pub fn x1en(&self) -> X1enR {
        X1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PTC X2 pin enable"]
    #[inline(always)]
    pub fn x2en(&self) -> X2enR {
        X2enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PTC X3 pin enable"]
    #[inline(always)]
    pub fn x3en(&self) -> X3enR {
        X3enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PTC X4 pin enable"]
    #[inline(always)]
    pub fn x4en(&self) -> X4enR {
        X4enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PTC X5 pin enable"]
    #[inline(always)]
    pub fn x5en(&self) -> X5enR {
        X5enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PTC X6 pin enable"]
    #[inline(always)]
    pub fn x6en(&self) -> X6enR {
        X6enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PTC X7 pin enable"]
    #[inline(always)]
    pub fn x7en(&self) -> X7enR {
        X7enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PTC X8 pin enable"]
    #[inline(always)]
    pub fn x8en(&self) -> X8enR {
        X8enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PTC X9 pin enable"]
    #[inline(always)]
    pub fn x9en(&self) -> X9enR {
        X9enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PTC X10 pin enable"]
    #[inline(always)]
    pub fn x10en(&self) -> X10enR {
        X10enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PTC X11 pin enable"]
    #[inline(always)]
    pub fn x11en(&self) -> X11enR {
        X11enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PTC X12 pin enable"]
    #[inline(always)]
    pub fn x12en(&self) -> X12enR {
        X12enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PTC X13 pin enable"]
    #[inline(always)]
    pub fn x13en(&self) -> X13enR {
        X13enR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PTC X14 pin enable"]
    #[inline(always)]
    pub fn x14en(&self) -> X14enR {
        X14enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PTC X15 pin enable"]
    #[inline(always)]
    pub fn x15en(&self) -> X15enR {
        X15enR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PTC X0 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn x0en(&mut self) -> X0enW<XselectenSpec> {
        X0enW::new(self, 0)
    }
    #[doc = "Bit 1 - PTC X1 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn x1en(&mut self) -> X1enW<XselectenSpec> {
        X1enW::new(self, 1)
    }
    #[doc = "Bit 2 - PTC X2 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn x2en(&mut self) -> X2enW<XselectenSpec> {
        X2enW::new(self, 2)
    }
    #[doc = "Bit 3 - PTC X3 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn x3en(&mut self) -> X3enW<XselectenSpec> {
        X3enW::new(self, 3)
    }
    #[doc = "Bit 4 - PTC X4 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn x4en(&mut self) -> X4enW<XselectenSpec> {
        X4enW::new(self, 4)
    }
    #[doc = "Bit 5 - PTC X5 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn x5en(&mut self) -> X5enW<XselectenSpec> {
        X5enW::new(self, 5)
    }
    #[doc = "Bit 6 - PTC X6 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn x6en(&mut self) -> X6enW<XselectenSpec> {
        X6enW::new(self, 6)
    }
    #[doc = "Bit 7 - PTC X7 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn x7en(&mut self) -> X7enW<XselectenSpec> {
        X7enW::new(self, 7)
    }
    #[doc = "Bit 8 - PTC X8 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn x8en(&mut self) -> X8enW<XselectenSpec> {
        X8enW::new(self, 8)
    }
    #[doc = "Bit 9 - PTC X9 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn x9en(&mut self) -> X9enW<XselectenSpec> {
        X9enW::new(self, 9)
    }
    #[doc = "Bit 10 - PTC X10 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn x10en(&mut self) -> X10enW<XselectenSpec> {
        X10enW::new(self, 10)
    }
    #[doc = "Bit 11 - PTC X11 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn x11en(&mut self) -> X11enW<XselectenSpec> {
        X11enW::new(self, 11)
    }
    #[doc = "Bit 12 - PTC X12 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn x12en(&mut self) -> X12enW<XselectenSpec> {
        X12enW::new(self, 12)
    }
    #[doc = "Bit 13 - PTC X13 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn x13en(&mut self) -> X13enW<XselectenSpec> {
        X13enW::new(self, 13)
    }
    #[doc = "Bit 14 - PTC X14 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn x14en(&mut self) -> X14enW<XselectenSpec> {
        X14enW::new(self, 14)
    }
    #[doc = "Bit 15 - PTC X15 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn x15en(&mut self) -> X15enW<XselectenSpec> {
        X15enW::new(self, 15)
    }
}
#[doc = "Enable X lines\n\nYou can [`read`](crate::Reg::read) this register and get [`xselecten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xselecten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XselectenSpec;
impl crate::RegisterSpec for XselectenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`xselecten::R`](R) reader structure"]
impl crate::Readable for XselectenSpec {}
#[doc = "`write(|w| ..)` method takes [`xselecten::W`](W) writer structure"]
impl crate::Writable for XselectenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets XSELECTEN to value 0"]
impl crate::Resettable for XselectenSpec {
    const RESET_VALUE: u16 = 0;
}
