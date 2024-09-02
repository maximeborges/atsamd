#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `CMP0` reader - Compare 0 Interrupt Enable"]
pub type Cmp0R = crate::BitReader;
#[doc = "Field `CMP0` writer - Compare 0 Interrupt Enable"]
pub type Cmp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1` reader - Compare 1 Interrupt Enable"]
pub type Cmp1R = crate::BitReader;
#[doc = "Field `CMP1` writer - Compare 1 Interrupt Enable"]
pub type Cmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCRDY` reader - Synchronization Ready Interrupt Enable"]
pub type SyncrdyR = crate::BitReader;
#[doc = "Field `SYNCRDY` writer - Synchronization Ready Interrupt Enable"]
pub type SyncrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF` reader - Overflow Interrupt Enable"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - Overflow Interrupt Enable"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Compare 0 Interrupt Enable"]
    #[inline(always)]
    pub fn cmp0(&self) -> Cmp0R {
        Cmp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub fn cmp1(&self) -> Cmp1R {
        Cmp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn syncrdy(&self) -> SyncrdyR {
        SyncrdyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0(&mut self) -> Cmp0W<IntenclrSpec> {
        Cmp0W::new(self, 0)
    }
    #[doc = "Bit 1 - Compare 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1(&mut self) -> Cmp1W<IntenclrSpec> {
        Cmp1W::new(self, 1)
    }
    #[doc = "Bit 6 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncrdy(&mut self) -> SyncrdyW<IntenclrSpec> {
        SyncrdyW::new(self, 6)
    }
    #[doc = "Bit 7 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OvfW<IntenclrSpec> {
        OvfW::new(self, 7)
    }
}
#[doc = "MODE1 Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u8 = 0;
}
