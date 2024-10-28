#[doc = "Register `YSELECTEN` reader"]
pub type R = crate::R<YselectenSpec>;
#[doc = "Register `YSELECTEN` writer"]
pub type W = crate::W<YselectenSpec>;
#[doc = "Field `Y0EN` reader - PTC Y0 pin enable"]
pub type Y0enR = crate::BitReader;
#[doc = "Field `Y0EN` writer - PTC Y0 pin enable"]
pub type Y0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Y1EN` reader - PTC Y1 pin enable"]
pub type Y1enR = crate::BitReader;
#[doc = "Field `Y1EN` writer - PTC Y1 pin enable"]
pub type Y1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Y2EN` reader - PTC Y2 pin enable"]
pub type Y2enR = crate::BitReader;
#[doc = "Field `Y2EN` writer - PTC Y2 pin enable"]
pub type Y2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Y3EN` reader - PTC Y3 pin enable"]
pub type Y3enR = crate::BitReader;
#[doc = "Field `Y3EN` writer - PTC Y3 pin enable"]
pub type Y3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Y4EN` reader - PTC Y4 pin enable"]
pub type Y4enR = crate::BitReader;
#[doc = "Field `Y4EN` writer - PTC Y4 pin enable"]
pub type Y4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Y5EN` reader - PTC Y5 pin enable"]
pub type Y5enR = crate::BitReader;
#[doc = "Field `Y5EN` writer - PTC Y5 pin enable"]
pub type Y5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Y6EN` reader - PTC Y6 pin enable"]
pub type Y6enR = crate::BitReader;
#[doc = "Field `Y6EN` writer - PTC Y6 pin enable"]
pub type Y6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Y7EN` reader - PTC Y7 pin enable"]
pub type Y7enR = crate::BitReader;
#[doc = "Field `Y7EN` writer - PTC Y7 pin enable"]
pub type Y7enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Y8EN` reader - PTC Y8 pin enable"]
pub type Y8enR = crate::BitReader;
#[doc = "Field `Y8EN` writer - PTC Y8 pin enable"]
pub type Y8enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Y9EN` reader - PTC Y9 pin enable"]
pub type Y9enR = crate::BitReader;
#[doc = "Field `Y9EN` writer - PTC Y9 pin enable"]
pub type Y9enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Y10EN` reader - PTC Y10 pin enable"]
pub type Y10enR = crate::BitReader;
#[doc = "Field `Y10EN` writer - PTC Y10 pin enable"]
pub type Y10enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Y11EN` reader - PTC Y11 pin enable"]
pub type Y11enR = crate::BitReader;
#[doc = "Field `Y11EN` writer - PTC Y11 pin enable"]
pub type Y11enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Y12EN` reader - PTC Y12 pin enable"]
pub type Y12enR = crate::BitReader;
#[doc = "Field `Y12EN` writer - PTC Y12 pin enable"]
pub type Y12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Y13EN` reader - PTC Y13 pin enable"]
pub type Y13enR = crate::BitReader;
#[doc = "Field `Y13EN` writer - PTC Y13 pin enable"]
pub type Y13enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Y14EN` reader - PTC Y14 pin enable"]
pub type Y14enR = crate::BitReader;
#[doc = "Field `Y14EN` writer - PTC Y14 pin enable"]
pub type Y14enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Y15EN` reader - PTC Y15 pin enable"]
pub type Y15enR = crate::BitReader;
#[doc = "Field `Y15EN` writer - PTC Y15 pin enable"]
pub type Y15enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PTC Y0 pin enable"]
    #[inline(always)]
    pub fn y0en(&self) -> Y0enR {
        Y0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PTC Y1 pin enable"]
    #[inline(always)]
    pub fn y1en(&self) -> Y1enR {
        Y1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PTC Y2 pin enable"]
    #[inline(always)]
    pub fn y2en(&self) -> Y2enR {
        Y2enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PTC Y3 pin enable"]
    #[inline(always)]
    pub fn y3en(&self) -> Y3enR {
        Y3enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PTC Y4 pin enable"]
    #[inline(always)]
    pub fn y4en(&self) -> Y4enR {
        Y4enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PTC Y5 pin enable"]
    #[inline(always)]
    pub fn y5en(&self) -> Y5enR {
        Y5enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PTC Y6 pin enable"]
    #[inline(always)]
    pub fn y6en(&self) -> Y6enR {
        Y6enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PTC Y7 pin enable"]
    #[inline(always)]
    pub fn y7en(&self) -> Y7enR {
        Y7enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PTC Y8 pin enable"]
    #[inline(always)]
    pub fn y8en(&self) -> Y8enR {
        Y8enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PTC Y9 pin enable"]
    #[inline(always)]
    pub fn y9en(&self) -> Y9enR {
        Y9enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PTC Y10 pin enable"]
    #[inline(always)]
    pub fn y10en(&self) -> Y10enR {
        Y10enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PTC Y11 pin enable"]
    #[inline(always)]
    pub fn y11en(&self) -> Y11enR {
        Y11enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PTC Y12 pin enable"]
    #[inline(always)]
    pub fn y12en(&self) -> Y12enR {
        Y12enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PTC Y13 pin enable"]
    #[inline(always)]
    pub fn y13en(&self) -> Y13enR {
        Y13enR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PTC Y14 pin enable"]
    #[inline(always)]
    pub fn y14en(&self) -> Y14enR {
        Y14enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PTC Y15 pin enable"]
    #[inline(always)]
    pub fn y15en(&self) -> Y15enR {
        Y15enR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PTC Y0 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn y0en(&mut self) -> Y0enW<YselectenSpec> {
        Y0enW::new(self, 0)
    }
    #[doc = "Bit 1 - PTC Y1 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn y1en(&mut self) -> Y1enW<YselectenSpec> {
        Y1enW::new(self, 1)
    }
    #[doc = "Bit 2 - PTC Y2 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn y2en(&mut self) -> Y2enW<YselectenSpec> {
        Y2enW::new(self, 2)
    }
    #[doc = "Bit 3 - PTC Y3 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn y3en(&mut self) -> Y3enW<YselectenSpec> {
        Y3enW::new(self, 3)
    }
    #[doc = "Bit 4 - PTC Y4 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn y4en(&mut self) -> Y4enW<YselectenSpec> {
        Y4enW::new(self, 4)
    }
    #[doc = "Bit 5 - PTC Y5 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn y5en(&mut self) -> Y5enW<YselectenSpec> {
        Y5enW::new(self, 5)
    }
    #[doc = "Bit 6 - PTC Y6 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn y6en(&mut self) -> Y6enW<YselectenSpec> {
        Y6enW::new(self, 6)
    }
    #[doc = "Bit 7 - PTC Y7 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn y7en(&mut self) -> Y7enW<YselectenSpec> {
        Y7enW::new(self, 7)
    }
    #[doc = "Bit 8 - PTC Y8 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn y8en(&mut self) -> Y8enW<YselectenSpec> {
        Y8enW::new(self, 8)
    }
    #[doc = "Bit 9 - PTC Y9 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn y9en(&mut self) -> Y9enW<YselectenSpec> {
        Y9enW::new(self, 9)
    }
    #[doc = "Bit 10 - PTC Y10 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn y10en(&mut self) -> Y10enW<YselectenSpec> {
        Y10enW::new(self, 10)
    }
    #[doc = "Bit 11 - PTC Y11 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn y11en(&mut self) -> Y11enW<YselectenSpec> {
        Y11enW::new(self, 11)
    }
    #[doc = "Bit 12 - PTC Y12 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn y12en(&mut self) -> Y12enW<YselectenSpec> {
        Y12enW::new(self, 12)
    }
    #[doc = "Bit 13 - PTC Y13 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn y13en(&mut self) -> Y13enW<YselectenSpec> {
        Y13enW::new(self, 13)
    }
    #[doc = "Bit 14 - PTC Y14 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn y14en(&mut self) -> Y14enW<YselectenSpec> {
        Y14enW::new(self, 14)
    }
    #[doc = "Bit 15 - PTC Y15 pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn y15en(&mut self) -> Y15enW<YselectenSpec> {
        Y15enW::new(self, 15)
    }
}
#[doc = "Enable Y lines\n\nYou can [`read`](crate::Reg::read) this register and get [`yselecten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`yselecten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct YselectenSpec;
impl crate::RegisterSpec for YselectenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`yselecten::R`](R) reader structure"]
impl crate::Readable for YselectenSpec {}
#[doc = "`write(|w| ..)` method takes [`yselecten::W`](W) writer structure"]
impl crate::Writable for YselectenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets YSELECTEN to value 0"]
impl crate::Resettable for YselectenSpec {
    const RESET_VALUE: u16 = 0;
}
